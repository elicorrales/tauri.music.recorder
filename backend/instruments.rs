use serde::Serialize;

// The sound-design data (oscillator type + ADSR envelope per instrument)
// lives here in compiled Rust instead of the HTML, mirroring get_theme.
#[derive(Serialize)]
pub struct Instrument {
    pub id: String,
    pub label: String,
    pub sublabel: String,
    pub wave: String, // oscillator type: sine | triangle | square | sawtooth
    pub attack: f32,
    pub decay: f32,
    pub sustain: f32,
    pub release: f32,
}

fn inst(id: &str, label: &str, sublabel: &str, wave: &str,
        attack: f32, decay: f32, sustain: f32, release: f32) -> Instrument {
    Instrument {
        id: id.into(), label: label.into(), sublabel: sublabel.into(), wave: wave.into(),
        attack, decay, sustain, release,
    }
}

#[tauri::command]
pub fn get_instruments() -> Vec<Instrument> {
    vec![
        inst("flute",    "Soft Flute",    "Pure / Gentle (Sine)",      "sine",     0.15, 0.2, 0.8, 0.25),
        inst("musicbox", "Music Box",     "Soft / Plucky (Triangle)",  "triangle", 0.01, 0.6, 0.0, 0.6),
        inst("organ",    "Church Organ",  "Full / Sustained (Triangle)","triangle",0.04, 0.0, 1.0, 0.15),
        inst("retro",    "8-Bit Arcade",  "Hollow / Retro (Square)",   "square",   0.01, 0.1, 0.4, 0.1),
        inst("brass",    "Synth Brass",   "Edgy / Buzzing (Sawtooth)", "sawtooth", 0.08, 0.2, 0.6, 0.2),
        inst("drone",    "Dungeon Drone", "Foreboding / Eerie (Sawtooth)","sawtooth",0.4, 0.4, 0.7, 1.2),
    ]
}
