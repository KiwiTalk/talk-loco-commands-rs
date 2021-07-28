/*
 * Created on Wed Jul 28 2021
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use std::io::{self, BufWriter, Read, Write};

/// Write large data with specific chunk size
#[derive(Debug, Clone)]
pub struct ChunkedWriteStream<S> {
    stream: S,
    max_size: usize,
}

impl<S> ChunkedWriteStream<S> {
    pub fn new(stream: S, max_size: usize) -> Self {
        Self {
            stream,
            max_size
        }
    }

    pub fn inner(&self) -> &S {
        &self.stream
    }

    pub fn inner_mut(&mut self) -> &mut S {
        &mut self.stream
    }

    pub fn unwrap(self) -> S {
        self.stream
    }
}

impl<S: Write> Write for ChunkedWriteStream<S> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut writer = BufWriter::with_capacity(self.max_size, &mut self.stream);

        writer.write_all(buf)?;
        writer.flush()?;

        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.stream.flush()
    }
}

impl<S: Read> Read for ChunkedWriteStream<S> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.stream.read(buf)
    }
}
