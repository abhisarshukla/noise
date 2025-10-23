use crate::traits::{Component, Source};
use std::error::Error;

pub struct SineWaveSource {
    pub frequency: f64,
}

impl SineWaveSource {
    pub fn new(frequency: f64) -> Self {
        Self { frequency }
    }
}

impl Source for SineWaveSource {
    fn generate(&self, duration: f64, sample_rate: f64) -> Vec<f64> {
        generate_sine_wave(self.frequency, duration, sample_rate)
    }
}

impl Component for SineWaveSource {
    fn process(&mut self, buffer: &mut Vec<f64>, duration: f64, sample_rate: f64) -> Result<(), Box<dyn Error>> {
        *buffer = self.generate(duration, sample_rate);
        Ok(())
    }

    fn is_source(&self) -> bool { true }
}

pub fn generate_sine_wave(frequency: f64, duration: f64, sample_rate: f64) -> Vec<f64> {
    let num_samples = (duration * sample_rate) as usize;
    let mut samples = Vec::with_capacity(num_samples);
    let angular_frequency = 2.0 * std::f64::consts::PI * frequency;

    for i in 0..num_samples {
        let t = i as f64 / sample_rate;
        let sample = (angular_frequency * t).sin();
        samples.push(sample);
    }

    samples
}
