use crate::note::Note;

pub struct DownArpeggiator {
    notes: Vec<Note>,
    pos: usize,
}

#[allow(dead_code)]
impl DownArpeggiator {
    pub fn new(notes: Vec<Note>) -> Self {
        Self { notes, pos: 0 }
    }

    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.pos = 0;
    }

    pub fn has_next(&self) -> bool {
        !self.notes.is_empty()
    }
}

impl Iterator for DownArpeggiator {
    type Item = Note;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.has_next() {
            return None;
        }

        let note = self
            .notes
            .get(self.notes.len() - self.pos - 1)
            .expect("should be got note")
            .clone();
        self.pos = (self.pos + 1) % self.notes.len();

        Some(note)
    }
}

#[cfg(test)]
mod tests {
    use crate::chord::{Chord, ChordLike};

    use super::*;

    #[test]
    fn test_next_empty() {
        let mut arp = DownArpeggiator::new(vec![]);
        assert_eq!(arp.next(), None);
        assert_eq!(arp.next(), None);
    }

    #[test]
    fn test_next_single() {
        let mut arp = DownArpeggiator::new(vec![Note::C3]);
        assert_eq!(arp.next(), Some(Note::C3));
        assert_eq!(arp.next(), Some(Note::C3));
    }

    #[test]
    fn test_next_double() {
        let mut arp = DownArpeggiator::new(vec![Note::C3, Note::G3]);
        assert_eq!(arp.next(), Some(Note::G3));
        assert_eq!(arp.next(), Some(Note::C3));
        assert_eq!(arp.next(), Some(Note::G3));
        assert_eq!(arp.next(), Some(Note::C3));
    }

    #[test]
    fn test_next_7th() {
        let mut arp = DownArpeggiator::new(Chord::Major7th(Note::C3).notes());
        assert_eq!(arp.next(), Some(Note::B3));
        assert_eq!(arp.next(), Some(Note::G3));
        assert_eq!(arp.next(), Some(Note::E3));
        assert_eq!(arp.next(), Some(Note::C3));
        assert_eq!(arp.next(), Some(Note::B3));
        assert_eq!(arp.next(), Some(Note::G3));
        assert_eq!(arp.next(), Some(Note::E3));
        assert_eq!(arp.next(), Some(Note::C3));
    }

    #[test]
    fn test_next_9th() {
        let mut arp = DownArpeggiator::new(Chord::Major9th(Note::C3).notes());
        assert_eq!(arp.next(), Some(Note::D4));
        assert_eq!(arp.next(), Some(Note::B3));
        assert_eq!(arp.next(), Some(Note::G3));
        assert_eq!(arp.next(), Some(Note::E3));
        assert_eq!(arp.next(), Some(Note::C3));
        assert_eq!(arp.next(), Some(Note::D4));
        assert_eq!(arp.next(), Some(Note::B3));
        assert_eq!(arp.next(), Some(Note::G3));
        assert_eq!(arp.next(), Some(Note::E3));
        assert_eq!(arp.next(), Some(Note::C3));
    }
}
