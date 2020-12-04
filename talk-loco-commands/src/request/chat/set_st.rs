/*
 * Created on Thu Dec 03 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Serialize, Deserialize};
use crate::BsonData;

/// Update client status
#[derive(Debug, Clone, Serialize, Deserialize, BsonData)]
pub struct SetSt {

    /// Status
    ///
    /// * Unlocked = 1
    /// * Locked = 2
    #[serde(rename = "st")]
    pub status: i8

}