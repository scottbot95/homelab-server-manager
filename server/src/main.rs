//! Example OAuth (Discord) implementation.
//!
//! 1) Create a new application at <https://discord.com/developers/applications>
//! 2) Visit the OAuth2 tab to get your CLIENT_ID and CLIENT_SECRET
//! 3) Add a new redirect URI (for this example: `http://localhost:9000/api/oauth/discord-callback`)
//! 4) Run with the following (replacing values appropriately):
//! ```not_rust
//! CLIENT_ID=REPLACE_ME CLIENT_SECRET=REPLACE_ME cargo run -p example-oauth
//! ```

use std::net::IpAddr;
use std::path::PathBuf;
use clap::Parser;
use server::AppError;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser, Debug)]
struct Args {
    /// Address to bind
    #[arg(short, long, default_value = "0.0.0.0")]
    addr: IpAddr,

    /// Port to bind
    #[arg(short, long, default_value_t = 9000)]
    port: u16,

    /// Path to the config file
    #[arg(short, long, default_value = "./config.json")]
    config_file: PathBuf,

    /// Whether service is behind a TLS proxy
    #[arg(short, long, default_value_t = false)]
    secure: bool,
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let args = Args::parse();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_file(true)
                .with_line_number(true),
        )
        .init();

    let server = server::Server {
        bind: (args.addr, args.port).into(),
        config_path: args.config_file,
        secure: args.secure,
    };

    server.run_server().await?;

    Ok(())
}
