use serde::Serialize;

#[derive(Serialize)]
pub struct MelodyNote {
    pub freq: f32,
    pub start: f32,
    pub dur: f32,
}

#[tauri::command]
pub fn get_theme(theme: String) -> Vec<MelodyNote> {
    match theme.as_str() {
        "doom" => vec![
            MelodyNote { freq: 164.81, start: 0.00, dur: 0.12 },
            MelodyNote { freq: 164.81, start: 0.15, dur: 0.12 },
            MelodyNote { freq: 329.63, start: 0.30, dur: 0.12 },
            MelodyNote { freq: 164.81, start: 0.45, dur: 0.12 },
            MelodyNote { freq: 164.81, start: 0.60, dur: 0.12 },
            MelodyNote { freq: 293.66, start: 0.75, dur: 0.12 },
            MelodyNote { freq: 164.81, start: 0.90, dur: 0.12 },
            MelodyNote { freq: 164.81, start: 1.05, dur: 0.12 },
            MelodyNote { freq: 261.63, start: 1.20, dur: 0.12 },
            MelodyNote { freq: 164.81, start: 1.35, dur: 0.12 },
            MelodyNote { freq: 164.81, start: 1.50, dur: 0.12 },
            MelodyNote { freq: 233.08, start: 1.65, dur: 0.15 },
        ],
        "doom_bridge" => vec![
            MelodyNote { freq: 164.81, start: 0.00, dur: 0.12 },
            MelodyNote { freq: 164.81, start: 0.15, dur: 0.12 },
            MelodyNote { freq: 196.00, start: 0.30, dur: 0.12 },
            MelodyNote { freq: 164.81, start: 0.45, dur: 0.12 },
            MelodyNote { freq: 164.81, start: 0.60, dur: 0.12 },
            MelodyNote { freq: 220.00, start: 0.75, dur: 0.12 },
            MelodyNote { freq: 164.81, start: 0.90, dur: 0.12 },
            MelodyNote { freq: 164.81, start: 1.05, dur: 0.12 },
            MelodyNote { freq: 233.08, start: 1.20, dur: 0.15 },
        ],
        "doom_suspense" => vec![
            MelodyNote { freq: 164.81, start: 0.00, dur: 0.45 },
            MelodyNote { freq: 196.00, start: 0.55, dur: 0.45 },
            MelodyNote { freq: 185.00, start: 1.10, dur: 0.45 },
            MelodyNote { freq: 164.81, start: 1.65, dur: 0.65 },
            MelodyNote { freq: 246.94, start: 2.40, dur: 0.55 },
        ],
        "doom_heavy" => vec![
            MelodyNote { freq: 130.81, start: 0.00, dur: 0.20 },
            MelodyNote { freq: 130.81, start: 0.25, dur: 0.20 },
            MelodyNote { freq: 146.83, start: 0.50, dur: 0.20 },
            MelodyNote { freq: 164.81, start: 0.75, dur: 0.35 },
            MelodyNote { freq: 130.81, start: 1.20, dur: 0.20 },
            MelodyNote { freq: 146.83, start: 1.45, dur: 0.20 },
            MelodyNote { freq: 138.59, start: 1.70, dur: 0.40 },
        ],
        _ => vec![],
    }
}
