use super::{Interval, Note};

pub trait ChordLike {
    fn intervals(&self) -> Vec<Interval>;

    fn notes(&self) -> Vec<Note>;

    fn root_note(&self) -> Note;
}
