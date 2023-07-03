use crate::notes::{note::Note, note_data::*};
use std::{collections::HashMap, hash::Hash};

/// The amount of times event was repeated

pub type RepeatTimes = u32;

/// Markov chains based map,
/// where key is the first event,
/// value is the map of next event,
/// followed after the first one,
/// and the amount of times it was repeated
///
/// # Deprecated
/// Idea of note parsing and analysis was
/// abandoned in favour of pure generation.
/// Yet, it may be useful in the future

#[deprecated]
type AnalyzedData<T> = HashMap<T, HashMap<T, RepeatTimes>>;

/// Markov chains based probabilities map,
/// where key is the first note,
/// value is the map of next note,
/// followed after the first one,
/// and the amount of times it was repeated
///
/// # Deprecated
/// Idea of note parsing and analysis was
/// abandoned in favour of pure generation.
/// Yet, it may be useful in the future

#[deprecated]
pub type AnalyzedNotes = AnalyzedData<Note>;

/// Markov chains based probabilities map,
/// where key is the first note's delay,
/// value is the map of next note's delay,
/// followed after the first one,
/// and the amount of times it was repeated
///
/// # Deprecated
/// Idea of note parsing and analysis was
/// abandoned in favour of pure generation.
/// Yet, it may be useful in the future

#[deprecated]
pub type AnalyzedDelays = AnalyzedData<DeltaTime>;

/// Constructs Markov chains based probabilities map
/// from the given the given dataset of MIDI notes
///
/// # Deprecated
/// Idea of note parsing and analysis was
/// abandoned in favour of pure generation.
/// Yet, it may be useful in the future

#[inline]
#[deprecated]
fn analyze_data<T, G>(midi_data: &Vec<Vec<NoteData>>, data_getter: G) -> AnalyzedData<T>
where
    T: Hash + Eq + Copy,
    G: Fn(&NoteData) -> T,
{
    midi_data
        .iter()
        .map(|notes| {
            // Parsing data from the single MIDI file
            let mut data_map = HashMap::new();

            notes
                .iter()
                .zip(notes.iter().skip(1)) // combining previous and next notes
                .for_each(|(first_note, second_note)| {
                    // Increasing the amount of appearances
                    *data_map
                        .entry(data_getter(first_note))
                        .or_insert(HashMap::new())
                        .entry(data_getter(second_note))
                        .or_insert(0) += 1;
                });

            data_map
        })
        .fold(HashMap::new(), |mut acc, x| {
            // Combining data from all files into a single map

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

/// Constructs Markov chains based probabilities map
/// from the given the given dataset of MIDI notes.
/// Calculates probabilities of next notes to appear
///
/// # Deprecated
/// Idea of note parsing and analysis was
/// abandoned in favour of pure generation.
/// Yet, it may be useful in the future

#[inline]
#[deprecated]
pub fn analyze_notes(midi_data: &Vec<Vec<NoteData>>) -> AnalyzedNotes {
    analyze_data(midi_data, NoteData::get_note)
}

/// Constructs Markov chains based probabilities map
/// from the given the given dataset of MIDI notes.
/// Calculates probabilities of next delays to appear
///
/// # Deprecated
/// Idea of note parsing and analysis was
/// abandoned in favour of pure generation.
/// Yet, it may be useful in the future

#[inline]
#[deprecated]
pub fn analyze_delays(midi_data: &Vec<Vec<NoteData>>) -> AnalyzedDelays {
    analyze_data(midi_data, NoteData::get_delay)
}
