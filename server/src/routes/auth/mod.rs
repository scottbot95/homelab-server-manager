mod discord;

use axum::Router;
use axum::routing::get;
use crate::AppState;

pub(super) fn make_auth_router() -> Router<AppState> {
    Router::new()
        .route("/discord", get(discord::discord_auth))
        .route("/discord/authorized", get(discord::login_authorized))
}
