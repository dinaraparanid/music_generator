use crate::notes::{note::Note, note_data::NoteData};
use rust_music_theory::note::PitchClass;

impl From<Note> for PitchClass {
    #[inline]
    fn from(value: Note) -> Self {
        Note::midi_iter()
            .map(|(note, midi)| (note, PitchClass::from_u8(midi % 12)))
            .find(|(note, _)| *note == value)
            .unwrap()
            .1
    }
}

impl From<NoteData> for PitchClass {
    #[inline]
    fn from(value: NoteData) -> Self {
        Self::from(value.get_note())
    }
}
