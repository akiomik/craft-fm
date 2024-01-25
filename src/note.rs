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
        440.0 * (2.0 as f32).powf((self.note_number() as i8 - 69) as f32 / 12.0)
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
