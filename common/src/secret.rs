use std::fmt::{Debug, Formatter};
use serde::Deserialize;
use smol_str::SmolStr;

#[derive(Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct Secret(SmolStr);

impl Secret {
    pub fn new(value: SmolStr) -> Secret {
        Self(value)
    }
    
    pub fn secret(&self) -> &SmolStr { &self.0 }
}

impl Debug for Secret {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Secret(<redacted>)")
    }
}

