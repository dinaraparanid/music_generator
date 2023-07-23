use crate::notes::{note::Note, note_data::*};
use ghakuf::messages::MetaEvent;
use ghakuf::{messages::MidiEvent, reader::Handler};
use itertools::Itertools;
use std::collections::{BTreeMap, HashMap};

/// Parses a single .mid file and converts
/// all events from it to the [NoteData]

#[derive(Debug, Default)]
pub struct MidiParser {
    notes: BTreeMap<Note, Vec<(Velocity, DeltaTime, DeltaTime, DeltaTime)>>,
    delta_timer: DeltaTime,
    notes_on_hash: HashMap<Note, (Velocity, DeltaTime, DeltaTime)>,
}

impl MidiParser {
    /// Constructs new MIDI parser with no analyzed data

    #[inline]
    pub fn new() -> Self {
        Self {
            notes: BTreeMap::new(),
            delta_timer: 0,
            notes_on_hash: HashMap::new(),
        }
    }

    /// Extracts all parsed notes after scanning was done

    #[inline]
    pub fn extract_notes(self) -> Vec<NoteData> {
        self.notes
            .into_iter()
            .map(|(note, plays)| {
                plays
                    .into_iter()
                    .map(|(vel, start, len, delay)| NoteData::new(note, vel, start, len, delay))
                    .collect::<Vec<_>>()
            })
            .flatten()
            .sorted()
            .collect()
    }
}

impl Handler for MidiParser {
    #[inline]
    fn header(&mut self, format: u16, track: u16, time_base: u16) {
        println!("HEADER; FORMAT: {format} TRACK {track} TIME BASE {time_base}");
    }

    #[inline]
    fn meta_event(&mut self, delta_time: u32, event: &MetaEvent, data: &Vec<u8>) {
        println!(
            "META; DELTA {delta_time} EVENT: {} DATA: {:?}",
            *event, *data
        );
    }

    #[inline]
    fn midi_event(&mut self, delta_time: u32, event: &MidiEvent) {
        // Increases whole file's timer
        // to construct start in NoteData

        self.delta_timer += delta_time;
        println!("MIDI; DELTA: {delta_time}, event: {}", *event);

        match event {
            MidiEvent::NoteOn {
                ch: _ch,
                note,
                velocity,
            } => {
                // Adds note to the map of current on notes

                let note = Note::from(*note);

                self.notes_on_hash
                    .entry(note)
                    .or_insert((*velocity, self.delta_timer, delta_time));
            }

            MidiEvent::NoteOff {
                ch: _ch,
                note,
                velocity: _velocity,
            } => {
                // Picks and removes entry with the note,
                // finishing the construction of the NoteData

                let note = Note::from(*note);
                let (vel, start, delay) = self.notes_on_hash.remove(&note).unwrap();

                // Inserts new note data to the tree map,
                // that sorts all notes in in according to the pitch

                let note_len = self.delta_timer - start;

                self.notes
                    .entry(note)
                    .or_insert(Vec::new())
                    .push((vel, start, note_len, delay))
            }

            _ => {}
        }
    }
}
