// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::process::Command;
use std::sync::Mutex;
use tauri::{command, State};
use std::path::PathBuf;

// State structure to hold selected files
struct AppState {
    selected_files: Mutex<Vec<PathBuf>>,
}

#[derive(Deserialize)]
struct ConvertRequest {
    format: String,
    delete_original: bool,
}

#[derive(Serialize)]
struct FileList {
    paths: Vec<String>,
    count: usize,
}

// Add files to state
#[command]
fn add_files(state: State<AppState>, paths: Vec<String>) -> FileList {
    let mut files = state.selected_files.lock().unwrap();
    for path in paths {
        files.push(PathBuf::from(path));
    }
    
    FileList {
        paths: files.iter().map(|p| p.to_string_lossy().into_owned()).collect(),
        count: files.len(),
    }
}

// Clear file selection
#[command]
fn clear_files(state: State<AppState>) {
    let mut files = state.selected_files.lock().unwrap();
    files.clear();
}

// Get current file list
#[command]
fn get_selected_files(state: State<AppState>) -> FileList {
    let files = state.selected_files.lock().unwrap();
    FileList {
        paths: files.iter().map(|p| p.to_string_lossy().into_owned()).collect(),
        count: files.len(),
    }
}

#[command]
fn convert(state: State<AppState>, payload: ConvertRequest) -> Vec<String> {
    let files = state.selected_files.lock().unwrap();
    let mut results = Vec::new();

    for file_path in files.iter() {
        let output = Command::new("./convert_heic.sh")
            .arg(file_path)
            .arg(format!("--format={}", payload.format))
            .arg(if payload.delete_original { "--delete" } else { "" })
            .output();

        let result = match output {
            Ok(o) if o.status.success() => {
                String::from_utf8_lossy(&o.stdout).to_string()
            }
            Ok(o) => format!("Script failed for {}: {}", 
                file_path.display(),
                String::from_utf8_lossy(&o.stderr)),
            Err(e) => format!("Error processing {}: {}", file_path.display(), e),
        };
        results.push(result);
    }
    results
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(AppState {
            selected_files: Mutex::new(Vec::new()),
        })
        .invoke_handler(tauri::generate_handler![
            add_files,
            clear_files,
            get_selected_files,
            convert
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}