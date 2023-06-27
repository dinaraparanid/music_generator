use ghakuf::writer::Writer;

use music_generator::midi::{
    generator::{analyzer::analyze_midi, composer::*},
    parser::midi_file_manager::*,
};

use music_generator::notes::note::Note;
use std::path::Path;

#[monoio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("./example.mid");
    let mut writer = Writer::new();

    writer.running_status(true);

    let notes = extract_notes().await?;

    let melody_notes = notes
        .into_iter()
        .map(|(_, notes)| {
            notes
                .into_iter()
                .filter(|&note| note.get_note() >= Note::A4)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let generated_lead = generate_from_midi_analyze(analyze_midi(melody_notes), generate_note)
        .expect("`midi` folder is empty");

    generated_lead.iter().for_each(|m| writer.push(m));

    writer.write(&path)?;
    Ok(())
}
