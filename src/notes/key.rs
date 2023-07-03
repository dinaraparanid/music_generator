use crate::notes::{note::Note, note_data::NoteData};
use rust_music_theory::note::PitchClass;

impl From<Note> for PitchClass {
    /// Constructs pitch from the [Note]
    ///
    /// # Example
    /// ```
    /// use rust_music_theory::note::PitchClass;
    /// use music_generator::notes::note::Note;
    ///
    /// let note = Note::A4;
    /// assert_eq!(PitchClass::from(note), PitchClass::A)
    /// ```

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
    /// Constructs pitch from the [NoteData]
    ///
    /// # Example
    /// ```
    /// use rust_music_theory::note::PitchClass;
    /// use music_generator::notes::{note::Note, note_data::NoteData};
    ///
    /// let note = NoteData::new(Note::A4, 100, 0, 100, 0);
    /// assert_eq!(PitchClass::from(note), PitchClass::A)
    /// ```

    #[inline]
    fn from(value: NoteData) -> Self {
        Self::from(value.get_note())
    }
}
