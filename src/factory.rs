use crate::traits::Component;
use crate::sources::{SineWaveSource, SquareWaveSource};
use crate::processors::VolumeProcessor;
use crate::analysers::PeakAnalyser;
use crate::composite::Parallel;
use color_eyre::eyre::{Result, bail};
use tracing::instrument;

#[instrument]
pub fn create_component(spec: &str) -> Result<Box<dyn Component>> {
    let parts: Vec<&str> = spec.split(':').collect();
    let comp: Box<dyn Component> = match parts[0] {
        "sine" => Box::new(SineWaveSource::from_spec(spec)?),
        "square" => Box::new(SquareWaveSource::from_spec(spec)?),
        "parallel" => Box::new(Parallel::from_spec(spec)?),
        "volume" => Box::new(VolumeProcessor::from_spec(spec)?),
        "peak" => Box::new(PeakAnalyser::from_spec(spec)?),
        _ => bail!("Unknown component type: {}", parts[0]),
    };
    Ok(comp)
}
