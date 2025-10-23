use std::error::Error;

pub trait Component {
    fn process(&mut self, buffer: &mut Vec<f64>, duration: f64, sample_rate: f64) -> Result<(), Box<dyn Error>>;
    fn is_source(&self) -> bool { false }
    fn get_analyser_result(&mut self) -> Option<f64> { None }
}

pub trait Source {
    fn generate(&self, duration: f64, sample_rate: f64) -> Vec<f64>;
}

pub trait Processor {
    fn process(&mut self, samples: &mut [f64]);
}

pub trait Analyser {
    type Output;
    fn analyze(&mut self, samples: &[f64]) -> Self::Output;
    fn get_result(&mut self) -> Option<Self::Output>;
}
