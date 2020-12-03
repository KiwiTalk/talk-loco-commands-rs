/*
 * Created on Thu Dec 03 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Serialize, Deserialize};

/// Minimal user info for chatroom display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayUserInfo {

    /// User id
    #[serde(rename = "userId")]
    pub user_id: i64,

    /// User nickname
    #[serde(rename = "nickName")]
    pub nickname: String,

    /// Profile image URL. None if profile image is default.
    #[serde(rename = "pi")]
    pub profile_image_url: Option<String>,

    /// Country Iso, does not present on openchat.
    #[serde(rename = "countryIso", skip_serializing_if = "Option::is_none")]
    pub country_iso: Option<String>

}