/*
 * Created on Thu Dec 03 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Serialize, Deserialize};
use crate::{BsonData, structs::chat::Chatlog};

#[derive(Debug, Clone, Serialize, Deserialize, BsonData)]
pub struct SyncMsg {

    #[serde(rename = "isOK")]
    is_ok: bool,

    #[serde(rename = "chatLogs")]
    chat_logs: Vec<Chatlog>,

    #[serde(rename = "jsi")]
    jsi: i64,

    #[serde(rename = "lastTokenId")]
    last_token_id: i64,

}