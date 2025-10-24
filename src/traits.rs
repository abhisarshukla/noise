use color_eyre::eyre::Result;

pub trait Component {
    fn process(&mut self, buffer: &mut Vec<f64>, duration: f64, sample_rate: f64) -> Result<()>;
    fn is_source(&self) -> bool {
        false
    }
    fn get_samples(&self, _duration: f64, _sample_rate: f64) -> Option<Vec<f64>> {
        None
    }

    fn render_html(
        &self,
        input_samples: &[f64],
        output_samples: &[f64],
        index: usize,
        total: usize,
    ) -> Result<String>;

    fn name(&self) -> String;

    fn component_type(&self) -> &'static str;
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
