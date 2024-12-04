#![allow(unused_variables, dead_code)]

mod buffer;
mod error;
mod frame;
mod header;
mod side_info;

use buffer::Buffer;
use error::ErrorType;
use header::Header;

use crate::frame::Frame;

fn main() {
    let mut frames: Vec<Frame> = Vec::new();
    let mut buffer = Buffer::create_buffer_from_file("mp3-examples/test_data_100kb.mp3");

    loop {
        if let Err(err) = buffer.set_pos_next_frame() {
            match err {
                ErrorType::OutOfIndex => break,
                _ => {
                    eprintln!("pos {}: {:?}", buffer.pos, err);
                    continue;
                }
            }
        }

        let pos = buffer.pos;
        let frame = Frame::create_from_buffer(&mut buffer);

        if let Err(err) = frame {
            eprintln!("pos {}: {:?}", pos, err);
            continue;
        }

        let pos = buffer.pos;
        if let Err(err) = frame.as_ref().unwrap().header().validate_header() {
            eprintln!("pos {}: {:?}", pos, err);
            continue;
        }

        frames.push(frame.unwrap());
    }

    println!("number of frames: {}", frames.len());
}
