use crate::{
    midi::generator::generator::randomize_note,
    notes::{note::Note, note_data::NoteData},
};

use rand::Rng;

#[inline]
pub fn mutate(lead: Vec<NoteData>, scale_notes: &Vec<Note>, mutation_rate: f32) -> Vec<NoteData> {
    let mut rng = rand::thread_rng();

    lead.into_iter()
        .map(|note| {
            if rng.gen_range(0.0..1.0) <= mutation_rate {
                randomize_note(note, scale_notes)
            } else {
                note
            }
        })
        .collect()
}
