use std::sync::Arc;
use std::time::Duration;
use anyhow::anyhow;
use moka::future::Cache;
use once_cell::sync::Lazy;
use rcon::Connection;
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use common::status::{HealthStatus, ServerStatus};
use crate::AppResult;
use crate::servers::StatusFetcher;

static CLIENTS: Lazy<Cache<FactorioConfig, Arc<Mutex<Connection<TcpStream>>>>> = Lazy::new(|| {
    Cache::builder()
        .max_capacity(10)
        .time_to_idle(Duration::from_secs(5 * 60))
        .build()
});

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct FactorioConfig {
    pub rcon_host: SmolStr,
    pub rcon_password: SmolStr,
}

impl FactorioConfig {
    async fn connect(&self) -> AppResult<Arc<Mutex<Connection<TcpStream>>>> {
        let conn =Connection::<TcpStream>::builder()
            .enable_factorio_quirks(true)
            .connect(&*self.rcon_host, &self.rcon_password)
            .await?;

        Ok(Arc::new(Mutex::new(conn)))
    }
}

impl StatusFetcher for FactorioConfig {
    async fn fetch_server_status(&self) -> AppResult<ServerStatus> {
        let mutex = CLIENTS
            .try_get_with_by_ref(self, self.connect())
            .await
            .map_err(|err|
                Arc::try_unwrap(err)
                    .unwrap_or_else(|e| anyhow!("{e}").into())
            )?;

        let mut conn = mutex.lock().await;

        // TODO parse online players
        let _players_text = conn.cmd("/players o").await?;

        Ok(ServerStatus {
            name: self.rcon_host.clone(),
            health: HealthStatus::Running,
        })
    }
}