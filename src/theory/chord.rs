use std::fmt::Display;

use super::{ChordLike, Interval, Note};

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
impl ChordLike for Chord {
    fn intervals(&self) -> Vec<Interval> {
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

    fn notes(&self) -> Vec<Note> {
        self.intervals()
            .iter()
            .filter_map(|interval| {
                Note::from_note_number(self.root_note().note_number() + interval.semitones())
            })
            .collect()
    }

    fn root_note(&self) -> Note {
        match self {
            Chord::Major(note)
            | Chord::Minor(note)
            | Chord::Major7th(note)
            | Chord::Minor7th(note)
            | Chord::Major9th(note)
            | Chord::Minor9th(note) => note.clone(),
        }
    }
}

impl Display for Chord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Chord::Major(note) => note.pitch_class().to_string(),
            Chord::Minor(note) => format!("{}m", note.pitch_class()),
            Chord::Major7th(note) => format!("{}M7", note.pitch_class()),
            Chord::Minor7th(note) => format!("{}m7", note.pitch_class()),
            Chord::Major9th(note) => format!("{}M9", note.pitch_class()),
            Chord::Minor9th(note) => format!("{}m9", note.pitch_class()),
        };
        write!(f, "{s}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(Chord::Major(Note::C3).to_string(), "C");
        assert_eq!(Chord::Minor(Note::C3).to_string(), "Cm");
        assert_eq!(Chord::Major7th(Note::C3).to_string(), "CM7");
        assert_eq!(Chord::Minor7th(Note::C3).to_string(), "Cm7");
        assert_eq!(Chord::Major9th(Note::C3).to_string(), "CM9");
        assert_eq!(Chord::Minor9th(Note::C3).to_string(), "Cm9");
    }

    #[test]
    fn test_notes() {
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
