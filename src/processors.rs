use crate::traits::{Component, Processor};
use std::error::Error;

pub struct VolumeProcessor {
    pub volume: f64,
}

impl VolumeProcessor {
    pub fn new(volume: f64) -> Self {
        Self { volume }
    }

    pub fn from_spec(spec: &str) -> Result<Self, String> {
        let parts: Vec<&str> = spec.split(':').collect();
        if parts[0] != "volume" {
            return Err("Not a volume spec".to_string());
        }
        if parts.len() != 2 {
            return Err("volume requires level: volume:0.5".to_string());
        }
        let volume = parts[1].parse().map_err(|_| "Invalid volume".to_string())?;
        Ok(Self::new(volume))
    }
}

impl Processor for VolumeProcessor {
    fn process(&mut self, samples: &mut [f64]) {
        for sample in samples.iter_mut() {
            *sample *= self.volume;
        }
    }
}

impl Component for VolumeProcessor {
    fn process(&mut self, buffer: &mut Vec<f64>, _duration: f64, _sample_rate: f64) -> Result<(), Box<dyn Error>> {
        if buffer.is_empty() {
            return Err("Processor requires input samples".into());
        }
        Processor::process(self, buffer);
        Ok(())
    }
}
