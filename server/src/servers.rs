use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use anyhow::Context;
use axum::extract::FromRef;
use moka::future::{Cache, CacheBuilder};
use oauth2::TokenResponse;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;
use tokio::net::TcpStream;
use common::discord::{RoleId, UserId};
use common::status::{HealthStatus, ServerStatus};
use crate::{AppError, AppResult, AppState, User};
use crate::auth::GuildMember;
use crate::servers::config::ConfigStore;
use crate::servers::factorio::FactorioConfig;

mod config;
mod factorio;

const GUILD_ID: u64 = 808535850030727198;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub enum GameConfig {
    Factorio(FactorioConfig)
}

impl Display for GameConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GameConfig::Factorio(_) => write!(f, "Factorio"),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerConfig {
    name: SmolStr,
    game: GameConfig,
    public_dns: SmolStr,
    required_role: Option<RoleId>,
}

#[derive(Clone)]
pub struct ServerManager {
    client: Client,
    config_store: Arc<ConfigStore>,
    statuses: Cache<GameConfig, ServerStatus>,
    user_roles: Cache<UserId, HashSet<RoleId>>,
}

impl ServerManager {
    pub fn new(client: Client, config_path: PathBuf) -> AppResult<Self> {
        Ok(Self {
            client,
            config_store: ConfigStore::new(config_path)?.into(),
            statuses: CacheBuilder::new(10)
                .time_to_live(Duration::from_secs(5))
                .build(),
            user_roles: CacheBuilder::new(20)
                .time_to_live(Duration::from_secs(10))
                .build()
        })
    }

    pub async fn get_servers_for_user(&self, user: &User) -> Result<Vec<ServerStatus>, AppError> {
        let roles = self.user_roles
            .get_with(user.discord_user.id, async move {
                self.fetch_user_roles(user)
                    .await
                    .unwrap_or_else(|err| {
                        tracing::error!("Failed to fetch user roles {}", err);
                        HashSet::with_capacity(0)
                    })
            })
            .await;

        Ok(self.get_servers(roles).await)
    }

    async fn get_servers(&self, roles: HashSet<RoleId>) -> Vec<ServerStatus> {
        let configs = self.config_store.configs().await;

        let futures = configs.iter()
            .filter_map(|c| {
                c.required_role
                    .filter(|r| roles.contains(r))
                    .map(|_| {
                        self.statuses.get_with_by_ref(&c.game, self.fetch_server_status(c))
                    })
            });

        futures::future::join_all(futures).await
    }

    async fn fetch_server_status(&self, config: &ServerConfig) -> ServerStatus {
        tracing::debug!("Updating server status: {:?}", config);
        let status = config.game
            .fetch_server_status()
            .await
            .map(|mut status| {
                status.name = config.name.clone();
                status
            });
        status.unwrap_or_else(|e| {
            tracing::error!("Failed fetching server status for {:?}: {}", config, e);
            ServerStatus {
                name: config.name.clone(),
                health: HealthStatus::Unknown,
            }
        })
    }

    async fn fetch_user_roles(&self, user: &User) -> AppResult<HashSet<RoleId>> {
        tracing::debug!("Updating user roles for {}", user.discord_user.username);
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

        Ok(roles)
    }
}

impl FromRef<AppState> for ServerManager {
    fn from_ref(input: &AppState) -> Self {
        input.server_manager.clone()
    }
}

trait StatusFetcher {
    async fn fetch_server_status(&self) -> Result<ServerStatus, AppError>;
}

impl StatusFetcher for GameConfig {
    async fn fetch_server_status(&self) -> Result<ServerStatus, AppError> {
        match self {
            GameConfig::Factorio(config) => config.fetch_server_status().await
        }
    }
}
