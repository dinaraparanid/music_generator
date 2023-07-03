use rust_music_theory::note::{Note as MTNote, PitchClass};
use std::{cmp::Ordering, iter::Zip, ops::RangeFrom, ops::Sub};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

/// All notes that adequately may be used in the MIDI file

#[derive(EnumIter, Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum Note {
    A0,
    Bb0,
    B0,
    C1,
    Db1,
    D1,
    Eb1,
    E1,
    F1,
    Gb1,
    G1,
    Ab1,
    A1,
    Bb1,
    B1,
    C2,
    Db2,
    D2,
    Eb2,
    E2,
    F2,
    Gb2,
    G2,
    Ab2,
    A2,
    Bb2,
    B2,
    C3,
    Db3,
    D3,
    Eb3,
    E3,
    F3,
    Gb3,
    G3,
    Ab3,
    A3,
    Bb3,
    B3,
    C4,
    Db4,
    D4,
    Eb4,
    E4,
    F4,
    Gb4,
    G4,
    Ab4,
    A4,
    Bb4,
    B4,
    C5,
    Db5,
    D5,
    Eb5,
    E5,
    F5,
    Gb5,
    G5,
    Ab5,
    A5,
    Bb5,
    B5,
    C6,
    Db6,
    D6,
    Eb6,
    E6,
    F6,
    Gb6,
    G6,
    Ab6,
    A6,
    Bb6,
    B6,
    C7,
    Db7,
    D7,
    Eb7,
    E7,
    F7,
    Gb7,
    G7,
    Ab7,
    A7,
    Bb7,
    B7,
    C8,
    Db8,
    D8,
    Eb8,
    E8,
    F8,
    Gb8,
    G8,
    Ab8,
    A8,
    Bb8,
    B8,
    C9,
    Db9,
    D9,
    Eb9,
    E9,
    F9,
    Gb9,
    G9,
    Ab9,
}

impl Note {
    /// Constructs an iterator of pairs (note, midi value)

    #[inline]
    pub fn midi_iter() -> Zip<NoteIter, RangeFrom<u8>> {
        Self::iter().zip(21..)
    }

    /// Gets midi value of the note
    ///
    /// # Example
    /// ```
    /// use music_generator::notes::note::Note;
    /// assert_eq!(Note::A4.midi(), 69)
    /// ```

    #[inline]
    pub fn midi(&self) -> u8 {
        Self::midi_iter()
            .find(|&(note, _)| note == *self)
            .unwrap()
            .1
    }

    /// Constructs the note from the given midi value.
    /// Value have to be in range (21..=128)
    ///
    /// # Example
    /// ```
    /// use music_generator::notes::note::Note;
    /// assert_eq!(Note::from_midi_or_none(69), Some(Note::A4));
    /// assert_eq!(Note::from_midi_or_none(130), None)
    /// ```

    #[inline]
    pub fn from_midi_or_none(midi_value: u8) -> Option<Self> {
        Self::midi_iter()
            .find(|&(_, midi)| midi == midi_value)
            .map(|(note, _)| note)
    }

    /// Increases note's pitch with the given number of semitones
    ///
    /// # Example
    /// ```
    /// use music_generator::notes::note::Note;
    /// assert_eq!(Note::A4.up(7), Some(Note::E5))
    /// ```

    #[inline]
    pub fn up(&self, semitones: u8) -> Option<Self> {
        Self::from_midi_or_none(self.midi() + semitones)
    }

    /// Increases note's pitch with the given number of semitones
    ///
    /// # Safety
    /// This version does not checks the correctness.
    /// See [Note::up] for the safe version
    ///
    /// # Example
    /// ```
    /// use music_generator::notes::note::Note;
    /// assert_eq!(unsafe { Note::A4.up_unchecked(7) }, Note::E5)
    /// ```

    #[inline]
    pub unsafe fn up_unchecked(&self, semitones: u8) -> Self {
        self.up(semitones).unwrap_unchecked()
    }

