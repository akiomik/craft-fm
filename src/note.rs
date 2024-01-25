use wasm_bindgen::prelude::*;

#[derive(Clone, PartialEq, Eq, Hash)]
#[wasm_bindgen]
pub enum Note {
    C3,
    Csharp3,
    D3,
    Dsharp3,
    E3,
    F3,
    Fsharp3,
    G3,
    Gsharp3,
    A3,
    Asharp3,
    B3,
    C4,
    Csharp4,
    D4,
    Dsharp4,
    E4,
    F4,
    Fsharp4,
    G4,
    Gsharp4,
    A4,
    Asharp4,
    B4,
}

impl Note {
    pub fn octave(&self) -> u8 {
        match self {
            Note::C3 => 3,
            Note::Csharp3 => 3,
            Note::D3 => 3,
            Note::Dsharp3 => 3,
            Note::E3 => 3,
            Note::F3 => 3,
            Note::Fsharp3 => 3,
            Note::G3 => 3,
            Note::Gsharp3 => 3,
            Note::A3 => 3,
            Note::Asharp3 => 3,
            Note::B3 => 3,
            Note::C4 => 4,
            Note::Csharp4 => 4,
            Note::D4 => 4,
            Note::Dsharp4 => 4,
            Note::E4 => 4,
            Note::F4 => 4,
            Note::Fsharp4 => 4,
            Note::G4 => 4,
            Note::Gsharp4 => 4,
            Note::A4 => 4,
            Note::Asharp4 => 4,
            Note::B4 => 4,
        }
    }

    // TODO: support other base frequencies (e.g. 442, 444)
    pub fn freq(&self) -> f32 {
        match self {
            Note::C3 => 130.81,
            Note::Csharp3 => 138.59,
            Note::D3 => 146.83,
            Note::Dsharp3 => 155.56,
            Note::E3 => 164.81,
            Note::F3 => 174.61,
            Note::Fsharp3 => 185.00,
            Note::G3 => 196.00,
            Note::Gsharp3 => 207.65,
            Note::A3 => 220.00,
            Note::Asharp3 => 233.08,
            Note::B3 => 246.94,
            Note::C4 => 261.63,
            Note::Csharp4 => 277.18,
            Note::D4 => 293.66,
            Note::Dsharp4 => 311.13,
            Note::E4 => 329.63,
            Note::F4 => 349.23,
            Note::Fsharp4 => 369.99,
            Note::G4 => 392.00,
            Note::Gsharp4 => 415.30,
            Note::A4 => 440.00,
            Note::Asharp4 => 466.16,
            Note::B4 => 493.88,
        }
    }
}
