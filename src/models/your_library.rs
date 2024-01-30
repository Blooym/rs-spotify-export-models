// Created from spotify export file `YourLibrary.json`.

use alloc::{string::String, vec::Vec};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "camelCase")
)]
pub struct SpotifyExportYourLibrary {
    pub tracks: Vec<SpotifyExportLibraryTrack>,
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "camelCase")
)]
pub struct SpotifyExportLibraryTrack {
    pub artist: String,
    pub album: String,
    pub track: String,
    pub uri: String,
}
