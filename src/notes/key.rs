use crate::notes::{note::Note, note_data::NoteData};
use rust_music_theory::note::PitchClass;

impl From<Note> for PitchClass {
    #[inline]
    fn from(value: Note) -> Self {
        Note::midi_iter()
            .map(|(note, midi)| match midi % 12 {
                0 => (note, PitchClass::C),
                1 => (note, PitchClass::Cs),
                2 => (note, PitchClass::D),
                3 => (note, PitchClass::Ds),
                4 => (note, PitchClass::E),
                5 => (note, PitchClass::F),
                6 => (note, PitchClass::Fs),
                7 => (note, PitchClass::G),
                8 => (note, PitchClass::Gs),
                9 => (note, PitchClass::A),
                10 => (note, PitchClass::As),
                11 => (note, PitchClass::B),
                _ => unreachable!(),
            })
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
