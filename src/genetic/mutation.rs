use crate::{
    midi::generator::generator::randomize_note,
    notes::{note::Note, note_data::NoteData},
};

use rand::Rng;

/// Performs the lead mutation by randomizing notes' pitches.
/// Every note may be randomized with probability equal to mutation rate.
/// Generated notes' frequencies are belong to the given scale

#[inline]
pub fn mutate(lead: Vec<NoteData>, scale_notes: &Vec<Note>, mutation_rate: f32) -> Vec<NoteData> {
    let mut rng = rand::thread_rng();

    lead.into_iter()
        .map(|note| {
            if rng.gen_bool(mutation_rate as f64) {
                randomize_note(note, scale_notes)
            } else {
                note
            }
        })
        .collect()
}
