use clap::Parser;
use reqwest::Url;
use server::AppError;
use std::net::IpAddr;
use std::path::PathBuf;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser, Debug)]
struct Args {
    /// Address to bind
    #[arg(long, default_value = "0.0.0.0")]
    addr: IpAddr,

    /// Port to bind
    #[arg(short, long, default_value_t = 9000)]
    port: u16,

    /// Path to the config file
    #[arg(short, long, default_value = "./config.json")]
    config_file: PathBuf,

    /// Public URL. Mainly used for oauth2 redirects
    #[arg(long, default_value = "http://localhost:9000")]
    public_url: Url,
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
                .with_file(cfg!(debug_assertions))
                .with_line_number(cfg!(debug_assertions)),
        )
        .init();

    let server = server::Server {
        bind: (args.addr, args.port).into(),
        config_path: args.config_file,
        public_url: args.public_url,
    };

    server.run_server().await?;

    Ok(())
}
