use std::fmt::Display;

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
    pub fn from_note_number(note_number: u8) -> Option<Self> {
        match note_number {
            48 => Some(Note::C3),
            49 => Some(Note::Csharp3),
            50 => Some(Note::D3),
            51 => Some(Note::Dsharp3),
            52 => Some(Note::E3),
            53 => Some(Note::F3),
            54 => Some(Note::Fsharp3),
            55 => Some(Note::G3),
            56 => Some(Note::Gsharp3),
            57 => Some(Note::A3),
            58 => Some(Note::Asharp3),
            59 => Some(Note::B3),
            60 => Some(Note::C4),
            61 => Some(Note::Csharp4),
            62 => Some(Note::D4),
            63 => Some(Note::Dsharp4),
            64 => Some(Note::E4),
            65 => Some(Note::F4),
            66 => Some(Note::Fsharp4),
            67 => Some(Note::G4),
            68 => Some(Note::Gsharp4),
            69 => Some(Note::A4),
            70 => Some(Note::Asharp4),
            71 => Some(Note::B4),
            _ => None,
        }
    }

    pub fn octave(&self) -> u8 {
        self.note_number() / 12 - 1
    }

    pub fn note_number(&self) -> u8 {
        match self {
            Note::C3 => 48,
            Note::Csharp3 => 49,
            Note::D3 => 50,
            Note::Dsharp3 => 51,
            Note::E3 => 52,
            Note::F3 => 53,
            Note::Fsharp3 => 54,
            Note::G3 => 55,
            Note::Gsharp3 => 56,
            Note::A3 => 57,
            Note::Asharp3 => 58,
            Note::B3 => 59,
            Note::C4 => 60,
            Note::Csharp4 => 61,
            Note::D4 => 62,
            Note::Dsharp4 => 63,
            Note::E4 => 64,
            Note::F4 => 65,
            Note::Fsharp4 => 66,
            Note::G4 => 67,
            Note::Gsharp4 => 68,
            Note::A4 => 69,
            Note::Asharp4 => 70,
            Note::B4 => 71,
        }
    }

    // TODO: support other base frequencies (e.g. 442, 444)
    pub fn freq(&self) -> f32 {
        440.0 * (2.0_f32).powf((self.note_number() as i8 - 69) as f32 / 12.0)
    }
}

impl Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Note::C3 => "C3",
            Note::Csharp3 => "C#3",
            Note::D3 => "D3",
            Note::Dsharp3 => "D#3",
            Note::E3 => "E3",
            Note::F3 => "F3",
            Note::Fsharp3 => "F#3",
            Note::G3 => "G3",
            Note::Gsharp3 => "G#3",
            Note::A3 => "A3",
            Note::Asharp3 => "A#3",
            Note::B3 => "B3",
            Note::C4 => "C4",
            Note::Csharp4 => "C#4",
            Note::D4 => "D4",
            Note::Dsharp4 => "D#4",
            Note::E4 => "E4",
            Note::F4 => "F4",
            Note::Fsharp4 => "F#4",
            Note::G4 => "G4",
            Note::Gsharp4 => "G#4",
            Note::A4 => "A4",
            Note::Asharp4 => "A#4",
            Note::B4 => "B4",
        };
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_octave() {
        assert_eq!(Note::C3.octave(), 3);
        assert_eq!(Note::A3.octave(), 3);
        assert_eq!(Note::C4.octave(), 4);
        assert_eq!(Note::A4.octave(), 4);
    }

    #[test]
    fn test_freq() {
        assert_eq!(Note::C3.freq(), 130.81277);
        assert_eq!(Note::A3.freq(), 220.0);
        assert_eq!(Note::C4.freq(), 261.62555);
        assert_eq!(Note::A4.freq(), 440.0);
    }
}
