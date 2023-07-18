use chrono::Local;

use ghakuf::{
    messages::{Message, MetaEvent, MidiEvent},
    writer::Writer,
};

use music_generator::{
    genetic::generate_lead_with_genetic_algorithm,
    midi::{
        bpm::BPM,
        generator::{
            composer::*,
            generator::{generate_bpm, generate_key},
        },
    },
    notes::{note::Note, note_data::DeltaTime},
};

use futures::future::join_all;
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
                ScaleType::Diatonic,
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

    let desired_fitness = 0.9;
    let mutation_rate = 0.25;
    let bpm = generate_bpm();

    let bar_time = bpm.get_bar_time().as_millis() as DeltaTime;
    let delay_between_parts = bar_time / 2;

    let generated_lead = join_all((0..2).map(|_| {
        generate_lead_with_genetic_algorithm(key, bpm, &scale_notes, desired_fitness, mutation_rate)
    }))
    .await
    .into_iter()
    .map(|mut lead| {
        let start = lead[0].clone_with_new_delay(delay_between_parts);
        lead[0] = start;
        lead
    })
    .flatten()
    .collect();

    println!("BPM: {}", bpm);
    println!("LEAD: {:?}", generated_lead);

    // Generating harmony from lead
    // Converting both to MIDI messages

    let (lead_midi_messages, harmony_midi_messages) = compose_lead_harmony_from_generated(
        key,
        bpm,
        generated_lead,
        &scale_notes,
        compose_note,
        compose_chord,
    );

    let lead2_midi_messages = lead_midi_messages
        .iter()
        .map(|m| change_channel(m, 2))
        .collect::<Vec<_>>();

    let harmony2_midi_messages = harmony_midi_messages
        .iter()
        .map(|m| change_channel(m, 3))
        .collect::<Vec<_>>();

    let harmony3_midi_messages = harmony_midi_messages
        .iter()
        .map(|m| change_channel(m, 4))
        .collect::<Vec<_>>();

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

    // 1, 53, 29, 120, 48, 117, 109,

    let lead1_instrument_msg = Message::MidiEvent {
        delta_time: 0,
        event: MidiEvent::ProgramChange { ch: 0, program: 1 }, // 5, 32, 48, 63, 85
    };

    let lead2_instrument_msg = Message::MidiEvent {
        delta_time: 0,
        event: MidiEvent::ProgramChange { ch: 2, program: 5 }, // 67, 88, 58, 60, 52, 79, 61, 48, 5, 87, 27, 33
    };

    let harmony1_instrument_msg = Message::MidiEvent {
        delta_time: 0,
        event: MidiEvent::ProgramChange { ch: 1, program: 5 },
    };

    let harmony2_instrument_msg = Message::MidiEvent {
        delta_time: 0,
        event: MidiEvent::ProgramChange { ch: 3, program: 28 },
    };

    let harmony3_instrument_msg = Message::MidiEvent {
        delta_time: 0,
        event: MidiEvent::ProgramChange { ch: 4, program: 33 },
    };

    // Initialise MIDI file with tempo and instrument

    midi_writer.push(&tempo_msg);
    midi_writer.push(&end_of_track_msg);

    // Pushes lead messages to the event holder
    midi_writer.push(&track_change_msg);
    midi_writer.push(&lead1_instrument_msg);
    lead_midi_messages.iter().for_each(|m| midi_writer.push(m));
    midi_writer.push(&end_of_track_msg);

    midi_writer.push(&track_change_msg);
    midi_writer.push(&lead2_instrument_msg);
    lead2_midi_messages.iter().for_each(|m| midi_writer.push(m));
    midi_writer.push(&end_of_track_msg);

    // Pushes harmony messages to the event holder
    midi_writer.push(&track_change_msg);
    midi_writer.push(&harmony1_instrument_msg);

    harmony_midi_messages
        .iter()
        .for_each(|m| midi_writer.push(m));

    midi_writer.push(&end_of_track_msg);

    midi_writer.push(&track_change_msg);
    midi_writer.push(&harmony2_instrument_msg);

    harmony2_midi_messages
        .iter()
        .for_each(|m| midi_writer.push(m));

    midi_writer.push(&end_of_track_msg);

    midi_writer.push(&track_change_msg);
    midi_writer.push(&harmony3_instrument_msg);

    harmony3_midi_messages
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
