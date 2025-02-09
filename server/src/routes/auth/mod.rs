mod discord;

use axum::response::IntoResponse;
use axum::Router;
use axum::routing::get;
use http::StatusCode;
use crate::AppState;

pub(super) fn make_auth_router() -> Router<AppState> {
    Router::new()
        .route("/discord", get(discord::discord_auth))
        .route("/discord/authorize", get(discord::login_authorized))
        .fallback(fallback)
}

async fn fallback() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Not Found")
}
