mod auth;
mod routes;
mod servers;

use axum::extract::{FromRequestParts, OptionalFromRequestParts};
use http::request::Parts;
use http::StatusCode;
use std::convert::Infallible;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::sync::Arc;
use anyhow::Context;
use serde::{Deserialize, Serialize};
use axum::response::{IntoResponse, Response};
use oauth2::basic::BasicTokenResponse;
use tokio::signal;
use tokio::task::AbortHandle;
use tower_sessions::{ExpiredDeletion, Session};
use tower_sessions_sqlx_store::SqliteStore;
use tower_sessions_sqlx_store::sqlx::SqlitePool;
use auth::DiscordUserData;
use crate::auth::OAuthClient;
use crate::routes::make_router;
use crate::servers::ServerManager;

pub async fn run_server() -> Result<(), AppError> {
    // let session_store = MemoryStore::default();
    let pool = SqlitePool::connect("sqlite:sessions.db?mode=rwc").await?;
    let session_store = SqliteStore::new(pool);
    session_store.migrate()
        .await
        .context("Failed to migrate session store")?;

    let deletion_task = tokio::task::spawn(
        session_store.clone()
            .continuously_delete_expired(tokio::time::Duration::from_secs(10))
    );

    let app = make_router(false, session_store).await?;

    let listener = tokio::net::TcpListener::bind("localhost:9000")
        .await
        .context("failed to bind TcpListener")?;

    tracing::debug!(
        "listening on {}",
        listener
            .local_addr()
            .context("failed to return local address")?
    );

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(deletion_task.abort_handle()))
        .await?;

    match deletion_task.await {
        Ok(res) => res?,
        // task being cancelled is expected, don't count as a real error
        Err(err) if err.is_cancelled() => {},
        Err(err) => Err(err)?,
    }

    Ok(())
}

async fn shutdown_signal(deletion_task_abort_handle: AbortHandle) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => { deletion_task_abort_handle.abort() },
        _ = terminate => { deletion_task_abort_handle.abort() },
    }
}

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug)]
pub struct AppError(anyhow::Error);

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!("Application error: {}", self.0);

        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {:#}", self.0)
        ).into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

#[derive(Clone)]
struct AppState {
    oauth_client: OAuthClient,
    server_manager: ServerManager,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub discord_user: DiscordUserData,
    pub tokens: BasicTokenResponse,
}

pub struct User {
    session: Session,
    user_data: UserData,
}

impl User {
    const USER_DATA_KEY: &'static str = "user";

    pub fn username(&self) -> &str {
        &self.user_data.discord_user.username
    }

    async fn update_session(session: &Session, data: &UserData) -> Result<(), AppError> {
        session
            .insert(Self::USER_DATA_KEY, data)
            .await
            .context("failed to insert user data")
            .map_err(From::from)
    }
}

impl Deref for User {
    type Target = UserData;

    fn deref(&self) -> &Self::Target {
        &self.user_data
    }
}

impl<S> FromRequestParts<S> for User
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let session = Session::from_request_parts(parts, state)
            .await
            .map_err(IntoResponse::into_response)?;

        let user_data = session
            .get::<UserData>(Self::USER_DATA_KEY)
            .await
            .expect("Failed to read session")
            .ok_or_else(|| StatusCode::UNAUTHORIZED.into_response())?;

        // Uncomment if we add something like a last seen time
        // Self::update_session(&session, &user_data)
        //     .await
        //     .map_err(IntoResponse::into_response)?;

        Ok(Self {
            session,
            user_data
        })
    }
}

impl<S> OptionalFromRequestParts<S> for User
where
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> anyhow::Result<Option<Self>, Self::Rejection> {
        match <User as FromRequestParts<S>>::from_request_parts(parts, state).await {
            Ok(res) => Ok(Some(res)),
            Err(_) => Ok(None),
        }
    }
}