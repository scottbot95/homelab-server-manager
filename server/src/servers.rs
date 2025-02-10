use std::collections::HashSet;
use std::num::NonZeroU64;
use std::str::FromStr;
use anyhow::Context;
use axum::extract::FromRef;
use oauth2::TokenResponse;
use reqwest::Client;
use smol_str::SmolStr;
use common::discord::RoleId;
use common::status::{HealthStatus, ServerStatus};
use crate::{AppError, AppState, User};
use crate::auth::GuildMember;

const GUILD_ID: u64 = 808535850030727198;


struct Server {
    required_role: RoleId,
    status: ServerStatus,
}

static SERVERS: once_cell::sync::Lazy<Vec<Server>> = once_cell::sync::Lazy::new(|| vec![
    Server {
        required_role: RoleId::from_raw(1297802542461227069).unwrap(),
        status: ServerStatus {
            name: SmolStr::new_static("Factorio Space Age"),
            health: HealthStatus::Unknown,
        }
    },
    Server {
        required_role: RoleId::from_raw(1338296379638022224).unwrap(),
        status: ServerStatus {
            name: SmolStr::new_static("Factorio Cardinal"),
            health: HealthStatus::Unknown,
        }
    }
]);

#[derive(Clone)]
pub struct ServerManager {
    client: Client,
}

impl ServerManager {
    pub fn new(client: Client) -> Self {
        Self {
            client
        }
    }

    pub async fn get_servers_for_user(&self, user: &User) -> Result<Vec<ServerStatus>, AppError> {
        let resp = self.client
            // https://discord.com/developers/docs/resources/user#get-current-user-guild-member
            .get(format!("https://discordapp.com/api/users/@me/guilds/{GUILD_ID}/member"))
            .bearer_auth(user.tokens.access_token().secret())
            .send()
            .await
            .context("failed in sending request to target Url")?;

        let guild_member = resp
            .json::<GuildMember>()
            .await
            .context("failed to deserialize response as JSON")?;

        let roles = guild_member.roles.into_iter()
            .filter_map(|s| NonZeroU64::from_str(&s).map(From::from).ok())
            .collect();
        self.get_servers(roles).await
    }

    async fn get_servers(&self, roles: HashSet<RoleId>) -> Result<Vec<ServerStatus>, AppError> {
        // Fetch user data from discord
        let servers = SERVERS.iter()
            .filter_map(|s| roles.contains(&s.required_role).then_some(s.status.clone()))
            .collect();

        Ok(servers)
    }
}

impl FromRef<AppState> for ServerManager {
    fn from_ref(input: &AppState) -> Self {
        input.server_manager.clone()
    }
}
