use color_eyre::Result;
use color_eyre::eyre::bail;

use crate::traits::{
    Analyser,
    Component,
};

pub struct PeakAnalyser {
    last_peak: Option<f64>,
}

impl PeakAnalyser {
    pub fn new() -> Self {
        Self { last_peak: None }
    }

    pub fn from_spec(spec: &str) -> Result<Self> {
        let parts: Vec<&str> = spec.split(':').collect();
        if parts[0] != "peak" {
            bail!("Not a peak spec");
        }
        if parts.len() != 1 {
            bail!("peak takes no params: peak");
        }
        Ok(Self::new())
    }
}

impl Analyser for PeakAnalyser {
    type Output = f64;

    fn analyze(&mut self, samples: &[f64]) -> Self::Output {
        let peak = samples.iter().cloned().fold(0.0, f64::max);
        self.last_peak = Some(peak);
        peak
    }

    fn get_result(&mut self) -> Option<Self::Output> {
        self.last_peak.take()
    }
}

impl Component for PeakAnalyser {
    fn process(&mut self, buffer: &mut Vec<f64>, _duration: f64, _sample_rate: f64) -> Result<()> {
        if buffer.is_empty() {
            bail!("Analyser requires input samples");
        }
        self.analyze(buffer);
        Ok(())
    }
}
