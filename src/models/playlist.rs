// Created from spotify export file `Playlists0.json`.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct SpotifyExportPlaylists {
    pub playlists: Option<Vec<SpotifyExportPlaylist>>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct SpotifyExportPlaylist {
    pub name: String,
    pub last_modified_date: String,
    pub items: Option<Vec<SpotifyExportPlaylistItem>>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct SpotifyExportPlaylistItem {
    pub track: Option<SpotifyExportPlaylistTrack>,
    pub added_date: String,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct SpotifyExportPlaylistTrack {
    pub track_name: String,
    pub artist_name: String,
    pub album_name: String,
    pub track_uri: String,
}
