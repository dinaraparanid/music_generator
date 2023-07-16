use chrono::Local;

use ghakuf::{
    messages::{Message, MetaEvent, MidiEvent},
    writer::Writer,
};

use music_generator::{
    genetic::generate_lead_with_genetic_algorithm,
    midi::{
        bpm::BPM,
        generator::{composer::*, generator::generate_key},
    },
    notes::note::Note,
};

use rust_music_theory::{note::Notes, scale::*};
use std::path::Path;

#[monoio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut midi_writer = Writer::new();
    midi_writer.running_status(true);

    let key = generate_key();
    println!("KEY: {}\n", key);

    // Picking all notes in octaves from 2 to 5.
    // This notes will help to construct
    // both lead melody and chords in harmony

    let scale_notes = (2..=5)
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

    println!("SCALE NOTES: {:?}\n", scale_notes);

    let desired_fitness = 0.85;
    let mutation_rate = 0.2;

    let (bpm, generated_lead) =
        generate_lead_with_genetic_algorithm(key, &scale_notes, desired_fitness, mutation_rate)
            .await;

    println!("BPM: {}", bpm);
    println!("LEAD: {:?}", generated_lead);

    // Generating harmony from lead
    // Converting both to MIDI messages

    let (lead_midi_messages, harmony_midi_messages) = compose_from_generated(
        key,
        generated_lead,
        &scale_notes,
        compose_note,
        compose_chord,
    );

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

    let lead_instrument_msg = Message::MidiEvent {
        delta_time: 0,
        event: MidiEvent::ProgramChange { ch: 0, program: 32 },
    };

    // Initialise MIDI file with tempo and instrument

    midi_writer.push(&tempo_msg);
    //midi_writer.push(&lead_instrument_msg);
    midi_writer.push(&end_of_track_msg);
    midi_writer.push(&track_change_msg);

    // Pushes lead messages to the event holder
    lead_midi_messages.iter().for_each(|m| midi_writer.push(m));
    midi_writer.push(&end_of_track_msg);

    midi_writer.push(&track_change_msg);

    // Pushes harmony messages to the event holder
    harmony_midi_messages
        .iter()
        .for_each(|m| midi_writer.push(m));

    midi_writer.push(&end_of_track_msg);

    std::fs::create_dir("./generated").unwrap_or_default();

    let path = format!("./generated/{}-{}BPM-{}.mid", key, bpm, Local::now());
    let path = Path::new(path.as_str());
    println!("PATH: {:?}", path.to_path_buf());

    midi_writer.write(&path)?;
    Ok(())
}
