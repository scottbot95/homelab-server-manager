use serde::Deserialize;
use smol_str::SmolStr;
use common::generic::GenericStatus;
use common::secret::Secret;
use common::status::HealthStatus;
use crate::servers::StatusFetcher;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize)]
pub struct GenericConfig {
    /// Name of the game, NOT the name of the server
    pub game_name: SmolStr,
    pub game_password: Secret,
}

impl StatusFetcher for GenericConfig {
    type Status = GenericStatus;

    async fn fetch_server_status(&self) -> GenericStatus {
        GenericStatus {
            name: SmolStr::default(), // Filled in later
            game_name: self.game_name.clone(),
            health: HealthStatus::Unknown,
            url: SmolStr::default(), // Filled in later
            game_password: self.game_password.secret().clone(),
        }
    }
}