// Created from spotify export file `Payments.json`.
// Note: I didn't have any data in this file, so I'm not sure what the structure looks like.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "camelCase")
)]
pub struct SpotifyExportPayments {}
