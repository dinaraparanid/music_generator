use crate::notes::note_data::NoteData;
use std::collections::HashMap;

#[inline]
pub fn analyze_midi(midi_data: Vec<Vec<NoteData>>) -> HashMap<NoteData, HashMap<NoteData, u32>> {
    midi_data
        .into_iter()
        .map(|notes| {
            let mut note_map = HashMap::new();

            notes
                .iter()
                .zip(notes.iter().skip(1))
                .for_each(|(first_note, second_note)| {
                    *note_map
                        .entry(first_note.clone())
                        .or_insert(HashMap::new())
                        .entry(second_note.clone())
                        .or_insert(0) += 1;
                });

            note_map
        })
        .fold(HashMap::new(), |mut acc, x| {
            x.into_iter().for_each(|(first_note, prob_map)| {
                prob_map.into_iter().for_each(|(next_note, repeat_times)| {
                    *acc.entry(first_note.clone())
                        .or_insert(HashMap::new())
                        .entry(next_note)
                        .or_insert(0) += repeat_times
                })
            });
            acc
        })
}
