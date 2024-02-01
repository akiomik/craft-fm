use crate::theory::Note;

pub struct UpDownArpeggiator {
    notes: Vec<Note>,
    steps: usize,
    pos: usize,
}

impl UpDownArpeggiator {
    pub fn new(notes: Vec<Note>) -> Self {
        let steps = match notes.len() {
            0 => 0,
            1 => 1,
            _ => (notes.len() * 2) - 2,
        };

        Self {
            notes,
            steps,
            pos: 0,
        }
    }

    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.pos = 0;
    }

    pub fn has_next(&self) -> bool {
        self.steps != 0
    }

    fn next_index(&self) -> Option<usize> {
        if !self.has_next() {
            return None;
        }

        let index = if self.pos < self.notes.len() {
            self.pos
        } else {
            self.steps - self.pos
        };

        Some(index)
    }
}

impl Iterator for UpDownArpeggiator {
    type Item = Note;
    fn next(&mut self) -> Option<Self::Item> {
        self.next_index().map(|index| {
            let note = self.notes.get(index).expect("should be got note").clone();
            self.pos = (self.pos + 1) % self.steps;
            note
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::theory::*;

    use super::*;

    #[test]
    fn test_next_empty() {
        let mut arp = UpDownArpeggiator::new(vec![]);
        assert_eq!(arp.next(), None);
        assert_eq!(arp.next(), None);
    }

    #[test]
    fn test_next_single() {
        let mut arp = UpDownArpeggiator::new(vec![Note::C3]);
        assert_eq!(arp.next(), Some(Note::C3));
        assert_eq!(arp.next(), Some(Note::C3));
    }

    #[test]
    fn test_next_double() {
        let mut arp = UpDownArpeggiator::new(vec![Note::C3, Note::G3]);
        assert_eq!(arp.next(), Some(Note::C3));
        assert_eq!(arp.next(), Some(Note::G3));
        assert_eq!(arp.next(), Some(Note::C3));
        assert_eq!(arp.next(), Some(Note::G3));
    }

    #[test]
    fn test_next_7th() {
        let mut arp = UpDownArpeggiator::new(Chord::Major7th(Note::C3).notes());
        assert_eq!(arp.next(), Some(Note::C3));
        assert_eq!(arp.next(), Some(Note::E3));
        assert_eq!(arp.next(), Some(Note::G3));
        assert_eq!(arp.next(), Some(Note::B3));
        assert_eq!(arp.next(), Some(Note::G3));
        assert_eq!(arp.next(), Some(Note::E3));
        assert_eq!(arp.next(), Some(Note::C3));
        assert_eq!(arp.next(), Some(Note::E3));
        assert_eq!(arp.next(), Some(Note::G3));
    }

    #[test]
    fn test_next_9th() {
        let mut arp = UpDownArpeggiator::new(Chord::Major9th(Note::C3).notes());
        assert_eq!(arp.next(), Some(Note::C3));
        assert_eq!(arp.next(), Some(Note::E3));
        assert_eq!(arp.next(), Some(Note::G3));
        assert_eq!(arp.next(), Some(Note::B3));
        assert_eq!(arp.next(), Some(Note::D4));
        assert_eq!(arp.next(), Some(Note::B3));
        assert_eq!(arp.next(), Some(Note::G3));
        assert_eq!(arp.next(), Some(Note::E3));
        assert_eq!(arp.next(), Some(Note::C3));
        assert_eq!(arp.next(), Some(Note::E3));
    }
}
