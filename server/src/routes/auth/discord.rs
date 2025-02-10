use axum::extract::{Query, State};
use axum::response::{IntoResponse, Redirect};
use oauth2::{AuthorizationCode, CsrfToken, Scope, TokenResponse};
use anyhow::{anyhow, Context};
use tower_sessions::Session;
use crate::{AppError, User, UserData};
use crate::auth::{AuthRequest, OAuthClient, DiscordUserData, CSRF_TOKEN};

pub async fn discord_auth(
    State(client): State<OAuthClient>,
    session: Session,
) -> anyhow::Result<impl IntoResponse, AppError> {
    tracing::debug!("Logging in via discord");
    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("identify".to_string()))
        .add_scope(Scope::new("guilds.members.read".to_string()))
        .url();

    // Clear old session
    if session.id().is_some() {
        session.delete()
            .await
            .context("failed to delete old session")?;
    }

    // Save csrf_token
    session.insert(CSRF_TOKEN, &csrf_token)
        .await
        .context("failed in inserting CSRF token into session")?;

    Ok(Redirect::to(auth_url.as_ref()))
}

async fn csrf_token_validation_workflow(
    auth_request: &AuthRequest,
    session: &Session,
) -> anyhow::Result<(), AppError> {
    let stored_csrf_token = session.get::<CsrfToken>(CSRF_TOKEN)
        .await
        .context("failed to read CSRF token from session")?
        .context("Csrf token missing")?;

    // Cleanup the CSRF token session
    session.delete()
        .await
        .context("Failed to destroy old session")?;

    // Validate CSRF token is the same as the one in the auth request
    if *stored_csrf_token.secret() != auth_request.state {
        return Err(anyhow!("CSRF token mismatch").into());
    }

    Ok(())
}

pub async fn login_authorized(
    Query(query): Query<AuthRequest>,
    State(oauth_client): State<OAuthClient>,
    session: Session,
) -> anyhow::Result<impl IntoResponse, AppError> {
    csrf_token_validation_workflow(&query, &session).await?;

    let client = reqwest::Client::new();

    // Get an auth token
    let tokens = oauth_client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .request_async(&client)
        .await
        .context("failed in sending request request to authorization server")?;

    // Fetch user data from discord
    let discord_user = client
        // https://discord.com/developers/docs/resources/user#get-current-user
        .get("https://discordapp.com/api/users/@me")
        .bearer_auth(tokens.access_token().secret())
        .send()
        .await
        .context("failed in sending request to target Url")?
        .json::<DiscordUserData>()
        .await
        .context("failed to deserialize response as JSON")?;
    
    let user_data = UserData {
        discord_user,
        tokens
    };

    // Insert user data into session
    User::update_session(&session, &user_data).await?;

    Ok(Redirect::to("/"))
}

