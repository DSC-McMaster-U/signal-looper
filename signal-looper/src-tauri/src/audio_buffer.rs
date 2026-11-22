//! Immutable, normalized audio shared by the upload library and playback code.
//! Imported audios have their own buffer and can be controlled seperately
use crate::resampler::{SAMPLE_RATE, to_48khz};

/// Interleaved f32 audio at 48 kHz. A frame contains one sample per channel:
/// stereo is [left_0, right_0, left_1, right_1, ...].
/// Share via Arc<AudioBuffer>; keep playback position in the playback controller.
#[derive(Debug)]
pub struct AudioBuffer {
    samples: Vec<f32>,
    channels: u16,
}

impl AudioBuffer {
    /// Normalize before creating a buffer, so consumers can rely on 48 kHz.
    pub(crate) fn from_source(
        samples: Vec<f32>,
        channels: u16,
        sample_rate: u32,
    ) -> Result<Self, String> {
        Ok(Self {
            samples: to_48khz(samples, channels, sample_rate)?,
            channels,
        })
    }

    pub fn samples(&self) -> &[f32] {
        &self.samples
    }

    pub fn channels(&self) -> u16 {
        self.channels
    }

    pub fn sample_rate(&self) -> u32 {
        SAMPLE_RATE
    }

    pub fn frame_count(&self) -> usize {
        self.samples.len() / usize::from(self.channels)
    }

    pub fn duration_seconds(&self) -> f64 {
        self.frame_count() as f64 / f64::from(SAMPLE_RATE)
    }

    /// Channel samples for a frame, or None when seeking past the end.
    pub fn frame(&self, index: usize) -> Option<&[f32]> {
        let channels = usize::from(self.channels);
        let start = index.checked_mul(channels)?;
        self.samples.get(start..start.checked_add(channels)?)
    }
}
