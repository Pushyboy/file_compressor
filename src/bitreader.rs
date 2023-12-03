use std::io::{Read, Bytes, Result};
use std::io::{Error, ErrorKind};

pub struct BitReader<R: Read> {
    inner: Bytes<R>,
    byte: u8,
    pos: u8,
}

impl<R:Read> BitReader<R> {
    /// Instantiates a new `BitReader`
    pub fn new(inner: R) -> BitReader<R> {
        BitReader {
            inner: inner.bytes(),
            byte: 0,
            pos: 0
        }
    }

    /// Reads in one bit of data.
    /// 
    /// Fails if EOF or can't read in data
    pub fn read_bit(&mut self) -> Result<u8> {
        if self.pos == 0 {
            match self.inner.next() {
                Some(Ok(byte)) => self.byte = byte,
                Some(Err(e)) => return Err(e),
                None => return Err(Error::new(ErrorKind::UnexpectedEof, "EOF reached")),
            }
        }

        let bit = (self.byte >> (7 - self.pos)) & 1;
        self.pos += 1;

        if self.pos == 8 {
            self.reset();
        }

        Ok(bit)
    }

    /// Reads in one byte of data.
    /// 
    /// Fails if EOF or can't to read in data
    pub fn read_byte(&mut self) -> Result<u8> {
        let mut byte = 0u8;
        for i in (0u8..=7).rev() {
            let bit = self.read_bit()?;
            byte |= bit << i;
        }

        Ok(byte)
    }

    /// Reads in two bytes of data in big endian
    ///
    /// Fails if EOF or can't read in data
    pub fn read_u16(&mut self) -> Result<u16> {
        let mut output = 0u16;
        for i in (0u8..=15).rev() {
            let bit = self.read_bit()?;
            output |= (bit as u16) << i;
        }

        Ok(output)
    }

    fn reset(&mut self) {
        self.byte = 0;
        self.pos = 0;
    }
}