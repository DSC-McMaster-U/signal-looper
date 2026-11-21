use std::{sync::mpsc, thread}; 
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use crate::{assets::AudioBuffer, command::AudioCommand, mixer::Mixer};

pub struct AudioEngine {
  tx: mpsc::Sender<AudioCommand>,
}

impl AudioEngine {
  // starts audio stream and is called at app startup
  pub fn new() -> Self {
    let (tx, rx) = mpsc::channel::<AudioCommand>(); 

    // cpal stream is created on its own thread so it does not get dropped
    // (and thus, the audio does not randomly stop)
    thread::spawn(move || {
      let device = cpal::default_host()
        .default_output_device()
        .expect("no output device available");
      let config = device
        .default_output_config()
        .expect("couldn't load config"); 
      let channels = config.channels() as usize; 
      
      let mut mixer = Mixer::new(); 

      let stream = device
        .build_output_stream(
          config.into(),
          move |output: &mut [f32], _| {
            mixer.process(output, channels, &rx); 
          },
          |e| eprintln!("output stream error: {e}"),
          None, 
        )
        .expect("couldn't build output stream"); 

      stream.play().expect("couldn't start stream"); 

      // keep thread alive while streaming audio
      thread::park(); 
    });

    Self { tx }
  }

  pub fn play(&self, buffer: AudioBuffer) {
    let _ = self.tx.send(AudioCommand::Play(buffer));
  }

  pub fn pause(&self) {
    let _ = self.tx.send(AudioCommand::Pause); 
  }

  pub fn resume(&self) {
    let _ = self.tx.send(AudioCommand::Resume); 
  }
  
  pub fn stop(&self) { 
    let _ = self.tx.send(AudioCommand::Stop);
  }
}

