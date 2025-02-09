use serde::{Deserialize, Serialize};
use smol_str::SmolStr;
use crate::discord::RoleId;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UserData {
    pub name: SmolStr,
    pub roles: Vec<RoleId>
}