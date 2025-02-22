use crate::servers::factorio::FactorioConfig;
use crate::servers::StatusFetcher;
use crate::{AppError, AppResult};
use common::discord::RoleId;
use common::status::ServerStatus;
use notify::{EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use serde::Deserialize;
use smol_str::SmolStr;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::mpsc::error::TrySendError;
use tokio::sync::{RwLock, RwLockReadGuard};
use tokio::task::AbortHandle;

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub(crate) name: SmolStr,
    pub(crate) game: GameConfig,
    pub(crate) public_dns: SmolStr,
    pub(crate) required_role: Option<RoleId>,
}

pub(super) struct ConfigStore {
    configs: Arc<RwLock<Vec<ServerConfig>>>,
    load_task: AbortHandle,
    _watcher: RecommendedWatcher,
}

impl ConfigStore {
    pub fn new(config_path: PathBuf) -> AppResult<Self> {
        let configs = Arc::new(RwLock::new(Vec::new()));

        let (tx, mut rx) = tokio::sync::mpsc::channel(1);

        let mut watcher = RecommendedWatcher::new(
            move |res| {
                // Don't care if channel is full since we read the whole file again every change
                if let Err(TrySendError::Closed(_)) = tx.try_send(res) {
                    tracing::error!("File change missed: watcher shutdown");
                }
            },
            notify::Config::default(),
        )?;
        watcher.watch(&config_path, RecursiveMode::NonRecursive)?;

        let handle = {
            let configs = configs.clone();
            tokio::spawn(async move {
                Self::load_config_file(&config_path, configs.clone()).await;

                tracing::debug!("Initial config loaded. Waiting for file changes...");
                while let Some(res) = rx.recv().await {
                    tracing::debug!("Event received {:?}", res);
                    match res {
                        Ok(event) => {
                            if !matches!(event.kind, EventKind::Create(_) | EventKind::Modify(_)) {
                                // File contents didn't change, ignore this event
                                continue;
                            }

                            Self::load_config_file(&config_path, configs.clone()).await
                        }
                        Err(e) => {
                            tracing::warn!("watch error: {:?}", e);
                        }
                    }
                }

                tracing::error!("Watcher closed unexpectedly");
            })
        };

        Ok(Self {
            configs,
            load_task: handle.abort_handle(),
            _watcher: watcher,
        })
    }

    pub async fn configs(&self) -> RwLockReadGuard<'_, Vec<ServerConfig>> {
        self.configs.read().await
    }

    async fn load_config_file(config_path: &Path, configs: Arc<RwLock<Vec<ServerConfig>>>) {
        // TODO figure out a nice way to use tokio's File
        let config_file = File::open(config_path).expect("failed to open server config file");
        let reader = BufReader::new(config_file);

        match serde_json::from_reader(reader) {
            Ok(new_configs) => {
                tracing::info!("Loaded new servers config");
                tracing::debug!("{:?}", new_configs);
                *configs.write().await = new_configs;
            }
            Err(e) => {
                tracing::warn!("Error parsing config file: {}", e);
            }
        }
    }
}

impl Drop for ConfigStore {
    fn drop(&mut self) {
        tracing::debug!("Dropping config store");
        self.load_task.abort();
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize)]
pub enum GameConfig {
    Factorio(FactorioConfig),
}

impl Display for GameConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GameConfig::Factorio(_) => write!(f, "Factorio"),
        }
    }
}

impl StatusFetcher for GameConfig {
    async fn fetch_server_status(&self) -> Result<ServerStatus, AppError> {
        match self {
            GameConfig::Factorio(config) => config.fetch_server_status().await,
        }
    }
}
