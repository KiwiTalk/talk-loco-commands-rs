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
use loco_protocol::command::{Command, builder::CommandBuilder, codec::{CommandCodec, StreamError}};
use serde::Serialize;

use crate::stream::ChunkedWriteStream;

use super::BsonCommand;

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

    /// Response command's status is not 0, means the request is corrupted
    Corrupted(Command),

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

/// [BsonCommand] read / write manager.
/// The default implementation will write command data with 2KB sized chunk.
#[derive(Debug)]
pub struct BsonCommandManager<S> {
    current_id: i32,
    codec: CommandCodec<ChunkedWriteStream<S>>,
}

impl<S> BsonCommandManager<S> {
    /// Create new [BsonCommandManager] from Stream
    pub fn new(stream: S) -> Self {
        Self::with_capacity(stream, 2048)
    }

    /// Create new [BsonCommandManager] from Stream with specific max write chunk size.
    pub fn with_capacity(stream: S, max_write_chunk_size: usize) -> Self {
        Self {
            current_id: 0,
            codec: CommandCodec::new(ChunkedWriteStream::new(stream, max_write_chunk_size)),
        }
    }

    pub fn codec(&self) -> &CommandCodec<ChunkedWriteStream<S>> {
        &self.codec
    }

    pub fn codec_mut(&mut self) -> &mut CommandCodec<ChunkedWriteStream<S>> {
        &mut self.codec
    }

    pub fn stream(&self) -> &S {
        self.codec.stream().inner()
    }

    pub fn stream_mut(&mut self) -> &mut S {
        self.codec.stream_mut().inner_mut()
    }

    pub fn current_id(&self) -> i32 {
        self.current_id
    }

    pub fn unwrap(self) -> S {
        self.codec.unwrap().unwrap()
    }
}

impl<S: Write> BsonCommandManager<S> {
    /// Write bson command. returns request_id on success
    pub fn write<T: Serialize>(&mut self, command: &BsonCommand<T>) -> Result<i32, WriteError> {
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

impl<S: Read> BsonCommandManager<S> {
    /// Read bson command. returns (request_id, BsonCommand) tuple
    pub fn read(&mut self) -> Result<(i32, BsonCommand<Document>), ReadError> {
        let (_, command) = self.codec.read()?;

        if command.header.status == 0 {
            let id = command.header.id;
            let method = command.header.method()?;
    
            let document = bson::Document::from_reader(&mut Cursor::new(command.data))?;
    
            let data = bson::from_document::<Document>(document)?;
    
            Ok((id, BsonCommand { method, data_type: command.header.data_type, data }))
        } else {
            Err(ReadError::Corrupted(command))
        }
    }
}
