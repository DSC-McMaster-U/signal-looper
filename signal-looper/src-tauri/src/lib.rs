// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use cpal::traits::{DeviceTrait, HostTrait};



//tester functions for frontend to call

#[tauri::command] //ping pong
fn ping() -> String {
    "pong".to_string()
}

#[tauri::command]
fn systemHealth() -> SystemHealth {
    SystemHealth {
        //replace with actual information when available
        status: "ok".to_string(),
        backend: "Rust".to_string(),
        message: "System is healthy AF".to_string()
    }
}

#[tauri::command]
fn get_input_devices() -> Vec<String> {

    let host = cpal::default_host();

    let input_devices = host.input_devices()
        .expect("Failed to get input devices")
        .map(|device| device.name().expect("Failed to get device name"))
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
        .map(|device| device.name().expect("Failed to get device name"))
        .collect();

    output_devices
}

#[tauri::command]
fn get_default_input_device() -> Option<String> {
    let host = cpal::default_host();

    host.default_input_device() //if there is a default input device, get its name and return it as an Option<String>
        .and_then(|device| device.name().ok())
}

#[tauri::command]
fn get_default_output_device() -> Option<String> {
    let host = cpal::default_host();

    host.default_output_device() 
        .and_then(|device| device.name().ok())
}

#[tauri::command]
fn get_default_input_device()





#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, ping, systemHealth, get_input_devices, get_output_devices])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
