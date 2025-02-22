use crate::{AppError, AppState, Server};
use anyhow::Context;
use axum::extract::FromRef;
use axum::response::{IntoResponse, Redirect, Response};
use common::discord::{RoleId, UserId};
use oauth2::basic::{
    BasicClient, BasicErrorResponse, BasicRevocationErrorResponse, BasicTokenIntrospectionResponse,
    BasicTokenResponse,
};
use oauth2::{
    AuthUrl, ClientId, ClientSecret, EndpointNotSet, EndpointSet, RedirectUrl,
    StandardRevocableToken, TokenUrl,
};
use serde::{Deserialize, Serialize};
use std::env;

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
    EndpointSet,
>;

impl FromRef<AppState> for OAuthClient {
    fn from_ref(state: &AppState) -> Self {
        state.oauth_client.clone()
    }
}

pub fn oauth_client(server: &Server) -> anyhow::Result<OAuthClient, AppError> {
    let client_id = env::var("DISCORD_CLIENT_ID").context("Missing CLIENT_ID!")?;
    let client_secret = env::var("DISCORD_CLIENT_SECRET").context("Missing CLIENT_SECRET!")?;
    let redirect_url = format!("{}/auth/discord/authorize", &server.public_url);

    let auth_url = env::var("AUTH_URL")
        .unwrap_or_else(|_| "https://discord.com/api/oauth2/authorize".to_string());

    let token_url = env::var("TOKEN_URL")
        .unwrap_or_else(|_| "https://discord.com/api/oauth2/token".to_string());

    Ok(BasicClient::new(ClientId::new(client_id))
        .set_client_secret(ClientSecret::new(client_secret))
        .set_auth_uri(
            AuthUrl::new(auth_url).context("failed to create new authorization server URL")?,
        )
        .set_token_uri(TokenUrl::new(token_url).context("failed to create new token endpoint URL")?)
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

// The user data we'll get back from Discord.
// https://discord.com/developers/docs/resources/user#user-object-user-structure
#[derive(Debug, Serialize, Deserialize)]
pub struct DiscordUserData {
    pub id: UserId,
    pub avatar: Option<String>,
    pub username: String,
    pub discriminator: String,
}

/// The guild member data we'll get back from Discord.
/// https://discord.com/developers/docs/resources/guild#guild-member-object
#[derive(Debug, Serialize, Deserialize)]
pub struct GuildMember {
    pub user: DiscordUserData,
    pub nick: Option<String>,
    pub roles: Vec<RoleId>,
}
