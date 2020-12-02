/*
 * Created on Thu Dec 03 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Serialize, Deserialize};
use crate::BsonData;

/// Request Chatroom info
#[derive(Debug, Clone, Serialize, Deserialize, BsonData)]
pub struct ChatInfo {

    /// Chatroom id
    #[serde(rename = "chatId")]
    pub chat_id: i64

}