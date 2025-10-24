use color_eyre::Result;
use color_eyre::eyre::{
    bail,
    eyre,
};

use crate::traits::{
    Component,
    Source,
};
use tracing::{debug, instrument};

pub struct SineParams {
    pub freq: f64,
}

impl Default for SineParams {
    fn default() -> Self {
        Self { freq: 440.0 }
    }
}

impl SineParams {
    #[instrument]
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
    #[instrument(level = "debug", fields(frequency = %frequency))]
    pub fn new(frequency: f64) -> Self {
        debug!("Creating sine wave source at {} Hz", frequency);
        Self { frequency }
    }

    #[instrument(level = "debug")]
    pub fn from_spec(spec: &str) -> Result<Self> {
        let parts: Vec<&str> = spec.split(':').collect();
        if parts[0] != "sine" {
            bail!("Not a sine spec");
        }
        let params = SineParams::parse(&parts[1..])?;
        debug!("Sine wave source created at {} Hz from spec", params.freq);
        Ok(Self::new(params.freq))
    }
}

impl Source for SineWaveSource {
    #[instrument(skip(self), fields(frequency = %self.frequency, duration = %duration, sample_rate = %sample_rate))]
    fn generate(&self, duration: f64, sample_rate: f64) -> Vec<f64> {
        debug!("Generating sine wave: {} Hz for {} seconds at {} Hz sample rate",
               self.frequency, duration, sample_rate);
        generate_sine_wave(self.frequency, duration, sample_rate)
    }
}

impl Component for SineWaveSource {
    #[instrument(skip(self, buffer), fields(frequency = %self.frequency))]
    fn process(&mut self, buffer: &mut Vec<f64>, duration: f64, sample_rate: f64) -> Result<()> {
        debug!("Processing sine wave source");
        *buffer = self.generate(duration, sample_rate);
        debug!("Generated {} samples", buffer.len());
        Ok(())
    }

    fn is_source(&self) -> bool {
        true
    }

    fn get_samples(&self, duration: f64, sample_rate: f64) -> Option<Vec<f64>> {
        Some(self.generate(duration, sample_rate))
    }
}

#[instrument(level = "debug", fields(frequency = %frequency, duration = %duration, sample_rate = %sample_rate))]
pub fn generate_sine_wave(frequency: f64, duration: f64, sample_rate: f64) -> Vec<f64> {
    let num_samples = (duration * sample_rate) as usize;
    debug!("Generating {} sine wave samples at {} Hz", num_samples, frequency);

    let mut samples = Vec::with_capacity(num_samples);
    let angular_frequency = 2.0 * std::f64::consts::PI * frequency;

    for i in 0..num_samples {
        let t = i as f64 / sample_rate;
        let sample = (angular_frequency * t).sin();
        samples.push(sample);
    }

    debug!("Sine wave generation complete: {} samples", samples.len());
    samples
}

pub struct SquareParams {
    pub freq: f64,
}

impl Default for SquareParams {
    fn default() -> Self {
        Self { freq: 440.0 }
    }
}

impl SquareParams {
    #[instrument]
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

pub struct SquareWaveSource {
    pub frequency: f64,
}

impl SquareWaveSource {
    #[instrument(level = "debug", fields(frequency = %frequency))]
    pub fn new(frequency: f64) -> Self {
        debug!("Creating square wave source at {} Hz", frequency);
        Self { frequency }
    }

    #[instrument(level = "debug")]
    pub fn from_spec(spec: &str) -> Result<Self> {
        let parts: Vec<&str> = spec.split(':').collect();
        if parts[0] != "square" {
            bail!("Not a square spec");
        }
        let params = SquareParams::parse(&parts[1..])?;
        debug!("Square wave source created at {} Hz from spec", params.freq);
        Ok(Self::new(params.freq))
    }
}

impl Source for SquareWaveSource {
    #[instrument(skip(self), fields(frequency = %self.frequency, duration = %duration, sample_rate = %sample_rate))]
    fn generate(&self, duration: f64, sample_rate: f64) -> Vec<f64> {
        debug!("Generating square wave: {} Hz for {} seconds at {} Hz sample rate",
               self.frequency, duration, sample_rate);
        generate_square_wave(self.frequency, duration, sample_rate)
    }
}

impl Component for SquareWaveSource {
    #[instrument(skip(self, buffer), fields(frequency = %self.frequency))]
    fn process(&mut self, buffer: &mut Vec<f64>, duration: f64, sample_rate: f64) -> Result<()> {
        debug!("Processing square wave source");
        *buffer = self.generate(duration, sample_rate);
        debug!("Generated {} samples", buffer.len());
        Ok(())
    }

    fn is_source(&self) -> bool {
        true
    }

    fn get_samples(&self, duration: f64, sample_rate: f64) -> Option<Vec<f64>> {
        Some(self.generate(duration, sample_rate))
    }
}

#[instrument(level = "debug", fields(frequency = %frequency, duration = %duration, sample_rate = %sample_rate))]
pub fn generate_square_wave(frequency: f64, duration: f64, sample_rate: f64) -> Vec<f64> {
    let num_samples = (duration * sample_rate) as usize;
    debug!("Generating {} samples of square wave at {} Hz", num_samples, frequency);

    let mut samples = Vec::with_capacity(num_samples);
    let period_samples = (sample_rate / frequency) as usize;

    for i in 0..num_samples {
        let phase = (i % period_samples) as f64 / period_samples as f64;
        let sample = if phase < 0.5 { 1.0 } else { -1.0 };
        samples.push(sample);
    }

    debug!("Square wave generation complete: {} samples", samples.len());
    samples
}


