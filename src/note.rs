#[derive(PartialEq, Eq, Hash)]
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
}

// TODO: support other base frequencies (e.g. 442, 444)
impl Note {
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
        }
    }
}
