//! Example OAuth (Discord) implementation.
//!
//! 1) Create a new application at <https://discord.com/developers/applications>
//! 2) Visit the OAuth2 tab to get your CLIENT_ID and CLIENT_SECRET
//! 3) Add a new redirect URI (for this example: `http://localhost:9000/api/oauth/discord-callback`)
//! 4) Run with the following (replacing values appropriately):
//! ```not_rust
//! CLIENT_ID=REPLACE_ME CLIENT_SECRET=REPLACE_ME cargo run -p example-oauth
//! ```


use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use server::run_server;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer()
            .with_file(true)
            .with_line_number(true)
        )
        .init();

    run_server().await;
}
