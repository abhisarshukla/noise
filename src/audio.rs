use color_eyre::Result;
use color_eyre::eyre::bail;
use hound;

use crate::traits::Component;

pub struct Pipeline {
    components: Vec<Box<dyn Component>>,
}

impl Pipeline {
    pub fn new() -> Self {
        Self { components: Vec::new() }
    }

    pub fn add_component(&mut self, component: Box<dyn Component>) -> Result<()> {
        if self.components.is_empty() && !component.is_source() {
            bail!("Pipeline must start with a Source");
        }
        self.components.push(component);
        Ok(())
    }

    pub fn run(&mut self, duration: f64, sample_rate: f64) -> Result<Vec<f64>> {
        let mut buffer = Vec::new();
        for component in &mut self.components {
            component.process(&mut buffer, duration, sample_rate)?;
        }
        Ok(buffer)
    }
}

pub fn write_wav(filename: &str, samples: &[f64], sample_rate: f64) -> Result<(), hound::Error> {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: sample_rate as u32,
        bits_per_sample: 24,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create(filename, spec)?;

    for &sample in samples {
        let pcm = (sample * 8388607.0) as i32;
        writer.write_sample(pcm)?;
    }

    writer.finalize()?;
    Ok(())
}
