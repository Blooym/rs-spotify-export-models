// Created from spotify export file `Identifiers.json`.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct SpotifyExportIdentifiers {
    pub identifier_type: String,
    pub identifier_value: String,
}
