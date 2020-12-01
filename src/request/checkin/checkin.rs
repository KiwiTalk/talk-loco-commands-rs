/*
 * Created on Tue Dec 01 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Serialize, Deserialize};
use crate::BsonData;

/// Request loco server host data
#[derive(Debug, Clone, Serialize, Deserialize, BsonData)]
pub struct Checkin {

    /// Client user id. Login to acquire.
    #[serde(rename = "userId")]
    pub user_id: i64,

    /// Current OS (win32, android, mac, etc.)
    pub os: String,

    /// Network type (-999 for wired)
    #[serde(rename = "ntype")]
    pub net_type: i16,

    /// Official app version
    #[serde(rename = "appVer")]
    pub app_version: String,

    /// Network MCCMNC
    #[serde(rename = "MCCMNC")]
    pub mccmnc: String,

    #[serde(rename = "lang")]
    pub language: String,

    #[serde(rename = "countryISO")]
    pub country_iso: String,

    /// Subdevice(PC, Tablet) or not
    #[serde(rename = "useSub")]
    pub use_usb: bool

}
