#![allow(unused_variables, dead_code)]

mod buffer;
mod error;
mod frame;
mod header;
mod side_info;

use std::{path::PathBuf, process::exit};

use buffer::Buffer;
use header::Header;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// MP3 file location
    #[arg(short, long)]
    file: String,

    /// Number of frames
    #[arg(short, long, default_value_t = true)]
    count: bool,

    /// Frame's header and side info
    #[arg(long, name = "FRAME NUMBER")]
    frame: Option<usize>,
}

fn main() {
    let args = Args::parse();

    if !PathBuf::from(&args.file).exists() {
        eprintln!("\n`{}` does not exist.", &args.file);
    }

    let mut buffer = Buffer::create_buffer_from_file(&args.file);
    let frames = buffer.extract_frames();

    if args.count {
        println!("\nNumber of frames: {}\n", frames.len());
    }

    if let Some(frame_number) = args.frame {
        if frame_number >= frames.len() {
            eprintln!("Frame Number is not in range: 0-{}", frames.len() - 1);
            exit(-1);
        }

        println!("Header:\n{}", frames[frame_number].header());
        println!("Side Info:\n{}", frames[frame_number].side_info());
    }
}
