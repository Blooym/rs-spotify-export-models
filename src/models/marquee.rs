// Created from spotify export file `Marquee.json`.

use alloc::string::String;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "camelCase")
)]
pub struct SpotifyExportMarquee {
    pub segment: String,
    pub artist_name: String,
}
