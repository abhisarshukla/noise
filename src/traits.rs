pub trait Source {
    fn generate(&self, duration: f64, sample_rate: f64) -> Vec<f64>;
}

pub trait Processor {
    fn process(&mut self, samples: &mut [f64]);
}

pub trait Analyser {
    type Output;
    fn analyze(&self, samples: &[f64]) -> Self::Output;
}
