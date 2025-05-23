use std::fmt::Display;

use crate::buffer::Buffer;
use crate::error;

const HALF_BITRATE_MPEG1_LAYER3: [u8; 15] =
    [0, 16, 20, 24, 28, 32, 40, 48, 56, 64, 80, 96, 112, 128, 160];

const FREQUENCY_MPEG1: [u16; 3] = [44100, 48000, 32000];

#[derive(PartialEq, Debug)]
pub enum Version {
    MPEG1,
    MPEG2,
}

#[derive(PartialEq, Debug)]
pub enum Layer {
    Layer1,
    Layer2,
    Layer3,
}

#[derive(PartialEq, Debug)]
pub enum Mode {
    Stereo,
    JointStereo,
    DualChannel,
    SingleChannel,
}

#[derive(Debug, PartialEq)]
pub struct Header {
    pub sync_word: u16,
    pub version: Version,
    pub layer: Layer,
    pub error_protection: bool,
    pub bitrate: u8,
    pub frequency: u8,
    pub padding_bit: bool,
    pub private_bit: bool,
    pub mode: Mode,
    pub intensity_stereo: bool,
    pub ms_stereo: bool,
    pub copy_right: bool,
    pub copy_of_original: bool,
    pub emphasis: u8,
    pos: usize,
}

impl Display for Layer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let layer = match self {
            Self::Layer3 => "Layer III",
            Self::Layer2 => "Layer II",
            Self::Layer1 => "Layer I",
        };

        write!(f, "{layer}")
    }
}

impl Layer {
    fn decode_layer(layer: u8) -> Result<Layer, error::ErrorType> {
        match layer {
            1 => Ok(Layer::Layer3),
            2 => Ok(Layer::Layer2),
            3 => Ok(Layer::Layer1),
            _ => Err(error::ErrorType::UnknownLayer),
        }
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let version = match self {
            Self::MPEG1 => "MPEG-1",
            Self::MPEG2 => "MPEG-2",
        };

        write!(f, "{version}")
    }
}

impl Version {
    fn decode_version(version: u8) -> Result<Version, error::ErrorType> {
        match version {
            0 => Ok(Version::MPEG2),
            1 => Ok(Version::MPEG1),
            _ => Err(error::ErrorType::UnknownVersion),
        }
    }
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mode = match self {
            Self::Stereo => "Stereo",
            Self::JointStereo => "Joint Stereo",
            Self::DualChannel => "Dual Channel",
            Self::SingleChannel => "Single Channel",
        };

        write!(f, "{mode}")
    }
}

impl Mode {
    fn decode_mode(mode: u8) -> Result<Mode, error::ErrorType> {
        match mode {
            0 => Ok(Mode::Stereo),
            1 => Ok(Mode::JointStereo),
            2 => Ok(Mode::DualChannel),
            3 => Ok(Mode::SingleChannel),
            _ => Err(error::ErrorType::UnknownMode),
        }
    }
}

impl Header {
    pub fn validate_header(&self) -> Result<(), error::ErrorType> {
        if self.sync_word == 0xfff && self.layer == Layer::Layer3 {
            return Ok(());
        }

        Err(error::ErrorType::InvalidHeader)
    }

    pub fn create_from_buffer(buffer: &mut Buffer) -> Self {
        let index = buffer.pos / 8;

        let sync_word =
            ((buffer.data[index] as u16) << 4) | (buffer.data[1 + index] as u16 & 0xf0) >> 4;
        let version = Version::decode_version((buffer.data[1 + index] & 8) >> 3).unwrap();
        let layer = Layer::decode_layer((buffer.data[1 + index] & 0b110) >> 1).unwrap();
        let error_protection = (buffer.data[1 + index] & 1) == 0;
        let bitrate = (buffer.data[2 + index] & 0xf0) >> 4;
        let frequency = (buffer.data[2 + index] & 0xc) >> 2;
        let padding_bit = ((buffer.data[2 + index] & 0x10) >> 1) == 1;
        let private_bit = buffer.data[2 + index] & 1 == 1;
        let mode = Mode::decode_mode((buffer.data[3 + index] & 0xc0) >> 6).unwrap();
        let intensity_stereo = (buffer.data[3 + index] & 0x20) >> 5 == 1;
        let ms_stereo = (buffer.data[3 + index] & 0x10) >> 4 == 1;
        let copy_right = (buffer.data[3 + index] & 0b1000) >> 3 == 1;
        let copy_of_original = (buffer.data[3 + index] & 0b100) >> 2 == 0;
        let emphasis = buffer.data[3 + index] & 0b11;

        buffer.move_pos(32).unwrap();

        Self {
            sync_word,
            version,
            layer,
            error_protection,
            bitrate,
            frequency,
            padding_bit,
            private_bit,
            mode,
            intensity_stereo,
            ms_stereo,
            copy_right,
            copy_of_original,
            emphasis,
            pos: buffer.pos - 32,
        }
    }

    pub fn get_bitrate(&self) -> Result<u16, error::ErrorType> {
        if self.version == Version::MPEG1 && self.layer == Layer::Layer3 && self.bitrate < 15 {
            return Ok(HALF_BITRATE_MPEG1_LAYER3[self.bitrate as usize] as u16 * 2);
        }

        Err(error::ErrorType::UnknownBitrate)
    }

    pub fn get_frequency(&self) -> Result<u16, error::ErrorType> {
        if self.version == Version::MPEG1 && self.frequency < 3 {
            return Ok(FREQUENCY_MPEG1[self.frequency as usize]);
        }

        Err(error::ErrorType::UnknownFrequency)
    }
}

impl Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            " \t Version: {}
\t Layer: {}
\t Error Protection: {}
\t Bitrate: {}kb/sec
\t Frequency: {}Hz
\t Padding: {}
\t Set Private Bit: {}
\t Channel Mode: {}
\t Intensity Stereo: {}
\t M/S Stereo: {}
\t Copy Right: {},
\t Copy of Original: {},
\t emphasis: {},
            ",
            self.version,
            self.layer,
            self.error_protection,
            self.get_bitrate().unwrap(),
            self.get_frequency().unwrap(),
            self.padding_bit,
            self.private_bit,
            self.mode,
            if self.intensity_stereo { "On" } else { "Off" },
            if self.ms_stereo { "On" } else { "Off" },
            if self.copy_right {
                "Has Copy Right"
            } else {
                "Not Set"
            },
            self.copy_of_original,
            match self.emphasis {
                0 => "None",
                1 => "50/15 ms",
                2 => "Reserved",
                3 => "CCIT J.17",
                _ => "",
            },
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_header_from_buffer() {
        let mut buffer = Buffer::create_buffer_from_file("mp3-examples/test_data_100kb.mp3");
        let header = Header::create_from_buffer(&mut buffer);

        assert_eq!(
            header,
            Header {
                pos: buffer.pos - 32,
                sync_word: 0xfff,
                version: Version::MPEG1,
                layer: Layer::Layer3,
                error_protection: false,
                bitrate: 0b1001,
                frequency: 0,
                padding_bit: false,
                private_bit: false,
                mode: Mode::JointStereo,
                intensity_stereo: true,
                ms_stereo: false,
                copy_right: false,
                copy_of_original: false,
                emphasis: 0,
            }
        );
    }
}
