/*
 * Created on Tue Dec 01 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Serialize, Deserialize};
use crate::{BsonData, structs::client::ClientInfo};

/// Request loco server host data
#[derive(Debug, Clone, Serialize, Deserialize, BsonData)]
pub struct Checkin {

    /// Client user id. Login to acquire.
    #[serde(rename = "userId")]
    pub user_id: i64,

    #[serde(flatten)]
    pub client: ClientInfo,

    #[serde(rename = "lang")]
    pub language: String,

    #[serde(rename = "countryISO")]
    pub country_iso: String,

    /// Subdevice(PC, Tablet) or not
    #[serde(rename = "useSub")]
    pub use_usb: bool

}
