// Created from spotify export file `Follow.json`.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "camelCase")
)]
pub struct SpotifyExportFollowers {
    pub follower_count: u32,
    pub following_users_count: u32,
    pub dismissing_users_count: u32,
}
