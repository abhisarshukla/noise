use clap::Parser;

use noise::sources::SineWaveSource;
use noise::processors::VolumeProcessor;
use noise::analysers::PeakAnalyser;
use noise::audio::{Pipeline, write_wav};

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

fn main() {
    let cli = Cli::parse();

    if cli.pipeline.is_empty() {
        eprintln!("Pipeline is required. Use --pipeline \"sine:440,peak,volume:0.5\"");
        std::process::exit(1);
    }

    let components: Vec<&str> = cli.pipeline.split(',').map(|s| s.trim()).collect();

    if components.is_empty() {
        eprintln!("Pipeline must have at least one component, starting with a source.");
        std::process::exit(1);
    }

    let mut pipeline = Pipeline::new();

    for spec in components {
        let parts: Vec<&str> = spec.split(':').collect();
        let comp: Box<dyn noise::traits::Component> = match parts[0] {
            "sine" => Box::new(SineWaveSource::from_spec(spec).map_err(|e| {
                eprintln!("Error creating sine: {}", e);
                std::process::exit(1);
            }).unwrap()),
            "volume" => Box::new(VolumeProcessor::from_spec(spec).map_err(|e| {
                eprintln!("Error creating volume: {}", e);
                std::process::exit(1);
            }).unwrap()),
            "peak" => Box::new(PeakAnalyser::from_spec(spec).map_err(|e| {
                eprintln!("Error creating peak: {}", e);
                std::process::exit(1);
            }).unwrap()),
            _ => {
                eprintln!("Unknown component type: {}", parts[0]);
                std::process::exit(1);
            }
        };
        if let Err(e) = pipeline.add_component(comp) {
            eprintln!("Error adding component: {}", e);
            std::process::exit(1);
        }
    }

    match pipeline.run(cli.duration, cli.sample_rate) {
        Ok(samples) => {
            println!("Generated {} samples", samples.len());

            if let Err(e) = write_wav(&cli.output, &samples, cli.sample_rate) {
                eprintln!("Error writing WAV file: {}", e);
                std::process::exit(1);
            } else {
                println!("Saved samples to {}", cli.output);
            }
        }
        Err(e) => {
            eprintln!("Error running pipeline: {}", e);
            std::process::exit(1);
        }
    }
}
