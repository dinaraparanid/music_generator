use crate::notes::{note::Note, note_data::*, ChordData};
use ghakuf::{messages::MidiEvent, reader::Handler};
use std::{collections::HashMap, time::Duration};

#[derive(Debug, Default)]
pub struct ChordParser {
    current_hash_chord_on: HashMap<Note, (Velocity, DeltaTime)>,
    current_chord_on: ChordData,
    extracted_chords: Vec<ChordData>,
}

impl ChordParser {
    #[inline]
    pub fn new() -> Self {
        Self {
            current_hash_chord_on: HashMap::new(),
            current_chord_on: Vec::new(),
            extracted_chords: Vec::new(),
        }
    }

    #[inline]
    fn extract_latest_chord(&mut self) {
        self.extracted_chords.push(self.current_chord_on.clone());
        self.current_chord_on.clear();
    }

    #[inline]
    pub fn extract_chords(mut self) -> Vec<ChordData> {
        if !self.current_chord_on.is_empty() {
            self.extract_latest_chord();
        }

        self.extracted_chords
    }
}

impl Handler for ChordParser {
    #[inline]
    fn midi_event(&mut self, delta_time: DeltaTime, event: &MidiEvent) {
        match event {
            MidiEvent::NoteOn {
                ch: _ch,
                note,
                velocity,
            } => {
                if !self.current_chord_on.is_empty() {
                    self.extract_latest_chord();
                }

                self.current_hash_chord_on
                    .insert(Note::from(*note), (*velocity, delta_time))
                    .unwrap();
            }

            MidiEvent::NoteOff {
                ch: _ch,
                note,
                velocity: _vel,
            } => {
                let note = Note::from(*note);
                let (vel, start) = *self.current_hash_chord_on.get(&note).unwrap();

                self.current_chord_on.push(NoteData::new(
                    note,
                    vel,
                    Duration::from_millis(start as u64),
                    Duration::from_millis(delta_time as u64),
                ))
            }

            _ => {}
        }
    }
}
