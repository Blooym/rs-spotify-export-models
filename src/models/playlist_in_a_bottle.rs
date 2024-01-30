// Created from spotify export file `PlaylistInABottle.json`.

use alloc::{string::String, vec::Vec};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "camelCase")
)]
pub struct SpotifyExportVessel {
    pub vessel: String,
    pub responses: Vec<SpotifyExportVesselResponse>,
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "camelCase")
)]
pub struct SpotifyExportVesselResponse {
    pub question: String,
    pub answer: String,
}
