//! Audio import/export I/O. Currently implements WAV importing only.
//! Export functionality can be added here later.
use crate::audio_buffer::AudioBuffer;
use crate::audio_library::StoredAudio;
use std::path::Path;
use std::sync::Arc;

/// Import entry point: decode samples and prepare a 48 kHz
/// buffer for the library. The decoding and sample-reading helpers stay separate.
pub(crate) fn import_audio(source: &Path) -> Result<StoredAudio, String> {
    decode_wav(source)
}

// === HELPER FUNCTIONS ===

/// Decode samples from an already-open WAV reader. If the file ends before its
/// declared length, keep complete audio frames; reject other decoding errors.
fn read_samples<S: hound::Sample, R: std::io::Read>(
    reader: &mut hound::WavReader<R>,
) -> Result<Vec<S>, String> {
    let channels = usize::from(reader.spec().channels);
    let mut samples = Vec::new();
    for sample in reader.samples::<S>() {
        match sample {
            Ok(sample) => samples.push(sample),
            // Tolerate headers declaring more data than the file contains.
            // Hound 3.5.1 uses Other with this message for an early EOF.
            Err(hound::Error::IoError(ref error))
                if error.kind() == std::io::ErrorKind::UnexpectedEof
                    || (error.kind() == std::io::ErrorKind::Other
                        && error.to_string() == "Failed to read enough bytes.") =>
            {
                samples.truncate(samples.len() / channels * channels);
                break;
            }
            Err(error) => return Err(format!("Invalid WAV data: {error}")),
        }
    }
    Ok(samples)
}

/// Open a WAV, validate its format, and return audio prepared at 48 kHz in RAM.
/// AudioLibrary::import() inserts the returned audio into the library.
fn decode_wav(source: &Path) -> Result<StoredAudio, String> {
    let mut reader =
        hound::WavReader::open(source).map_err(|error| format!("Cannot open WAV: {error}"))?;
    let spec = reader.spec();
    if spec.channels == 0
        || spec.sample_rate == 0
        || spec.bits_per_sample == 0
        || spec.bits_per_sample > 32
    {
        return Err("Unsupported WAV format".into());
    }

    // Decode samples as f32, scaling integer PCM values to floating-point amplitude.
    let samples = match spec.sample_format {
        hound::SampleFormat::Float => read_samples::<f32, _>(&mut reader)?,
        hound::SampleFormat::Int => {
            let scale = 2_f32.powi(i32::from(spec.bits_per_sample) - 1);
            read_samples::<i32, _>(&mut reader)?
                .into_iter()
                .map(|sample| sample as f32 / scale)
                .collect()
        }
    };

    // Resample to 48 kHz and create the in-memory buffer before library insertion.
    let buffer = AudioBuffer::from_source(samples, spec.channels, spec.sample_rate)?;
    Ok(StoredAudio {
        name: source
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned(),
        original_sample_rate: spec.sample_rate,
        buffer: Arc::new(buffer),
    })
}
