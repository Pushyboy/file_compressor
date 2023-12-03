use std::io::{Read, self, Write, Result, ErrorKind};
use crate::bitwriter::{BitWriter, self};
use crate::bitreader::{BitReader, self};
use circular_buffer::CircularBuffer;

pub enum Output {
    Literal(u8),
    Reference{offset: u16, length: u8}
}

const BUFFER_SIZE: usize = 1_000_000;

pub struct LZSS;

impl LZSS {
    /// Reads in from a reader than implements `Read`
    /// and outputs to a writer than implements `Write`
    /// 
    /// Returns [`Result<()>`] if it fails to read or write
    pub fn write_output<R: Read, W: Write>(
        reader: &mut R,
        writer: &mut W
    ) -> Result<()> {
        let mut buffer = [0u8; BUFFER_SIZE];
        let mut writer = BitWriter::new(writer);

        // Read into the buffer until reader is empty - in each read 
        // encode the data and write it to the output
        while let Ok(len) = reader.read(&mut buffer) {
            if len == 0 { break; }

            println!("Read into buffer");
            let encoded_output = Self::encode(&buffer, len);
            for val in encoded_output.into_iter() {
                match val {
                    Output::Literal(val) => {
                        print!("{}", std::char::from_u32(val as u32).unwrap());
                        writer.write_bit(true)?;
                        writer.write_u8(val)?;
                    },
                    Output::Reference { offset, length } => {
                        print!("<{},{}>", offset, length);
                        writer.write_bit(false)?;
                        // Offsets by 4 since the length is 4 bytes in size
                        let double_byte = offset << 4 | length as u16;
                        writer.write_u16(double_byte)?;
                    },
                }
            }

        }
        writer.flush()?;

        Ok(())
    }

    fn encode(data: &[u8], len: usize) -> Vec<Output> {
        let (max_search_size, look_ahead_size) = (4095, 15);
        let mut output: Vec<Output> = Vec::with_capacity(len);

        // Sliding window
        let mut pos = 0;
        while pos < len {
            let byte = data[pos];

            // Set search buffer bounds
            let min = pos.checked_sub(max_search_size).unwrap_or(0);
            let max = std::cmp::min(len, pos + look_ahead_size);
            // Search the search buffer for a reference
            let (offset, length) = Self::search_buffer(
                &data[min..max],
                pos
            );

            // Set the return value as a byte or reference
            let return_value = match (offset, length) {
                (0, 0) => {
                    pos += 1;
                    Output::Literal(byte)
                },
                (offset, length) => {
                    pos += length as usize;
                    Output::Reference{ offset, length }
                }
            };

            output.push(return_value);
        }   

        output
    }

    // Searches the buffer for a match
    fn search_buffer(data: &[u8], pos: usize) -> (u16, u8) {
        let (mut offset, mut length) = (0, 0);

        if pos == 0 || pos >= data.len() {
            return (0, 0);
        }

        // Search the bugger
        for i in (0..=(pos - 1)).rev() {
            let temp_offset = pos - i;
            let mut temp_len = 0;

            while i + temp_len < pos &&                               // Prevent search buffer going into look_ahead
                  pos + temp_len < data.len() &&                      // Prevent look_ahead overflow
                  data[i + temp_len] == data[pos + temp_len]          // Compare bytes
            {
                temp_len += 1;
            }

            // Set the offset and length if the reference reduces the size
            if temp_len > length && temp_len > 2 {
                (offset, length) = (temp_offset, temp_len);
            }
        }

        (offset as u16, length as u8)
    }

    /// 
    pub fn read_input<R: Read, W: Write>(
        reader: &mut R,
        writer: &mut W
    ) -> Result<()> {
        let mut reader = BitReader::new(reader);
        let mut sliding_window = CircularBuffer::<4095, u8>::new();

        println!("{}", "Starting read of input");
        loop {
            if let Ok(flag) = reader.read_bit() {
                match flag {
                    1 => match reader.read_byte() {
                        Ok(literal) => { 
                            println!("{}", literal);
                            writer.write(&[literal])?; 
                            sliding_window.push_front(literal);
                        },
                        Err(_) => break,
                    },
                    0 => match reader.read_u16() {
                        Ok(reference) => {
                            let offset = (reference >> 4) as usize;
                            let length = (reference & 0b1111) as usize;

                            println!("{},{}", offset, length);

                            if offset >= sliding_window.capacity() {
                                break;
                            }

                            sliding_window.range((offset - length)..(offset))
                                .rev()
                                .copied()
                                .collect::<Vec<u8>>()
                                .into_iter()
                                .for_each(|literal| {
                                    writer.write(&[literal]);
                                    sliding_window.push_front(literal);
                                });
                        },
                        Err(_) => break,
                    },
                    _ => unreachable!()
                }
            } else {
                break;
            }
        }

        Ok(())
    }
} 