/*
 * Created on Thu Dec 03 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Serialize, Deserialize};
use crate::{BsonData, structs::user::UserVariant};

/// Responses simplified member list of chatroom.
#[derive(Debug, Clone, Serialize, Deserialize, BsonData)]
pub struct GetMem {

    /// User list
    pub members: Vec<UserVariant>

}