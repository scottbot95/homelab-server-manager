use crate::status::HealthStatus;
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FactorioStatus {
    pub name: SmolStr,
    pub health: HealthStatus,
    pub url: SmolStr,
    pub game_password: SmolStr,
    pub players_online: Vec<SmolStr>,
    pub game_time: SmolStr,
    pub game_version: SmolStr,
}
