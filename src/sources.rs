use color_eyre::Result;
use color_eyre::eyre::{
    bail,
    eyre,
};

use crate::traits::{
    Component,
    Source,
};

pub struct SineWaveSource {
    pub frequency: f64,
}

impl SineWaveSource {
    pub fn new(frequency: f64) -> Self {
        Self { frequency }
    }

    pub fn from_spec(spec: &str) -> Result<Self> {
        let parts: Vec<&str> = spec.split(':').collect();
        if parts[0] != "sine" {
            bail!("Not a sine spec");
        }
        if parts.len() != 2 {
            bail!("sine requires frequency: sine:440");
        }
        let freq = parts[1].parse().map_err(|_| eyre!("Invalid frequency"))?;
        Ok(Self::new(freq))
    }
}

impl Source for SineWaveSource {
    fn generate(&self, duration: f64, sample_rate: f64) -> Vec<f64> {
        generate_sine_wave(self.frequency, duration, sample_rate)
    }
}

impl Component for SineWaveSource {
    fn process(&mut self, buffer: &mut Vec<f64>, duration: f64, sample_rate: f64) -> Result<()> {
        *buffer = self.generate(duration, sample_rate);
        Ok(())
    }

    fn is_source(&self) -> bool {
        true
    }
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
