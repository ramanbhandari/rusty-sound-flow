use rustfft::{num_complex::Complex, num_traits::Zero, FftPlanner};

pub struct Processor {
    fft_planner: FftPlanner<f32>,
}

impl Processor {
    pub fn new() -> Self {
        Self {
            fft_planner: FftPlanner::new(),
        }
    }

    pub fn process(&mut self, samples: &[f32]) -> Vec<f32> {
        if samples.is_empty() {
            return Vec::new();
        }

        // Zero-pad or truncate samples to a power of two
        let n = samples.len().next_power_of_two();
        let mut buffer: Vec<Complex<f32>> = samples
            .iter()
            .cloned()
            .map(|s| Complex { re: s, im: 0.0 })
            .collect();
        buffer.resize(n, Complex::zero());

        // Perform FFT
        let fft = self.fft_planner.plan_fft_forward(n);
        fft.process(&mut buffer);

        // Compute magnitude spectrum
        buffer.iter().map(|c| c.norm()).collect()
    }
}
