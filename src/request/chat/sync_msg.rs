/*
 * Created on Thu Dec 03 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Serialize, Deserialize};
use crate::BsonData;

/// Sync skipped chats.
/// Official client send this when last log id written is different with actual last log id.
#[derive(Debug, Clone, Serialize, Deserialize, BsonData)]
pub struct SyncMsg {

    /// Chatroom id
    #[serde(rename = "chatId")]
    pub chat_id: i64,

    /// Current written last chat log id in client.
    #[serde(rename = "cur")]
    pub current: i64,

    /// Max number to receive once.
    /// The default is 300. But the server always seems to send up to 300 regardless of the number.
    #[serde(rename = "cnt")]
    pub count: i32,

    /// Last chat log id received by server.
    pub max: i64,
}