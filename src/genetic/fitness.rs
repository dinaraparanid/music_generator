use crate::{midi::bpm::BPM, notes::note_data::NoteData, WithNextIterable};
use itertools::Itertools;

/// Calculates fitness function of the given lead
/// by comparing it with the ideal lead.
///
/// Algorithm considers distances (pauses) between notes
/// and gives 3/4 for the complete match only.
/// Pitch coincidence is also taken into account and receives 1/4.
/// Partial pitch coincidence is also considered.
///
/// Additionally, algorithm uses next filters:
/// 1. Same note cannot repeat 3 times in a row
/// 2. Distance between notes in semitones is less than 7
/// 3. Parts with 7+ notes with zero delay are not allowed
/// 4. Number of pauses is less than 4
///
/// Resulting fitness is in range 0..=1

#[inline]
pub fn fitness(bpm: impl BPM, lead: &Vec<NoteData>, ideal_lead: &Vec<NoteData>) -> f32 {
    let note_match_ratio = 1.0 / lead.len() as f32;
    let note_dif_match_ratio = note_match_ratio / 26.0;

    let bar_time = bpm.bar_time().as_millis() as u32;
    let single_note_len = (bar_time as f64 / 16.0).round() as u32;

    let fitness = lead.with_next().zip(ideal_lead.with_next()).fold(
        note_match_ratio,
        |fit_val, ((next, prev), (ideal_next, ideal_prev))| {
            fit_val
                + calc_fitness_for_next_note(
                    next,
                    prev,
                    ideal_next,
                    ideal_prev,
                    note_match_ratio,
                    note_dif_match_ratio,
                    single_note_len,
                )
        },
    );

    if is_without_three_times_repetition(&lead)
        && is_distance_between_notes_not_big(&lead)
        && is_not_too_big_parts(&lead)
        && is_not_many_delays(&lead)
    {
        fitness
    } else {
        0.0
    }
}

/// Same note cannot repeat 3 times in a row

#[inline]
pub fn is_without_three_times_repetition(lead: &Vec<NoteData>) -> bool {
    lead.windows(3)
        .map(|arr| {
            arr.iter()
                .map(|&x| x)
                .collect_tuple::<(NoteData, NoteData, NoteData)>()
                .unwrap()
        })
        .find(|(f, s, t)| f.note() == s.note() && s.note() == t.note())
        .is_none()
}

/// Distance between notes in semitones is less than 7

#[inline]
pub fn is_distance_between_notes_not_big(lead: &Vec<NoteData>) -> bool {
    lead.with_next()
        .all(|(next, prev)| next.note().midi().abs_diff(prev.note().midi()) < 7)
}

/// Parts with 7+ notes with zero delay are not allowed

#[inline]
pub fn is_not_too_big_parts(lead: &Vec<NoteData>) -> bool {
    lead.iter()
        .skip(1)
        .map(|x| x.delay())
        .collect::<Vec<_>>()
        .windows(6)
        .find(|&arr| arr == [0, 0, 0, 0, 0, 0])
        .is_none()
}

/// Notes with start and the delay both equal to 1/16 of bar are not allowed.
/// | |N|?|?| - such 1/4 of bars are not allowed

#[inline]
pub fn is_not_bad_rhythm_with_odd_notes(lead: &Vec<NoteData>) -> bool {
    lead.iter()
        .map(|x| (x.start() / 32 % 2, x.delay() / 32 % 2))
        .find(|(start, delay)| *start == 1 && *delay == 1)
        .is_none()
}

/// At least two notes go one after another without any pause

#[inline]
pub fn is_at_least_one_zero_delay(lead: &Vec<NoteData>) -> bool {
    lead.iter().map(|x| x.delay()).find(|x| *x == 0).is_some()
}

/// Number of pauses is less than 4

#[inline]
pub fn is_not_many_delays(lead: &Vec<NoteData>) -> bool {
    lead.iter().map(|x| x.delay()).filter(|d| *d != 0).count() < 4
}

/// Algorithm considers distances (pauses) between notes
/// and gives 3/4 for the complete match only.
/// Pitch coincidence is also taken into account and receives 1/4.
/// Partial pitch coincidence is also considered

#[inline]
fn calc_fitness_for_next_note(
    next: &NoteData,
    prev: &NoteData,
    ideal_next: &NoteData,
    ideal_prev: &NoteData,
    note_match_ratio: f32,
    note_dif_match_ratio: f32,
    single_note_len: u32,
) -> f32 {
    let cur_delay = next.delay() / single_note_len;
    let next_delay = ideal_next.delay() / 32;
    let delay_dif_match = if cur_delay == next_delay {
        note_match_ratio * 3.0 / 4.0
    } else {
        0.0
    };

    let cur_pitch_dif = next.note() - prev.note();
    let ideal_pitch_dif = ideal_next.note() - ideal_prev.note();
    let pitch_dif = (cur_pitch_dif - ideal_pitch_dif).abs() as f32;
    let pitch_dif_match = note_match_ratio - pitch_dif * note_dif_match_ratio;
    let pitch_dif_match = pitch_dif_match / 4.0;

    delay_dif_match + pitch_dif_match
}
