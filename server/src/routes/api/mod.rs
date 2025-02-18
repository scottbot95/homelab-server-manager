mod servers;

use crate::routes::api::servers::get_servers;
use crate::{AppError, AppState, User};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use common::user::UserData;

pub(super) fn make_api_router() -> Router<AppState> {
    Router::new()
        .route("/me", get(get_user_data))
        .route("/servers/status", get(get_servers))
}

async fn get_user_data(user: Option<User>) -> anyhow::Result<impl IntoResponse, AppError> {
    let Some(user) = user else {
        return Ok(Json(None));
    };

    let data = UserData {
        name: user.user_data.discord_user.username.into(),
    };

    Ok(Json(Some(data)))
}
