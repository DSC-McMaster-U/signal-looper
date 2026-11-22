use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, Sample, SampleFormat, SizedSample, Stream, StreamConfig};
use std::error::Error;
use std::f32::consts::TAU;
use std::time::Duration;

const TEST_TONE_FREQUENCY_HZ: f32 = 440.0;
const TEST_TONE_AMPLITUDE: f32 = 0.1;
const TEST_TONE_DURATION: Duration = Duration::from_secs(1);

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
