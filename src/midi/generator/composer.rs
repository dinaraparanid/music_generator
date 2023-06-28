use crate::{
    midi::{bpm::BPM, generator::analyzer::AnalyzedNotes},
    notes::{note::Note, note_data::*, ChordData},
};

use ghakuf::messages::Message;
use rand::{prelude::SliceRandom, Rng};
use rust_music_theory::note::{Note as MTNote, PitchClass};
use std::{collections::HashMap, hash::Hash};

#[inline]
pub fn generate_note(note: NoteData, delay: DeltaTime, length: DeltaTime) -> Vec<Message> {
    println!("{:?}", note.get_note());

    vec![
        note.into_on_midi_event(delay),
        note.into_off_midi_event(delay + length),
    ]
}

#[inline]
pub fn create_chord(mut chord: ChordData) -> Vec<Message> {
    let mut result = chord
        .iter()
        .map(|note_data| note_data.into_on_midi_event(0))
        .collect::<Vec<_>>();

    chord.sort_by_key(|nd| nd.get_length());

    result.extend(
        chord
            .iter()
            .map(|nd| nd.get_length())
            .scan((0, 0), |(time_offset, prev_note_end), cur_note_end| {
                *time_offset = cur_note_end - *prev_note_end;
                *prev_note_end = cur_note_end;
                Some((*time_offset, *prev_note_end))
            })
            .map(|(time_offset, _)| time_offset)
            .zip(chord.iter())
            .map(|(end, nd)| nd.into_off_midi_event(end)),
    );

    result
}

#[inline]
pub fn generate_key(analyzed_melody_notes: &AnalyzedNotes) -> Option<PitchClass> {
    let mut rng = rand::thread_rng();

    let mut first_notes = analyzed_melody_notes
        .keys()
        .map(|&note| note)
        .collect::<Vec<_>>();

    first_notes.shuffle(&mut rng);
    first_notes.into_iter().next().map(PitchClass::from)
}

#[inline]
fn get_bar_ratio(bar_time: DeltaTime, ratio: u32) -> DeltaTime {
    bar_time / 16 * ratio
}

#[inline]
fn generate_melody_ratios(bar_time: DeltaTime) -> Vec<(DeltaTime, DeltaTime)> {
    let mut rng = rand::thread_rng();
    let melody_len = rng.gen_range(3..=8);

    let mut delays = (0..=2).collect::<Vec<_>>();
    let mut lengths = (1..=2).collect::<Vec<_>>();

    (0..melody_len)
        .map(|_| {
            delays.shuffle(&mut rng);
            lengths.shuffle(&mut rng);

            (
                get_bar_ratio(bar_time, *delays.first().unwrap()),
                get_bar_ratio(bar_time, *lengths.first().unwrap()),
            )
        })
        .collect()
}

#[inline]
pub fn generate_lead_from_analyze<C>(
    key: PitchClass,
    scale_notes: Vec<Note>,
    analyzed_notes: &AnalyzedNotes,
    notes_to_data: HashMap<Note, Vec<NoteData>>,
    composer: C,
) -> Option<Vec<Message>>
where
    C: Fn(NoteData, DeltaTime, DeltaTime) -> Vec<Message>,
{
    let bpm = 100;
    let bar_time = bpm.get_bar_time().as_millis() as DeltaTime;
    let melody_ratios = generate_melody_ratios(bar_time);

    let mut rng = rand::thread_rng();

    let mut first_notes = analyzed_notes
        .keys()
        .filter(|&note| scale_notes.contains(note))
        .map(|&note| note)
        .collect::<Vec<_>>();

    let mut write_messages = Vec::new();
    let mut current_note_index = 0;

    while write_messages.len() < melody_ratios.len() * 2 {
        first_notes.shuffle(&mut rng);

        let first_note = *first_notes.iter().next()?;
        let mut first_note_datas = notes_to_data.get(&first_note)?.clone();
        first_note_datas.shuffle(&mut rng);

        let first_note_data = first_note_datas.into_iter().next()?;
        current_note_index = (current_note_index + 1) % melody_ratios.len();
        let (delay, len) = melody_ratios[current_note_index];
        write_messages.extend(composer(first_note_data, delay, len));

        write_messages.extend(
            (1..melody_ratios.len())
                .scan(first_note, |prev_note, _| {
                    let mut second_notes = analyzed_notes
                        .get(prev_note)?
                        .iter()
                        .filter(|&(note, _)| scale_notes.contains(note))
                        .fold(vec![], |mut acc, (&second_note, &times)| {
                            acc.extend(vec![second_note; times as usize]);
                            acc
                        });

                    let mut rng = rand::thread_rng();
                    second_notes.shuffle(&mut rng);
                    *prev_note = *second_notes.first()?;
                    Some(*prev_note)
                })
                .map(|second_note| {
                    let mut rng = rand::thread_rng();
                    let mut second_note_datas = notes_to_data.get(&second_note)?.clone();
                    second_note_datas.shuffle(&mut rng);

                    let next_note = second_note_datas.into_iter().next()?;
                    Some(next_note)
                })
                .take_while(|note_opt| note_opt.is_some())
                .filter_map(|note_opt| {
                    current_note_index = (current_note_index + 1) % melody_ratios.len();
                    let (delay, len) = melody_ratios[current_note_index];
                    note_opt.map(|note| composer(note, delay, len))
                })
                .flatten(),
        );

        current_note_index = (current_note_index + 1) % melody_ratios.len();
        let (delay, len) = melody_ratios[current_note_index];
        write_messages.extend(generate_tonic_lead_note(key, &notes_to_data, delay, len)?);
    }

    Some(write_messages)
}

#[inline]
fn generate_tonic_lead_note(
    key: PitchClass,
    notes_to_data: &HashMap<Note, Vec<NoteData>>,
    delay: DeltaTime,
    length: DeltaTime,
) -> Option<Vec<Message>> {
    let mut rng = rand::thread_rng();
    let mut note_datas = notes_to_data.get(&Note::from(MTNote::new(key, 5)))?.clone();
    note_datas.shuffle(&mut rng);
    Some(generate_note(*note_datas.iter().next()?, delay, length))
}

#[inline]
pub fn generate_from_midi_analyze<T, G>(
    midi_data: HashMap<T, HashMap<T, u32>>,
    generator: G,
) -> Option<Vec<Message>>
where
    T: Eq + Hash + Clone,
    G: Fn(T) -> Vec<Message>,
{
    let mut first_parts = midi_data.keys().map(|nd| nd.clone()).collect::<Vec<_>>();
    let mut rng = rand::thread_rng();
    first_parts.shuffle(&mut rng);

    let first_part = first_parts.first()?.clone();
    let mut prev_part = first_part.clone();

    let mut write_messages = generator(first_part);

    write_messages.extend(
        (1..24)
            .map(|_| {
                let mut second_parts = midi_data.get(&prev_part)?.iter().fold(
                    vec![],
                    |mut acc, (second_note, &times)| {
                        acc.extend(vec![second_note.clone(); times as usize]);
                        acc
                    },
                );

                second_parts.shuffle(&mut rng);

                prev_part = second_parts.first()?.clone();
                Some(second_parts.first()?.clone())
            })
            .filter_map(|part_opt| part_opt.map(|part| generator(part)))
            .flatten(),
    );

    Some(write_messages)
}
