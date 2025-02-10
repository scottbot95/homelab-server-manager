use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UserData {
    pub name: SmolStr,
}