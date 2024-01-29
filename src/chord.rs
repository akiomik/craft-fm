use std::fmt::Display;

use crate::note::Note;

#[allow(dead_code)]
pub enum Quality {
    P,
    A,
    D,
    Min,
    Maj,
}

impl Display for Quality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Quality::P => "P",
            Quality::A => "A",
            Quality::D => "d",
            Quality::Min => "m",
            Quality::Maj => "M",
        };
        write!(f, "{s}")
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Interval {
    P1,
    A1,
    D2,
    Min2,
    Maj2,
    A2,
    D3,
    Min3,
    Maj3,
    A3,
    D4,
    P4,
    A4,
    D5,
    P5,
    A5,
    D6,
    Min6,
    Maj6,
    A6,
    D7,
    Min7,
    Maj7,
    A7,
    D8,
    P8,
    A8,
    Min9,
    Maj9,
}

#[allow(dead_code)]
impl Interval {
    pub fn degree(&self) -> u8 {
        match self {
            Interval::P1 => 1,
            Interval::A1 => 1,
            Interval::D2 => 2,
            Interval::Min2 => 2,
            Interval::Maj2 => 2,
            Interval::A2 => 2,
            Interval::D3 => 3,
            Interval::Min3 => 3,
            Interval::Maj3 => 3,
            Interval::A3 => 3,
            Interval::D4 => 4,
            Interval::P4 => 4,
            Interval::A4 => 4,
            Interval::D5 => 5,
            Interval::P5 => 5,
            Interval::A5 => 5,
            Interval::D6 => 6,
            Interval::Min6 => 6,
            Interval::Maj6 => 6,
            Interval::A6 => 6,
            Interval::D7 => 7,
            Interval::Min7 => 7,
            Interval::Maj7 => 7,
            Interval::A7 => 7,
            Interval::D8 => 8,
            Interval::P8 => 8,
            Interval::A8 => 8,
            Interval::Min9 => 9,
            Interval::Maj9 => 9,
        }
    }

    pub fn semitones(&self) -> u8 {
        match self {
            Interval::P1 => 0,
            Interval::A1 => 1,
            Interval::D2 => 0,
            Interval::Min2 => 1,
            Interval::Maj2 => 2,
            Interval::A2 => 3,
            Interval::D3 => 2,
            Interval::Min3 => 3,
            Interval::Maj3 => 4,
            Interval::A3 => 5,
            Interval::D4 => 4,
            Interval::P4 => 5,
            Interval::A4 => 6,
            Interval::D5 => 6,
            Interval::P5 => 7,
            Interval::A5 => 8,
            Interval::D6 => 7,
            Interval::Min6 => 8,
            Interval::Maj6 => 9,
            Interval::A6 => 10,
            Interval::D7 => 9,
            Interval::Min7 => 10,
            Interval::Maj7 => 11,
            Interval::A7 => 12,
            Interval::D8 => 11,
            Interval::P8 => 12,
            Interval::A8 => 13,
            Interval::Min9 => 13,
            Interval::Maj9 => 14,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Chord {
    Major(Note),
    Minor(Note),
    Major7th(Note),
    Minor7th(Note),
    Major9th(Note),
    Minor9th(Note),
}

#[allow(dead_code)]
impl Chord {
    pub fn intervals(&self) -> Vec<Interval> {
        match self {
            Chord::Major(_) => vec![Interval::P1, Interval::Maj3, Interval::P5],
            Chord::Minor(_) => vec![Interval::P1, Interval::Min3, Interval::P5],
            Chord::Major7th(_) => vec![Interval::P1, Interval::Maj3, Interval::P5, Interval::Maj7],
            Chord::Minor7th(_) => vec![Interval::P1, Interval::Min3, Interval::P5, Interval::Min7],
            Chord::Major9th(_) => vec![
                Interval::P1,
                Interval::Maj3,
                Interval::P5,
                Interval::Maj7,
                Interval::Maj9,
            ],
            Chord::Minor9th(_) => vec![
                Interval::P1,
                Interval::Min3,
                Interval::P5,
                Interval::Min7,
                Interval::Maj9,
            ],
        }
    }

    pub fn notes(&self) -> Vec<Note> {
        match self {
            Chord::Major(note)
            | Chord::Minor(note)
            | Chord::Major7th(note)
            | Chord::Minor7th(note)
            | Chord::Major9th(note)
            | Chord::Minor9th(note) => self
                .intervals()
                .iter()
                .filter_map(|interval| {
                    Note::from_note_number(note.note_number() + interval.semitones())
                })
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chord_notes() {
        assert_eq!(
            Chord::Major(Note::C3).notes(),
            vec![Note::C3, Note::E3, Note::G3]
        );
        assert_eq!(
            Chord::Minor(Note::C3).notes(),
            vec![Note::C3, Note::Dsharp3, Note::G3]
        );
        assert_eq!(
            Chord::Major7th(Note::C3).notes(),
            vec![Note::C3, Note::E3, Note::G3, Note::B3]
        );
        assert_eq!(
            Chord::Minor7th(Note::C3).notes(),
            vec![Note::C3, Note::Dsharp3, Note::G3, Note::Asharp3]
        );
        assert_eq!(
            Chord::Major9th(Note::C3).notes(),
            vec![Note::C3, Note::E3, Note::G3, Note::B3, Note::D4]
        );
        assert_eq!(
            Chord::Minor9th(Note::C3).notes(),
            vec![Note::C3, Note::Dsharp3, Note::G3, Note::Asharp3, Note::D4]
        );
    }
}
