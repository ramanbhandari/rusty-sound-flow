use anyhow::Result;
use env_logger;
use log::info;
use clap::Parser;

mod audio;
mod processing;
mod rendering;

use audio::AudioInput;
use processing::Processor;
use rendering::Renderer;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// path to the audio file (WAV format), if not provided, uses live microphone input
    #[arg(short, long)]
    file: Option<String>,
}

fn main() -> Result<()> {
    // initialize the logger
    env_logger::init();

    // parse command-line arguments
    let args = Args::parse();

    // create the event loop and window for rendering
    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::Window::new(&event_loop)?;

    // initialize the audio input (live or from file)
    let mut audio_input = AudioInput::new(args.file)?;

    // initialize the processor
    let mut processor = Processor::new();

    // initialize the renderer
    let mut renderer = futures::executor::block_on(Renderer::new(&window))?;

    info!("Starting the audio visualization tool...");

    // start the event loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = winit::event_loop::ControlFlow::Poll;

        // handle events here
        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => {
                    *control_flow = winit::event_loop::ControlFlow::Exit
                }
                _ => {}
            },
            winit::event::Event::MainEventsCleared => {
                // get audio samples
                let samples = audio_input.get_samples();

                // process samples (e.g., compute FFT)
                let frequency_data = processor.process(&samples);

                // render visualization
                if let Err(e) = renderer.render(&samples, &frequency_data) {
                    eprintln!("Render error: {}", e);
                }
            }
            _ => {}
        }
    });

}