use axum::response::IntoResponse;
use axum::{Json, Router};
use axum::routing::get;
use common::user::UserData;
use crate::{AppState, User};

pub(super) fn make_api_router() -> Router<AppState> {
    Router::new()
        .route("/me", get(get_user_data))
}

async fn get_user_data(user: Option<User>) -> impl IntoResponse {
    let data = user.map(|u| UserData {
        name: u.user_data.username.into(),
        roles: Vec::new(),
    });

    Json(data)
}