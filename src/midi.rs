use crate::note_data::NoteData;
use ghakuf::messages::{Message, MidiEvent};
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
        .map(|note_data| Message::MidiEvent {
            delta_time: 0,
            event: MidiEvent::NoteOn {
                ch: 0,
                note: note_data.get_note().midi(),
                velocity: note_data.get_velocity(),
            },
        })
        .collect::<Vec<_>>();

    notes.sort_by_key(|nd| nd.get_duration());

    result.extend(
        notes
            .iter()
            .map(|nd| nd.get_duration())
            .scan(
                (
                    Duration::from_millis(0), // time offset
                    Duration::from_millis(0), // previous note duration
                ),
                |(time_offset, prev_note_end), cur_note_end| {
                    *time_offset = cur_note_end - *prev_note_end;
                    *prev_note_end = cur_note_end;
                    Some((*time_offset, *prev_note_end))
                },
            )
            .map(|(time_offset, _)| time_offset)
            .zip(notes.iter())
            .map(|(end, nd)| Message::MidiEvent {
                delta_time: end.as_millis() as u32,
                event: MidiEvent::NoteOff {
                    ch: 0,
                    note: nd.get_note().midi(),
                    velocity: nd.get_velocity(),
                },
            }),
    );

    result
}
