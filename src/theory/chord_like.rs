use super::Note;

pub trait ChordLike {
    fn notes(&self) -> Vec<Note>;

    fn root_note(&self) -> Note;
}
