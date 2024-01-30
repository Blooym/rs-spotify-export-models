// Created from spotify export file `Identifiers.json`.

use alloc::string::String;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "camelCase")
)]
pub struct SpotifyExportIdentifiers {
    pub identifier_type: String,
    pub identifier_value: String,
}
