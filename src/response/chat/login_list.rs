/*
 * Created on Wed Dec 02 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Serialize, Deserialize};
use crate::{BsonData, structs::client::ClientInfo};

/// Contains userId, chatroom list
#[derive(Debug, Clone, Serialize, Deserialize, BsonData)]
pub struct LoginList {
    
    pub status: i16,


}