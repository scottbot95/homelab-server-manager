use anyhow::Context;
use async_session::{MemoryStore, SessionStore};
use axum::extract::State;
use axum::response::{IntoResponse, Redirect};
use axum::Router;
use axum::routing::get;
use axum_extra::extract::CookieJar;
use crate::{AppError, AppState, User};
use crate::auth::COOKIE_NAME;
use crate::routes::auth::make_auth_router;

mod auth;

pub fn make_router() -> Router {
    // `MemoryStore` is just used as an example. Don't use this in production.
    let store = MemoryStore::new();
    let oauth_client = crate::auth::oauth_client().unwrap();
    let app_state = AppState {
        store,
        oauth_client,
    };

    Router::new()
        .route("/", get(index))
        .route("/protected", get(protected))
        .route("/logout", get(logout))
        .nest("/auth", make_auth_router())
        .with_state(app_state)
}

// Session is optional
async fn index(user: Option<User>) -> impl IntoResponse {
    match user {
        Some(u) => format!(
            "Hey {}! You're logged in!\nYou may now access `/protected`.\nLog out with `/logout`.",
            u.username
        ),
        None => "You're not logged in.\nVisit `/auth/discord` to do so.".to_string(),
    }
}

async fn protected(user: User) -> impl IntoResponse {
    format!("Welcome to the protected area :)\nHere's your info:\n{user:?}")
}

pub async fn logout(
    State(store): State<MemoryStore>,
    cookie_jar: CookieJar,
) -> anyhow::Result<impl IntoResponse, AppError> {
    let cookie = cookie_jar
        .get(COOKIE_NAME)
        .context("unexpected error getting cookie name")?;

    let session = match store
        .load_session(cookie.to_string())
        .await
        .context("failed to load session")?
    {
        Some(s) => s,
        // No session active, just redirect
        None => return Ok((cookie_jar, Redirect::to("/"))),
    };

    store
        .destroy_session(session)
        .await
        .context("failed to destroy session")?;
    
    Ok((
        cookie_jar.remove(COOKIE_NAME),
        Redirect::to("/")
    ))
}
