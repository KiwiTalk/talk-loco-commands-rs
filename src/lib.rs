/*
 * Created on Mon Nov 30 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

//! Official client/server compatible Loco commands implementation.
//! See request, response module for command datas.
//! See structs module for types used in command datas.

pub mod request;
pub mod response;

pub mod structs;

pub mod session;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct BsonCommand<T> {
    pub method: String,
    pub data: T,
}

/// Common Response data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseData<T> {
    pub status: i16,

    #[serde(flatten)]
    pub data: Option<T>,
}
