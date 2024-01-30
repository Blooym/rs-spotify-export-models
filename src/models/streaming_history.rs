// Created from spotify export file `StreamingHistory0.json`.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "camelCase")
)]
pub struct SpotifyExportStreamingHistory {
    pub end_time: String,
    pub artist_name: String,
    pub track_name: String,
    pub ms_played: u32,
}
