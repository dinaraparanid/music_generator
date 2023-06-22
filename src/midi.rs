use crate::note_data::NoteData;
use ghakuf::messages::Message;
use std::time::Duration;

#[inline]
pub fn get_bar_time(bpm: u64) -> Duration {
    Duration::from_millis(240000 / bpm)
}

#[inline]
pub fn get_tempo(bpm: u64) -> u64 {
    60000000 / bpm
}

#[inline]
pub fn create_chord(mut notes: Vec<NoteData>) -> Vec<Message> {
    let mut result = notes
        .iter()
        .map(|note_data| note_data.into_on_midi_event(Duration::default()))
        .collect::<Vec<_>>();

    notes.sort_by_key(|nd| nd.get_duration());

    result.extend(
        notes
            .iter()
            .map(|nd| nd.get_duration())
            .scan(
                (Duration::default(), Duration::default()),
                |(time_offset, prev_note_end), cur_note_end| {
                    *time_offset = cur_note_end - *prev_note_end;
                    *prev_note_end = cur_note_end;
                    Some((*time_offset, *prev_note_end))
                },
            )
            .map(|(time_offset, _)| time_offset)
            .zip(notes.iter())
            .map(|(end, nd)| nd.into_off_midi_event(end)),
    );

    result
}
