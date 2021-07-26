/*
 * Created on Thu Dec 03 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Serialize, Deserialize};

/// Leave chatroom
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Leave {

    /// Last token(?) id
    #[serde(rename = "lastTokenId")]
    pub last_token_id: i64

}