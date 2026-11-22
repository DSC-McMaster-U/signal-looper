//! In-memory library: IDs, metadata, storage, and shared playback buffers.
use crate::audio_buffer::AudioBuffer;
use crate::audio_io::import_audio;
use serde::Serialize;
use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};

/// Frontend metadata for one imported file. Audio samples stay in its buffer.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportedAudio {
    pub id: String,
    pub name: String,
    pub original_sample_rate: u32,
    pub sample_rate: u32,
    pub channels: u16,
    pub duration_seconds: f64,
}

/// Backend storage for one imported file: its name, original sample rate,
/// and a shared reference to its normalized 48 kHz audio buffer in RAM.
/// The library assigns its ID; the frontend receives ImportedAudio metadata.
pub(crate) struct StoredAudio {
    pub(crate) name: String,
    pub(crate) original_sample_rate: u32,
    pub(crate) buffer: Arc<AudioBuffer>,
}

// Build frontend metadata from a stored file and the ID assigned by the library.
// Audio samples stay in the shared buffer and are not included in the response.
impl StoredAudio {
    fn metadata(&self, id: u64) -> ImportedAudio {
        ImportedAudio {
            id: id.to_string(),
            name: self.name.clone(),
            original_sample_rate: self.original_sample_rate,
            sample_rate: self.buffer.sample_rate(),
            channels: self.buffer.channels(),
            duration_seconds: self.buffer.duration_seconds(),
        }
    }
}

#[derive(Default)]
struct Library {
    next_id: u64,
    files: HashMap<u64, StoredAudio>,
}

/// In-memory collection managed by Tauri. Clones share the same library.
/// Each imported file has a separate buffer; imports are not saved to disk.
#[derive(Clone, Default)]
pub struct AudioLibrary {
    inner: Arc<Mutex<Library>>,
}

// Audio library and its functions
impl AudioLibrary {

    /// Decode a WAV, normalize it to 48 kHz, and store its buffer under a new ID.
    /// Return metadata for the imported file.
    pub fn import(&self, source: &Path) -> Result<ImportedAudio, String> {
        // Decode and resample outside the lock. Only normalized audio is inserted.
        let stored = import_audio(source)?;
        let mut library = self.inner.lock().map_err(|error| error.to_string())?;
        let id = library.next_id;
        library.next_id = id.checked_add(1).ok_or("Audio ID limit reached")?;
        let metadata = stored.metadata(id);
        library.files.insert(id, stored);
        Ok(metadata)
    }

    /// Return metadata for all imported files in unspecified order.
    /// Sort the returned list separately when a specific display order is needed.
    pub fn list(&self) -> Result<Vec<ImportedAudio>, String> {
        let library = self.inner.lock().map_err(|error| error.to_string())?;
        Ok(library
            .files
            .iter()
            .map(|(id, audio)| audio.metadata(*id))
            .collect())
    }

    /// Return one imported file's metadata by ID, or an error if it is missing.
    pub fn get(&self, id: &str) -> Result<ImportedAudio, String> {
        let id = parse_id(id)?;
        let library = self.inner.lock().map_err(|error| error.to_string())?;
        library
            .files
            .get(&id)
            .map(|audio| audio.metadata(id))
            .ok_or_else(|| format!("No imported audio with ID {id}"))
    }

    /// Remove a library entry; return false if the ID is not found.
    /// Its buffer is freed when the last shared reference is dropped.
    /// Existing playback references remain valid.
    pub fn remove(&self, id: &str) -> Result<bool, String> {
        let id = parse_id(id)?;
        let mut library = self.inner.lock().map_err(|error| error.to_string())?;
        Ok(library.files.remove(&id).is_some())
    }

    /// Get shared 48 kHz audio for playback. The library lock is released before
    /// returning and samples are not copied. Removing the import does not stop an
    /// active reader: memory is freed when the last Arc is dropped.
    pub fn get_buffer(&self, id: &str) -> Result<Arc<AudioBuffer>, String> {
        let id = parse_id(id)?;
        let library = self.inner.lock().map_err(|error| error.to_string())?;
        library
            .files
            .get(&id)
            .map(|audio| Arc::clone(&audio.buffer))
            .ok_or_else(|| format!("No imported audio with ID {id}"))
    }
}

fn parse_id(id: &str) -> Result<u64, String> {
    id.parse().map_err(|_| format!("Invalid audio ID {id:?}"))
}
