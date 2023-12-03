use std::io::{Write, BufWriter, Error, Result};

pub struct BitWriter<W: Write> {
    inner: W,
    byte: u8,
    pos: u8
}

impl<W: Write> BitWriter<W> {
    /// Instantiates a new `BitWriter`
    pub fn new(inner: W) -> BitWriter<W> {
        BitWriter { 
            inner, 
            byte: 0, 
            pos: 0 
        }
    }

    /// Adds one bit to the buffer and attempts to write 
    /// one enough bits are formed to create a byte
    pub fn write_bit(&mut self, input: bool) -> Result<()> {
        if input {
            self.byte |= 1 << (7 - self.pos);
        }
        self.pos += 1;

        if self.pos == 8 {
            self.inner.write_all(&[self.byte])?;
            self.reset();
        }

        Ok(())
    }

    /// Attempts to write a byte to the writer
    pub fn write_u8(&mut self, input: u8) -> Result<()> {
        for i in (0u8..=7).rev() {
            let bit = ((input >> i) & 1) == 1;
            self.write_bit(bit)?;
        }

        Ok(())
    }

    /// Attempts to write two bytes to the writer
    pub fn write_u16(&mut self, input: u16) -> Result<()> {
        for i in (0u8..=15).rev() {
            let bit = ((input >> i) & 1) == 1;
            self.write_bit(bit)?;
        }

        Ok(())
    }

    /// Attempts to flush the writer
    pub fn flush(&mut self) -> Result<()> {
        if self.pos > 0 {
            self.inner.write_all(&[self.byte])?;
            self.reset();
        } 
        self.inner.flush()?;

        Ok(())
    }

    fn reset(&mut self) {
        self.byte = 0;
        self.pos = 0;
    }
}

impl<W: Write> Drop for BitWriter<W> {
    fn drop(&mut self) {
        self.flush();
    }
}