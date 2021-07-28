/*
 * Created on Wed Jul 28 2021
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

pub mod manager;
pub mod session;

#[derive(Debug, Clone)]
pub struct BsonCommand<T> {
    pub method: String,
    pub data_type: i8,
    pub data: T,
}
