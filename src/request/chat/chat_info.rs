/*
 * Created on Thu Dec 03 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Serialize, Deserialize};

/// Request Chatroom info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatInfo {

    /// Chatroom id
    #[serde(rename = "chatId")]
    pub chat_id: i64

}