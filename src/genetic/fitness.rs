use crate::{midi::bpm::BPM, notes::note_data::NoteData, WithNextIterable};
use itertools::Itertools;

#[inline]
pub fn fitness(bpm: impl BPM, lead: &Vec<NoteData>, ideal_lead: &Vec<NoteData>) -> f32 {
    let note_match_ratio = 1.0 / lead.len() as f32;
    let note_dif_match_ratio = note_match_ratio / 26.0;

    let bar_time = bpm.get_bar_time().as_millis() as u32;
    let single_note_len = bar_time / 16;

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

    if is_ok_len(&lead)
        && is_without_three_times_repetition(&lead)
        && is_distance_between_notes_not_big(&lead)
        && is_not_too_big_parts(&lead)
        && is_no_5_big_part(&lead)
        && is_not_many_delays(&lead)
    {
        fitness
    } else {
        0.0
    }
}

#[inline]
fn is_ok_len(lead: &Vec<NoteData>) -> bool {
    lead.len() <= 12
}

#[inline]
fn is_without_three_times_repetition(lead: &Vec<NoteData>) -> bool {
    lead.windows(3)
        .map(|arr| {
            arr.iter()
                .map(|&x| x)
                .collect_tuple::<(NoteData, NoteData, NoteData)>()
                .unwrap()
        })
        .find(|(f, s, t)| f.get_note() == s.get_note() && s.get_note() == t.get_note())
        .is_none()
}

#[inline]
fn is_distance_between_notes_not_big(lead: &Vec<NoteData>) -> bool {
    lead.with_next()
        .all(|(next, prev)| next.get_note().midi().abs_diff(prev.get_note().midi()) < 7)
}

#[inline]
fn is_not_too_big_parts(lead: &Vec<NoteData>) -> bool {
    lead.iter()
        .skip(1)
        .map(|x| x.get_delay())
        .collect::<Vec<_>>()
        .windows(6)
        .find(|&arr| arr == [0, 0, 0, 0, 0, 0])
        .is_none()
}

#[inline]
fn is_not_many_delays(lead: &Vec<NoteData>) -> bool {
    lead.iter()
        .map(|x| x.get_delay())
        .filter(|d| *d != 0)
        .count()
        < 4
}

#[inline]
fn is_no_5_big_part(lead: &Vec<NoteData>) -> bool {
    lead.iter()
        .skip(1)
        .map(|x| x.get_delay())
        .collect::<Vec<_>>()
        .windows(4)
        .find(|&arr| arr == [0, 0, 0, 0])
        .is_none()
}

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
    let cur_delay = next.get_delay() / single_note_len;
    let next_delay = ideal_next.get_delay() / 32;
    let delay_dif_match = if cur_delay == next_delay {
        note_match_ratio * 3.0 / 4.0
    } else {
        0.0
    };

    let cur_pitch_dif = next.get_note() - prev.get_note();
    let ideal_pitch_dif = ideal_next.get_note() - ideal_prev.get_note();
    let pitch_dif = (cur_pitch_dif - ideal_pitch_dif).abs() as f32;
    let pitch_dif_match = note_match_ratio - pitch_dif * note_dif_match_ratio;
    let pitch_dif_match = pitch_dif_match / 4.0;

    delay_dif_match + pitch_dif_match
}
