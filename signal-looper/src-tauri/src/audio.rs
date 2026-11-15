use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, Sample, SampleFormat, SizedSample, Stream, StreamConfig};
use std::error::Error;
use std::f32::consts::TAU;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Duration;

const TEST_TONE_FREQUENCY_HZ: f32 = 440.0;
const TEST_TONE_AMPLITUDE: f32 = 0.1;
const TEST_TONE_DURATION: Duration = Duration::from_secs(1);

pub fn record(path: &Path, seconds: f32) -> Result<(), Box<dyn Error>> {
    // initialize default audio backend and set input device
    let device = cpal::default_host()
        .default_input_device()
        .expect("no input device found");
    println!("input device: {}", device.description()?);

    // reject devices that do not support f32 samples (to be extended later!)
    let supported = device.default_input_config()?;
    if supported.sample_format() != SampleFormat::F32 {
        return Err(format!(
            "unsupported input configuration type: {:?}",
            supported.sample_format()
        )
        .into());
    }
    let config: cpal::StreamConfig = supported.into();
    let channels = config.channels as usize;
    let rate = config.sample_rate;

    // a mutex-locked list of samples shared between this thread and the audio thread
    let recorded = Arc::new(Mutex::new(Vec::<f32>::new()));
    let recorded_for_callback = Arc::clone(&recorded);

    // build input stream
    let stream = device.build_input_stream(
        config,
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            let mut buf = recorded_for_callback.lock().unwrap();
            for frame in data.chunks(channels) {
                // take the average of the channels of each frame to produce a mono sample
                buf.push(frame.iter().sum::<f32>() / channels as f32);
            }
        },
        |err| eprintln!("input stream error: {err}"),
        None,
    )?;

    // record input stream
    stream.play()?;
    std::thread::sleep(Duration::from_secs_f32(seconds));
    drop(stream);

    let samples = recorded.lock().unwrap();
    println!(
        "captured {} samples ({:.1}s at {} Hz)",
        samples.len(),
        samples.len() as f32 / rate as f32,
        rate
    );

    // write samples to .wav file
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: rate,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };
    let mut writer = hound::WavWriter::create(path, spec)?;
    for &s in samples.iter() {
        writer.write_sample(s)?;
    }
    writer.finalize()?;

    println!("saved {} samples to {}", samples.len(), path.display());
    Ok(())
}

pub fn play(path: &Path) -> Result<(), Box<dyn Error>> {
    // load samples from .wav file
    let mut reader = hound::WavReader::open(path)
        .map_err(|err| format!("unable to open {}: {err}", path.display()))?;
    let in_rate = reader.spec().sample_rate;
    let samples = reader.samples::<f32>().collect::<Result<Vec<f32>, _>>()?;
    if samples.is_empty() {
        return Err("{FILE} is empty".into());
    }
    let duration = samples.len() as f32 / in_rate as f32;

    // initialize default audio backend and set output device
    let device = cpal::default_host()
        .default_output_device()
        .expect("no output device found");
    println!("output device: {}", device.description()?);

    let supported = device.default_output_config()?;
    if supported.sample_format() != SampleFormat::F32 {
        return Err(format!(
            "unsupported output configuration type: {:?}",
            supported.sample_format()
        )
        .into());
    }
    let config: cpal::StreamConfig = supported.into();
    let channels = config.channels as usize;

    // if output device runs at at a faster/slower rate than the recording,
    // step through samples faster or slower to maintain the original pitch
    let ratio = in_rate as f64 / config.sample_rate as f64;
    let mut pos = 0.0_f64;

    let stream = device.build_output_stream(
        config,
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            for frame in data.chunks_mut(channels) {
                let i = pos as usize;
                let value = if i + 1 < samples.len() {
                    // blend two neighbouring samples when pos is between them
                    let frac = (pos - i as f64) as f32;
                    samples[i] * (1.0 - frac) + samples[i + 1] * frac
                } else {
                    0.0
                };
                pos += ratio;
                frame.fill(value);
            }
        },
        |err| eprintln!("output stream error: {err}"),
        None,
    )?;

    // play recording
    stream.play()?;
    std::thread::sleep(Duration::from_secs_f32(duration + 0.5));
    Ok(())
}

pub fn play_test_tone() -> Result<(), Box<dyn Error>> {
    // Use the system's default output device and negotiate its supported format.
    let device = cpal::default_host()
        .default_output_device()
        .ok_or("no default output device found")?;
    let supported_config = device.default_output_config()?;
    let sample_format = supported_config.sample_format();
    let config: StreamConfig = supported_config.into();

    // CPAL requires the callback sample type to match the device format, so
    // select the appropriately typed stream builder at runtime.
    let stream = match sample_format {
        SampleFormat::F32 => build_test_tone_stream::<f32>(&device, config)?,
        SampleFormat::I16 => build_test_tone_stream::<i16>(&device, config)?,
        SampleFormat::U16 => build_test_tone_stream::<u16>(&device, config)?,
        format => return Err(format!("unsupported output sample format: {format}").into()),
    };

    stream.play()?;
    // Keep the stream alive while the one-second test tone is generated.
    std::thread::sleep(TEST_TONE_DURATION);
    Ok(())
}

fn build_test_tone_stream<T>(
    device: &cpal::Device,
    config: StreamConfig,
) -> Result<Stream, cpal::Error>
where
    T: Sample + SizedSample + FromSample<f32>,
{
    // Advance the oscillator once per frame so all output channels stay in sync.
    let channels = usize::from(config.channels);
    let phase_step = TAU * TEST_TONE_FREQUENCY_HZ / config.sample_rate as f32;
    let mut phase = 0.0_f32;

    device.build_output_stream(
        config,
        move |buffer: &mut [T], _| {
            for frame in buffer.chunks_mut(channels) {
                // Convert the generated f32 sine wave to the device's sample type.
                let sample = T::from_sample(TEST_TONE_AMPLITUDE * phase.sin());
                frame.fill(sample);
                phase = (phase + phase_step) % TAU;
            }
        },
        |err| eprintln!("test tone output stream error: {err}"),
        None,
    )
}
