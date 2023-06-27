use music_generator::midi::parser::midi_file_manager::*;

#[monoio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    extract_notes()
        .await?
        .into_iter()
        .for_each(|(path, notes)| {
            println!("------------------ File: {:?} ------------------", path);

            notes
                .into_iter()
                .for_each(|note| println!("Note: {:?}", note));

            println!("\n\n\n")
        });

    Ok(())
}
