use hound;

use crate::traits::Source;

pub fn write_wav(filename: &str, samples: &[f64], sample_rate: f64) -> Result<(), hound::Error> {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: sample_rate as u32,
        bits_per_sample: 24,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create(filename, spec)?;

    for &sample in samples {
        let pcm = (sample * 8388607.0) as i32;
        writer.write_sample(pcm)?;
    }

    writer.finalize()?;
    Ok(())
}

pub fn generate_audio<S: Source>(source: &S, duration: f64, sample_rate: f64, output: &str) -> Result<(), Box<dyn std::error::Error>> {
    let samples = source.generate(duration, sample_rate);
    println!("Generated {} samples", samples.len());
    write_wav(output, &samples, sample_rate)?;
    Ok(())
}
