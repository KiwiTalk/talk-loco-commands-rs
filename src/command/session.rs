/*
 * Created on Wed Jul 28 2021
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use std::{io::{self, Read, Write}, time::Duration, vec::Drain};

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
/// Using non blocking mode highly recommended to prevent blocking.
#[derive(Debug)]
pub struct BsonCommandSession<S> {
    store: Vec<BsonCommand<Document>>,
    manager: BsonCommandManager<S>,
}

impl<S> BsonCommandSession<S> {
    /// Create new [BsonCommandSession]
    pub fn new(stream: S) -> Self {
        Self {
            store: Vec::new(),
            manager: BsonCommandManager::new(stream),
        }
    }

    /// Create new [BsonCommandSession] with specific max write chunk size.
    pub fn with_capacity(stream: S, max_write_chunk_size: usize) -> Self {
        Self {
            store: Vec::new(),
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
                        self.store.push(read);
                    }
                }

                Err(ReadError::Codec(StreamError::Io(err)))
                    if err.kind() == io::ErrorKind::WouldBlock =>
                {
                    std::thread::sleep(Duration::from_millis(1));
                }

                Err(err) => return Err(RequestError::from(err)),
            }
        }
    }
}

impl<S: Read> BsonCommandSession<S> {
    /// Poll one broadcast command.
    /// This method can block depending on stream.
    pub fn poll(&mut self) -> Result<(), ReadError> {
        self.poll_many(1)
    }

    /// Poll broadcast commands up to max limit
    pub fn poll_many(&mut self, max: usize) -> Result<(), ReadError> {
        for _ in 0..max {
            match self.manager.read() {
                Ok((_, read)) => {
                    self.store.push(read)
                },
    
                Err(ReadError::Codec(StreamError::Io(err)))
                    if err.kind() == io::ErrorKind::WouldBlock =>
                {
                    break;
                }
    
                Err(err) => return Err(ReadError::from(err)),
            }
        }

        Ok(())
    }

    /// Drain every broadcast commands stored
    pub fn broadcasts(&mut self) -> Drain<'_, BsonCommand<Document>> {
        self.store.drain(..)
    }
}
