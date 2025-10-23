use clap::Parser;
use color_eyre::{
    Result,
    eyre,
};
use noise::analysers::PeakAnalyser;
use noise::audio::{
    Pipeline,
    write_wav,
};
use noise::processors::VolumeProcessor;
use noise::sources::SineWaveSource;

#[derive(Parser)]
#[command(author, version, about = "Generate audio with composable pipeline")]
struct Cli {
    /// Pipeline: comma-separated components (e.g., "sine:440,peak,volume:0.5,peak")
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

fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    if cli.pipeline.is_empty() {
        eyre::bail!("Pipeline is required. Use --pipeline 'sine:440,peak,volume:0.5'");
    }

    let components: Vec<&str> = cli.pipeline.split(',').map(|s| s.trim()).collect();

    if components.is_empty() {
        eyre::bail!("Pipeline must have at least one component, starting with a source.");
    }

    let mut pipeline = Pipeline::new();

    for spec in components {
        let parts: Vec<&str> = spec.split(':').collect();
        let comp: Box<dyn noise::traits::Component> = match parts[0] {
            "sine" => Box::new(SineWaveSource::from_spec(spec)?),
            "volume" => Box::new(VolumeProcessor::from_spec(spec)?),
            "peak" => Box::new(PeakAnalyser::from_spec(spec)?),
            _ => eyre::bail!("Unknown component type: {}", parts[0]),
        };
        pipeline.add_component(comp)?;
    }

    let samples = pipeline.run(cli.duration, cli.sample_rate)?;
    println!("Generated {} samples", samples.len());

    write_wav(&cli.output, &samples, cli.sample_rate)?;
    println!("Saved samples to {}", cli.output);

    Ok(())
}
