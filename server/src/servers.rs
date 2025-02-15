use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::sync::Arc;
use anyhow::Context;
use axum::extract::FromRef;
use oauth2::TokenResponse;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;
use common::discord::RoleId;
use common::status::{HealthStatus, ServerStatus};
use crate::{AppError, AppResult, AppState, User};
use crate::auth::GuildMember;
use crate::servers::config::ConfigStore;

mod config;

const GUILD_ID: u64 = 808535850030727198;

#[derive(Debug, Deserialize, Serialize)]
pub enum GameKind {
    Factorio
}

impl Display for GameKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GameKind::Factorio => write!(f, "Factorio"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerConfig {
    game: GameKind,
    host: SmolStr,
    required_role: Option<RoleId>,
}

#[derive(Clone)]
pub struct ServerManager {
    client: Client,
    config_store: Arc<ConfigStore>,
}

impl ServerManager {
    pub fn new(client: Client, config_path: PathBuf) -> AppResult<Self> {
        Ok(Self {
            client,
            config_store: ConfigStore::new(config_path)?.into(),
        })
    }

    pub async fn get_servers_for_user(&self, user: &User) -> Result<Vec<ServerStatus>, AppError> {
        let resp = self.client
            // https://discord.com/developers/docs/resources/user#get-current-user-guild-member
            .get(format!("https://discordapp.com/api/users/@me/guilds/{GUILD_ID}/member"))
            .bearer_auth(user.tokens.access_token().secret())
            .send()
            .await
            .context("failed in sending request to target Url")?;

        let body = resp.text().await?;
        tracing::trace!("Discord response: {}", body);

        let guild_member = serde_json::from_str::<GuildMember>(&body)
            .context("failed to deserialize response as JSON")?;

        let roles = guild_member.roles.into_iter()
            .collect();

        Ok(self.get_servers(roles).await)
    }

    async fn get_servers(&self, roles: HashSet<RoleId>) -> Vec<ServerStatus> {
        let configs = self.config_store.configs().await;

        let futures = configs.iter()
            .filter_map(|c| {
                c.required_role
                    .filter(|r| roles.contains(r))
                    .map(|_| self.fetch_server_status(c))
            });

        futures::future::join_all(futures).await
    }

    async fn fetch_server_status(&self, config: &ServerConfig) -> ServerStatus {
        let status = config.game.fetch_server_status(&config.host).await;
        status.unwrap_or_else(|e| {
            tracing::error!("Failed fetching server status for {:?}: {}", config, e);
            ServerStatus {
                name: format!("Unknown server {}", &config.game).into(),
                health: HealthStatus::Unknown,
            }
        })
    }
}

impl FromRef<AppState> for ServerManager {
    fn from_ref(input: &AppState) -> Self {
        input.server_manager.clone()
    }
}

trait StatusFetcher {
    async fn fetch_server_status(&self, host: &str) -> Result<ServerStatus, AppError>;
}

impl StatusFetcher for GameKind {
    async fn fetch_server_status(&self, host: &str) -> Result<ServerStatus, AppError> {
        match self {
            GameKind::Factorio => {
                // FIXME actually fetch status
                Ok(ServerStatus {
                    name: host.into(),
                    health: HealthStatus::Unknown,
                })
            }
        }
    }
}
