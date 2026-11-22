use std::path::Path;

// stores entire clip decoded into memory
pub struct AudioBuffer {
  pub samples: Vec<f32>,
  pub channels: usize, 
}

// decodes .wav file into f32 samples
pub fn load_wav(path: &Path) -> Result<AudioBuffer, String> {
  let mut reader = hound::WavReader::open(path)
        .map_err(|err| format!("unable to open {}: {err}", path.display()))?;
  let spec = reader.spec(); 

  // scale samples according to sample format, filtering out unreadable samples
  let samples: Vec<f32> = match spec.sample_format {
    hound::SampleFormat::Float => reader.samples::<f32>().filter_map(|s| s.ok()).collect(),
    hound::SampleFormat::Int => {
      let max = (1i64 << (spec.bits_per_sample - 1)) as f32; 
      reader.samples::<i32>().filter_map(|s| s.ok()).map(|v| v as f32 / max).collect() 
    }
  };

  Ok(AudioBuffer {samples, channels: spec.channels as usize })
}