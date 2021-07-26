/*
 * Created on Thu Dec 03 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Serialize, Deserialize};

/// Leave chatroom
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Leave {

    /// Chatroom id
    #[serde(rename = "chatId")]
    pub chat_id: i64,

    /// Block chatroom. Cannot rejoin chatroom if true.
    pub block: bool

}