use chrono::Local;

use music_generator::{
    midi::{
        bpm::BPM,
        generator::{analyzer::analyze_notes, composer::*},
        parser::midi_file_manager::*,
    },
    notes::{note::Note, note_data::*},
};

use ghakuf::{
    messages::{Message, MetaEvent, MidiEvent},
    writer::Writer,
};

use rust_music_theory::{note::Notes, scale::*};
use std::{collections::HashMap, path::Path};

#[monoio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut writer = Writer::new();

    writer.running_status(true);

    let notes = extract_notes().await?;

    let notes_to_data = notes
        .iter()
        .map(|(_, notes)| notes.iter().map(|note| (note.get_note(), *note)))
        .flatten()
        .fold(HashMap::new(), |mut notes_to_data, (note, data)| {
            notes_to_data.entry(note).or_insert(Vec::new()).push(data);
            notes_to_data
        });

    let melody_notes = notes
        .into_iter()
        .map(|(_, notes)| {
            notes
                .into_iter()
                .filter(|note| note.get_note() > Note::A4)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let analyzed_melody_notes = analyze_notes(&melody_notes);

    let key = generate_key();
    println!("KEY: {:?}\n", key);

    let scale_notes = Scale::new(
        ScaleType::MelodicMinor,
        key,
        5,
        Some(Mode::Aeolian),
        Direction::Ascending,
    )?
    .notes()
    .into_iter()
    .map(Note::from)
    .collect::<Vec<_>>();

    println!("SCALED NOTES: {:?}\n", scale_notes);

    let (bpm, generated_lead) = {
        let (bpm, generated_lead) =
            generate_lead_from_analyze(&scale_notes, &analyzed_melody_notes, notes_to_data)
                .expect("Not enough data. Try again");

        (bpm, generated_lead)
    };

    println!("BPM: {}", bpm);
    println!("NOTES: {:?}", generated_lead);

    let midi_messages = compose_lead_from_generated(
        bpm.get_bar_time().as_millis() as DeltaTime,
        generated_lead,
        &scale_notes,
        compose_note,
    );

    let tempo = bpm.get_tempo();

    let tempo_msg = Message::MetaEvent {
        delta_time: 0,
        event: MetaEvent::SetTempo,
        data: [(tempo >> 16) as u8, (tempo >> 8) as u8, tempo as u8].to_vec(),
    };

    let lead_instrument_msg = Message::MidiEvent {
        delta_time: 0,
        event: MidiEvent::ProgramChange { ch: 0, program: 8 },
    };

    writer.push(&tempo_msg);
    writer.push(&lead_instrument_msg);
    midi_messages.iter().for_each(|m| writer.push(m));

    let end_of_melody = Message::MetaEvent {
        delta_time: 0,
        event: MetaEvent::EndOfTrack,
        data: Vec::new(),
    };

    writer.push(&end_of_melody);

    let path = format!("./generated/{}-{}.mid", key, Local::now());
    let path = Path::new(path.as_str());
    println!("PATH: {:?}", path.to_path_buf());

    writer.write(&path)?;
    Ok(())
}
