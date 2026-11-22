//! Normalize imported audio to a fixed output sample rate of 48 kHz.
//! Other output rates could be added for future exports.
use rubato::audioadapter_buffers::direct::InterleavedSlice;
use rubato::{Fft, FixedSync, Resampler};

pub(crate) const SAMPLE_RATE: u32 = 48_000;

/// Convert interleaved f32 audio to 48 kHz without mixing channels.
/// Already-48-kHz data is validated and returned without interpolation.

pub(crate) fn to_48khz(samples: Vec<f32>, channels: u16, source_rate: u32, ) -> Result<Vec<f32>, String> {

    let channels = usize::from(channels);

    if channels == 0 || source_rate == 0 {
        return Err("Channel count and sample rate must be positive".into());
    }

    if samples.is_empty() || !samples.len().is_multiple_of(channels) {
        return Err("Audio must contain complete, nonempty frames".into());
    }

    if samples.iter().any(|sample| !sample.is_finite()) {
        return Err("Audio contains non-finite samples".into());
    }

    if source_rate == SAMPLE_RATE {
        return Ok(samples);
    }

    let frames = samples.len() / channels;

    let input =
        InterleavedSlice::new(&samples, channels, frames).map_err(|error| error.to_string())?;

    // Find the greatest common divisor to reduce the input/output rate ratio.
    let (mut a, mut b) = (source_rate, SAMPLE_RATE);

    while b != 0 {
        (a, b) = (b, a % b);
    }

    // Even FFT lengths keep Rubato's half-block filter delay on whole frames.
    let block = 2 * (source_rate / a) as usize;
    let chunk_size = 1024_usize.div_ceil(block) * block;

    let mut resampler = Fft::<f32>::new(
        source_rate as usize,
        SAMPLE_RATE as usize,
        chunk_size,
        channels,
        FixedSync::Both,
    )
    .map_err(|error| error.to_string())?;

    // Whole-clip processing flushes the last chunk and removes filter latency.
    let mut output = resampler
        .process_all(&input, frames, None)
        .map_err(|error| format!("Resampling failed: {error}"))?
        .take_data();

    // Calculate the target frame count exactly, rounding up to a complete frame.
    // Trimming below removes any extra frame caused by Rubato's float rounding.
    let output_frames = usize::try_from(
        (frames as u128 * u128::from(SAMPLE_RATE)).div_ceil(u128::from(source_rate)),
    )
    .map_err(|_| "Audio is too large")?;

    let output_samples = output_frames
        .checked_mul(channels)
        .ok_or("Audio is too large")?;
    if output.len() < output_samples || output.iter().any(|sample| !sample.is_finite()) {
        return Err("Resampler produced invalid or incomplete audio".into());
    }

    output.truncate(output_samples);
    Ok(output)
}
