use crate::notes::{note::Note, note_data::*};
use ghakuf::{messages::MidiEvent, reader::Handler};
use std::time::Duration;

#[derive(Debug, Default)]
pub struct LeadParser {
    current_note_on: Option<(Note, Velocity, DeltaTime)>,
    extracted_lead: Vec<NoteData>,
}

impl LeadParser {
    #[inline]
    pub const fn new() -> Self {
        Self {
            current_note_on: None,
            extracted_lead: Vec::new(),
        }
    }

    #[inline]
    pub fn extract_lead(self) -> Vec<NoteData> {
        self.extracted_lead
    }
}

impl Handler for LeadParser {
    #[inline]
    fn midi_event(&mut self, delta_time: DeltaTime, event: &MidiEvent) {
        match event {
            MidiEvent::NoteOn {
                ch: _,
                note,
                velocity,
            } => {
                let _ = self
                    .current_note_on
                    .insert((Note::from(*note), *velocity, delta_time));
            }

            MidiEvent::NoteOff { .. } => {
                let (key, vel, start) = self.current_note_on.unwrap();

                self.extracted_lead.push(NoteData::new(
                    key,
                    vel,
                    Duration::from_millis(start as u64),
                    Duration::from_millis(delta_time as u64),
                ))
            }

            _ => {}
        }
    }
}
