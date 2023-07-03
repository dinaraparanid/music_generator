use crate::{midi::parser::midi_parser::MidiParser, notes::note_data::NoteData};
use ghakuf::reader::Reader;

use std::{
    fs::DirEntry,
    path::{Path, PathBuf},
};

const MIDI_PATH: &str = "./midi";

/// Extracts all notes from the `midi` folder from all files.
/// Notes are parsed from the MIDI events in files.
/// It is assumed that all files in `midi` folder are `*.mid` files.
/// Fetching and scanning is implemented asynchronously with
/// native polling mechanism (epoll for Linux, kqueue for FreeBSD),
/// using [monoio] crate
///
/// # Deprecated
/// There were a lot of issues with parsing lead notes,
/// because it is not easy to contrast lead and harmony in the single MIDI file.
/// As a result, melodies were to random and the result is too unstable.
/// Currently, abandoned, but may be integrated in the future

#[inline]
#[deprecated]
pub async fn extract_notes() -> Result<Vec<(PathBuf, Vec<NoteData>)>, Box<dyn std::error::Error>> {
    let lead_path = Path::new(MIDI_PATH);

    if !lead_path.exists() {
        std::fs::create_dir(lead_path)?;
    }

    // Walks through the `midi` directory and scans all files asynchronously

    let tasks = std::fs::read_dir(lead_path)?
        .into_iter()
        .map(to_file_opt)
        .map(|file_opt| {
            monoio::spawn(async {
                file_opt.map(|lead_file| {
                    let mut midi_parser = MidiParser::new();
                    let mut midi_reader = Reader::new(&mut midi_parser, &*lead_file).unwrap();
                    midi_reader.read().unwrap();
                    (lead_file, midi_parser.extract_notes())
                })
            })
        })
        .collect::<Vec<_>>();

    // Awaits all tasks and collects all data

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
