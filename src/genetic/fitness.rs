use crate::{midi::bpm::BPM, notes::note_data::NoteData, WithNextIterable};

#[inline]
pub fn fitness(bpm: impl BPM, lead: &Vec<NoteData>, ideal_lead: &Vec<NoteData>) -> f32 {
    let note_match_ratio = 1.0 / lead.len() as f32;
    let note_dif_match_ratio = note_match_ratio / 13.0;

    let bar_time = bpm.get_bar_time().as_millis() as u32;
    let single_note_len = bar_time / 16;

    lead.with_next().zip(ideal_lead.with_next()).fold(
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
    )
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
    let cur_delay = next.get_delay() / single_note_len + 1;
    let delay_dif = cur_delay.abs_diff(ideal_next.get_delay()) < 32;
    let delay_dif = if delay_dif { note_match_ratio } else { 0.0 };

    let cur_dif = next.get_note() - prev.get_note();
    let ideal_dif = ideal_next.get_note() - ideal_prev.get_note();

    let dif = (cur_dif - ideal_dif).abs() as f32;
    let dif_match = note_match_ratio - dif * note_dif_match_ratio * 2.0;
    let dif_match = dif_match;

    delay_dif + dif_match
}
