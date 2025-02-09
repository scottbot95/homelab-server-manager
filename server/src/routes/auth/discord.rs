use axum::extract::{Query, State};
use async_session::{MemoryStore, Session, SessionStore};
use axum::response::{IntoResponse, Redirect};
use oauth2::{AuthorizationCode, CsrfToken, Scope, TokenResponse};
use anyhow::{anyhow, Context};
use axum_extra::extract::cookie::{Cookie, SameSite};
use axum_extra::extract::CookieJar;
use crate::{AppError, User};
use crate::auth::{AuthRequest, OAuthClient, COOKIE_NAME, CSRF_TOKEN};

pub async fn discord_auth(
    State(client): State<OAuthClient>,
    State(store): State<MemoryStore>,
    cookie_jar: CookieJar
) -> anyhow::Result<impl IntoResponse, AppError> {
    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("identify".to_string()))
        .add_scope(Scope::new("guilds.member.read".to_string()))
        .url();

    // Create session to store csrf_token
    let mut session = Session::new();
    session
        .insert(CSRF_TOKEN, &csrf_token)
        .context("failed in inserting CSRF token into session")?;

    // Store the session in MemoryStore and retrieve the session cookie
    let cookie_value = store
        .store_session(session)
        .await
        .context("failed to store CSRF token session")?
        .context("unexpected error retrieving CSRF cookie value")?;

    // Create the cookie
    let mut cookie = Cookie::new(COOKIE_NAME, cookie_value);
    cookie.set_same_site(Some(SameSite::Lax));
    cookie.set_http_only(true);
    cookie.set_secure(true);
    cookie.set_path("/");

    let cookie_jar = cookie_jar.add(cookie);
    Ok((cookie_jar, Redirect::to(auth_url.as_ref())))
}

async fn csrf_token_validation_workflow(
    auth_request: &AuthRequest,
    cookie_jar: &CookieJar,
    store: &MemoryStore,
) -> anyhow::Result<(), AppError> {
    // Extract the cookie from the request
    let cookie = cookie_jar
        .get(COOKIE_NAME)
        .context("unexpected error getting cookie name")?
        .value();

    // Load the session
    let session = match store
        .load_session(cookie.to_owned())
        .await
        .context("failed to load session")?
    {
        Some(session) => session,
        None => return Err(anyhow!("Session not found").into()),
    };

    // Extract the CSRF token from the session
    let stored_csrf_token = session
        .get::<CsrfToken>(CSRF_TOKEN)
        .context("CSRF token not found in session")?
        .to_owned();

    // Cleanup the CSRF token session
    store
        .destroy_session(session)
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
    State(store): State<MemoryStore>,
    State(oauth_client): State<OAuthClient>,
    cookie_jar: CookieJar,
) -> anyhow::Result<impl IntoResponse, AppError> {
    csrf_token_validation_workflow(&query, &cookie_jar, &store).await?;

    let client = reqwest::Client::new();

    // Get an auth token
    let token = oauth_client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .request_async(&client)
        .await
        .context("failed in sending request request to authorization server")?;

    // Fetch user data from discord
    let user_data: User = client
        // https://discord.com/developers/docs/resources/user#get-current-user
        .get("https://discordapp.com/api/users/@me")
        .bearer_auth(token.access_token().secret())
        .send()
        .await
        .context("failed in sending request to target Url")?
        .json::<User>()
        .await
        .context("failed to deserialize response as JSON")?;

    // Create a new session filled with user data
    let mut session = Session::new();
    session
        .insert("user", &user_data)
        .context("failed in inserting serialized value into session")?;

    // Store session and get corresponding cookie
    let cookie_value = store
        .store_session(session)
        .await
        .context("failed to store session")?
        .context("unexpected error retrieving cookie value")?;

    // Build the cookie
    // Create the cookie
    let mut cookie = Cookie::new(COOKIE_NAME, cookie_value);
    cookie.set_same_site(Some(SameSite::Lax));
    cookie.set_http_only(true);
    cookie.set_secure(true);
    cookie.set_path("/");

    Ok((
        cookie_jar.add(cookie),
        Redirect::to("/")
    ))
}

