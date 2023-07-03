use crate::notes::note_data::NoteData;

pub mod key;
pub mod note;
pub mod note_data;

/// Data of notes in the chord
pub type ChordData = Vec<NoteData>;
