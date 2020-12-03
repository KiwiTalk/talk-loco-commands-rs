/*
 * Created on Thu Dec 03 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Serialize, Deserialize};
use crate::BsonData;

/// Set Chatroom meta
#[derive(Debug, Clone, Serialize, Deserialize, BsonData)]
pub struct SetMeta {

    /// Chatroom id
    #[serde(rename = "chatId")]
    pub chat_id: i64,

    /// Meta type. See `structs/chatroom.rs` ChatroomMetaType for predefined types.
    #[serde(rename = "type")]
    pub meta_type: i8,

    /// Json or String content. Different depending on type.
    pub content: String

}