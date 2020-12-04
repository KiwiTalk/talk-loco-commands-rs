/*
 * Created on Mon Nov 30 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

//! Official client/server compatible Loco commands implementation.
//! Check request, response module for command datas.
//! Check structs module for types used in command datas.

pub mod request;
pub mod response;

pub mod structs;

use std::io::Cursor;

pub use talk_loco_commands_derive::BsonData;

use loco_protocol::command::{self, CommandData};
use serde::{de::DeserializeOwned, Serialize, Deserialize};

pub trait BsonData: Serialize + DeserializeOwned {
    
    fn method() -> &'static str;

}

/// Wrapping request data
pub struct BsonReqData<D>(D);

impl<D: BsonData> CommandData for BsonReqData<D> {

    fn method(&self) -> &'static str {
        D::method()
    }

    fn encode(&self) -> Result<Vec<u8>, command::Error> {
        Ok(self.serialize()?)
    }

    fn decode(data: &Vec<u8>) -> Result<Self, command::Error> {
        Ok(Self::deserialize(data)?)
    }

}

impl<D: BsonData> BsonReqData<D> {

    pub fn new(data: D) -> Self {
        Self(data)
    }

    pub fn unwrap(self) -> D {
        self.0
    }

    pub fn as_ref(&self) -> &D {
        &self.0
    }

    pub fn as_mut(&mut self) -> &mut D {
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

/// Wrapping response status, data
///
/// Note: Official server doesn't include response data if operation failed.
/// Check `src/structs/client.rs` Status enum for predefined status ids.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BsonResData<D> {

    status: i16,

    #[serde(flatten)]
    response: Option<D>

}

impl<D: BsonData> CommandData for BsonResData<D> {

    fn method(&self) -> &'static str {
        D::method()
    }

    fn encode(&self) -> Result<Vec<u8>, command::Error> {
        Ok(self.serialize()?)
    }

    fn decode(data: &Vec<u8>) -> Result<Self, command::Error> {
        Ok(Self::deserialize(data)?)
    }

}

impl<D: BsonData> BsonResData<D> {

    pub fn new(status: i16, response: Option<D>) -> Self {
        Self {
            status,
            response
        }
    }

    pub fn status(&self) -> i16 {
        self.status
    }

    pub fn response(self) -> Option<D> {
        self.response
    }

    pub fn as_ref(&self) -> Option<&D> {
        self.response.as_ref()
    }

    pub fn as_mut(&mut self) -> Option<&mut D> {
        self.response.as_mut()
    }

    fn serialize(&self) -> Result<Vec<u8>, BsonError> {
        let mut encoded = Vec::new();

        let doc = bson::to_document(&self)?;
        doc.to_writer(&mut encoded)?;
    
        Ok(encoded)
    }

    fn deserialize(data: &Vec<u8>) -> Result<Self, BsonError> {
        let doc = bson::Document::from_reader(&mut Cursor::new(data))?;

        let decoded = bson::from_document::<Self>(doc)?;

        Ok(decoded)
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
