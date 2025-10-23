use crate::traits::{Component, Analyser};
use std::error::Error;

pub struct PeakAnalyser {
    last_peak: Option<f64>,
}

impl PeakAnalyser {
    pub fn new() -> Self {
        Self { last_peak: None }
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
    fn process(&mut self, buffer: &mut Vec<f64>, _duration: f64, _sample_rate: f64) -> Result<(), Box<dyn Error>> {
        if buffer.is_empty() {
            return Err("Analyser requires input samples".into());
        }
        self.analyze(buffer);
        Ok(())
    }

    fn get_analyser_result(&mut self) -> Option<f64> {
        self.get_result()
    }
}
