use crate::{
    midi::parser::{chord_parser::ChordParser, lead_parser::LeadParser},
    notes::{note_data::NoteData, ChordData},
};

use ghakuf::reader::Reader;

use std::{
    fs::DirEntry,
    path::{Path, PathBuf},
};

const LEAD_PATH: &str = "./lead";
const CHORDS_PATH: &str = "./chords";

#[inline]
pub async fn extract_leads() -> Result<Vec<Vec<NoteData>>, Box<dyn std::error::Error>> {
    let lead_path = Path::new(LEAD_PATH);

    if !lead_path.exists() {
        std::fs::create_dir(lead_path)?;
    }

    let tasks = std::fs::read_dir(lead_path)?
        .into_iter()
        .map(|file_res| to_file_opt(file_res))
        .map(|file_opt| {
            monoio::spawn(async {
                file_opt.map(|lead_file| {
                    let mut lead_parser = LeadParser::new();
                    let mut lead_reader = Reader::new(&mut lead_parser, &*lead_file).unwrap();
                    lead_reader.read().unwrap();
                    lead_parser.extract_lead()
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
pub async fn extract_chords() -> Result<Vec<Vec<ChordData>>, Box<dyn std::error::Error>> {
    let chords_path = Path::new(CHORDS_PATH);

    if !chords_path.exists() {
        std::fs::create_dir(chords_path)?;
    }

    let tasks = std::fs::read_dir(chords_path)?
        .into_iter()
        .map(|file_res| to_file_opt(file_res))
        .map(|file_opt| {
            monoio::spawn(async {
                file_opt.map(|chord_file| {
                    let mut chords_parser = ChordParser::new();
                    let mut chord_reader = Reader::new(&mut chords_parser, &*chord_file).unwrap();
                    chord_reader.read().unwrap();
                    chords_parser.extract_chords()
                })
            })
        })
        .collect::<Vec<_>>();

    let mut chords_vec = Vec::with_capacity(tasks.len());

    for task in tasks {
        if let Some(chords) = monoio::join!(task).0 {
            chords_vec.push(chords)
        }
    }

    Ok(chords_vec)
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
