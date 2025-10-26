use clap::Parser;
use color_eyre::eyre::{
    Result,
    bail,
};
use noise::audio::write_wav;
use noise::factory::create_component;
use noise::pipeline::Pipeline;
use tracing::{
    Level,
    debug,
    info,
    instrument,
    span,
};
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
#[command(author, version, about = "Generate audio with composable pipeline")]
struct Cli {
    /// Pipeline: comma-separated components (e.g., "sine:freq=440,peak,volume:level=0.5")
    #[arg(long)]
    pipeline: String,

    /// Duration in seconds
    #[arg(long, default_value_t = 1.0)]
    duration: f64,

    /// Sample rate in Hz
    #[arg(long, default_value_t = 44100.0)]
    sample_rate: f64,

    /// Output WAV file
    #[arg(long, default_value = "output.wav")]
    output: String,
}

#[instrument(level = "debug")]
fn parse_components(pipeline: &str) -> Vec<String> {
    let mut components = Vec::new();
    let mut current = String::new();
    let mut bracket_level = 0;

    for ch in pipeline.chars() {
        match ch {
            '[' => {
                bracket_level += 1;
                current.push(ch);
            }
            ']' => {
                bracket_level -= 1;
                current.push(ch);
            }
            ',' if bracket_level == 0 => {
                if !current.trim().is_empty() {
                    components.push(current.trim().to_string());
                }
                current.clear();
            }
            _ => current.push(ch),
        }
    }
    if !current.trim().is_empty() {
        components.push(current.trim().to_string());
    }

    debug!("Parsed {} components from pipeline", components.len());
    for (i, comp) in components.iter().enumerate() {
        debug!("Component {}: {}", i, comp);
    }

    components
}

#[instrument(skip(cli), fields(pipeline = %cli.pipeline, duration = %cli.duration, sample_rate = %cli.sample_rate, output = %cli.output))]
fn run_pipeline(cli: &Cli) -> Result<()> {
    if cli.pipeline.is_empty() {
        bail!("Pipeline is required. Use --pipeline 'sine:freq=440,peak,volume:level=0.5'");
    }

    let components = parse_components(&cli.pipeline);

    if components.is_empty() {
        bail!("Pipeline must have at least one component, starting with a source.");
    }

    info!("Building pipeline with {} components", components.len());
    let mut pipeline = Pipeline::new();

    for (i, spec) in components.iter().enumerate() {
        let _span = span!(Level::DEBUG, "add_component", index = i, spec = %spec).entered();
        debug!("Creating component {} from spec: {}", i, spec);
        let comp = create_component(spec)?;
        pipeline.add_component(comp)?;
        info!("Added component {}: {}", i, spec);
    }

    info!("Running pipeline");
    let samples = pipeline.run(cli.duration, cli.sample_rate)?;
    info!("Generated {} samples", samples.len());

    let _span = span!(Level::INFO, "write_output", file = %cli.output).entered();
    write_wav(&cli.output, &samples, cli.sample_rate)?;
    info!("Saved {} samples to {}", samples.len(), cli.output);

    Ok(())
}

fn init_tracing() {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("noise=info"));

    // Using pretty format for readable, human-friendly output
    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter(env_filter)
        .with_target(true)
        .with_line_number(true)
        .with_file(true)
        .init();
}

fn main() -> Result<()> {
    color_eyre::install()?;
    init_tracing();

    info!("Audio pipeline application started");

    let cli = Cli::parse();
    debug!(
        "Parsed CLI arguments: pipeline={}, duration={}s, sample_rate={}Hz, output={}",
        cli.pipeline, cli.duration, cli.sample_rate, cli.output
    );

    run_pipeline(&cli)?;

    info!("Application completed successfully");
    Ok(())
}
