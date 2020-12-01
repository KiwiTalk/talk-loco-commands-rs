/*
 * Created on Mon Nov 30 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

pub mod booking;
pub mod checkin;
pub mod media;
pub mod chat;

use crate::BsonData;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, BsonData)]
pub struct SampleData {
    
    pub status: i16

}