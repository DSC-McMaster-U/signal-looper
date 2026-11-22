use crate::assets::AudioBuffer; 

pub struct SampleNode {
  samples: Vec<f32>,
  channels: usize,
  frame: usize, // how many frames have played so far
}

impl SampleNode {
  pub fn new(buffer: AudioBuffer) -> Self {
    Self { samples: buffer.samples, channels: buffer.channels, frame: 0 }
  }

  // add next frame into `out`, return false when clip is finished
  pub fn mix_into(&mut self, out: &mut [f32]) -> bool {
    let start = self.frame * self.channels; 
    if start + self.channels > self.samples.len() {
      return false; 
    }
    for (ch, sample) in out.iter_mut().enumerate() {
      // mono file gets copied to each output channel
      *sample += self.samples[start + ch.min(self.channels - 1)]; 
    }
    self.frame += 1;
    true
  }
}