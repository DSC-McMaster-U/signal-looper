// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::SampleFormat;
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::time::Duration;

const FILE: &str = "recording.wav"; 

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect(); 

    match args.first().map(String::as_str) {
        Some("record") => {
            let seconds = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(5.0); 
            record(seconds)?; 
        }
        Some("play") => {
            play()?
        }
        _ => {
            println!("cargo run -- record [seconds]     record from your default mic + safe to {FILE}");
            println!("cargo run -- play                 play back recording from {FILE}"); 
        }
    }
    Ok(())
}

fn record(seconds: f32) -> Result<(), Box<dyn Error>> {
    // initialize default audio backend and set input device  
    let device = cpal::default_host()
        .default_input_device()
        .expect("no input device found");
    println!("input device: {}", device.description()?);

    // reject devices that do not support f32 samples (to be extended later)
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
    println!("recording for {seconds} seconds... speak now!"); 
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
    let mut writer = hound::WavWriter::create(FILE, spec)?;
    for &s in samples.iter() {
        writer.write_sample(s)?;
    }
    writer.finalize()?; 

    println!("saved {} samples to {FILE}", samples.len());
    Ok(())
}

fn play() -> Result<(), Box<dyn Error>> {
    // load samples from .wav file
    let mut reader = hound::WavReader::open(FILE)
        .map_err(|err| format!("unable to open {FILE}: {err}"))?; 
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
    println!("playing {FILE} ({duration:.1}s)...");
    std::thread::sleep(Duration::from_secs_f32(duration + 0.5)); 
    Ok(())
}