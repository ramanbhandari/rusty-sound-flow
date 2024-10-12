# Rusty Sound Flow

Rusty-Sound-Flow is a real-time audio visualization tool written in Rust, as the name suggests. It captures data from either a live microphone input or an audio file and displays visualizations of the waveform using a graphics window.

## Getting Started
### Pre-requisites
- Rust
- Cargo (installed with Rust)

### Installation

Clone the Repository:
```bash
git clone https://github.com/ramanbhandari/rusty-sound-flow.git
cd rusty-sound-flow
```
Build the Project:
```bash
cargo build --release
```

## Running the Application
### Run with microphone input
By default, this project will use your system's microphone
```bash
cargo run
```

### Run with Audio File
You can also visualize with an audio file, currently supporting only `.wav`
```bash
cargo run -- --file path/to/audio/file.wav
```






