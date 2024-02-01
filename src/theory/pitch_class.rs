use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(dead_code)]
pub enum PitchClass {
    C,
    CSharp,
    D,
    DSharp,
    E,
    F,
    FSharp,
    G,
    GSharp,
    A,
    ASharp,
    B,
}

impl Display for PitchClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            PitchClass::C => "C",
            PitchClass::CSharp => "C#",
            PitchClass::D => "D",
            PitchClass::DSharp => "D#",
            PitchClass::E => "E",
            PitchClass::F => "F",
            PitchClass::FSharp => "F#",
            PitchClass::G => "G",
            PitchClass::GSharp => "G#",
            PitchClass::A => "A",
            PitchClass::ASharp => "A#",
            PitchClass::B => "B",
        };
        write!(f, "{s}")
    }
}
