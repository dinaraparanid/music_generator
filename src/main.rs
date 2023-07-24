use chrono::Local;

use ghakuf::{
    messages::{Message, MetaEvent, MidiEvent},
    writer::Writer,
};

use music_generator::{
    midi::{
        bpm::BPM,
        generator::{composer::*, generator::generate_bpm},
        key_list, melody_types, mode_list, scale_list,
    },
    notes::note::Note,
};

use rust_music_theory::{note::Notes, scale::*};
use std::{fmt::Debug, fs::File, io::Write, path::Path};

#[inline]
fn select_from_list<T: Clone + Debug>(inp_msg: &str, list: Vec<T>) -> T {
    println!("{inp_msg}");

    list.iter()
        .enumerate()
        .for_each(|(ind, v)| println!("{}. {:?}", ind + 1, v.clone()));

    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Unable to read string");

    let index = input.trim().parse::<usize>().expect("Wrong index input");
    list.get(index - 1).expect("Wrong index").clone()
}

#[monoio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut midi_writer = Writer::new();
    midi_writer.running_status(true);

    let key = select_from_list("Select key's number:", key_list());
    let scale = select_from_list("Select scale's number:", scale_list());
    let mode = select_from_list("Select mode's number:", mode_list());
    let melody_type = select_from_list("Select melody type's number:", melody_types());

    // Picking all notes in 5 octave.
    // This notes will help to construct
    // both lead melody and chords in harmony

    let scale_notes = (4..=5)
        .map(|octave| {
            Scale::new(scale, key, octave, Some(mode), Direction::Ascending)
                .unwrap()
                .notes()
                .into_iter()
                .map(Note::from)
        })
        .flatten()
        .collect::<Vec<_>>();

    println!("SCALE NOTES: {:?}\n", scale_notes);

    let bpm = generate_bpm();
    let generated_lead = melody_type.generate_synthwave_melody(key, &scale_notes);

    println!("BPM: {}", bpm);
    println!("LEAD: {:?}", generated_lead);

    let lead_midi_messages = compose_lead_from_generated(generated_lead, compose_note);

    let tempo = bpm.tempo();

    let tempo_msg = Message::MetaEvent {
        delta_time: 0,
        event: MetaEvent::SetTempo,
        data: vec![(tempo >> 16) as u8, (tempo >> 8) as u8, tempo as u8],
    };

    let end_of_track_msg = Message::MetaEvent {
        delta_time: 0,
        event: MetaEvent::EndOfTrack,
        data: Vec::new(),
    };

    let track_change_msg = Message::TrackChange;

    // 1, 53, 29, 120, 48, 117, 109,
    // 67, 88, 58, 60, 52, 79, 61, 48, 5, 87, 27, 33
    // 5, 32, 48, 63, 85

    let lead_instrument_msg = Message::MidiEvent {
        delta_time: 0,
        event: MidiEvent::ProgramChange { ch: 0, program: 80 },
    };

    let reverb_effect_msg = Message::MidiEvent {
        delta_time: 0,
        event: MidiEvent::ControlChange {
            ch: 0,
            control: 91,
            data: 110,
        },
    };

    let chorus_effect_msg = Message::MidiEvent {
        delta_time: 0,
        event: MidiEvent::ControlChange {
            ch: 0,
            control: 93,
            data: 90,
        },
    };

    // Initialise MIDI file with tempo and instrument

    midi_writer.format(1);
    midi_writer.time_base(128);
    midi_writer.push(&tempo_msg);
    midi_writer.push(&end_of_track_msg);

    // Pushes lead messages to the event holder
    midi_writer.push(&track_change_msg);
    midi_writer.push(&lead_instrument_msg);
    midi_writer.push(&reverb_effect_msg);
    midi_writer.push(&chorus_effect_msg);
    lead_midi_messages.iter().for_each(|m| midi_writer.push(m));
    midi_writer.push(&end_of_track_msg);

    std::fs::create_dir("./generated").unwrap_or_default();

    let mut file = File::create("track_settings.txt")?;
    let track_settings = format!("{key}\n{scale}\n{mode}\n{:?}\n{bpm}", melody_type);
    file.write_all(track_settings.as_bytes())?;

    let path = format!("./generated/{}-{}BPM-{}.mid", key, bpm, Local::now());
    let path = Path::new(path.as_str());
    println!("PATH: {:?}", path.to_path_buf());

    midi_writer.write(&path)?;
    Ok(())
}
