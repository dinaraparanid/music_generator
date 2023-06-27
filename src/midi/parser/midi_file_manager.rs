use crate::{midi::parser::midi_parser::MidiParser, notes::note_data::NoteData};
use ghakuf::reader::Reader;

use std::{
    fs::DirEntry,
    path::{Path, PathBuf},
};

const MIDI_PATH: &str = "./midi";

#[inline]
pub async fn extract_notes() -> Result<Vec<(PathBuf, Vec<NoteData>)>, Box<dyn std::error::Error>> {
    let lead_path = Path::new(MIDI_PATH);

    if !lead_path.exists() {
        std::fs::create_dir(lead_path)?;
    }

    let tasks = std::fs::read_dir(lead_path)?
        .into_iter()
        .map(to_file_opt)
        .map(|file_opt| {
            monoio::spawn(async {
                file_opt.map(|lead_file| {
                    let mut test_parser = MidiParser::new();
                    let mut test_reader = Reader::new(&mut test_parser, &*lead_file).unwrap();
                    test_reader.read().unwrap();
                    (lead_file, test_parser.extract_notes())
                })
            })
        })
        .collect::<Vec<_>>();

    let mut leads_vec = Vec::with_capacity(tasks.len());

    for task in tasks {
        if let Some(lead) = monoio::join!(task).0 {
            leads_vec.push(lead)
        }
    }

    Ok(leads_vec)
}

#[inline]
fn to_file_opt(file_res: std::io::Result<DirEntry>) -> Option<PathBuf> {
    file_res
        .map(|lead_file| lead_file.path())
        .map(|lead_file| {
            if lead_file.is_file() {
                Some(lead_file)
            } else {
                None
            }
        })
        .ok()
        .flatten()
}
