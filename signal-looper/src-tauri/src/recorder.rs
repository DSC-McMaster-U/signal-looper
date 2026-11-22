use std::{path::PathBuf, sync::{Arc, Mutex}, thread};
use cpal::{SampleFormat, traits::{DeviceTrait, HostTrait, StreamTrait}};
use crossbeam::channel::{bounded, Sender};

type Reply = Sender<Result<(), String>>; 

enum RecorderCommand {
  Start(PathBuf, Reply),
  Stop(Reply),
}

struct ActiveRecording {
  stream: cpal::Stream,
  samples: Arc<Mutex<Vec<f32>>>,
  path: PathBuf,
  sample_rate: u32,
}

pub struct Recorder {
  tx: Sender<RecorderCommand>,
}

impl Recorder {
  pub fn new() -> Self {
    let (tx, rx) = bounded::<RecorderCommand>(8); 

    thread::spawn(move || {
      let mut active: Option<ActiveRecording> = None; 

      for cmd in rx {
        match cmd {
          RecorderCommand::Start(path, reply) => {
            let result = if active.is_none() {
              open_input_stream(path).map(|rec| active = Some(rec))
            } else {
              Err("already recording".to_string())
            }; 
            let _ = reply.send(result); 
          }
          RecorderCommand::Stop(reply) => {
            let result = match active.take() {
              Some(rec) => write_wav(rec),
              None => Err("not recording".to_string())
            };
            let _ = reply.send(result); 
          }
        }
      }
    });

    Self { tx }
  }

  pub fn start(&self, path: PathBuf) -> Result<(), String> {
    let (reply_tx, reply_rx) = bounded(1); 
    self.tx
      .send(RecorderCommand::Start(path, reply_tx))
      .map_err(|err| err.to_string())?;
    reply_rx.recv().map_err(|err| err.to_string())?
  }

  pub fn stop(&self) -> Result<(), String> {
    let (reply_tx, reply_rx) = bounded(1); 
    self.tx
      .send(RecorderCommand::Stop(reply_tx))
      .map_err(|err| err.to_string())?;
    reply_rx.recv().map_err(|err| err.to_string())?      
  }
}

fn open_input_stream(path: PathBuf) -> Result<ActiveRecording, String> {
    // initialize default audio backend and set input device
    let device = cpal::default_host()
        .default_input_device()
        .expect("no input device found");
    println!("input device: {:?}", device.description());

    // reject devices that do not support f32 samples (to be extended later!)
    let supported = device.default_input_config().map_err(|err| err.to_string())?;
    if supported.sample_format() != SampleFormat::F32 {
        return Err(format!(
            "unsupported input configuration type: {:?}",
            supported.sample_format()
        )
        .into());
    }
    let config: cpal::StreamConfig = supported.into();
    let channels = config.channels as usize;
    let sample_rate = config.sample_rate;

    // a mutex-locked list of samples shared between this thread and the audio thread
    // reserve a minute up front so the callback rarely has to grow the Vec
    let samples = Arc::new(Mutex::new(Vec::<f32>::with_capacity(sample_rate as usize * 60)));
    let samples_for_callback = Arc::clone(&samples);

    // build input stream
    let stream = device.build_input_stream(
        config,
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            let mut buf = samples_for_callback.lock().unwrap();
            for frame in data.chunks(channels) {
                // take the average of the channels of each frame to produce a mono sample
                buf.push(frame.iter().sum::<f32>() / channels as f32);
            }
        },
        |err| eprintln!("input stream error: {err}"),
        None,
    )
    .map_err(|err| err.to_string())?;

    stream.play().map_err(|err| err.to_string())?;   

    Ok(ActiveRecording {stream: stream, samples, path, sample_rate })
}

fn write_wav(rec: ActiveRecording) -> Result<(), String> {
  let ActiveRecording { stream, samples, path, sample_rate } = rec;
  drop(stream); // end input capturing

  let data = samples.lock().unwrap(); 
  println!(
      "captured {} samples ({:.1}s at {} Hz)",
      data.len(),
      data.len() as f32 / sample_rate as f32,
      sample_rate
  );

  // write data to .wav file
  let spec = hound::WavSpec {
      channels: 1,
      sample_rate,
      bits_per_sample: 32,
      sample_format: hound::SampleFormat::Float,
  };

  let mut writer = hound::WavWriter::create(&path, spec)
    .map_err(|err| err.to_string())?;
  for &s in data.iter() {
    writer.write_sample(s).map_err(|err| err.to_string())?;
  }
  writer.finalize().map_err(|err| err.to_string())?;

  println!("saved {} samples to {}", data.len(), path.display());
  Ok(())
}