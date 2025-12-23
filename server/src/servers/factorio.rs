use crate::servers::StatusFetcher;
use crate::AppResult;
use anyhow::anyhow;
use common::factorio::FactorioStatus;
use common::secret::Secret;
use common::status::HealthStatus;
use moka::future::Cache;
use once_cell::sync::Lazy;
use rcon::Connection;
use serde::Deserialize;
use smol_str::SmolStr;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::sync::Mutex;

static CLIENTS: Lazy<Cache<FactorioConfig, Arc<Mutex<Connection<TcpStream>>>>> = Lazy::new(|| {
    Cache::builder()
        .max_capacity(10)
        .time_to_idle(Duration::from_secs(5 * 60))
        .build()
});

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize)]
pub struct FactorioConfig {
    pub rcon_host: SmolStr,
    pub rcon_password: Secret,
    pub game_password: Secret,
}

impl FactorioConfig {
    async fn connect(&self) -> AppResult<Arc<Mutex<Connection<TcpStream>>>> {
        let conn = Connection::<TcpStream>::builder()
            .enable_factorio_quirks(true)
            .connect(&*self.rcon_host, self.rcon_password.secret())
            .await?;

        Ok(Arc::new(Mutex::new(conn)))
    }
}

const UNKNOWN_TEXT: SmolStr = SmolStr::new_static("unknown");

impl FactorioConfig {
    async fn populate_status(&self, status: &mut FactorioStatus) -> AppResult<()> {
        let mutex = CLIENTS
            .try_get_with_by_ref(self, self.connect())
            .await
            .map_err(|err| Arc::try_unwrap(err).unwrap_or_else(|e| anyhow!("{e}").into()))?;

        let mut conn = mutex.lock().await;

        let players_text = conn.cmd("/players o").await?;
        status.players_online = players_text
            .trim()
            .split("\n")
            .skip(1)
            .map(|line| line.trim().split(' ').next().unwrap().into())
            .collect();

        status.game_time = conn.cmd("/time").await?.into();
        status.game_version = conn.cmd("/version").await?.into();

        Ok(())
    }
}

impl StatusFetcher for FactorioConfig {
    type Status = FactorioStatus;

    async fn fetch_server_status(&self) -> FactorioStatus {
        let mut status = FactorioStatus {
            name: self.rcon_host.clone(),
            health: HealthStatus::Unknown,
            game_password: self.game_password.secret().clone(),
            url: SmolStr::default(),
            players_online: Vec::new(),
            game_time: UNKNOWN_TEXT,
            game_version: UNKNOWN_TEXT,
        };

        if let Err(e) = self.populate_status(&mut status).await {
            tracing::error!("Failed to fetch server status: {}", e);
        }

        status
    }
}
