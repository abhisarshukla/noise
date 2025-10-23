use clap::Parser;

use noise::sources::SineWaveSource;
use noise::processors::VolumeProcessor;
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

fn create_component(spec: &str) -> Result<Box<dyn noise::traits::Component>, String> {
    let parts: Vec<&str> = spec.split(':').collect();
    match parts[0] {
        "sine" => {
            if parts.len() != 2 {
                return Err("sine requires frequency: sine:440".to_string());
            }
            let freq = parts[1].parse().map_err(|_| "Invalid frequency".to_string())?;
            Ok(Box::new(SineWaveSource::new(freq)))
        }
        "volume" => {
            if parts.len() != 2 {
                return Err("volume requires level: volume:0.5".to_string());
            }
            let volume = parts[1].parse().map_err(|_| "Invalid volume".to_string())?;
            Ok(Box::new(VolumeProcessor::new(volume)))
        }
        "peak" => {
            if parts.len() != 1 {
                return Err("peak takes no params: peak".to_string());
            }
            Ok(Box::new(noise::analysers::PeakAnalyser::new()))
        }
        _ => Err(format!("Unknown component type: {}", parts[0])),
    }
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
        let comp = create_component(spec).map_err(|e| {
            eprintln!("Error creating component '{}': {}", spec, e);
            std::process::exit(1);
        }).unwrap();
        if let Err(e) = pipeline.add_component(comp) {
            eprintln!("Error adding component: {}", e);
            std::process::exit(1);
        }
    }

    match pipeline.run(cli.duration, cli.sample_rate) {
        Ok(samples) => {
            println!("Generated {} samples", samples.len());

            // Collect and print analyser results
            let analyser_results = pipeline.collect_analyser_results();
            for (i, result) in analyser_results.iter().enumerate() {
                println!("Analyser {} peak: {:.6}", i + 1, result);
            }

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
