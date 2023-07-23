use crate::{
    midi::generator::generator::randomize_note,
    notes::{note::Note, note_data::NoteData},
};

use rand::Rng;

#[inline]
pub fn mutate(lead: Vec<NoteData>, scale_notes: &Vec<Note>, mutation_rate: f32) -> Vec<NoteData> {
    let mut rng = rand::thread_rng();

    lead.into_iter()
        .enumerate()
        .map(|(ind, note)| {
            if ind == 0 {
                return note;
            }

            if rng.gen_bool(mutation_rate as f64) {
                randomize_note(note, scale_notes)
            } else {
                note
            }
        })
        .collect()
}
