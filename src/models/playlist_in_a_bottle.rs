// Created from spotify export file `PlaylistInABottle.json`.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct SpotifyExportVessel {
    pub vessel: String,
    pub responses: Vec<SpotifyExportVesselResponse>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct SpotifyExportVesselResponse {
    pub question: String,
    pub answer: String,
}
