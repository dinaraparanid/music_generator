use crate::{midi::bpm::BPM, notes::note_data::NoteData};

#[inline]
pub fn fitness(bpm: impl BPM, lead: &Vec<NoteData>, ideal_lead: &Vec<NoteData>) -> f32 {
    let note_match_ratio = 0.75 / lead.len() as f32;
    let note_dif_match_ratio = note_match_ratio / 13.0;

    let bar_time = bpm.get_bar_time().as_millis() as u32;
    let single_note_len = bar_time / 16;

    let fit = lead
        .iter()
        .skip(1)
        .zip(lead.iter())
        .zip(ideal_lead.iter().skip(1).zip(ideal_lead.iter()))
        .fold(
            note_match_ratio,
            |fit_val, ((next, prev), (ideal_next, ideal_prev))| {
                let cur_delay = next.get_delay() / single_note_len + 1;
                let delay_dif = cur_delay.abs_diff(ideal_next.get_delay()) < 32;
                let delay_dif = if delay_dif { note_match_ratio } else { 0.0 };
                //println!("{}", delay_dif);

                let cur_dif = next.get_note() - prev.get_note();
                let ideal_dif = ideal_next.get_note() - ideal_prev.get_note();

                let dif = (cur_dif - ideal_dif).abs() as f32;
                let dif_match = note_match_ratio - dif * note_dif_match_ratio;
                let dif_match = dif_match / 2.0;

                fit_val + delay_dif + dif_match
            },
        );

    //println!("{}", fit);
    fit
}
