use anyhow::Result;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{SampleFormat, SizedSample};
use std::sync::{Arc, Mutex};

pub struct AudioInput {
    samples: Arc<Mutex<Vec<f32>>>,
    #[allow(dead_code)]
    stream: cpal::Stream,
}

impl AudioInput {
    pub fn new() -> Result<Self> {
        let samples = Arc::new(Mutex::new(Vec::new()));

        // Set up the audio input device and stream
        let host = cpal::default_host();
        let device = host.default_input_device().expect("No input device available");
        let config = device.default_input_config()?;

        let sample_format = config.sample_format();
        let config: cpal::StreamConfig = config.into();

        let samples_clone = samples.clone();

        // Build the input stream
        let err_fn = move |err| {
            eprintln!("Stream error: {}", err);
        };

        let stream = match sample_format {
            SampleFormat::F32 => Self::build_input_stream::<f32>(&device, &config, samples_clone, err_fn)?,
            SampleFormat::I16 => Self::build_input_stream::<i16>(&device, &config, samples_clone, err_fn)?,
            SampleFormat::U16 => Self::build_input_stream::<u16>(&device, &config, samples_clone, err_fn)?,
            _ => panic!("Unsupported sample format"),
        };

        // Start the stream
        stream.play()?;

        Ok(Self { samples, stream })
    }

    fn build_input_stream<T>(
        device: &cpal::Device,
        config: &cpal::StreamConfig,
        samples: Arc<Mutex<Vec<f32>>>,
        err_fn: impl FnMut(cpal::StreamError) + Send + 'static,
    ) -> Result<cpal::Stream>
    where
        T: cpal::Sample + SizedSample + Into<f32>,
    {
        let data_fn = move |data: &[T], _: &cpal::InputCallbackInfo| {
            let mut buffer = samples.lock().unwrap();
            buffer.clear();
            buffer.extend(data.iter().map(|&s| s.into()));
        };

        let stream = device.build_input_stream(
            config,
            data_fn,
            err_fn,
            None, // This is the missing argument
        )?;

        Ok(stream)
    }

    pub fn get_samples(&self) -> Vec<f32> {
        let data = self.samples.lock().unwrap();
        data.clone()
    }
}