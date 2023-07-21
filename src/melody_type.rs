use crate::{
    midi::{
        bpm::BPM,
        generator::generator::{
            generate_lead_melody_with_bpm_and_len, generate_synthwave_melody_length,
        },
    },
    notes::{
        note::Note,
        note_data::{DeltaTime, NoteData},
    },
};

use rust_music_theory::note::PitchClass;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum SynthwaveMelodyType {
    ABAB,
    AAAB,
    ABAC,
}

impl SynthwaveMelodyType {
    #[inline]
    pub fn generate_synthwave_melody(
        &self,
        key: PitchClass,
        scale_notes: &Vec<Note>,
        bpm: impl BPM,
    ) -> Vec<NoteData> {
        let melody_length = generate_synthwave_melody_length();
        println!("MELODY LEN: {melody_length}");

        match self {
            SynthwaveMelodyType::ABAB => generate_abab_melody(key, scale_notes, bpm, melody_length),
            SynthwaveMelodyType::AAAB => generate_aaab_melody(key, scale_notes, bpm, melody_length),
            SynthwaveMelodyType::ABAC => generate_abac_melody(key, scale_notes, bpm, melody_length),
        }
    }
}

#[inline]
fn generate_abab_melody(
    key: PitchClass,
    scale_notes: &Vec<Note>,
    bpm: impl BPM,
    melody_length: usize,
) -> Vec<NoteData> {
    let a_melody = generate_lead_melody_with_bpm_and_len(key, scale_notes, bpm, melody_length);
    let b_melody = generate_lead_melody_with_bpm_and_len(key, scale_notes, bpm, melody_length);

    println!("A MELODY {:?}", a_melody);
    println!("B MELODY {:?}", b_melody);

    let a_delay = time_before_bar_end(*a_melody.last().unwrap(), bpm);
    let b_delay = time_before_bar_end(*b_melody.last().unwrap(), bpm);

    let mut second_part = b_melody.clone();
    let first_note = second_part[0];
    second_part[0] = first_note.clone_with_new_delay(a_delay);

    let mut third_part = a_melody.clone();
    let first_note = third_part[0];
    third_part[0] = first_note.clone_with_new_delay(b_delay);

    let fourth_part = second_part.clone();

    vec![a_melody, second_part, third_part, fourth_part]
        .into_iter()
        .flatten()
        .collect()
}

#[inline]
fn generate_aaab_melody(
    key: PitchClass,
    scale_notes: &Vec<Note>,
    bpm: impl BPM,
    melody_length: usize,
) -> Vec<NoteData> {
    let melody_length = generate_synthwave_melody_length();
    let a_melody = generate_lead_melody_with_bpm_and_len(key, scale_notes, bpm, melody_length);
    let b_melody = generate_lead_melody_with_bpm_and_len(key, scale_notes, bpm, melody_length);
    let a_delay = time_before_bar_end(*a_melody.last().unwrap(), bpm);

    let mut second_part = a_melody.clone();
    let first_note = second_part[0];
    second_part[0] = first_note.clone_with_new_delay(a_delay);

    let third_part = second_part.clone();

    let mut fourth_part = b_melody;
    let first_note = fourth_part[0];
    fourth_part[0] = first_note.clone_with_new_delay(a_delay);

    vec![a_melody, second_part, third_part, fourth_part]
        .into_iter()
        .flatten()
        .collect()
}

#[inline]
fn generate_abac_melody(
    key: PitchClass,
    scale_notes: &Vec<Note>,
    bpm: impl BPM,
    melody_length: usize,
) -> Vec<NoteData> {
    let melody_length = generate_synthwave_melody_length();
    let a_melody = generate_lead_melody_with_bpm_and_len(key, scale_notes, bpm, melody_length);
    let b_melody = generate_lead_melody_with_bpm_and_len(key, scale_notes, bpm, melody_length);
    let c_melody = generate_lead_melody_with_bpm_and_len(key, scale_notes, bpm, melody_length);

    let a_delay = time_before_bar_end(*a_melody.last().unwrap(), bpm);
    let b_delay = time_before_bar_end(*b_melody.last().unwrap(), bpm);

    let mut second_part = b_melody.clone();
    let first_note = second_part[0];
    second_part[0] = first_note.clone_with_new_delay(a_delay);

    let mut third_part = a_melody.clone();
    let first_note = third_part[0];
    third_part[0] = first_note.clone_with_new_delay(b_delay);

    let mut fourth_part = c_melody;
    let first_note = fourth_part[0];
    fourth_part[0] = first_note.clone_with_new_delay(a_delay);

    vec![a_melody, second_part, third_part, fourth_part]
        .into_iter()
        .flatten()
        .collect()
}

#[inline]
fn time_before_bar_end(last_note: NoteData, bpm: impl BPM) -> DeltaTime {
    let bar_time = bpm.bar_time().as_millis() as DeltaTime;
    bar_time - last_note.start() - last_note.length()
}
