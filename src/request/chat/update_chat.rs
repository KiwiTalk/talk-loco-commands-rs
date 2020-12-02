/*
 * Created on Thu Dec 03 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Serialize, Deserialize};
use crate::BsonData;

/// Update chatroom push setting
#[derive(Debug, Clone, Serialize, Deserialize, BsonData)]
pub struct UpdateChat {

    /// Chatroom id
    #[serde(rename = "chatId")]
    pub chat_id: i64,

    #[serde(rename = "pushAlert")]
    pub push_alert: bool

}