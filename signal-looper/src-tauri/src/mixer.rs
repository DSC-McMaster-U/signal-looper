use std::sync::mpsc::Receiver;
use crate::{command::AudioCommand, node::SampleNode}; 

// contains "queue" of pending audio
pub struct Mixer {
  nodes: Vec<SampleNode>,
  paused: bool,
}

impl Mixer {
  pub fn new() -> Self {
    Self { nodes : Vec::new(), paused: false }
  }

  // called by audio stream each time the speakers need more samples
  pub fn process(&mut self, output: &mut [f32], channels: usize, rx: &Receiver<AudioCommand>) {
    // handle commands that have arrived since the last call
    while let Ok(cmd) = rx.try_recv() {
      match cmd {
        AudioCommand::Play(buffer) => {
          self.nodes.clear(); 
          self.nodes.push(SampleNode::new(buffer)); 
          self.paused = false; 
        }
        AudioCommand::Stop => {
          self.nodes.clear(); 
          self.paused = false; 
        }
      }
    }

    // start from silence, since the buffer might hold leftover data from the previous call
    output.fill(0.0);

    // if paused, leave nodes untouched so their positions are kept
    if self.paused {
      return; 
    }

    for frame in output.chunks_mut(channels) {
      self.nodes.retain_mut(|node| node.mix_into(frame));
    }
  }
}