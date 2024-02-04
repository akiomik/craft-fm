use std::collections::VecDeque;

use super::{ChordLike, Note};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Inversion<T>(pub T);

impl<T: ChordLike> ChordLike for Inversion<T> {
    fn notes(&self) -> Vec<Note> {
        let mut notes: VecDeque<Note> = self.0.notes().into();
        if let Some(first) = notes.pop_front().and_then(|note| note.octave_up()) {
            notes.push_back(first);
        }
        notes.into()
    }

    #[inline]
    fn root_note(&self) -> Note {
        self.0.root_note()
    }
}

#[cfg(test)]
mod tests {
    use crate::theory::Chord;

    use super::*;

    #[test]
    fn test_notes() {
        assert_eq!(
            Inversion(Chord::Major(Note::C3)).notes(),
            vec![Note::E3, Note::G3, Note::C4]
        );
        assert_eq!(
            Inversion(Inversion(Chord::Major(Note::C3))).notes(),
            vec![Note::G3, Note::C4, Note::E4]
        );
        assert_eq!(
            Inversion(Inversion(Inversion(Chord::Major(Note::C3)))).notes(),
            vec![Note::C4, Note::E4, Note::G4]
        );

        assert_eq!(
            Inversion(Chord::Major7th(Note::C3)).notes(),
            vec![Note::E3, Note::G3, Note::B3, Note::C4]
        );
        assert_eq!(
            Inversion(Inversion(Chord::Major7th(Note::C3))).notes(),
            vec![Note::G3, Note::B3, Note::C4, Note::E4]
        );
        assert_eq!(
            Inversion(Inversion(Inversion(Chord::Major7th(Note::C3)))).notes(),
            vec![Note::B3, Note::C4, Note::E4, Note::G4]
        );
        assert_eq!(
            Inversion(Inversion(Inversion(Inversion(Chord::Major7th(Note::C3))))).notes(),
            vec![Note::C4, Note::E4, Note::G4, Note::B4]
        );

        assert_eq!(
            Inversion(Chord::Major9th(Note::C3)).notes(),
            vec![Note::E3, Note::G3, Note::B3, Note::D4, Note::C4]
        );
        assert_eq!(
            Inversion(Inversion(Chord::Major9th(Note::C3))).notes(),
            vec![Note::G3, Note::B3, Note::D4, Note::C4, Note::E4]
        );
        assert_eq!(
            Inversion(Inversion(Inversion(Chord::Major9th(Note::C3)))).notes(),
            vec![Note::B3, Note::D4, Note::C4, Note::E4, Note::G4]
        );
        assert_eq!(
            Inversion(Inversion(Inversion(Inversion(Chord::Major9th(Note::C3))))).notes(),
            vec![Note::D4, Note::C4, Note::E4, Note::G4, Note::B4]
        );
        assert_eq!(
            Inversion(Inversion(Inversion(Inversion(Inversion(Chord::Major9th(
                Note::C3
            ))))))
            .notes(),
            vec![Note::C4, Note::E4, Note::G4, Note::B4, Note::D5]
        );
    }
}
