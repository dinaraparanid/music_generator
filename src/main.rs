use ghakuf::writer::Writer;

use music_generator::{
    midi::{
        generator::{analyzer::analyze_notes, composer::*},
        parser::midi_file_manager::*,
    },
    notes::note::Note,
};

use rust_music_theory::{note::Notes, scale::*};
use std::{collections::HashMap, path::Path};

#[monoio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("./example.mid");
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
    //let analyzed_melody_delays = analyze_delays(&melody_notes);

    let key = generate_key(&analyzed_melody_notes).expect("`midi` folder is empty");
    println!("Key: {:?}\n", key);

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

    println!("Scale notes: {:?}\n", scale_notes);

    let generated_lead = generate_lead_from_analyze(
        key,
        scale_notes,
        &analyzed_melody_notes,
        notes_to_data,
        generate_note,
    )
    .expect("Not enough data. Try again");

    println!("Notes: {:?}", generated_lead);

    generated_lead.iter().for_each(|m| writer.push(m));

    writer.write(&path)?;
    Ok(())
}
