/*
 * Created on Thu Dec 03 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Serialize, Deserialize};

/// Update client status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetSt {

    /// Status
    ///
    /// * Unlocked = 1
    /// * Locked = 2
    #[serde(rename = "st")]
    pub status: i8

}