use serde::{Deserialize, Serialize};
use smol_str::SmolStr;
use crate::status::HealthStatus;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenericStatus {
    pub name: SmolStr,
    pub game_name: SmolStr,
    pub health: HealthStatus,
    pub url: SmolStr,
    pub game_password: SmolStr,
}
