use std::str::FromStr;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use actix_web::web::Query;
use oauth2::basic::{BasicClient, BasicErrorResponse, BasicRevocationErrorResponse, BasicTokenIntrospectionResponse, BasicTokenResponse};
use oauth2::{reqwest, AuthUrl, AuthorizationCode, Client, ClientId, ClientSecret, EndpointNotSet, EndpointSet, RedirectUrl, StandardRevocableToken, TokenResponse, TokenUrl};
use once_cell::sync::Lazy;
use serde::Deserialize;
use serenity::all::{ApplicationId, GuildId, Scope};
use serenity::builder::CreateBotAuthParameters;

static CLIENT_ID: Lazy<ClientId> = Lazy::new(|| {
    let var = std::env::var("DISCORD_CLIENT_ID")
        .expect("DISCORD_CLIENT_ID env var not set");

    ClientId::new(var)
});

static DISCORD_AUTHORIZE_URL: Lazy<String> = Lazy::new(|| {
    let redirect_uri = std::env::var("DISCORD_REDIRECT_URI")
        .unwrap_or_else(|_| "http://localhost:9000/api/oauth/discord-callback".to_string());

    let base = CreateBotAuthParameters::new()
        .client_id(ApplicationId::from_str(&*CLIENT_ID).expect("CLIENT_ID could not be parsed"))
        .guild_id(808535850030727198)
        .disable_guild_select(true)
        .scopes(&[Scope::GuildsMembersRead])
        .build();


    base
        + "&response_type=code"
        + "&redirect_uri=" + &*urlencoding::encode(&redirect_uri)
});

type OauthClient = Client<BasicErrorResponse, BasicTokenResponse, BasicTokenIntrospectionResponse, StandardRevocableToken, BasicRevocationErrorResponse, EndpointSet, EndpointNotSet, EndpointNotSet, EndpointNotSet, EndpointSet>;
static OAUTH_CLIENT: Lazy<OauthClient> = Lazy::new(|| {
    BasicClient::new(CLIENT_ID.clone())
        .set_client_secret(ClientSecret::new(std::env::var("DISCORD_CLIENT_SECRET").unwrap()))
        .set_auth_uri(AuthUrl::new("https://discord.com/oauth2/authorize".to_owned()).unwrap())
        .set_token_uri(TokenUrl::new("https://discord.com/api/oauth2/token".to_owned()).unwrap())
        .set_redirect_uri(RedirectUrl::new("http://localhost:9000/api/oauth/discord-callback".to_owned()).unwrap())
});

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[get("/login")]
async fn login() -> impl Responder {
    HttpResponse::Found()
        .append_header(("Location", DISCORD_AUTHORIZE_URL.clone()))
        .finish()
}

#[derive(Deserialize)]
struct OauthCallback {
    code: String,
}

#[get("/api/oauth/discord-callback")]
async fn discord_callback(Query(OauthCallback {code}): Query<OauthCallback>) -> impl Responder {
    let http_client = reqwest::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Client should build");
    let request = OAUTH_CLIENT
        .exchange_code(AuthorizationCode::new(code));

    tracing::info!("{:?}", request);

    let token = request
        .request_async(&http_client)
        .await?;

    let discord_client = serenity::http::Http::new(
        &format!("Bearer {}", token.access_token().secret())
    );

    let member = discord_client
        .get_current_user_guild_member(GuildId::from(808535850030727198))
        .await?;

    Ok::<HttpResponse, Box<dyn std::error::Error>>(HttpResponse::Ok().json(member))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    let server = HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(login)
            .service(discord_callback)
    })
        .bind(("127.0.0.1", 9000))?
        .workers(2)
        .run();

    server.await
}