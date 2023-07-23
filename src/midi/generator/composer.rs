use crate::notes::{note_data::*, ChordData};
use ghakuf::messages::{Message, MidiEvent};

/// Constructs vector of ON and OFF MIDI events.
/// ON event is happened after the note's delay,
/// OFF event is happened after the note's end
/// (when length is reached)

#[inline]
pub fn compose_note(note: NoteData) -> Vec<Message> {
    vec![
        note.into_on_midi_event(note.delay(), 0),
        note.into_off_midi_event(note.length(), 0),
    ]
}

/// Constructs vector of ON/OFF MIDI events for all notes in a chord.
/// It is assumed that all notes in a chord have the same delay.
/// However, notes length may differ, so OFF events are sorted by the length

#[inline]
pub fn compose_chord(mut chord: ChordData) -> Vec<Message> {
    // Constructing on events

    let mut result = chord
        .iter()
        .map(|nd| nd.into_on_midi_event(nd.delay(), 1))
        .collect::<Vec<_>>();

    chord.sort_by_key(|nd| nd.delay() + nd.length());

    result.extend(
        chord
            .iter()
            .map(|nd| nd.delay() + nd.length())
            .scan((0, 0), |(time_offset, prev_note_end), cur_note_end| {
                *time_offset = cur_note_end - *prev_note_end;
                *prev_note_end = cur_note_end;
                Some((*time_offset, *prev_note_end))
            })
            .map(|(time_offset, _)| time_offset)
            .zip(chord.iter())
            .map(|(end, nd)| nd.into_off_midi_event(end, 1)),
    );

    result
}

/// Generates harmony from the given lead.
/// Then constructs vectors of MIDI messages
/// from both lead and generated harmony

#[inline]
pub fn compose_lead_from_generated<L>(
    generated_lead: Vec<NoteData>,
    lead_composer: L,
) -> Vec<Message>
where
    L: Fn(NoteData) -> Vec<Message>,
{
    generated_lead
        .into_iter()
        .map(lead_composer)
        .flatten()
        .collect()
}

#[inline]
pub fn change_note_msg_channel(midi_msg: &Message, channel: u8) -> Message {
    match midi_msg {
        Message::MidiEvent {
            delta_time,
            event: MidiEvent::NoteOn { ch, note, velocity },
        } => Message::MidiEvent {
            delta_time: *delta_time,
            event: MidiEvent::NoteOn {
                ch: channel,
                note: *note,
                velocity: *velocity,
            },
        },

        Message::MidiEvent {
            delta_time,
            event: MidiEvent::NoteOff { ch, note, velocity },
        } => Message::MidiEvent {
            delta_time: *delta_time,
            event: MidiEvent::NoteOff {
                ch: channel,
                note: *note,
                velocity: *velocity,
            },
        },

        _ => midi_msg.clone(),
    }
}
