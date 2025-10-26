use color_eyre::Result;
use color_eyre::eyre::bail;
use tracing::{
    debug,
    info,
    instrument,
};

use crate::traits::{
    Analyser,
    Component,
};

pub struct PeakAnalyser {
    last_peak: Option<f64>,
}

impl PeakAnalyser {
    #[instrument(level = "debug")]
    pub fn new() -> Self {
        debug!("Creating new peak analyser");
        Self { last_peak: None }
    }

    #[instrument(level = "debug")]
    pub fn from_spec(spec: &str) -> Result<Self> {
        let parts: Vec<&str> = spec.split(':').collect();
        if parts[0] != "peak" {
            bail!("Not a peak spec");
        }
        // No parameters expected for peak
        if parts.len() > 1 {
            bail!("peak takes no params: peak");
        }
        debug!("Peak analyser created from spec");
        Ok(Self::new())
    }
}

impl Analyser for PeakAnalyser {
    type Output = f64;

    #[instrument(skip(self, samples), fields(num_samples = %samples.len()))]
    fn analyze(&mut self, samples: &[f64]) -> Self::Output {
        debug!("Analyzing {} samples for peak", samples.len());
        let peak = samples.iter().cloned().fold(0.0, f64::max);
        self.last_peak = Some(peak);
        info!("Peak value found: {:.6}", peak);
        peak
    }

    fn get_result(&mut self) -> Option<Self::Output> {
        let result = self.last_peak.take();
        debug!("Returning peak result: {:?}", result);
        result
    }
}

impl Component for PeakAnalyser {
    #[instrument(skip(self, buffer), fields(buffer_len = %buffer.len()))]
    fn process(&mut self, buffer: &mut Vec<f64>, _duration: f64, _sample_rate: f64) -> Result<()> {
        if buffer.is_empty() {
            bail!("Analyser requires input samples");
        }
        debug!("Processing buffer of {} samples through peak analyser", buffer.len());
        self.analyze(buffer);
        Ok(())
    }

    fn name(&self) -> String {
        "peak".to_string()
    }

    fn component_type(&self) -> &'static str {
        "Analyser"
    }
}
