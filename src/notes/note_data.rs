use crate::notes::note::Note;
use ghakuf::messages::{Message, MidiEvent};
use std::cmp::Ordering;

/// Velocity of the note.
/// Typically, the number in 0..=200
pub type Velocity = u8;

/// Time in milliseconds for the ticks in .mid file.
/// Each beat is measured in the number of ticks,
/// and the delta time is the time of a single tick
pub type DeltaTime = u32;

/// Holder of the pitch, velocity, start time,
/// length and the delay time (in [DeltaTime]) of the note

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

    /// Gets note (or pitch) of the data

    #[inline]
    pub fn note(&self) -> Note {
        self.note
    }

    /// Gets velocity of the note

    #[inline]
    pub fn velocity(&self) -> u8 {
        self.velocity
    }

    /// Gets the start time of the note
    /// (time when note appeared in the .mid file,
    /// calculated in [DeltaTime])

    #[inline]
    pub fn start(&self) -> DeltaTime {
        self.start
    }

    /// Gets length of the note

    #[inline]
    pub fn length(&self) -> DeltaTime {
        self.length
    }

    /// Gets the delay (pause between notes) of the note

    #[inline]
    pub fn delay(&self) -> DeltaTime {
        self.delay
    }

    /// Constructs [MidiEvent::NoteOn] event from the note
    /// with the given start and the velocity of the note

    #[inline]
    pub fn into_on_midi_event(self, start: DeltaTime, channel: u8) -> Message {
        Message::MidiEvent {
            delta_time: start,
            event: MidiEvent::NoteOn {
                ch: channel,
                note: self.note.midi(),
                velocity: self.velocity,
            },
        }
    }

    /// Constructs [MidiEvent::NoteOn] event from the note
    /// with the given start and the velocity of the note

    #[inline]
    pub fn into_off_midi_event(self, end: DeltaTime, channel: u8) -> Message {
        Message::MidiEvent {
            delta_time: end,
            event: MidiEvent::NoteOn {
                ch: channel,
                note: self.note.midi(),
                velocity: 0,
            },
        }
    }

    /// Constructs [MidiEvent::NoteOn] and [MidiEvent::NoteOff] events
    /// from the note with the given start and the velocity of the note

    #[inline]
    pub fn into_on_off_midi_events(
        self,
        start: DeltaTime,
        end: DeltaTime,
        channel: u8,
    ) -> (Message, Message) {
        (
            self.into_on_midi_event(start, channel),
            self.into_off_midi_event(end, channel),
        )
    }

    /// Clones the data with the new note

    #[inline]
    pub fn clone_with_new_note(&self, note: Note) -> Self {
        Self::new(note, self.velocity, self.start, self.length, self.delay)
    }

    /// Clones the data with the new velocity

    #[inline]
    pub fn clone_with_velocity(&self, velocity: Velocity) -> Self {
        Self::new(self.note, velocity, self.start, self.length, self.delay)
    }

    /// Clones the data with the new start

    #[inline]
    pub fn clone_with_new_start(&self, start: DeltaTime) -> Self {
        Self::new(self.note, self.velocity, start, self.length, self.delay)
    }

    /// Clones the data with the new length

    #[inline]
    pub fn clone_with_new_length(&self, length: DeltaTime) -> Self {
        Self::new(self.note, self.velocity, self.start, length, self.delay)
    }

    /// Clones the data with the new delay

    #[inline]
    pub fn clone_with_new_delay(&self, delay: DeltaTime) -> Self {
        Self::new(self.note, self.velocity, self.start, self.length, delay)
    }

    /// Increases note's pitch with the given number of semitones

    #[inline]
    pub fn up(&self, semitones: u8) -> Option<Self> {
        self.note
            .up(semitones)
            .map(|note| self.clone_with_new_note(note))
    }

    /// Increases note's pitch with the given number of semitones
    ///
    /// # Safety
    /// This version does not checks the correctness.
    /// See [NoteData::up] for the safe version

    #[inline]
    pub unsafe fn up_unchecked(&self, semitones: u8) -> Self {
        self.up(semitones).unwrap_unchecked()
    }

    /// Decreases note's pitch with the given number of semitones

    #[inline]
    pub fn down(&self, semitones: u8) -> Option<Self> {
        self.note
            .down(semitones)
            .map(|note| self.clone_with_new_note(note))
    }

    /// Decreases note's pitch with the given number of semitones
    ///
    /// # Safety
    /// This version does not checks the correctness.
    /// See [NoteData::down] for the safe version

    #[inline]
    pub unsafe fn down_unchecked(&self, semitones: u8) -> Self {
        self.down(semitones).unwrap_unchecked()
    }

    /// Increases note's pitch by an octave

    #[inline]
    pub fn octave_up(&self) -> Option<Self> {
        self.note
            .octave_up()
            .map(|note| self.clone_with_new_note(note))
    }

    /// Increases note's pitch by an octave
    ///
    /// # Safety
    /// This version does not checks the correctness.
    /// See [NoteData::octave_up] for the safe version

    #[inline]
    pub unsafe fn octave_up_unchecked(&self) -> Self {
        self.octave_up().unwrap_unchecked()
    }

    /// Lowers note's pitch by an octave

    #[inline]
    pub fn octave_down(&self) -> Option<Self> {
        self.note
            .octave_down()
            .map(|note| self.clone_with_new_note(note))
    }

    /// Lowers note's pitch by an octave
    ///
    /// # Safety
    /// This version does not checks the correctness.
    /// See [Note::octave_down] for the safe version

    #[inline]
    pub unsafe fn octave_down_unchecked(&self) -> Self {
        self.octave_down().unwrap_unchecked()
    }
}

impl PartialOrd for NoteData {
    /// Compares notes by the start time.
    /// If both are equal, compares by midi value

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
    /// Compares notes by the start time.
    /// If both are equal, compares by midi value

    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        unsafe { self.partial_cmp(other).unwrap_unchecked() }
    }
}
