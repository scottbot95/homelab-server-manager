use std::path::Path;
use anyhow::Context;
use axum::response::{IntoResponse, Redirect};
use axum::Router;
use axum::routing::get;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;
use tower_sessions::{Expiry, MemoryStore, Session, SessionManagerLayer};
use tower_sessions::cookie::SameSite;
use tower_sessions::cookie::time::Duration;
use crate::{AppError, AppState, User};
use crate::routes::api::make_api_router;
use crate::routes::auth::make_auth_router;

mod auth;
mod api;

pub fn make_router(secure: bool) -> Router {
    // `MemoryStore` is just used as an example. Don't use this in production.
    let oauth_client = crate::auth::oauth_client().unwrap();
    let app_state = AppState {
        oauth_client,
    };

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(secure)
        .with_same_site(SameSite::Lax)
        .with_expiry(Expiry::OnInactivity(Duration::seconds(30)));

    let static_dir = Path::new("./dist");

    Router::new()
        // .route("/", get(index))
        .route("/protected", get(protected))
        .route("/logout", get(logout))
        .nest("/auth", make_auth_router())
        .nest("/api", make_api_router())
        .nest_service("/assets", ServeDir::new(static_dir))
        .fallback_service(ServeFile::new(static_dir.join("index.html")))
        .with_state(app_state)
        .layer(session_layer)
        .layer(TraceLayer::new_for_http())
}

async fn protected(user: User) -> impl IntoResponse {
    format!("Welcome to the protected area :)\nHere's your info:\n{:?}", &*user)
}

pub async fn logout(
    session: Session,
) -> anyhow::Result<impl IntoResponse, AppError> {
    session.delete()
        .await
        .context("failed to destroy session")?;
    
    Ok(Redirect::to("/"))
}
