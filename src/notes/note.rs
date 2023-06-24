use std::{cmp::Ordering, iter::Zip, ops::RangeFrom};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum Note {
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
}

impl Note {
    #[inline]
    fn midi_iter() -> Zip<NoteIter, RangeFrom<u8>> {
        Self::iter().zip(36..)
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
