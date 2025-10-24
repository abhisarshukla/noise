use crate::traits::{Component, Source};
use crate::factory::create_component;
use color_eyre::eyre::{Result, bail};
use tracing::{instrument, debug, info, span, Level};

pub struct Parallel {
    pub components: Vec<Box<dyn Component>>,
}

impl Parallel {
    #[instrument(skip(components), fields(num_components = %components.len()))]
    pub fn new(components: Vec<Box<dyn Component>>) -> Self {
        debug!("Creating parallel composite with {} components", components.len());
        Self { components }
    }

    #[instrument(level = "debug")]
    pub fn from_spec(spec: &str) -> Result<Self> {
        if !spec.starts_with("parallel:") {
            bail!("Not a parallel spec");
        }
        let list = &spec["parallel:".len()..];
        if !list.starts_with('[') || !list.ends_with(']') {
            bail!("parallel specs must be enclosed in [ ]");
        }
        let inner = &list[1..list.len() - 1];
        if inner.is_empty() {
            bail!("parallel requires at least one component spec");
        }

        debug!("Parsing parallel component specs from: {}", inner);
        let comp_specs: Vec<&str> = inner.split(',').map(|s| s.trim()).collect();
        debug!("Found {} component specs in parallel", comp_specs.len());

        let mut components = Vec::new();
        for (i, c_spec) in comp_specs.iter().enumerate() {
            if c_spec.is_empty() {
                continue;
            }
            let _span = span!(Level::DEBUG, "create_parallel_component", index = i, spec = %c_spec).entered();
            debug!("Creating parallel component {}: {}", i, c_spec);
            let comp = create_component(c_spec)?;
            components.push(comp);
        }

        info!("Created parallel with {} components", components.len());
        Ok(Self::new(components))
    }
}

impl Source for Parallel {
    #[instrument(skip(self), fields(duration = %duration, sample_rate = %sample_rate, num_components = %self.components.len()))]
    fn generate(&self, duration: f64, sample_rate: f64) -> Vec<f64> {
        let num_samples = (duration * sample_rate) as usize;
        debug!("Generating {} samples from {} parallel components", num_samples, self.components.len());

        let mut mixed = vec![0.0; num_samples];

        for (i, component) in self.components.iter().enumerate() {
            let _span = span!(Level::DEBUG, "mix_component", index = i).entered();

            if let Some(samples) = component.get_samples(duration, sample_rate) {
                debug!("Mixing component {} with {} samples", i, samples.len());
                for (j, &sample) in samples.iter().enumerate() {
                    if j < mixed.len() {
                        mixed[j] += sample;
                    }
                }
            } else {
                debug!("Component {} provided no samples", i);
            }
        }

        // Normalize by number of source components to prevent clipping
        let num_sources = self.components.iter().filter(|c| c.is_source()).count() as f64;
        if num_sources > 0.0 {
            debug!("Normalizing by {} source components", num_sources);
            for sample in &mut mixed {
                *sample /= num_sources;
            }
        }

        debug!("Parallel generation complete, {} samples", mixed.len());
        mixed
    }
}

impl Component for Parallel {
    fn process(&mut self, buffer: &mut Vec<f64>, duration: f64, sample_rate: f64) -> Result<()> {
        *buffer = self.generate(duration, sample_rate);
        Ok(())
    }

    fn is_source(&self) -> bool {
        true
    }

    fn get_samples(&self, duration: f64, sample_rate: f64) -> Option<Vec<f64>> {
        Some(self.generate(duration, sample_rate))
    }
}
