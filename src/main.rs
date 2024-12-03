#![allow(unused_variables, dead_code)]

mod buffer;
mod error;
mod frame;
mod header;
mod side_info;

use buffer::Buffer;
use header::Header;

use crate::frame::Frame;

fn main() {
    let mut buffer = Buffer::create_buffer_from_file("mp3-examples/test_data_100kb.mp3");
    buffer.set_pos(14192).unwrap();

    let frame = Frame::create_from_buffer(&mut buffer);
    if frame.header().validate_header().is_err() {
        panic!("Header is not valid");
    }

    println!("header: {}\n\n", &frame.header());
    println!("{}\n\n", &frame.side_info);
}
