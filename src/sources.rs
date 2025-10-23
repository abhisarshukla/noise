use color_eyre::Result;
use color_eyre::eyre::{
    bail,
    eyre,
};

use crate::traits::{
    Component,
    Source,
};

pub struct SineParams {
    pub freq: f64,
}

impl Default for SineParams {
    fn default() -> Self {
        Self { freq: 440.0 }
    }
}

impl SineParams {
    pub fn parse(params: &[&str]) -> Result<Self> {
        let mut result = Self::default();
        for param in params {
            let kv: Vec<&str> = param.split('=').collect();
            if kv.len() != 2 {
                bail!("Invalid parameter format: {}", param);
            }
            match kv[0] {
                "freq" => result.freq = kv[1].parse().map_err(|_| eyre!("Invalid freq value"))?,
                _ => bail!("Unknown parameter: {}", kv[0]),
            }
        }
        Ok(result)
    }
}

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
        let params = SineParams::parse(&parts[1..])?;
        Ok(Self::new(params.freq))
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
