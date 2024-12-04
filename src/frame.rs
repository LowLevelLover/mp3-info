use crate::{buffer::Buffer, error::ErrorType, side_info::SideInfo, Header};

#[derive(Debug)]
pub struct Frame {
    header: Header,
    crc: Option<u16>,
    pub side_info: SideInfo,
    length_byte: usize,
}

impl Frame {
    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn create_from_buffer(buffer: &mut Buffer) -> Result<Self, ErrorType> {
        let header = Header::create_from_buffer(buffer);
        let crc = if header.error_protection {
            Some(buffer.get_bits(16)? as u16)
        } else {
            None
        };

        let side_info = SideInfo::create_from_buffer(buffer, &header.mode)?;
        let length_byte = 144000 * (header.get_bitrate()? / header.get_frequency()?) as usize
            + header.padding_bit as usize;

        Ok(Self {
            header,
            crc,
            side_info,
            length_byte,
        })
    }

    pub fn check_crc(&self) {
        todo!("Implement CRC check for");
    }
}
