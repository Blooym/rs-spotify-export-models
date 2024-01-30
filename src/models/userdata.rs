// Created from spotify export file `Userdata.json`.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "camelCase")
)]
pub struct SpotifyExportUserdata {
    pub username: String,
    pub email: String,
    pub country: String,
    pub created_from_facebook: bool,
    pub facebook_uid: Option<String>,
    pub birthdate: String,
    pub gender: String,
    pub postal_code: Option<String>,
    pub mobile_number: Option<String>,
    pub mobile_operator: Option<String>,
    pub mobile_brand: Option<String>,
    pub creation_time: String,
}
