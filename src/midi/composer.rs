use crate::notes::ChordData;
use ghakuf::messages::Message;
use std::time::Duration;

#[inline]
pub fn create_chord(mut chord: ChordData) -> Vec<Message> {
    let mut result = chord
        .iter()
        .map(|note_data| note_data.into_on_midi_event(note_data.get_start()))
        .collect::<Vec<_>>();

    chord.sort_by_key(|nd| nd.get_end());

    result.extend(
        chord
            .iter()
            .map(|nd| nd.get_end())
            .scan(
                (Duration::default(), Duration::default()),
                |(time_offset, prev_note_end), cur_note_end| {
                    *time_offset = cur_note_end - *prev_note_end;
                    *prev_note_end = cur_note_end;
                    Some((*time_offset, *prev_note_end))
                },
            )
            .map(|(time_offset, _)| time_offset)
            .zip(chord.iter())
            .map(|(end, nd)| nd.into_off_midi_event(end)),
    );

    result
}
