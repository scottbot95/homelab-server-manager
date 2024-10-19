use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServerStatus {
    pub name: String,
    pub health: HealthStatus,
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