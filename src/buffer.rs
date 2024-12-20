use std::{fs, io::Read};

use crate::{error::ErrorType, frame::Frame};

pub struct Buffer {
    pub data: Vec<u8>,
    pub pos: usize,
    pub total_bits: usize,
}

impl Buffer {
    pub fn create_buffer_from_file(path: &str) -> Buffer {
        let mut file = fs::File::open(path).expect("Cannot open mp3 file.");
        let mut data: Vec<u8> = Vec::new();

        file.read_to_end(&mut data)
            .expect("Cannot read data from file");

        let total_bits = data.len() * 8;

        Self {
            data,
            pos: 0,
            total_bits,
        }
    }

    pub fn get_bits(&mut self, n: u32) -> Result<u32, ErrorType> {
        if self.pos + n as usize >= self.total_bits {
            return Err(ErrorType::OutOfIndex);
        }

        if n > 32 {
            return Err(ErrorType::Overflow);
        }

        if n == 0 {
            return Ok(0);
        }

        let start_byte_index = self.pos / 8;
        let end_byte_index = (self.pos + n as usize - 1) / 8;

        let start_offset: u32 = ((0xff >> (self.pos as u8 % 8)) as u32) << (((n - 1) / 8) * 8);
        let end_offset: u32 = 0xff << (7 - ((self.pos as u32 + n - 1) as u8 % 8));

        let mask: u32 = if start_byte_index == end_byte_index {
            (start_offset) & (end_offset)
        } else {
            (start_offset) | (end_offset)
        };

        let mut result: u32 = 0;

        for i in 0..=(n as usize - 1) / 8 {
            result |= (self.data[i + start_byte_index] as u32) << (((n as usize - 1) / 8 - i) * 8)
        }

        result = (result & mask) >> (7 - ((self.pos as u32 + n - 1) as u8 % 8));

        self.move_pos(n as isize)?;

        Ok(result)
    }

    pub fn set_pos(&mut self, pos: usize) -> Result<(), ErrorType> {
        if pos >= self.total_bits {
            return Err(ErrorType::OutOfIndex);
        }
        self.pos = pos;

        Ok(())
    }

    pub fn move_pos(&mut self, n: isize) -> Result<(), ErrorType> {
        if self.pos as isize + n >= 0 && self.pos as isize + n <= self.total_bits as isize {
            self.pos = (n + self.pos as isize) as usize;
            return Ok(());
        }

        Err(ErrorType::OutOfIndex)
    }

    pub fn set_pos_next_frame(&mut self) -> Result<(), ErrorType> {
        let mut bits = self.get_bits(15)?;
        loop {
            bits &= 0x7fff;
            bits |= self.get_bits(1)? & 0b1;

            if bits == 0x7ffd {
                self.move_pos(-15)?;
                break Ok(());
            }
            bits <<= 1;
        }
    }

    pub fn extract_frames(&mut self) -> Vec<Frame> {
        let mut frames: Vec<Frame> = Vec::new();

        loop {
            if let Err(err) = self.set_pos_next_frame() {
                match err {
                    ErrorType::OutOfIndex => break,
                    _ => continue,
                }
            }

            let pos = self.pos;
            let frame = Frame::create_from_buffer(self);

            if frame.is_err() {
                continue;
            }

            let pos = self.pos;
            if frame.as_ref().unwrap().header().validate_header().is_err() {
                continue;
            }

            frames.push(frame.unwrap());
        }

        frames
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_buffer_from_file_100kb() {
        let buffer = Buffer::create_buffer_from_file("mp3-examples/test_data_100kb.mp3");

        assert_eq!(buffer.data.len() / 1024, 100);
    }

    #[test]
    fn test_get_bits() {
        let mut buffer = Buffer::create_buffer_from_file("mp3-examples/test_data_100kb.mp3");

        assert_eq!(buffer.get_bits(12).unwrap(), 0xfff);
        assert_eq!(buffer.get_bits(1).unwrap(), 1); // MPEG-1
        assert_eq!(buffer.get_bits(2).unwrap(), 1); // LAYER III
        assert_eq!(buffer.get_bits(1).unwrap(), 1); // Error Protection
    }
}
