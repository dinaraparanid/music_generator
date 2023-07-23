use crate::{
    genetic::{fitness::*, mutation::mutate},
    midi::generator::generator::{
        generate_lead_melody_with_bpm_and_len, generate_synthwave_melody_length,
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
    ) -> Vec<NoteData> {
        match self.try_generate_synthwave_melody(key, scale_notes) {
            None => self.generate_synthwave_melody(key, scale_notes),
            Some(lead) => lead,
        }
    }

    #[inline]
    fn try_generate_synthwave_melody(
        &self,
        key: PitchClass,
        scale_notes: &Vec<Note>,
    ) -> Option<Vec<NoteData>> {
        let melody_length = generate_synthwave_melody_length();

        let lead = match self {
            SynthwaveMelodyType::ABAB => generate_abab_melody(key, scale_notes, melody_length),
            SynthwaveMelodyType::AAAB => generate_aaab_melody(key, scale_notes, melody_length),
            SynthwaveMelodyType::ABAC => generate_abac_melody(key, scale_notes, melody_length),
        };

        if is_without_three_times_repetition(&lead)
            && is_distance_between_notes_not_big(&lead)
            && is_not_too_big_parts(&lead)
        {
            Some(lead)
        } else {
            None
        }
    }
}

#[inline]
fn generate_abab_melody(
    key: PitchClass,
    scale_notes: &Vec<Note>,
    melody_length: usize,
) -> Vec<NoteData> {
    let a_melody = generate_lead_melody_with_bpm_and_len(key, scale_notes, melody_length);
    let b_melody = mutate(a_melody.clone(), scale_notes, 0.75);

    let a_delay = time_before_bar_end(*a_melody.last().unwrap());
    let b_delay = time_before_bar_end(*b_melody.last().unwrap());

    let mut second_part = b_melody.clone();
    let first_note = second_part[0];
    second_part[0] = first_note.clone_with_new_delay(a_delay);

    let mut third_part = a_melody.clone();
    let first_note = third_part[0];
    third_part[0] = first_note.clone_with_new_delay(b_delay);

    let fourth_part = second_part.clone();

    let bar_4 = vec![a_melody, second_part, third_part, fourth_part]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    let mut bar_8 = bar_4.clone();
    let first_note = bar_8[0];
    bar_8[0] = first_note.clone_with_new_delay(b_delay);

    vec![bar_4, bar_8].into_iter().flatten().collect()
}

#[inline]
fn generate_aaab_melody(
    key: PitchClass,
    scale_notes: &Vec<Note>,
    melody_length: usize,
) -> Vec<NoteData> {
    let a_melody = generate_lead_melody_with_bpm_and_len(key, scale_notes, melody_length);
    let b_melody = generate_lead_melody_with_bpm_and_len(key, scale_notes, melody_length);

    let a_delay = time_before_bar_end(*a_melody.last().unwrap());
    let b_delay = time_before_bar_end(*b_melody.last().unwrap());

    let mut second_part = a_melody.clone();
    let first_note = second_part[0];
    second_part[0] = first_note.clone_with_new_delay(a_delay);

    let third_part = second_part.clone();

    let mut fourth_part = b_melody;
    let first_note = fourth_part[0];
    fourth_part[0] = first_note.clone_with_new_delay(a_delay);

    let bar_4 = vec![a_melody, second_part, third_part, fourth_part]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    let mut bar_8 = bar_4.clone();
    let first_note = bar_8[0];
    bar_8[0] = first_note.clone_with_new_delay(b_delay);

    vec![bar_4, bar_8].into_iter().flatten().collect()
}

#[inline]
fn generate_abac_melody(
    key: PitchClass,
    scale_notes: &Vec<Note>,
    melody_length: usize,
) -> Vec<NoteData> {
    let a_melody = generate_lead_melody_with_bpm_and_len(key, scale_notes, melody_length);
    let b_melody = generate_lead_melody_with_bpm_and_len(key, scale_notes, melody_length);
    let c_melody = mutate(a_melody.clone(), scale_notes, 0.75);

    let a_delay = time_before_bar_end(*a_melody.last().unwrap());
    let b_delay = time_before_bar_end(*b_melody.last().unwrap());
    let c_delay = time_before_bar_end(*c_melody.last().unwrap());

    let mut second_part = b_melody.clone();
    let first_note = second_part[0];
    second_part[0] = first_note.clone_with_new_delay(a_delay);

    let mut third_part = a_melody.clone();
    let first_note = third_part[0];
    third_part[0] = first_note.clone_with_new_delay(b_delay);

    let mut fourth_part = c_melody;
    let first_note = fourth_part[0];
    fourth_part[0] = first_note.clone_with_new_delay(a_delay);

    let bar_4 = vec![a_melody, second_part, third_part, fourth_part]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    let mut bar_8 = bar_4.clone();
    let first_note = bar_8[0];
    bar_8[0] = first_note.clone_with_new_delay(c_delay);

    vec![bar_4, bar_8].into_iter().flatten().collect()
}

#[inline]
fn time_before_bar_end(last_note: NoteData) -> DeltaTime {
    512 - last_note.start() - last_note.length()
}
