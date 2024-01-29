use crate::note::Note;

pub struct UpDownArpeggiator {
    notes: Vec<Note>,
    steps: usize,
    pos: usize,
}

impl UpDownArpeggiator {
    pub fn new(notes: Vec<Note>, steps: Option<usize>) -> Self {
        let steps = match steps {
            _ if notes.is_empty() => 0,
            _ if notes.len() == 1 => 1,
            Some(steps) => steps,
            None => (notes.len() * 2) - 2,
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
        !(self.steps == 0 || self.notes.is_empty())
    }

    fn cycle_steps(&self) -> usize {
        match self.notes.len() {
            0 => 0,
            1 => 1,
            len => (len * 2) - 2,
        }
    }

    fn next_index(&self) -> Option<usize> {
        if !self.has_next() {
            return None;
        }

        let cycle_steps = self.cycle_steps();
        let cycle_pos = self.pos % cycle_steps;
        let index = if cycle_pos < self.notes.len() {
            cycle_pos
        } else {
            cycle_steps - cycle_pos
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
    use crate::chord::Chord;

    use super::*;

    #[test]
    fn test_next_empty_none() {
        let mut arp = UpDownArpeggiator::new(vec![], None);
        assert_eq!(arp.next(), None);
        assert_eq!(arp.next(), None);
    }

    #[test]
    fn test_next_empty_some_0() {
        let mut arp = UpDownArpeggiator::new(vec![], Some(0));
        assert_eq!(arp.next(), None);
        assert_eq!(arp.next(), None);
    }

    #[test]
    fn test_next_empty_some_1() {
        let mut arp = UpDownArpeggiator::new(vec![], Some(1));
        assert_eq!(arp.next(), None);
        assert_eq!(arp.next(), None);
    }

    #[test]
    fn test_next_single_none() {
        let mut arp = UpDownArpeggiator::new(vec![Note::C3], None);
        assert_eq!(arp.next(), Some(Note::C3));
        assert_eq!(arp.next(), Some(Note::C3));
    }

    #[test]
    fn test_next_single_some_1() {
        let mut arp = UpDownArpeggiator::new(vec![Note::C3], Some(1));
        assert_eq!(arp.next(), Some(Note::C3));
        assert_eq!(arp.next(), Some(Note::C3));
    }

    #[test]
    fn test_next_single_some_2() {
        let mut arp = UpDownArpeggiator::new(vec![Note::C3], Some(2));
        assert_eq!(arp.next(), Some(Note::C3));
        assert_eq!(arp.next(), Some(Note::C3));
    }

    #[test]
    fn test_next_double_none() {
        let mut arp = UpDownArpeggiator::new(vec![Note::C3, Note::G3], None);
        assert_eq!(arp.next(), Some(Note::C3));
        assert_eq!(arp.next(), Some(Note::G3));
        assert_eq!(arp.next(), Some(Note::C3));
        assert_eq!(arp.next(), Some(Note::G3));
    }

    #[test]
    fn test_next_double_some_1() {
        let mut arp = UpDownArpeggiator::new(vec![Note::C3, Note::G3], Some(1));
        assert_eq!(arp.next(), Some(Note::C3));
        assert_eq!(arp.next(), Some(Note::C3));
    }

    #[test]
    fn test_next_double_some_2() {
        let mut arp = UpDownArpeggiator::new(vec![Note::C3, Note::G3], Some(2));
        assert_eq!(arp.next(), Some(Note::C3));
        assert_eq!(arp.next(), Some(Note::G3));
        assert_eq!(arp.next(), Some(Note::C3));
        assert_eq!(arp.next(), Some(Note::G3));
    }

    #[test]
    fn test_next_some_0() {
        let mut arp = UpDownArpeggiator::new(Chord::Major(Note::C3).notes(), Some(0));
        assert_eq!(arp.next(), None);
        assert_eq!(arp.next(), None);
    }

    #[test]
    fn test_next_some_1() {
        let mut arp = UpDownArpeggiator::new(Chord::Major(Note::C3).notes(), Some(1));
        assert_eq!(arp.next(), Some(Note::C3));
        assert_eq!(arp.next(), Some(Note::C3));
    }

    #[test]
    fn test_next_7th_some_8() {
        let mut arp = UpDownArpeggiator::new(Chord::Major7th(Note::C3).notes(), Some(8));
        assert_eq!(arp.next(), Some(Note::C3));
        assert_eq!(arp.next(), Some(Note::E3));
        assert_eq!(arp.next(), Some(Note::G3));
        assert_eq!(arp.next(), Some(Note::B3));
        assert_eq!(arp.next(), Some(Note::G3));
        assert_eq!(arp.next(), Some(Note::E3));
        assert_eq!(arp.next(), Some(Note::C3));
        assert_eq!(arp.next(), Some(Note::E3));
        // reset here
        assert_eq!(arp.next(), Some(Note::C3));
        assert_eq!(arp.next(), Some(Note::E3));
        assert_eq!(arp.next(), Some(Note::G3));
    }

    #[test]
    fn test_next_9th_some_8() {
        let mut arp = UpDownArpeggiator::new(Chord::Major9th(Note::C3).notes(), Some(8));
        assert_eq!(arp.next(), Some(Note::C3));
        assert_eq!(arp.next(), Some(Note::E3));
        assert_eq!(arp.next(), Some(Note::G3));
        assert_eq!(arp.next(), Some(Note::B3));
        assert_eq!(arp.next(), Some(Note::D4));
        assert_eq!(arp.next(), Some(Note::B3));
        assert_eq!(arp.next(), Some(Note::G3));
        assert_eq!(arp.next(), Some(Note::E3));
        // reset here
        assert_eq!(arp.next(), Some(Note::C3));
        assert_eq!(arp.next(), Some(Note::E3));
    }

    #[test]
    fn test_next_7th_none() {
        let mut arp = UpDownArpeggiator::new(Chord::Major7th(Note::C3).notes(), None);
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
    fn test_next_9th_none() {
        let mut arp = UpDownArpeggiator::new(Chord::Major9th(Note::C3).notes(), None);
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
