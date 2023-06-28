use rust_music_theory::note::{Note as MTNote, PitchClass};
use std::{cmp::Ordering, iter::Zip, ops::RangeFrom};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

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
    #[inline]
    pub fn midi_iter() -> Zip<NoteIter, RangeFrom<u8>> {
        Self::iter().zip(21..)
    }

    #[inline]
    pub fn midi(&self) -> u8 {
        Self::midi_iter()
            .find(|&(note, _)| note == *self)
            .unwrap()
            .1
    }

    #[inline]
    pub fn from_byte_or_none(semitones: u8) -> Option<Self> {
        Self::midi_iter()
            .find(|&(_, midi)| midi == semitones)
            .map(|(note, _)| note)
    }

    #[inline]
    pub fn up(&self, semitones: u8) -> Option<Self> {
        Self::from_byte_or_none(self.midi() + semitones)
    }

    #[inline]
    pub unsafe fn up_unchecked(&self, semitones: u8) -> Self {
        self.up(semitones).unwrap_unchecked()
    }

    #[inline]
    pub fn down(&self, semitones: u8) -> Option<Self> {
        Self::from_byte_or_none(self.midi() - semitones)
    }

    #[inline]
    pub unsafe fn down_unchecked(&self, semitones: u8) -> Self {
        self.down(semitones).unwrap_unchecked()
    }

    #[inline]
    pub fn octave_up(&self) -> Option<Self> {
        self.up(12)
    }

    #[inline]
    pub unsafe fn octave_up_unchecked(&self) -> Self {
        self.octave_up().unwrap_unchecked()
    }

    #[inline]
    pub fn octave_down(&self) -> Option<Self> {
        self.down(12)
    }

    #[inline]
    pub unsafe fn octave_down_unchecked(&self) -> Self {
        self.octave_down().unwrap_unchecked()
    }
}

impl From<u8> for Note {
    #[inline]
    fn from(value: u8) -> Self {
        Self::from_byte_or_none(value).expect("Illegal MIDI value when converting from u8 to Note")
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
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.midi().partial_cmp(&other.midi())
    }
}

impl Ord for Note {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.midi().cmp(&other.midi())
    }
}
