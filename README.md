# MP3-Info

**MP3-Info** is a simple command-line tool written in Rust for inspecting the header and side information of MP3 file frames. This is a toy project created to practice Rust programming and explore the structure of MP3 files.

## Features

- **MP3 Frame Inspection**: Extracts and displays header and side information from MP3 files.
- **Lightweight CLI**: Easy-to-use interface for quick analysis of MP3 files.
- **Rust Practice**: Built as a learning exercise to deepen understanding of Rust and low-level file parsing.

## Usage

To use MP3-Info, you need to have [Rust](https://www.rust-lang.org/tools/install) installed. Follow these steps:

1. **Clone the Repository**:
   ```bash
   git clone https://github.com/LowLevelLover/mp3-info.git
   cd mp3-info
   ```

2. **Run the Tool**:
   Inspect an MP3 file by specifying its path:
   ```bash
   cargo run -- --file <PATH_TO_MP3_FILE>
   ```
   Example:
   ```bash
   cargo run -- --file mp3-examples/test_data_100kb.mp3
   ```

3. **View Help**:
   For additional options and usage details:
   ```bash
   cargo run -- --help
   ```

## Installation

Ensure you have Rust installed. You can install it using:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then, build the project:
```bash
cargo build
```

## Contributing

This is a toy project, but contributions are welcome! If you'd like to enhance MP3-Info, feel free to:

- Report bugs or suggest features by opening an issue.
- Submit a pull request with your improvements.

Please ensure your code aligns with the project's style and includes relevant tests.

## License

This project is licensed under the [MIT License](LICENSE). See the LICENSE file for details.

## About

MP3-Info is a learning project by [LowLevelLover](https://github.com/LowLevelLover) to practice Rust and explore MP3 file structures. It’s a simple tool for inspecting MP3 frame headers and side information, built for fun and education.
