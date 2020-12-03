/*
 * Created on Thu Dec 03 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Serialize, Deserialize};
use crate::{BsonData, structs::chatroom::ChatroomInfo};

/// Chatroom info response
#[derive(Debug, Clone, Serialize, Deserialize, BsonData)]
pub struct ChatInfo {

    /// Chatroom info
    #[serde(rename = "chatInfo")]
    pub chat_info: ChatroomInfo,

    /// Unknown. Only appears on openchat rooms.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o: Option<i32>

}