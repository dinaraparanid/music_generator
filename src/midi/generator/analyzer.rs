use crate::notes::{note::Note, note_data::*};
use std::{collections::HashMap, hash::Hash};

pub type RepeatTimes = u32;
type AnalyzedData<T> = HashMap<T, HashMap<T, RepeatTimes>>;
pub type AnalyzedNotes = AnalyzedData<Note>;
pub type AnalyzedDelays = AnalyzedData<DeltaTime>;

#[inline]
fn analyze_data<T, G>(midi_data: &Vec<Vec<NoteData>>, data_getter: G) -> AnalyzedData<T>
where
    T: Hash + Eq + Copy,
    G: Fn(&NoteData) -> T,
{
    midi_data
        .iter()
        .map(|notes| {
            let mut data_map = HashMap::new();

            notes
                .iter()
                .zip(notes.iter().skip(1))
                .for_each(|(first_note, second_note)| {
                    *data_map
                        .entry(data_getter(first_note))
                        .or_insert(HashMap::new())
                        .entry(data_getter(second_note))
                        .or_insert(0) += 1;
                });

            data_map
        })
        .fold(HashMap::new(), |mut acc, x| {
            x.into_iter().for_each(|(first_note_data, prob_map)| {
                prob_map
                    .into_iter()
                    .for_each(|(next_note_data, repeat_times)| {
                        *acc.entry(first_note_data)
                            .or_insert(HashMap::new())
                            .entry(next_note_data)
                            .or_insert(0) += repeat_times
                    })
            });
            acc
        })
}

#[inline]
pub fn analyze_notes(midi_data: &Vec<Vec<NoteData>>) -> AnalyzedNotes {
    analyze_data(midi_data, NoteData::get_note)
}

#[inline]
pub fn analyze_delays(midi_data: &Vec<Vec<NoteData>>) -> AnalyzedDelays {
    analyze_data(midi_data, NoteData::get_delay)
}
