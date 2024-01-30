// Created from spotify export file `SearchQueries.json`.

use alloc::{string::String, vec::Vec};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "camelCase")
)]
pub struct SpotifyExportSearchQuery {
    pub platform: String,
    pub search_time: String,
    pub search_query: String,
    pub search_interaction_uris: Vec<String>,
}
