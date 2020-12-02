/*
 * Created on Thu Dec 03 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Serialize, Deserialize};
use crate::BsonData;

#[derive(Debug, Clone, Serialize, Deserialize, BsonData)]
pub struct DeleteMsg;