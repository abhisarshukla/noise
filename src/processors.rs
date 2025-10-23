use color_eyre::Result;
use color_eyre::eyre::{
    bail,
    eyre,
};

use crate::traits::{
    Component,
    Processor,
};

pub struct VolumeProcessor {
    pub volume: f64,
}

impl VolumeProcessor {
    pub fn new(volume: f64) -> Self {
        Self { volume }
    }

    pub fn from_spec(spec: &str) -> Result<Self> {
        let parts: Vec<&str> = spec.split(':').collect();
        if parts[0] != "volume" {
            bail!("Not a volume spec");
        }
        if parts.len() != 2 {
            bail!("volume requires level: volume:0.5");
        }
        let volume = parts[1].parse().map_err(|_| eyre!("Invalid volume"))?;
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
    fn process(&mut self, buffer: &mut Vec<f64>, _duration: f64, _sample_rate: f64) -> Result<()> {
        if buffer.is_empty() {
            bail!("Processor requires input samples");
        }
        Processor::process(self, buffer);
        Ok(())
    }
}
