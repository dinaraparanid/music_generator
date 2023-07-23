use crate::melody_type::SynthwaveMelodyType;

use rust_music_theory::{
    note::PitchClass,
    scale::{Mode, ScaleType},
};

pub mod bpm;
pub mod generator;
pub mod parser;

#[inline]
pub fn key_list() -> Vec<PitchClass> {
    vec![
        PitchClass::C,
        PitchClass::Cs,
        PitchClass::D,
        PitchClass::Ds,
        PitchClass::E,
        PitchClass::F,
        PitchClass::Fs,
        PitchClass::G,
        PitchClass::Gs,
        PitchClass::A,
        PitchClass::As,
        PitchClass::B,
    ]
}

#[inline]
pub fn scale_list() -> Vec<ScaleType> {
    vec![
        ScaleType::Diatonic,
        ScaleType::MelodicMinor,
        ScaleType::HarmonicMinor,
    ]
}

#[inline]
pub fn mode_list() -> Vec<Mode> {
    vec![
        Mode::HarmonicMinor,
        Mode::MelodicMinor,
        Mode::Aeolian,
        Mode::Dorian,
        Mode::Ionian,
        Mode::Locrian,
        Mode::Lydian,
        Mode::Mixolydian,
        Mode::Phrygian,
    ]
}

#[inline]
pub fn melody_types() -> Vec<SynthwaveMelodyType> {
    vec![
        SynthwaveMelodyType::ABAB,
        SynthwaveMelodyType::AAAB,
        SynthwaveMelodyType::ABAC,
    ]
}
