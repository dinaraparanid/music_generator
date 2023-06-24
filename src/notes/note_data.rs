use crate::notes::note::Note;
use ghakuf::messages::{Message, MidiEvent};
use std::time::Duration;

pub type Velocity = u8;
pub type DeltaTime = u32;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct NoteData {
    note: Note,
    velocity: Velocity,
    start: Duration,
    end: Duration,
}

impl NoteData {
    #[inline]
    pub fn new(note: Note, velocity: Velocity, start: Duration, end: Duration) -> Self {
        Self {
            note,
            velocity,
            start,
            end,
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
    pub fn get_start(&self) -> Duration {
        self.start
    }

    #[inline]
    pub fn get_end(&self) -> Duration {
        self.end
    }

    #[inline]
    pub fn get_duration(&self) -> Duration {
        self.end - self.start
    }

    #[inline]
    pub fn into_on_midi_event(self, start: Duration) -> Message {
        Message::MidiEvent {
            delta_time: start.as_millis() as DeltaTime,
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
            delta_time: end.as_millis() as DeltaTime,
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

    #[inline]
    pub fn clone_with_new_note(&self, note: Note) -> Self {
        Self::new(note, self.velocity, self.start, self.end)
    }

    #[inline]
    pub fn up(&self, semitones: u8) -> Option<Self> {
        self.note
            .up(semitones)
            .map(|note| self.clone_with_new_note(note))
    }

    #[inline]
    pub unsafe fn up_unchecked(&self, semitones: u8) -> Self {
        self.up(semitones).unwrap_unchecked()
    }

    #[inline]
    pub fn down(&self, semitones: u8) -> Option<Self> {
        self.note
            .down(semitones)
            .map(|note| self.clone_with_new_note(note))
    }

    #[inline]
    pub unsafe fn down_unchecked(&self, semitones: u8) -> Self {
        self.down(semitones).unwrap_unchecked()
    }

    #[inline]
    pub fn octave_up(&self) -> Option<Self> {
        self.note
            .octave_up()
            .map(|note| self.clone_with_new_note(note))
    }

    #[inline]
    pub unsafe fn octave_up_unchecked(&self) -> Self {
        self.octave_up().unwrap_unchecked()
    }

    #[inline]
    pub fn octave_down(&self) -> Option<Self> {
        self.note
            .octave_down()
            .map(|note| self.clone_with_new_note(note))
    }

    #[inline]
    pub unsafe fn octave_down_unchecked(&self) -> Self {
        self.octave_down().unwrap_unchecked()
    }
}