    /// Decreases note's pitch with the given number of semitones
    ///
    /// # Example
    /// ```
    /// use music_generator::notes::note::Note;
    /// assert_eq!(Note::A4.down(7), Some(Note::D4))
    /// ```

    #[inline]
    pub fn down(&self, semitones: u8) -> Option<Self> {
        Self::from_midi_or_none(self.midi() - semitones)
    }

    /// Decreases note's pitch with the given number of semitones
    ///
    /// # Safety
    /// This version does not checks the correctness.
    /// See [Note::down] for the safe version
    ///
    /// # Example
    /// ```
    /// use music_generator::notes::note::Note;
    /// assert_eq!(unsafe { Note::A4.down_unchecked(7) }, Note::D4)
    /// ```

    #[inline]
    pub unsafe fn down_unchecked(&self, semitones: u8) -> Self {
        self.down(semitones).unwrap_unchecked()
    }

    /// Increases note's pitch by an octave
    ///
    /// # Example
    /// ```
    /// use music_generator::notes::note::Note;
    /// assert_eq!(Note::A4.octave_up(), Some(Note::A5))
    /// ```

    #[inline]
    pub fn octave_up(&self) -> Option<Self> {
        self.up(12)
    }

    /// Increases note's pitch by an octave
    ///
    /// # Safety
    /// This version does not checks the correctness.
    /// See [Note::octave_up] for the safe version
    ///
    /// # Example
    /// ```
    /// use music_generator::notes::note::Note;
    /// assert_eq!(unsafe { Note::A4.octave_up_unchecked() }, Note::A5)
    /// ```

    #[inline]
    pub unsafe fn octave_up_unchecked(&self) -> Self {
        self.octave_up().unwrap_unchecked()
    }

    /// Lowers note's pitch by an octave
    ///
    /// # Example
    /// ```
    /// use music_generator::notes::note::Note;
    /// assert_eq!(Note::A4.octave_down(), Some(Note::A3))
    /// ```

    #[inline]
    pub fn octave_down(&self) -> Option<Self> {
        self.down(12)
    }

    /// Lowers note's pitch by an octave
    ///
    /// # Safety
    /// This version does not checks the correctness.
    /// See [Note::octave_down] for the safe version
    ///
    /// # Example
    /// ```
    /// use music_generator::notes::note::Note;
    /// assert_eq!(unsafe { Note::A4.octave_down_unchecked() }, Note::A3)
    /// ```

    #[inline]
    pub unsafe fn octave_down_unchecked(&self) -> Self {
        self.octave_down().unwrap_unchecked()
    }
}

impl From<u8> for Note {
    /// Constructs the note from the given midi value.
    /// Value have to be in range (21..=128)

    #[inline]
    fn from(value: u8) -> Self {
        Self::from_midi_or_none(value).expect("Illegal MIDI value when converting from u8 to Note")
    }
}

impl From<MTNote> for Note {
    #[inline]
    fn from(value: MTNote) -> Self {
        Self::from(
            (value.octave + 1) * 12
                + match value.pitch_class {
                    PitchClass::C => 0,
                    PitchClass::Cs => 1,
                    PitchClass::D => 2,
                    PitchClass::Ds => 3,
                    PitchClass::E => 4,
                    PitchClass::F => 5,
                    PitchClass::Fs => 6,
                    PitchClass::G => 7,
                    PitchClass::Gs => 8,
                    PitchClass::A => 9,
                    PitchClass::As => 10,
                    PitchClass::B => 11,
                },
        )
    }
}

impl PartialOrd for Note {
    /// Compares notes by their MIDI value

    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.midi().partial_cmp(&other.midi())
    }
}

impl Ord for Note {
    /// Compares notes by their MIDI value

    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.midi().cmp(&other.midi())
    }
}

impl Sub for Note {
    type Output = i8;

    /// Calculates the difference between two notes in semitones

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        self.midi() as i8 - rhs.midi() as i8
    }
}
