use std::fmt::Display;

use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[wasm_bindgen]
pub enum Note {
    C2,
    Csharp2,
    D2,
    Dsharp2,
    E2,
    F2,
    Fsharp2,
    G2,
    Gsharp2,
    A2,
    Asharp2,
    B2,
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
    const SEMITONES: u8 = 12;

    pub fn from_note_number(note_number: u8) -> Option<Self> {
        match note_number {
            36 => Some(Note::C2),
            37 => Some(Note::Csharp2),
            38 => Some(Note::D2),
            39 => Some(Note::Dsharp2),
            40 => Some(Note::E2),
            41 => Some(Note::F2),
            42 => Some(Note::Fsharp2),
            43 => Some(Note::G2),
            44 => Some(Note::Gsharp2),
            45 => Some(Note::A2),
            46 => Some(Note::Asharp2),
            47 => Some(Note::B2),
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

    pub fn octave(&self) -> i8 {
        (self.note_number() / Self::SEMITONES) as i8 - 1
    }

    pub fn octave_up(&self) -> Option<Note> {
        Self::from_note_number(self.note_number() + Self::SEMITONES)
    }

    pub fn octave_down(&self) -> Option<Note> {
        Self::from_note_number(self.note_number() - Self::SEMITONES)
    }

    pub fn note_number(&self) -> u8 {
        match self {
            Note::C2 => 36,
            Note::Csharp2 => 37,
            Note::D2 => 38,
            Note::Dsharp2 => 39,
            Note::E2 => 40,
            Note::F2 => 41,
            Note::Fsharp2 => 42,
            Note::G2 => 43,
            Note::Gsharp2 => 44,
            Note::A2 => 45,
            Note::Asharp2 => 46,
            Note::B2 => 47,
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
        let relative_note_number = self.note_number() as i16 - Note::A4.note_number() as i16;
        440.0 * (2.0_f32).powf(relative_note_number as f32 / Self::SEMITONES as f32)
    }

    pub fn pitch_class(&self) -> u8 {
        self.note_number() % Self::SEMITONES
    }

    pub fn pitch_class_label(&self) -> &str {
        match self.pitch_class() {
            0 => "C",
            1 => "C#",
            2 => "D",
            3 => "C#",
            4 => "E",
            5 => "F",
            6 => "F#",
            7 => "G",
            8 => "G#",
            9 => "A",
            10 => "A#",
            11 => "B",
            _ => unreachable!("unsupported pitch class"),
        }
    }
}

impl Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = self.pitch_class_label();
        let octave = self.octave();
        write!(f, "{label}{octave}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_octave() {
        assert_eq!(Note::C2.octave(), 2);
        assert_eq!(Note::A2.octave(), 2);
        assert_eq!(Note::C3.octave(), 3);
        assert_eq!(Note::A3.octave(), 3);
        assert_eq!(Note::C4.octave(), 4);
        assert_eq!(Note::A4.octave(), 4);
    }

    #[test]
    fn test_octave_up() {
        assert_eq!(Note::C2.octave_up(), Some(Note::C3));
        assert_eq!(Note::A2.octave_up(), Some(Note::A3));
        assert_eq!(Note::C3.octave_up(), Some(Note::C4));
        assert_eq!(Note::A3.octave_up(), Some(Note::A4));
        assert_eq!(Note::C4.octave_up(), None);
        assert_eq!(Note::A4.octave_up(), None);
    }

    #[test]
    fn test_octave_down() {
        assert_eq!(Note::C2.octave_down(), None);
        assert_eq!(Note::A2.octave_down(), None);
        assert_eq!(Note::C3.octave_down(), Some(Note::C2));
        assert_eq!(Note::A3.octave_down(), Some(Note::A2));
        assert_eq!(Note::C4.octave_down(), Some(Note::C3));
        assert_eq!(Note::A4.octave_down(), Some(Note::A3));
    }

    #[test]
    fn test_freq() {
        assert_eq!(Note::C2.freq(), 65.40639);
        assert_eq!(Note::A2.freq(), 110.0);
        assert_eq!(Note::C3.freq(), 130.81277);
        assert_eq!(Note::A3.freq(), 220.0);
        assert_eq!(Note::C4.freq(), 261.62555);
        assert_eq!(Note::A4.freq(), 440.0);
    }

    #[test]
    fn test_pitch_class() {
        assert_eq!(Note::C2.pitch_class(), 0);
        assert_eq!(Note::A2.pitch_class(), 9);
        assert_eq!(Note::C3.pitch_class(), 0);
        assert_eq!(Note::A3.pitch_class(), 9);
        assert_eq!(Note::C4.pitch_class(), 0);
        assert_eq!(Note::A4.pitch_class(), 9);
    }

    #[test]
    fn test_pitch_class_label() {
        assert_eq!(Note::C2.pitch_class_label(), "C");
        assert_eq!(Note::A2.pitch_class_label(), "A");
        assert_eq!(Note::C3.pitch_class_label(), "C");
        assert_eq!(Note::A3.pitch_class_label(), "A");
        assert_eq!(Note::C4.pitch_class_label(), "C");
        assert_eq!(Note::A4.pitch_class_label(), "A");
    }

    #[test]
    fn test_format() {
        assert_eq!(Note::C2.to_string(), "C2");
        assert_eq!(Note::A2.to_string(), "A2");
        assert_eq!(Note::C3.to_string(), "C3");
        assert_eq!(Note::A3.to_string(), "A3");
        assert_eq!(Note::C4.to_string(), "C4");
        assert_eq!(Note::A4.to_string(), "A4");
    }
}
