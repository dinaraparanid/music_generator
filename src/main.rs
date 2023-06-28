use ghakuf::writer::Writer;

use music_generator::{
    midi::{
        generator::{analyzer::analyze_midi, composer::*},
        parser::midi_file_manager::*,
    },
    notes::note::Note,
};

use rust_music_theory::{
    note::Notes,
    scale::{Direction, Scale, ScaleType},
};

use std::path::Path;

#[monoio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("./example.mid");
    let mut writer = Writer::new();

    writer.running_status(true);

    let notes = extract_notes().await?;

    let analyzed_melody_notes = analyze_midi(
        notes
            .into_iter()
            .map(|(_, notes)| notes.into_iter().collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    );

    let key = generate_key(&analyzed_melody_notes).expect("`midi` folder is empty");
    println!("Key: {:?}\n", key);

    let scale_notes = (4..8)
        .map(|octave| {
            Scale::new(
                ScaleType::MelodicMinor,
                key,
                octave,
                None,
                Direction::Ascending,
            )
        })
        .map(|scale_res| scale_res.unwrap())
        .map(|scale| scale.notes())
        .flatten()
        .map(Note::from)
        .collect::<Vec<_>>();

    println!("Scale notes: {:?}\n", scale_notes);

    let generated_lead =
        generate_lead_from_analyze(scale_notes, &analyzed_melody_notes, generate_note)
            .expect("Not enough data. Try again");

    println!("Notes: {:?}", generated_lead);

    generated_lead.iter().for_each(|m| writer.push(m));

    writer.write(&path)?;
    Ok(())
}
