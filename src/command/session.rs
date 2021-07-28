/*
 * Created on Wed Jul 28 2021
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use std::{
    collections::VecDeque,
    io::{self, Read, Write},
};

use bson::Document;
use loco_protocol::command::codec::StreamError;
use serde::Serialize;

use super::{
    manager::{BsonCommandManager, ReadError, WriteError},
    BsonCommand,
};

#[derive(Debug)]
pub enum RequestError {
    Write(WriteError),
    Read(ReadError),
}

impl From<WriteError> for RequestError {
    fn from(err: WriteError) -> Self {
        Self::Write(err)
    }
}

impl From<ReadError> for RequestError {
    fn from(err: ReadError) -> Self {
        Self::Read(err)
    }
}

/// Command session with command cache.
/// Provide methods for requesting command response and broadcast command handling.
/// Useful when creating client.
/// You must use non blocking mode to prevent blocking. Also using async is recommended.
#[derive(Debug)]
pub struct BsonCommandSession<S> {
    store: VecDeque<BsonCommand<Document>>,
    manager: BsonCommandManager<S>,
}

impl<S> BsonCommandSession<S> {
    /// Create new [BsonCommandSession]
    pub fn new(stream: S) -> Self {
        Self {
            store: VecDeque::new(),
            manager: BsonCommandManager::new(stream),
        }
    }

    /// Create new [BsonCommandSession] with specific max write chunk size.
    pub fn with_capacity(stream: S, max_write_chunk_size: usize) -> Self {
        Self {
            store: VecDeque::new(),
            manager: BsonCommandManager::with_capacity(stream, max_write_chunk_size),
        }
    }

    pub fn manager(&self) -> &BsonCommandManager<S> {
        &self.manager
    }

    pub fn stream(&self) -> &S {
        self.manager.stream()
    }

    pub fn stream_mut(&mut self) -> &mut S {
        self.manager.stream_mut()
    }

    pub fn unwrap(self) -> S {
        self.manager.unwrap()
    }
}

impl<S: Read + Write> BsonCommandSession<S> {
    /// Request response for given command.
    /// This method blocks socket.
    /// The response is guaranteed to have same id and method of request command.
    pub fn request<T: Serialize>(
        &mut self,
        command: &BsonCommand<T>,
    ) -> Result<Document, RequestError> {
        let request_id = self.manager.write(command)?;

        loop {
            match self.manager.read() {
                Ok((id, read)) => {
                    if id == request_id && read.method == command.method {
                        return Ok(read.data);
                    } else {
                        self.store.push_back(read);
                    }
                }

                Err(ReadError::Codec(StreamError::Io(err)))
                    if err.kind() == io::ErrorKind::WouldBlock => {}

                Err(err) => return Err(RequestError::from(err)),
            }
        }
    }
}

impl<S: Read> BsonCommandSession<S> {
    /// Poll next broadcast command.
    pub fn poll_broadcast(&mut self) -> Result<Option<BsonCommand<Document>>, ReadError> {
        match self.store.pop_front() {
            Some(command) => Ok(Some(command)),

            None => match self.manager.read() {
                Ok((_, read)) => Ok(Some(read)),

                Err(ReadError::Codec(StreamError::Io(err)))
                    if err.kind() == io::ErrorKind::WouldBlock =>
                {
                    Ok(None)
                }

                Err(err) => Err(ReadError::from(err)),
            }
        }
    }
}
