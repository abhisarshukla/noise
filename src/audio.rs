use color_eyre::Result;
use color_eyre::eyre::bail;
use hound;
use tracing::{instrument, info, debug, span, Level};

use crate::traits::Component;

pub struct Pipeline {
    components: Vec<Box<dyn Component>>,
}

impl Pipeline {
    #[instrument(level = "debug")]
    pub fn new() -> Self {
        debug!("Creating new pipeline");
        Self { components: Vec::new() }
    }

    #[instrument(skip(self, component), fields(is_source = %component.is_source()))]
    pub fn add_component(&mut self, component: Box<dyn Component>) -> Result<()> {
        if self.components.is_empty() && !component.is_source() {
            bail!("Pipeline must start with a Source");
        }
        debug!("Adding component to pipeline (total: {})", self.components.len() + 1);
        self.components.push(component);
        Ok(())
    }

    #[instrument(skip(self), fields(duration = %duration, sample_rate = %sample_rate, num_components = %self.components.len()))]
    pub fn run(&mut self, duration: f64, sample_rate: f64) -> Result<Vec<f64>> {
        info!("Running pipeline with {} components", self.components.len());
        let mut buffer = Vec::new();

        for (i, component) in self.components.iter_mut().enumerate() {
            let _span = span!(Level::DEBUG, "process_component", index = i, buffer_len = %buffer.len()).entered();
            debug!("Processing component {} (buffer has {} samples)", i, buffer.len());

            component.process(&mut buffer, duration, sample_rate)?;

            debug!("Component {} processed, buffer now has {} samples", i, buffer.len());
        }

        info!("Pipeline completed with {} samples", buffer.len());
        Ok(buffer)
    }
}

#[instrument(skip(samples), fields(filename = %filename, num_samples = %samples.len(), sample_rate = %sample_rate))]
pub fn write_wav(filename: &str, samples: &[f64], sample_rate: f64) -> Result<(), hound::Error> {
    debug!("Creating WAV file with {} samples at {}Hz", samples.len(), sample_rate);

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: sample_rate as u32,
        bits_per_sample: 24,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create(filename, spec)?;
    debug!("WAV writer created for {}", filename);

    for (i, &sample) in samples.iter().enumerate() {
        let pcm = (sample * 8388607.0) as i32;
        writer.write_sample(pcm)?;

        // Log progress for very long samples
        if samples.len() > 100000 && i % 50000 == 0 {
            debug!("Written {}/{} samples", i, samples.len());
        }
    }

    debug!("All samples written, finalizing file");
    writer.finalize()?;
    info!("WAV file written successfully: {}", filename);
    Ok(())
}
