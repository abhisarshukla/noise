use crate::traits::{Component, Source};
use crate::factory::create_component;
use color_eyre::eyre::{Result, bail};

pub struct Parallel {
    pub components: Vec<Box<dyn Component>>,
}

impl Parallel {
    pub fn new(components: Vec<Box<dyn Component>>) -> Self {
        Self { components }
    }

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
        let comp_specs: Vec<&str> = inner.split(',').map(|s| s.trim()).collect();
        let mut components = Vec::new();
        for c_spec in comp_specs {
            if c_spec.is_empty() {
                continue;
            }
            let comp = create_component(c_spec)?;
            components.push(comp);
        }
        Ok(Self::new(components))
    }
}

impl Source for Parallel {
    fn generate(&self, duration: f64, sample_rate: f64) -> Vec<f64> {
        let num_samples = (duration * sample_rate) as usize;
        let mut mixed = vec![0.0; num_samples];
        for component in &self.components {
            if let Some(samples) = component.get_samples(duration, sample_rate) {
                for (i, &sample) in samples.iter().enumerate() {
                    if i < mixed.len() {
                        mixed[i] += sample;
                    }
                }
            }
        }
        // Normalize by number of source components to prevent clipping
        let num_sources = self.components.iter().filter(|c| c.is_source()).count() as f64;
        if num_sources > 0.0 {
            for sample in &mut mixed {
                *sample /= num_sources;
            }
        }
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
