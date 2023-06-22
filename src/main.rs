use ghakuf::{
    messages::{Message, MetaEvent, MidiEvent},
    writer::Writer,
};

use std::{path::Path, time::Duration};

#[inline]
fn get_bar_time(bpm: u64) -> Duration {
    Duration::from_millis(240000 / bpm)
}

#[inline]
fn get_tempo(bpm: u64) -> u64 {
    60000000 / bpm
}

#[inline]
fn create_loop(duration: Duration, notes: Vec<(u8, u8)>) -> Vec<Message> {
    let mut result = vec![Message::MidiEvent {
        delta_time: 0,
        event: MidiEvent::NoteOn {
            ch: 0,
            note: notes[0].0,
            velocity: notes[0].1,
        },
    }];

    result.extend(
        notes
            .iter()
            .skip(1)
            .map(|(note, velocity)| Message::MidiEvent {
                delta_time: 0,
                event: MidiEvent::NoteOn {
                    ch: 0,
                    note: *note,
                    velocity: *velocity,
                },
            }),
    );

    result.push(Message::MidiEvent {
        delta_time: duration.as_millis() as u32,
        event: MidiEvent::NoteOff {
            ch: 0,
            note: notes[0].0,
            velocity: notes[0].1,
        },
    });

    result.extend(
        notes
            .into_iter()
            .skip(1)
            .map(|(note, velocity)| Message::MidiEvent {
                delta_time: 0,
                event: MidiEvent::NoteOff {
                    ch: 0,
                    note,
                    velocity,
                },
            }),
    );

    result
}

fn main() {
    let bpm = 90;
    let bar_time = get_bar_time(bpm);
    let tempo = get_tempo(bpm);

    let mut write_messages = vec![Message::MetaEvent {
        delta_time: 0,
        event: MetaEvent::SetTempo,
        data: [(tempo >> 16) as u8, (tempo >> 8) as u8, tempo as u8].to_vec(),
    }];

    (0..128).for_each(|i| {
        write_messages.push(Message::MidiEvent {
            delta_time: 0,
            event: MidiEvent::ProgramChange { ch: 0, program: i },
        });

        write_messages.extend(create_loop(bar_time, vec![(66, 100), (69, 100), (73, 100)]));

        write_messages.extend(create_loop(bar_time, vec![(61, 100), (64, 100), (68, 100)]));

        write_messages.extend(create_loop(
            bar_time,
            vec![(61, 100), (64, 100), (68, 100), (71, 100)],
        ));

        write_messages.extend(create_loop(bar_time, vec![(63, 100), (66, 100), (69, 100)]))
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
    writer.write(&path).unwrap()
}
