use crate::notes::{note::Note, note_data::*};
use ghakuf::{messages::MidiEvent, reader::Handler};
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Default)]
pub struct MidiParser {
    notes: BTreeMap<Note, Vec<(Velocity, (DeltaTime, DeltaTime, DeltaTime))>>,
    delta_timer: DeltaTime,
    notes_on_hash: HashMap<Note, (Velocity, DeltaTime)>,
}

impl MidiParser {
    #[inline]
    pub fn new() -> Self {
        Self {
            notes: BTreeMap::new(),
            delta_timer: 0,
            notes_on_hash: HashMap::new(),
        }
    }

    #[inline]
    pub fn extract_notes(self) -> Vec<NoteData> {
        let mut notes = self
            .notes
            .into_iter()
            .map(|(note, plays)| {
                plays
                    .into_iter()
                    .map(|(vel, (start, len, delay))| NoteData::new(note, vel, start, len, delay))
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect::<Vec<_>>();

        notes.sort();
        notes
    }
}

impl Handler for MidiParser {
    #[inline]
    fn midi_event(&mut self, delta_time: u32, event: &MidiEvent) {
        self.delta_timer += delta_time;

        match event {
            MidiEvent::NoteOn {
                ch: _ch,
                note,
                velocity,
            } => {
                let note = Note::from(*note);
                self.notes_on_hash
                    .entry(note)
                    .or_insert((*velocity, self.delta_timer));
            }

            MidiEvent::NoteOff {
                ch: _ch,
                note,
                velocity: _velocity,
            } => {
                let note = Note::from(*note);
                let (vel, start) = self.notes_on_hash.remove(&note).unwrap();

                self.notes
                    .entry(note)
                    .or_insert(Vec::new())
                    .push((vel, (start, self.delta_timer - start, delta_time)))
            }

            _ => {}
        }
    }
}
