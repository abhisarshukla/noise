use clap::Parser;
use color_eyre::{eyre::Result, eyre::bail};
use noise::audio::{Pipeline, write_wav};
use noise::factory::create_component;

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
    components
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    if cli.pipeline.is_empty() {
        bail!("Pipeline is required. Use --pipeline 'sine:freq=440,peak,volume:level=0.5'");
    }

    let components = parse_components(&cli.pipeline);

    if components.is_empty() {
        bail!("Pipeline must have at least one component, starting with a source.");
    }

    let mut pipeline = Pipeline::new();

    for spec in &components {
        let comp = create_component(spec)?;
        pipeline.add_component(comp)?;
    }

    let samples = pipeline.run(cli.duration, cli.sample_rate)?;
    println!("Generated {} samples", samples.len());

    write_wav(&cli.output, &samples, cli.sample_rate)?;
    println!("Saved samples to {}", cli.output);

    Ok(())
}
