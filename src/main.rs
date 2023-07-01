use chrono::Local;

use music_generator::{
    midi::{
        bpm::BPM,
        generator::{
            composer::*,
            generator::{generate_key, generate_lead_melody},
        },
    },
    notes::note::Note,
};

use ghakuf::{
    messages::{Message, MetaEvent, MidiEvent},
    writer::Writer,
};

use rust_music_theory::{note::Notes, scale::*};
use std::path::Path;

#[monoio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut writer = Writer::new();

    writer.running_status(true);

    let key = generate_key();
    println!("KEY: {}\n", key);

    let scale_notes = (3..=5)
        .map(|octave| {
            Scale::new(
                ScaleType::MelodicMinor,
                key,
                octave,
                Some(Mode::Aeolian),
                Direction::Ascending,
            )
            .unwrap()
            .notes()
            .into_iter()
            .filter(|note| note.octave < 6)
            .map(Note::from)
        })
        .flatten()
        .collect::<Vec<_>>();

    println!("SCALED NOTES: {:?}\n", scale_notes);

    let (bpm, generated_lead) = generate_lead_melody(key, &scale_notes);

    println!("BPM: {}", bpm);
    println!("NOTES: {:?}", generated_lead);

    let (lead_midi_messages, harmony_midi_messages) =
        compose_from_generated(generated_lead, &scale_notes, compose_note, compose_chord);

    let tempo = bpm.get_tempo();

    let tempo_msg = Message::MetaEvent {
        delta_time: 0,
        event: MetaEvent::SetTempo,
        data: [(tempo >> 16) as u8, (tempo >> 8) as u8, tempo as u8].to_vec(),
    };

    let end_of_track_msg = Message::MetaEvent {
        delta_time: 0,
        event: MetaEvent::EndOfTrack,
        data: Vec::new(),
    };

    let track_change_msg = Message::TrackChange;

    writer.push(&tempo_msg);
    writer.push(&end_of_track_msg);
    writer.push(&track_change_msg);

    let lead_instrument_msg = Message::MidiEvent {
        delta_time: 0,
        event: MidiEvent::ProgramChange { ch: 0, program: 18 },
    };

    writer.push(&lead_instrument_msg);
    lead_midi_messages.iter().for_each(|m| writer.push(m));
    writer.push(&end_of_track_msg);

    // writer.push(&track_change_msg);
    // harmony_midi_messages.iter().for_each(|m| writer.push(m));
    // writer.push(&end_of_track_msg);

    let path = format!("./generated/{}-{}.mid", key, Local::now());
    let path = Path::new(path.as_str());
    println!("PATH: {:?}", path.to_path_buf());

    writer.write(&path)?;
    Ok(())
}
