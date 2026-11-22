mod assets;
mod audio;
pub mod audio_buffer;
pub mod audio_io;
pub mod audio_library;
mod command;
mod engine;
mod mixer;
mod node;
mod recorder;
mod resampler;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use cpal::traits::HostTrait;
use engine::AudioEngine;
use recorder::Recorder;
use serde::Serialize;
use std::path::PathBuf;
use std::sync::{
    Arc,
    atomic::{AtomicU32, Ordering},
};
use tauri::{AppHandle, Manager, State};
use tauri_plugin_dialog::DialogExt;

struct PlaybackVolume(Arc<AtomicU32>);

impl Default for PlaybackVolume {
    fn default() -> Self {
        Self(Arc::new(AtomicU32::new(1.0f32.to_bits())))
    }
}

impl PlaybackVolume {
    fn set(&self, volume: f32) {
        self.0.store(volume.to_bits(), Ordering::Relaxed);
    }

    fn get(&self) -> f32 {
        f32::from_bits(self.0.load(Ordering::Relaxed))
    }
}

#[tauri::command]
fn set_master_volume(volume: f32, state: State<'_, PlaybackVolume>) -> Result<(), String> {
    if !volume.is_finite() || !(0.0..=1.0).contains(&volume) {
        return Err("Volume must be between 0.0 and 1.0".to_string());
    }

    state.set(volume);

    Ok(())
}

//tester functions for frontend to call

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {name}! You've been greeted from Rust!")
}

#[tauri::command] //ping pong
fn ping() -> String {
    "pong".to_string()
}

#[derive(Serialize)]
struct SystemHealth {
    status: String,
    backend: String,
    message: String,
}

#[tauri::command]
fn systemHealth() -> SystemHealth {
    SystemHealth {
        //replace with actual information when available
        status: "ok".to_string(),
        backend: "Rust".to_string(),
        message: "System is healthy AF".to_string(),
    }
}

#[tauri::command]
fn get_input_devices() -> Vec<String> {
    let host = cpal::default_host();

    let input_devices = host
        .input_devices()
        .expect("Failed to get input devices")
        .map(|device| device.to_string())
        //loops through each device and gets its name
        .collect();

    input_devices //vec<string> of input devices returned
    //treat it as String[] in frontend
}

#[tauri::command]
fn get_output_devices() -> Vec<String> {
    let host = cpal::default_host();

    let output_devices = host
        .output_devices()
        .expect("Failed to get output devices")
        .map(|device| device.to_string())
        .collect();

    output_devices
}

#[tauri::command]
fn get_default_input_device() -> Option<String> {
    let host = cpal::default_host();

    host.default_input_device() //if there is a default input device, get its name and return it as an Option<String>
        .map(|device| device.to_string())
}

#[tauri::command]
fn get_default_output_device() -> Option<String> {
    let host = cpal::default_host();

    host.default_output_device()
        .map(|device| device.to_string())
}

fn recording_path(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = app.path().app_data_dir().map_err(|err| err.to_string())?;
    std::fs::create_dir_all(&dir).map_err(|err| err.to_string())?;
    Ok(dir.join("recording.wav"))
}

#[tauri::command(async)]
fn start_recording(app: AppHandle, recorder: State<Recorder>) -> Result<(), String> {
    let path = recording_path(&app)?;
    recorder.start(path)
}

#[tauri::command(async)]
fn stop_recording(recorder: State<Recorder>) -> Result<(), String> {
    recorder.stop()
}

#[tauri::command]
async fn play_audio(app: AppHandle, state: State<'_, PlaybackVolume>) -> Result<(), String> {
    let path = recording_path(&app)?;

    let volume = Arc::clone(&state.0);

    tauri::async_runtime::spawn_blocking(move || {
        audio::play(&path, volume).map_err(|err| err.to_string())
    })
    .await
    .map_err(|err| err.to_string())?
}

#[tauri::command]
fn pause_audio(engine: State<AudioEngine>) {
    engine.pause();
}

#[tauri::command]
fn resume_audio(engine: State<AudioEngine>) {
    engine.resume();
}

#[tauri::command]
fn stop_audio(engine: State<AudioEngine>) {
    engine.stop();
}

#[tauri::command]
async fn play_test_tone() -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(|| audio::play_test_tone().map_err(|err| err.to_string()))
        .await
        .map_err(|err| err.to_string())?
}

/// Import a WAV from a source path, or open the file picker when path is omitted.
/// The audio is decoded and resampled to 48 kHz before its buffer enters the
/// in-memory library. Return its ID and metadata, or null if selection is cancelled.
#[tauri::command]
async fn import_audio(
    app: AppHandle,
    library: tauri::State<'_, audio_library::AudioLibrary>,
    path: Option<String>,
) -> Result<Option<audio_library::ImportedAudio>, String> {
    // Share the library with the background task without copying audio samples.
    let library = library.inner().clone();

    tauri::async_runtime::spawn_blocking(move || {
        // Resolve the source in this background task so file selection and import
        // processing do not block the UI thread.
        let source = match path {
            Some(path) => {
                // Use the supplied path, rejecting empty input before opening a file.
                if path.trim().is_empty() {
                    return Err("Audio source path must not be empty".into());
                }
                PathBuf::from(path)
            }
            None => {
                // No path was supplied: open the native WAV file picker.
                let Some(file) = app
                    .dialog()
                    .file()
                    .add_filter("WAV audio", &["wav"])
                    .blocking_pick_file()
                else {
                    return Ok(None);
                };
                // Convert the selected file to a local filesystem path.
                file.into_path().map_err(|error| error.to_string())?
            }
        };

        // Decode -> resample to 48 kHz -> create AudioBuffer -> store under a new ID.
        // Wrap returned metadata in Some; import errors propagate to the frontend.
        library.import(&source).map(Some)
    })
    // Wait for the task, convert task errors to strings, and return its result.
    .await
    .map_err(|error| error.to_string())?
}

/// Return metadata for all imported files in unspecified order. Audio samples
/// remain in the library's shared buffers and are not sent to the frontend.
#[tauri::command]
fn list_imported_audio(
    library: tauri::State<'_, audio_library::AudioLibrary>,
) -> Result<Vec<audio_library::ImportedAudio>, String> {
    library.list()
}

/// Return one imported file's metadata by ID. An invalid or missing ID returns
/// an error. Backend playback retrieves the samples with AudioLibrary::get_buffer.
#[tauri::command]
fn get_imported_audio(
    id: String,
    library: tauri::State<'_, audio_library::AudioLibrary>,
) -> Result<audio_library::ImportedAudio, String> {
    library.get(&id)
}

/// Remove an imported file from the library; return false if its ID is missing.
/// Its buffer is freed when the last shared reference is dropped, so active
/// playback references remain valid after removal.
#[tauri::command]
fn remove_imported_audio(
    id: String,
    library: tauri::State<'_, audio_library::AudioLibrary>,
) -> Result<bool, String> {
    library.remove(&id)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(audio_library::AudioLibrary::default())
        .manage(PlaybackVolume::default())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            app.manage(AudioEngine::new());
            app.manage(Recorder::new());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            ping,
            systemHealth,
            get_input_devices,
            get_output_devices,
            get_default_output_device,
            get_default_input_device,
            start_recording,
            stop_recording,
            play_audio,
            pause_audio,
            resume_audio,
            stop_audio,
            play_test_tone,
            import_audio,
            list_imported_audio,
            get_imported_audio,
            remove_imported_audio,
            set_master_volume
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
