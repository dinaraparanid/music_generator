use ghakuf::writer::Writer;

use music_generator::midi::{
    generator::{analyzer::analyze_midi, composer::*},
    parser::midi_file_manager::*,
};

use std::path::Path;

#[monoio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("./example.mid");
    let mut writer = Writer::new();

    writer.running_status(true);

    let generated_lead =
        generate_from_midi_analyze(analyze_midi(extract_leads().await?), create_note)
            .expect("Lead folder is empty");

    generated_lead.iter().for_each(|m| writer.push(m));

    writer.write(&path)?;
    Ok(())
}
