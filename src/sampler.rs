use std::collections::HashSet;

use crate::note::Note;

pub struct Sampler {
    notes: HashSet<Note>,
}

impl Sampler {
    pub fn new(notes: HashSet<Note>) -> Self {
        Self { notes }
    }

    pub fn calc_playback_at_note(&self, note: Note) -> (Note, f32) {
        if self.notes.contains(&note) {
            (note, 1.0)
        } else {
            // TODO: find closest note from notes
            (Note::A3, note.freq() / Note::A3.freq())
        }
    }
}
