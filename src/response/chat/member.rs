/*
 * Created on Thu Dec 03 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Serialize, Deserialize};
use crate::{ structs::user::UserVariant};

/// Responses detailed members of chatroom.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Member {

    /// Chatroom id
    #[serde(rename = "chatId")]
    pub chat_id: i64,

    /// List of requested user list
    #[serde(rename = "members")]
    pub members: Vec<UserVariant>

}