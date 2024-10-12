use anyhow::Result;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{SampleFormat, SizedSample};
use hound;
use std::sync::{Arc, Mutex};
use std::time::{Instant};

pub enum AudioSource {
    LiveInput,
    FileInput {
        samples: Vec<f32>,
        sample_rate: u32,
    },
}

pub struct AudioInput {
    samples: Arc<Mutex<Vec<f32>>>,
    #[allow(dead_code)]
    stream: Option<cpal::Stream>,
    source: AudioSource,
    file_position: usize,
    last_update: Instant,
}

impl AudioInput {
    pub fn new(file_path: Option<String>) -> Result<Self> {
        let samples = Arc::new(Mutex::new(Vec::new()));
        let mut stream = None;

        let source = if let Some(path) = file_path {
            // Read audio file
            let reader = hound::WavReader::open(path)?;
            let spec = reader.spec();
            let sample_rate = spec.sample_rate;
            let samples_iter = reader.into_samples::<i16>();
            let mut audio_samples = Vec::new();

            for sample in samples_iter {
                let s = sample?;
                let s_norm = s as f32 / i16::MAX as f32;
                audio_samples.push(s_norm);
            }

            // Set up file input source
            AudioSource::FileInput {
                samples: audio_samples,
                sample_rate,
            }
        } else {
            // Set up live input source
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

            let audio_stream = match sample_format {
                SampleFormat::F32 => Self::build_input_stream::<f32>(&device, &config, samples_clone, err_fn)?,
                SampleFormat::I16 => Self::build_input_stream::<i16>(&device, &config, samples_clone, err_fn)?,
                SampleFormat::U16 => Self::build_input_stream::<u16>(&device, &config, samples_clone, err_fn)?,
                _ => panic!("Unsupported sample format"),
            };

            // Start the stream
            audio_stream.play()?;
            stream = Some(audio_stream);

            AudioSource::LiveInput
        };

        Ok(Self {
            samples,
            stream,
            source,
            file_position: 0,
            last_update: Instant::now(),
        })
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
            None,
        )?;

        Ok(stream)
    }

    pub fn get_samples(&mut self) -> Vec<f32> {
        match &self.source {
            AudioSource::LiveInput => {
                let data = self.samples.lock().unwrap();
                data.clone()
            }
            AudioSource::FileInput {
                samples: audio_samples,
                sample_rate,
            } => {
                // Calculate how many samples to read based on time elapsed
                let elapsed = self.last_update.elapsed();
                let samples_to_read = (*sample_rate as f32 * elapsed.as_secs_f32()) as usize;
                self.last_update = Instant::now();

                let start = self.file_position;
                let end = (self.file_position + samples_to_read).min(audio_samples.len());
                let samples = audio_samples[start..end].to_vec();
                self.file_position = end % audio_samples.len();

                samples
            }
        }
    }
}