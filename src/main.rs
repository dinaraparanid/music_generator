use ghakuf::{
    messages::{Message, MetaEvent, MidiEvent},
    writer::Writer,
};

use music_generator::{
    midi::{create_chord, get_bar_time, get_tempo},
    note::Note,
    note_data::NoteData,
};

use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bpm = 90;
    let bar_time = get_bar_time(bpm);
    let tempo = get_tempo(bpm);

    let mut write_messages = vec![Message::MetaEvent {
        delta_time: 0,
        event: MetaEvent::SetTempo,
        data: [(tempo >> 16) as u8, (tempo >> 8) as u8, tempo as u8].to_vec(),
    }];

    (0..=127).for_each(|i| {
        write_messages.push(Message::MidiEvent {
            delta_time: 0,
            event: MidiEvent::ProgramChange { ch: 0, program: i },
        });

        write_messages.extend(create_chord(vec![
            NoteData::new(Note::Gb4, 100, bar_time),
            NoteData::new(Note::A4, 100, bar_time),
            NoteData::new(Note::Db5, 100, bar_time),
        ]));

        write_messages.extend(create_chord(vec![
            NoteData::new(Note::Db4, 100, bar_time),
            NoteData::new(Note::E4, 100, bar_time),
            NoteData::new(Note::Ab4, 100, bar_time),
        ]));

        write_messages.extend(create_chord(vec![
            NoteData::new(Note::Db4, 100, bar_time),
            NoteData::new(Note::E4, 100, bar_time),
            NoteData::new(Note::Ab4, 100, bar_time),
            NoteData::new(Note::B4, 100, bar_time),
        ]));

        write_messages.extend(create_chord(vec![
            NoteData::new(Note::Eb4, 100, bar_time),
            NoteData::new(Note::Gb4, 100, bar_time),
            NoteData::new(Note::A4, 100, bar_time),
        ]))
    });

    write_messages.push(Message::MetaEvent {
        delta_time: 0,
        event: MetaEvent::EndOfTrack,
        data: Vec::new(),
    });

    let path = Path::new("./example.mid");
    let mut writer = Writer::new();

    writer.running_status(true);
    write_messages.iter().for_each(|m| writer.push(m));
    writer.write(&path)?;
    Ok(())
}
