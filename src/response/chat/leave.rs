/*
 * Created on Thu Dec 03 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Serialize, Deserialize};
use crate::BsonData;

/// Leave chatroom
#[derive(Debug, Clone, Serialize, Deserialize, BsonData)]
pub struct Leave {

    /// Last token(?) id
    #[serde(rename = "lastTokenId")]
    pub last_token_id: i64

}