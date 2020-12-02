/*
 * Created on Thu Dec 03 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Serialize, Deserialize};
use crate::BsonData;

/// Delete chat. Official server only deletes message sent before 5 mins max.
#[derive(Debug, Clone, Serialize, Deserialize, BsonData)]
pub struct DeleteMsg {

    /// Chatroom id
    #[serde(rename = "chatId")]
    pub chat_id: i64,

    /// Chat log id
    #[serde(rename = "logId")]
    pub log_id: i64

}