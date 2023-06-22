use crate::note::Note;
use ghakuf::messages::{Message, MidiEvent};
use std::time::Duration;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct NoteData {
    note: Note,
    velocity: u8,
    duration: Duration,
}

impl NoteData {
    #[inline]
    pub fn new(note: Note, velocity: u8, duration: Duration) -> Self {
        Self {
            note,
            velocity,
            duration,
        }
    }

    #[inline]
    pub fn get_note(&self) -> Note {
        self.note
    }

    #[inline]
    pub fn get_velocity(&self) -> u8 {
        self.velocity
    }

    #[inline]
    pub fn get_duration(&self) -> Duration {
        self.duration
    }

    #[inline]
    pub fn into_on_midi_event(self, start: Duration) -> Message {
        Message::MidiEvent {
            delta_time: start.as_millis() as u32,
            event: MidiEvent::NoteOn {
                ch: 0,
                note: self.note.midi(),
                velocity: self.velocity,
            },
        }
    }

    #[inline]
    pub fn into_off_midi_event(self, end: Duration) -> Message {
        Message::MidiEvent {
            delta_time: end.as_millis() as u32,
            event: MidiEvent::NoteOn {
                ch: 0,
                note: self.note.midi(),
                velocity: self.velocity,
            },
        }
    }

    #[inline]
    pub fn into_on_off_midi_events(self, start: Duration, end: Duration) -> (Message, Message) {
        (
            self.into_on_midi_event(start),
            self.into_off_midi_event(end),
        )
    }
}
