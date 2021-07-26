/*
 * Created on Thu Dec 03 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Serialize, Deserialize};
use crate::{ structs::chatroom::ChatroomMeta};

/// SETMETA response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetMeta {

    /// Chatroom id
    #[serde(rename = "chatId")]
    pub chat_id: i64,

    /// Updated chatroom meta item.
    pub meta: ChatroomMeta

}