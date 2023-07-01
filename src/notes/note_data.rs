use crate::notes::note::Note;
use ghakuf::messages::{Message, MidiEvent};
use std::cmp::Ordering;

pub type Velocity = u8;
pub type DeltaTime = u32;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct NoteData {
    note: Note,
    velocity: Velocity,
    start: DeltaTime,
    length: DeltaTime,
    delay: DeltaTime,
}

impl NoteData {
    #[inline]
    pub fn new(
        note: Note,
        velocity: Velocity,
        start: DeltaTime,
        length: DeltaTime,
        delay: DeltaTime,
    ) -> Self {
        Self {
            note,
            velocity,
            start,
            length,
            delay,
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
    pub fn get_start(&self) -> DeltaTime {
        self.start
    }

    #[inline]
    pub fn get_length(&self) -> DeltaTime {
        self.length
    }

    #[inline]
    pub fn get_delay(&self) -> DeltaTime {
        self.delay
    }

    #[inline]
    pub fn into_on_midi_event(self, start: DeltaTime) -> Message {
        Message::MidiEvent {
            delta_time: start,
            event: MidiEvent::NoteOn {
                ch: 0,
                note: self.note.midi(),
                velocity: self.velocity,
            },
        }
    }

    #[inline]
    pub fn into_off_midi_event(self, end: DeltaTime) -> Message {
        Message::MidiEvent {
            delta_time: end,
            event: MidiEvent::NoteOff {
                ch: 0,
                note: self.note.midi(),
                velocity: self.velocity,
            },
        }
    }

    #[inline]
    pub fn into_on_off_midi_events(self, start: DeltaTime, end: DeltaTime) -> (Message, Message) {
        (
            self.into_on_midi_event(start),
            self.into_off_midi_event(end),
        )
    }

    #[inline]
    pub fn clone_with_new_note(&self, note: Note) -> Self {
        Self::new(note, self.velocity, self.start, self.length, self.delay)
    }

    #[inline]
    pub fn clone_with_velocity(&self, velocity: Velocity) -> Self {
        Self::new(self.note, velocity, self.start, self.length, self.delay)
    }

    #[inline]
    pub fn clone_with_new_start(&self, start: DeltaTime) -> Self {
        Self::new(self.note, self.velocity, start, self.length, self.delay)
    }

    #[inline]
    pub fn clone_with_new_length(&self, length: DeltaTime) -> Self {
        Self::new(self.note, self.velocity, self.start, length, self.delay)
    }

    #[inline]
    pub fn clone_with_new_delay(&self, delay: DeltaTime) -> Self {
        Self::new(self.note, self.velocity, self.start, self.length, delay)
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

impl PartialOrd for NoteData {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let start_cmp = self.start.cmp(&other.start);

        if start_cmp != Ordering::Equal {
            return Some(start_cmp);
        }

        Some(self.note.cmp(&other.note))
    }
}

impl Ord for NoteData {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        unsafe { self.partial_cmp(other).unwrap_unchecked() }
    }
}
