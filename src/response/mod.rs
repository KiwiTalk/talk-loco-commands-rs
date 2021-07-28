/*
 * Created on Mon Nov 30 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

pub mod booking;
pub mod checkin;
pub mod media;
pub mod chat;

use serde::{Deserialize, Serialize};

/// Common Response data with status code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseData<T> {
    pub status: i16,

    #[serde(flatten)]
    pub data: Option<T>,
}
