use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use serde_with::DisplayFromStr;
use std::num::NonZeroU64;

macro_rules! u64_id {
    ($name: ident) => {
        #[serde_as]
        #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub struct $name(#[serde_as(as = "DisplayFromStr")] NonZeroU64);

        impl $name {
            pub const fn from_raw(id: u64) -> Option<Self> {
                match NonZeroU64::new(id) {
                    Some(id) => Some(Self(id)),
                    None => None,
                }
            }
        }

        impl From<u64> for $name {
            fn from(id: u64) -> Self {
                Self(NonZeroU64::new(id).unwrap())
            }
        }

        impl From<NonZeroU64> for $name {
            fn from(id: NonZeroU64) -> Self {
                Self(id)
            }
        }
    };
}

u64_id!(UserId);
u64_id!(RoleId);
u64_id!(GuildId);
