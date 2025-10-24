use color_eyre::Result;
use hound;
use tracing::{
    debug,
    info,
    instrument,
};

#[instrument(skip(samples), fields(filename = %filename, num_samples = %samples.len(), sample_rate = %sample_rate))]
pub fn write_wav(filename: &str, samples: &[f64], sample_rate: f64) -> Result<(), hound::Error> {
    debug!("Creating WAV file with {} samples at {}Hz", samples.len(), sample_rate);

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: sample_rate as u32,
        bits_per_sample: 24,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create(filename, spec)?;
    debug!("WAV writer created for {}", filename);

    for (i, &sample) in samples.iter().enumerate() {
        let pcm = (sample * 8388607.0) as i32;
        writer.write_sample(pcm)?;

        // Log progress for very long samples
        if samples.len() > 100000 && i % 50000 == 0 {
            debug!("Written {}/{} samples", i, samples.len());
        }
    }

    debug!("All samples written, finalizing file");
    writer.finalize()?;
    info!("WAV file written successfully: {}", filename);
    Ok(())
}

pub fn write_wav_to_bytes(samples: &[f64], sample_rate: f64) -> Result<Vec<u8>, hound::Error> {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: sample_rate as u32,
        bits_per_sample: 24,
        sample_format: hound::SampleFormat::Int,
    };

    let mut cursor = std::io::Cursor::new(Vec::new());
    let mut writer = hound::WavWriter::new(&mut cursor, spec)?;

    for &sample in samples {
        let pcm = (sample * 8388607.0) as i32;
        writer.write_sample(pcm)?;
    }

    writer.finalize()?;
    Ok(cursor.into_inner())
}
