use anyhow::Result;
use env_logger;
use log::info;

mod audio;
mod processing;
mod rendering;

use audio::AudioInput;
use processing::Processor;
use rendering::Renderer;

fn main() -> Result<()> {
    // Initialize the logger
    env_logger::init();

    // Create the event loop and window for rendering
    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::Window::new(&event_loop)?;

    // Initialize the audio input (live or from file)
    let audio_input = AudioInput::new()?;

    // Initialize the processor
    let mut processor = Processor::new();

    // Initialize the renderer
    let mut renderer = futures::executor::block_on(Renderer::new(&window))?;

    info!("Starting the audio visualization tool...");

    // Start the event loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = winit::event_loop::ControlFlow::Poll;

        // Handle events here
        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => {
                    *control_flow = winit::event_loop::ControlFlow::Exit
                }
                _ => {}
            },
            winit::event::Event::MainEventsCleared => {
                // Get audio samples
                let samples = audio_input.get_samples();

                // Process samples (e.g., compute FFT)
                let frequency_data = processor.process(&samples);

                // Render the visualization
                if let Err(e) = renderer.render(&samples, &frequency_data) {
                    eprintln!("Render error: {}", e);
                }
            }
            _ => {}
        }
    });
}