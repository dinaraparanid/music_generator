use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Copy, Clone, Eq, PartialEq, Debug)]
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
    pub fn midi(&self) -> u8 {
        Self::iter()
            .zip(36..)
            .find(|&(note, _)| note == *self)
            .unwrap()
            .1
    }
}

impl From<u8> for Note {
    #[inline]
    fn from(value: u8) -> Self {
        Self::iter()
            .zip(36..)
            .find(|&(_, midi)| midi == value)
            .expect("Illegal MIDI value when converting from u8 to Note")
            .0
    }
}
