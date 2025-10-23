use clap::Parser;

use noise::sources::SineWaveSource;
use noise::audio;

#[derive(Parser)]
#[command(author, version, about = "Generate sine wave audio files")]
struct Cli {
    /// Frequency in Hz
    #[arg(long, default_value_t = 440.0)]
    freq: f64,

    /// Duration in seconds
    #[arg(long, default_value_t = 1.0)]
    duration: f64,

    /// Sample rate in Hz
    #[arg(long, default_value_t = 44100.0)]
    sample_rate: f64,

    /// Output WAV file
    #[arg(long, default_value = "sine_wave.wav")]
    output: String,
}

fn main() {
    let cli = Cli::parse();

    let source = SineWaveSource::new(cli.freq);

    if let Err(e) = audio::generate_audio(&source, cli.duration, cli.sample_rate, &cli.output) {
        eprintln!("Error generating audio: {}", e);
        std::process::exit(1);
    } else {
        println!("Saved samples to {}", cli.output);
    }
}
