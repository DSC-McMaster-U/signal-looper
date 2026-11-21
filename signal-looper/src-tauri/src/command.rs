use crate::assets::AudioBuffer;

pub enum AudioCommand {
  Play(AudioBuffer),
  Stop,
}