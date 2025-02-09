mod auth;
mod routes;

use axum::extract::{FromRequestParts, OptionalFromRequestParts};
use http::request::Parts;
use http::StatusCode;
use std::convert::Infallible;
use std::ops::Deref;
use anyhow::Context;
use serde::{Deserialize, Serialize};
use axum::response::{IntoResponse, Response};
use tower_sessions::Session;
use crate::auth::{AuthRedirect, OAuthClient};
use crate::routes::make_router;

pub async fn run_server() {
    let app = make_router(false);

    let listener = tokio::net::TcpListener::bind("localhost:9000")
        .await
        .context("failed to bind TcpListener")
        .unwrap();

    tracing::debug!(
        "listening on {}",
        listener
            .local_addr()
            .context("failed to return local address")
            .unwrap()
    );

    axum::serve(listener, app).await.unwrap();
}


#[derive(Debug)]
struct AppError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!("Application error: {:#}", self.0);

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
}

// The user data we'll get back from Discord.
// https://discord.com/developers/docs/resources/user#user-object-user-structure
#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub id: String,
    pub avatar: Option<String>,
    pub username: String,
    pub discriminator: String,
}

pub struct User {
    session: Session,
    user_data: UserData,
}

impl User {
    const USER_DATA_KEY: &'static str = "user";

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
            .ok_or_else(|| AuthRedirect.into_response())?;

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