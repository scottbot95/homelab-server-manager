use axum::response::{IntoResponse, Redirect, Response};
use serde::Deserialize;
use oauth2::basic::{BasicClient, BasicErrorResponse, BasicRevocationErrorResponse, BasicTokenIntrospectionResponse, BasicTokenResponse};
use oauth2::{AuthUrl, ClientId, ClientSecret, EndpointNotSet, EndpointSet, RedirectUrl, StandardRevocableToken, TokenUrl};
use std::env;
use axum::extract::FromRef;
use anyhow::Context;
use crate::{AppError, AppState};

pub static CSRF_TOKEN: &str = "csrf_token";

pub type OAuthClient = oauth2::Client<
    BasicErrorResponse,
    BasicTokenResponse,
    BasicTokenIntrospectionResponse,
    StandardRevocableToken,
    BasicRevocationErrorResponse,
    EndpointSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointSet
>;

impl FromRef<AppState> for OAuthClient {
    fn from_ref(state: &AppState) -> Self {
        state.oauth_client.clone()
    }
}

pub fn oauth_client() -> anyhow::Result<OAuthClient, AppError> {
    let client_id = env::var("DISCORD_CLIENT_ID").context("Missing CLIENT_ID!")?;
    let client_secret = env::var("DISCORD_CLIENT_SECRET").context("Missing CLIENT_SECRET!")?;
    let host_url = env::var("HOST_URL")
        .unwrap_or_else(|_| "http://localhost:9000".to_string());
    let redirect_url = format!("{}/auth/discord/authorize", host_url);

    let auth_url = env::var("AUTH_URL").unwrap_or_else(|_| {
        "https://discord.com/api/oauth2/authorize".to_string()
    });

    let token_url = env::var("TOKEN_URL")
        .unwrap_or_else(|_| "https://discord.com/api/oauth2/token".to_string());

    Ok(BasicClient::new(ClientId::new(client_id))
        .set_client_secret(ClientSecret::new(client_secret))
        .set_auth_uri(
            AuthUrl::new(auth_url).context("failed to create new authorization server URL")?
        )
        .set_token_uri(
            TokenUrl::new(token_url).context("failed to create new token endpoint URL")?
        )
        .set_redirect_uri(
            RedirectUrl::new(redirect_url).context("failed to create new redirection URL")?,
        ))
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct AuthRequest {
    pub code: String,
    pub state: String,
}

pub struct AuthRedirect;

impl IntoResponse for AuthRedirect {
    fn into_response(self) -> Response {
        Redirect::temporary("/auth/discord").into_response()
    }
}
