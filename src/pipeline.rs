use std::fs;
use std::path::Path;

use askama::Template;
use base64::Engine as _;
use base64::engine::general_purpose;
use color_eyre::Result;
use color_eyre::eyre::bail;
use tracing::{
    Level,
    debug,
    info,
    instrument,
    span,
};

use crate::traits::Component;

#[derive(Debug, Clone)]
pub struct AnalysisResult {
    pub analyser: String,
    pub value: String,
}

#[derive(Template)]
#[template(path = "pipeline.html")]
pub struct PipelineTemplate {
    pub pipeline_name: String,
    pub duration: f64,
    pub sample_rate: f64,
    pub total_samples: usize,
    pub audio_base64: String,
    pub component_htmls: Vec<String>,
}

impl PipelineTemplate {
    pub fn new(
        pipeline_name: String,
        duration: f64,
        sample_rate: f64,
        total_samples: usize,
        component_htmls: Vec<String>,
        wav_data: &[u8],
    ) -> Self {
        let audio_base64 = general_purpose::STANDARD.encode(wav_data);

        Self { pipeline_name, duration, sample_rate, total_samples, audio_base64, component_htmls }
    }

    pub fn render_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let html = self.render()?;
        fs::write(path, html)?;
        Ok(())
    }
}

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
            let _span =
                span!(Level::DEBUG, "process_component", index = i, buffer_len = %buffer.len())
                    .entered();
            debug!("Processing component {} (buffer has {} samples)", i, buffer.len());

            component.process(&mut buffer, duration, sample_rate)?;

            debug!("Component {} processed, buffer now has {} samples", i, buffer.len());
        }

        info!("Pipeline completed with {} samples", buffer.len());
        Ok(buffer)
    }

    pub fn run_with_tracking(
        &mut self,
        duration: f64,
        sample_rate: f64,
    ) -> Result<(Vec<f64>, Vec<String>)> {
        info!("Running pipeline with tracking enabled");
        let mut buffer = Vec::new();
        let mut component_htmls = Vec::new();
        let total = self.components.len();

        for (i, component) in self.components.iter_mut().enumerate() {
            let input_buffer = buffer.clone();

            component.process(&mut buffer, duration, sample_rate)?;

            // Let each component render its own HTML
            let html = component.render_html(&input_buffer, &buffer, i + 1, total)?;
            component_htmls.push(html);
        }

        Ok((buffer, component_htmls))
    }
}
