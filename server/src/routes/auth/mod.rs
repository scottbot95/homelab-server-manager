mod discord;

use crate::AppState;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use http::StatusCode;

pub(super) fn make_auth_router() -> Router<AppState> {
    Router::new()
        .route("/discord", get(discord::discord_auth))
        .route("/discord/authorize", get(discord::login_authorized))
        .fallback(fallback)
}

async fn fallback() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Not Found")
}
