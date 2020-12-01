/*
 * Created on Mon Nov 30 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

//! Loco commands implemention used in KakaoTalk

pub mod request;
pub mod response;

pub mod structs;

use std::io::Cursor;

pub use bson_data_derive::BsonData;

use loco_protocol::command::{self, CommandData};
use serde::{de::DeserializeOwned, Serialize};

pub trait BsonData: Serialize + DeserializeOwned {
    
    fn method(&self) -> &'static str;

}

pub struct BsonCommandData<D: BsonData>(D);

impl<D: BsonData> CommandData for BsonCommandData<D> {

    fn method(&self) -> &'static str {
        self.0.method()
    }

    fn encode(&self) -> Result<Vec<u8>, command::Error> {
        Ok(self.serialize()?)
    }

    fn decode(data: &Vec<u8>) -> Result<Self, command::Error> {
        Ok(Self::deserialize(data)?)
    }

}

impl<D: BsonData> BsonCommandData<D> {

    pub fn new(data: D) -> Self {
        Self(data)
    }

    pub fn unwrap(self) -> D {
        self.0
    }

    pub fn as_ref(&self) -> &D {
        &self.0
    }

    pub fn as_ref_mut(&mut self) -> &mut D {
        &mut self.0
    }

    fn serialize(&self) -> Result<Vec<u8>, BsonError> {
        let mut encoded = Vec::new();

        let doc = bson::to_document(&self.0)?;
        doc.to_writer(&mut encoded)?;
    
        Ok(encoded)
    }

    fn deserialize(data: &Vec<u8>) -> Result<Self, BsonError> {
        let doc = bson::Document::from_reader(&mut Cursor::new(data))?;

        let decoded = bson::from_document::<D>(doc)?;

        Ok(Self::new(decoded))
    }

}

impl<D: BsonData> From<D> for BsonCommandData<D> {

    fn from(data: D) -> Self {
        Self::new(data)
    }

}

pub(crate) enum BsonError {

    Encode(bson::ser::Error),
    Decode(bson::de::Error)

}

impl From<bson::ser::Error> for BsonError {
    fn from(err: bson::ser::Error) -> Self {
        Self::Encode(err)
    }
}

impl From<bson::de::Error> for BsonError {
    fn from(err: bson::de::Error) -> Self {
        Self::Decode(err)
    }
}

impl From<BsonError> for command::Error {
    fn from(err: BsonError) -> Self {
        match err {
            BsonError::Encode(err) => Self::Encode(err.to_string()),
            BsonError::Decode(err) => Self::Decode(err.to_string())
        }
    }
}

