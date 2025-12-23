use crate::factorio::FactorioStatus;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use crate::generic::GenericStatus;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ServerStatus {
    Factorio(FactorioStatus),
    Generic(GenericStatus),
}

impl From<FactorioStatus> for ServerStatus {
    fn from(value: FactorioStatus) -> Self {
        Self::Factorio(value)
    }
}

impl From<GenericStatus> for ServerStatus {
    fn from(value: GenericStatus) -> Self {
        Self::Generic(value)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum HealthStatus {
    Running,
    Starting,
    Offline,
    Unknown,
}

impl Display for HealthStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HealthStatus::Running => write!(f, "Running"),
            HealthStatus::Starting => write!(f, "Starting"),
            HealthStatus::Offline => write!(f, "Offline"),
            HealthStatus::Unknown => write!(f, "Unknown"),
        }
    }
}
