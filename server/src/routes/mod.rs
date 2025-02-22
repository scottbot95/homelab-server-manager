use crate::routes::api::make_api_router;
use crate::routes::auth::make_auth_router;
use crate::servers::ServerManager;
use crate::{AppError, AppResult, AppState, Server, User};
use anyhow::Context;
use axum::response::{IntoResponse, Redirect};
use axum::routing::get;
use axum::Router;
use reqwest::Client;
use std::path::Path;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;
use tower_sessions::cookie::time::Duration;
use tower_sessions::cookie::SameSite;
use tower_sessions::{Expiry, Session, SessionManagerLayer, SessionStore};

mod api;
mod auth;

pub async fn make_router<Store: SessionStore + Clone>(
    server: &Server,
    session_store: Store,
) -> AppResult<Router> {
    // `MemoryStore` is just used as an example. Don't use this in production.
    let oauth_client = crate::auth::oauth_client(server)?;
    let app_state = AppState {
        oauth_client,
        server_manager: ServerManager::new(Client::new(), server.config_path.clone())?,
    };

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(server.public_url.scheme() == "https")
        .with_same_site(SameSite::Lax)
        .with_expiry(Expiry::OnInactivity(Duration::days(1)));

    let static_dir = match option_env!("FRONTEND_DIST") {
        Some(path) => Path::new(path),
        None => Path::new("./dist"),
    };

    Ok(Router::new()
        // .route("/", get(index))
        .route("/protected", get(protected))
        .route("/logout", get(logout))
        .nest("/auth", make_auth_router())
        .nest("/api", make_api_router())
        .nest_service("/assets", ServeDir::new(static_dir))
        .fallback_service(ServeFile::new(static_dir.join("index.html")))
        .with_state(app_state)
        .layer(session_layer)
        .layer(TraceLayer::new_for_http()))
}

async fn protected(user: User) -> impl IntoResponse {
    format!(
        "Welcome to the protected area :)\nHere's your info:\n{:?}",
        &*user
    )
}

pub async fn logout(session: Session) -> anyhow::Result<impl IntoResponse, AppError> {
    session
        .delete()
        .await
        .context("failed to destroy session")?;

    Ok(Redirect::to("/"))
}
