use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize, Clone)]
pub struct Note {
    freq: f32,
    start: f32,
    dur: f32,
    inst: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Recording {
    name: String,
    notes: Vec<Note>,
}

const DB_FILE: &str = "music_recordings.json";

#[tauri::command]
pub fn load_recordings() -> Vec<Recording> {
    if let Ok(data) = fs::read_to_string(DB_FILE) {
        serde_json::from_str(&data).unwrap_or_else(|_| vec![])
    } else {
        vec![]
    }
}

#[tauri::command]
pub fn save_recording(recording: Recording) -> Result<(), String> {
    let mut recs = load_recordings();
    recs.push(recording);
    fs::write(DB_FILE, serde_json::to_string(&recs).unwrap()).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_recording(index: usize) -> Result<(), String> {
    let mut recs = load_recordings();
    if index < recs.len() {
        recs.remove(index);
        fs::write(DB_FILE, serde_json::to_string(&recs).unwrap()).map_err(|e| e.to_string())?;
    }
    Ok(())
}
