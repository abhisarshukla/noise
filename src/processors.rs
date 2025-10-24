use color_eyre::Result;
use color_eyre::eyre::{
    bail,
    eyre,
};

use crate::traits::{
    Component,
    Processor,
};
use tracing::{instrument, debug, info};

pub struct VolumeParams {
    pub level: f64,
}

impl Default for VolumeParams {
    fn default() -> Self {
        Self { level: 1.0 }
    }
}

impl VolumeParams {
    #[instrument]
    pub fn parse(params: &[&str]) -> Result<Self> {
        let mut result = Self::default();
        for param in params {
            let kv: Vec<&str> = param.split('=').collect();
            if kv.len() != 2 {
                bail!("Invalid parameter format: {}", param);
            }
            match kv[0] {
                "level" => result.level = kv[1].parse().map_err(|_| eyre!("Invalid level value"))?,
                _ => bail!("Unknown parameter: {}", kv[0]),
            }
        }
        Ok(result)
    }
}

pub struct VolumeProcessor {
    pub volume: f64,
}

impl VolumeProcessor {
    #[instrument(level = "debug", fields(volume = %volume))]
    pub fn new(volume: f64) -> Self {
        debug!("Creating volume processor with level: {:.2}", volume);
        Self { volume }
    }

    #[instrument(level = "debug")]
    pub fn from_spec(spec: &str) -> Result<Self> {
        let parts: Vec<&str> = spec.split(':').collect();
        if parts[0] != "volume" {
            bail!("Not a volume spec");
        }
        let params = VolumeParams::parse(&parts[1..])?;
        info!("Volume processor created with level: {:.2}", params.level);
        Ok(Self::new(params.level))
    }
}

impl Processor for VolumeProcessor {
    #[instrument(skip(self, samples), fields(num_samples = %samples.len(), volume = %self.volume))]
    fn process(&mut self, samples: &mut [f64]) {
        debug!("Applying volume {:.2} to {} samples", self.volume, samples.len());
        for sample in samples.iter_mut() {
            *sample *= self.volume;
        }
        debug!("Volume processing complete");
    }
}

impl Component for VolumeProcessor {
    #[instrument(skip(self, buffer), fields(buffer_len = %buffer.len(), volume = %self.volume))]
    fn process(&mut self, buffer: &mut Vec<f64>, _duration: f64, _sample_rate: f64) -> Result<()> {
        if buffer.is_empty() {
            bail!("Processor requires input samples");
        }
        debug!("Processing {} samples through volume processor", buffer.len());
        Processor::process(self, buffer);
        Ok(())
    }
}
