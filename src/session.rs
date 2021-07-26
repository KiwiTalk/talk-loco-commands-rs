/*
 * Created on Sun Jul 25 2021
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use std::{
    io::{Cursor, Read, Write},
    string::FromUtf8Error,
};

use bson::Document;
use loco_protocol::command::{
    builder::CommandBuilder,
    codec::{CommandCodec, StreamError},
};
use serde::Serialize;

use crate::{BsonCommand, ResponseData};

#[derive(Debug)]
pub enum WriteError {
    Codec(StreamError),
    Encode(bson::ser::Error),
}

impl From<StreamError> for WriteError {
    fn from(err: StreamError) -> Self {
        Self::Codec(err)
    }
}

impl From<bson::ser::Error> for WriteError {
    fn from(err: bson::ser::Error) -> Self {
        Self::Encode(err)
    }
}

#[derive(Debug)]
pub enum ReadError {
    Codec(StreamError),
    InvalidMethod(FromUtf8Error),
    Decode(bson::de::Error),
}

impl From<StreamError> for ReadError {
    fn from(err: StreamError) -> Self {
        Self::Codec(err)
    }
}

impl From<FromUtf8Error> for ReadError {
    fn from(err: FromUtf8Error) -> Self {
        Self::InvalidMethod(err)
    }
}

impl From<bson::de::Error> for ReadError {
    fn from(err: bson::de::Error) -> Self {
        Self::Decode(err)
    }
}

/// [BsonCommand] command session with automated Command management
pub struct BsonCommandSession<S> {
    current_id: i32,
    codec: CommandCodec<S>,
}

impl<S> BsonCommandSession<S> {
    pub fn new(codec: CommandCodec<S>) -> Self {
        Self {
            current_id: 0,
            codec,
        }
    }

    pub fn codec(&self) -> &CommandCodec<S> {
        &self.codec
    }

    pub fn codec_mut(&mut self) -> &mut CommandCodec<S> {
        &mut self.codec
    }

    pub fn current_id(&self) -> i32 {
        self.current_id
    }

    pub fn unwrap(self) -> S {
        self.codec.unwrap()
    }
}

impl<S: Write> BsonCommandSession<S> {
    /// Send bson command. returns request_id on success
    pub fn send<T: Serialize>(&mut self, command: &BsonCommand<T>) -> Result<i32, WriteError> {
        let request_id = self.current_id;
        self.current_id += 1;

        let builder = CommandBuilder::new(request_id, &command.method);

        let mut raw_data = Vec::new();

        let doc = bson::to_document(&command.data)?;
        doc.to_writer(&mut raw_data)?;

        let command = builder.build(0, raw_data);

        self.codec.write(&command)?;

        Ok(request_id)
    }
}

impl<S: Read> BsonCommandSession<S> {
    /// Read bson command. returns (request_id, BsonCommand) tuple
    pub fn read(
        &mut self,
    ) -> Result<(i32, BsonCommand<ResponseData<Document>>), ReadError> {
        let (_, command) = self.codec.read()?;

        let id = command.header.id;
        let method = command.header.method()?;

        let document = bson::Document::from_reader(&mut Cursor::new(command.data))?;

        let data = bson::from_document::<ResponseData<Document>>(document)?;

        Ok((id, BsonCommand { method, data }))
    }
}
