/*
 * Created on Wed Dec 02 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Serialize, Deserialize};
use crate::{BsonData, structs::client::ClientInfo};

use super::LChatList;

/// Login to loco server
#[derive(Debug, Clone, Serialize, Deserialize, BsonData)]
pub struct LoginList {

    #[serde(flatten)]
    pub client: ClientInfo,

    /// Protocol version, seems like always "1"
    #[serde(rename = "prtVer")]
    pub protocol_version: String,

    /// Device uuid String. Usually hashed unique id.
    #[serde(rename = "duuid")]
    pub device_uuid: String,
    
    /// OAuth access token
    #[serde(rename = "oauthToken")]
    pub oauth_token: String,

    #[serde(rename = "lang")]
    pub language: String,

    /// Device type (2 for pc)
    #[serde(rename = "dtype")]
    pub device_type: i8,

    /// Unknown
    pub revision: i32,

    /// Unknown. Always None(?)
    pub rp: (),

    #[serde(flatten)]
    pub chat_list: LChatList,

    /// Unknown
    #[serde(rename = "lbk")]
    pub last_block_token: i32,

    /// background checking(?)
    #[serde(rename = "bg")]
    pub background: bool

}