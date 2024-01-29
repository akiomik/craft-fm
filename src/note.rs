use std::fmt::Display;

use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[wasm_bindgen]
pub enum Note {
    C0 = 12,
    Csharp0 = 13,
    D0 = 14,
    Dsharp0 = 15,
    E0 = 16,
    F0 = 17,
    Fsharp0 = 18,
    G0 = 19,
    Gsharp0 = 20,
    A0 = 21,
    Asharp0 = 22,
    B0 = 23,
    C1 = 24,
    Csharp1 = 25,
    D1 = 26,
    Dsharp1 = 27,
    E1 = 28,
    F1 = 29,
    Fsharp1 = 30,
    G1 = 31,
    Gsharp1 = 32,
    A1 = 33,
    Asharp1 = 34,
    B1 = 35,
    C2 = 36,
    Csharp2 = 37,
    D2 = 38,
    Dsharp2 = 39,
    E2 = 40,
    F2 = 41,
    Fsharp2 = 42,
    G2 = 43,
    Gsharp2 = 44,
    A2 = 45,
    Asharp2 = 46,
    B2 = 47,
    C3 = 48,
    Csharp3 = 49,
    D3 = 50,
    Dsharp3 = 51,
    E3 = 52,
    F3 = 53,
    Fsharp3 = 54,
    G3 = 55,
    Gsharp3 = 56,
    A3 = 57,
    Asharp3 = 58,
    B3 = 59,
    C4 = 60,
    Csharp4 = 61,
    D4 = 62,
    Dsharp4 = 63,
    E4 = 64,
    F4 = 65,
    Fsharp4 = 66,
    G4 = 67,
    Gsharp4 = 68,
    A4 = 69,
    Asharp4 = 70,
    B4 = 71,
}

impl Note {
    const SEMITONES: u8 = 12;

    pub fn from_note_number(note_number: u8) -> Option<Self> {
        match note_number {
            12 => Some(Note::C0),
            13 => Some(Note::Csharp0),
            14 => Some(Note::D0),
            15 => Some(Note::Dsharp0),
            16 => Some(Note::E0),
            17 => Some(Note::F0),
            18 => Some(Note::Fsharp0),
            19 => Some(Note::G0),
            20 => Some(Note::Gsharp0),
            21 => Some(Note::A0),
            22 => Some(Note::Asharp0),
            23 => Some(Note::B0),
            24 => Some(Note::C1),
            25 => Some(Note::Csharp1),
            26 => Some(Note::D1),
            27 => Some(Note::Dsharp1),
            28 => Some(Note::E1),
            29 => Some(Note::F1),
            30 => Some(Note::Fsharp1),
            31 => Some(Note::G1),
            32 => Some(Note::Gsharp1),
            33 => Some(Note::A1),
            34 => Some(Note::Asharp1),
            35 => Some(Note::B1),
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

    #[inline]
    pub fn note_number(&self) -> u8 {
        self.clone() as u8
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
        assert_eq!(Note::C0.octave(), 0);
        assert_eq!(Note::A0.octave(), 0);
        assert_eq!(Note::C1.octave(), 1);
        assert_eq!(Note::A1.octave(), 1);
        assert_eq!(Note::C2.octave(), 2);
        assert_eq!(Note::A2.octave(), 2);
        assert_eq!(Note::C3.octave(), 3);
        assert_eq!(Note::A3.octave(), 3);
        assert_eq!(Note::C4.octave(), 4);
        assert_eq!(Note::A4.octave(), 4);
    }

    #[test]
    fn test_octave_up() {
        assert_eq!(Note::C0.octave_up(), Some(Note::C1));
        assert_eq!(Note::A0.octave_up(), Some(Note::A1));
        assert_eq!(Note::C1.octave_up(), Some(Note::C2));
        assert_eq!(Note::A1.octave_up(), Some(Note::A2));
        assert_eq!(Note::C2.octave_up(), Some(Note::C3));
        assert_eq!(Note::A2.octave_up(), Some(Note::A3));
        assert_eq!(Note::C3.octave_up(), Some(Note::C4));
        assert_eq!(Note::A3.octave_up(), Some(Note::A4));
        assert_eq!(Note::C4.octave_up(), None);
        assert_eq!(Note::A4.octave_up(), None);
    }

    #[test]
    fn test_octave_down() {
        assert_eq!(Note::C0.octave_down(), None);
        assert_eq!(Note::A0.octave_down(), None);
        assert_eq!(Note::C1.octave_down(), Some(Note::C0));
        assert_eq!(Note::A1.octave_down(), Some(Note::A0));
        assert_eq!(Note::C2.octave_down(), Some(Note::C1));
        assert_eq!(Note::A2.octave_down(), Some(Note::A1));
        assert_eq!(Note::C3.octave_down(), Some(Note::C2));
        assert_eq!(Note::A3.octave_down(), Some(Note::A2));
        assert_eq!(Note::C4.octave_down(), Some(Note::C3));
        assert_eq!(Note::A4.octave_down(), Some(Note::A3));
    }

    #[test]
    fn test_freq() {
        assert_eq!(Note::C0.freq(), 16.351597);
        assert_eq!(Note::A0.freq(), 27.5);
        assert_eq!(Note::C1.freq(), 32.703194);
        assert_eq!(Note::A1.freq(), 55.0);
        assert_eq!(Note::C2.freq(), 65.40639);
        assert_eq!(Note::A2.freq(), 110.0);
        assert_eq!(Note::C3.freq(), 130.81277);
        assert_eq!(Note::A3.freq(), 220.0);
        assert_eq!(Note::C4.freq(), 261.62555);
        assert_eq!(Note::A4.freq(), 440.0);
    }

    #[test]
    fn test_pitch_class() {
        assert_eq!(Note::C0.pitch_class(), 0);
        assert_eq!(Note::A0.pitch_class(), 9);
        assert_eq!(Note::C1.pitch_class(), 0);
        assert_eq!(Note::A1.pitch_class(), 9);
        assert_eq!(Note::C2.pitch_class(), 0);
        assert_eq!(Note::A2.pitch_class(), 9);
        assert_eq!(Note::C3.pitch_class(), 0);
        assert_eq!(Note::A3.pitch_class(), 9);
        assert_eq!(Note::C4.pitch_class(), 0);
        assert_eq!(Note::A4.pitch_class(), 9);
    }

    #[test]
    fn test_pitch_class_label() {
        assert_eq!(Note::C0.pitch_class_label(), "C");
        assert_eq!(Note::A0.pitch_class_label(), "A");
        assert_eq!(Note::C1.pitch_class_label(), "C");
        assert_eq!(Note::A1.pitch_class_label(), "A");
        assert_eq!(Note::C2.pitch_class_label(), "C");
        assert_eq!(Note::A2.pitch_class_label(), "A");
        assert_eq!(Note::C3.pitch_class_label(), "C");
        assert_eq!(Note::A3.pitch_class_label(), "A");
        assert_eq!(Note::C4.pitch_class_label(), "C");
        assert_eq!(Note::A4.pitch_class_label(), "A");
    }

    #[test]
    fn test_format() {
        assert_eq!(Note::C1.to_string(), "C1");
        assert_eq!(Note::A1.to_string(), "A1");
        assert_eq!(Note::C2.to_string(), "C2");
        assert_eq!(Note::A2.to_string(), "A2");
        assert_eq!(Note::C3.to_string(), "C3");
        assert_eq!(Note::A3.to_string(), "A3");
        assert_eq!(Note::C4.to_string(), "C4");
        assert_eq!(Note::A4.to_string(), "A4");
    }
}
