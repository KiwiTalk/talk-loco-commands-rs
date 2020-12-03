/*
 * Created on Thu Dec 03 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Serialize, Deserialize};
use crate::BsonData;

/// Request simplified member list of chatroom.
#[derive(Debug, Clone, Serialize, Deserialize, BsonData)]
pub struct GetMem {

    /// Chatroom id
    #[serde(rename = "chatId")]
    pub chat_id: i64

}