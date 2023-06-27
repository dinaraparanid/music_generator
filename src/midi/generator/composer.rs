use crate::notes::{note_data::NoteData, ChordData};
use ghakuf::messages::Message;
use rand::{prelude::SliceRandom, Rng};
use std::{collections::HashMap, hash::Hash};

#[inline]
pub fn generate_note(note: NoteData) -> Vec<Message> {
    let mut rng = rand::thread_rng();
    let delay = rng.gen_range(0..500);

    vec![
        note.into_on_midi_event(delay),
        note.into_off_midi_event(delay + note.get_length()),
    ]
}

#[inline]
pub fn create_chord(mut chord: ChordData) -> Vec<Message> {
    let mut result = chord
        .iter()
        .map(|note_data| note_data.into_on_midi_event(0))
        .collect::<Vec<_>>();

    chord.sort_by_key(|nd| nd.get_length());

    result.extend(
        chord
            .iter()
            .map(|nd| nd.get_length())
            .scan((0, 0), |(time_offset, prev_note_end), cur_note_end| {
                *time_offset = cur_note_end - *prev_note_end;
                *prev_note_end = cur_note_end;
                Some((*time_offset, *prev_note_end))
            })
            .map(|(time_offset, _)| time_offset)
            .zip(chord.iter())
            .map(|(end, nd)| nd.into_off_midi_event(end)),
    );

    result
}

#[inline]
pub fn generate_from_midi_analyze<T, G>(
    midi_data: HashMap<T, HashMap<T, u32>>,
    generator: G,
) -> Option<Vec<Message>>
where
    T: Eq + Hash + Clone,
    G: Fn(T) -> Vec<Message>,
{
    let mut first_parts = midi_data.keys().map(|nd| nd.clone()).collect::<Vec<_>>();
    let mut rng = rand::thread_rng();
    first_parts.shuffle(&mut rng);

    let first_part = first_parts.first()?.clone();
    let mut prev_part = first_part.clone();

    let mut write_messages = generator(first_part);

    write_messages.extend(
        (1..24)
            .map(|_| {
                let mut second_parts = midi_data.get(&prev_part)?.iter().fold(
                    vec![],
                    |mut acc, (second_note, &times)| {
                        acc.extend(vec![second_note.clone(); times as usize]);
                        acc
                    },
                );

                second_parts.shuffle(&mut rng);

                prev_part = second_parts.first()?.clone();
                Some(second_parts.first()?.clone())
            })
            .filter_map(|part_opt| part_opt.map(|part| generator(part)))
            .flatten(),
    );

    Some(write_messages)
}
