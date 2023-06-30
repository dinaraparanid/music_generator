use crate::{
    midi::{bpm::BPM, generator::analyzer::AnalyzedNotes},
    notes::{note::Note, note_data::*, ChordData},
};

use astro_float::{ctx::Context, Consts, RoundingMode};
use ghakuf::messages::Message;
use itertools::Itertools;
use rand::{prelude::SliceRandom, Rng};
use rust_music_theory::chord::{Chord, Number, Quality};
use rust_music_theory::note::{Note as MTNote, Notes, PitchClass};
use std::collections::HashMap;

const DIRECTION_UP: u32 = 0;
const DIRECTION_STAY: u32 = 1;
const DIRECTION_DOWN: u32 = 2;

const NOTE_TAKE: u32 = 0;
const NOTE_COMBINE: u32 = 1;

#[inline]
pub fn generate_key() -> PitchClass {
    let mut rng = rand::thread_rng();
    PitchClass::from_u8(rng.gen())
}

#[inline]
fn generate_melody_length() -> u32 {
    rand::thread_rng().gen_range(3..=8)
}

#[inline]
fn pi_numbers(from: usize, len: usize) -> Vec<u32> {
    let pi = Context::new(1024 * 1024, RoundingMode::ToEven, Consts::new().unwrap()).const_pi();
    let pi = pi.mantissa_digits().unwrap();

    pi.into_iter()
        .skip(from)
        .take(len)
        .map(|&x| (x % 10) as u32)
        .collect()
}

#[inline]
fn generate_with_pi(len: usize) -> Vec<u32> {
    pi_numbers(rand::thread_rng().gen::<usize>() % 50, len)
        .into_iter()
        .unique()
        .collect()
}

#[inline]
fn get_bar_ratio(bar_time: DeltaTime, ratio: u32) -> DeltaTime {
    bar_time / 16 * ratio
}

#[inline]
fn fixed_to_tempo(note: NoteData, lengths: &Vec<DeltaTime>, delays: &Vec<DeltaTime>) -> NoteData {
    let new_note = note
        .clone_with_new_length(
            lengths
                .iter()
                .map(|&len| (len, (len as i32 - note.get_length() as i32).abs()))
                .min_by_key(|(_, dif)| *dif)
                .map(|(len, _)| len + generate_with_pi(1)[0])
                .unwrap_or(note.get_length()),
        )
        .clone_with_new_delay(
            delays
                .iter()
                .map(|&delay| (delay, (delay as i32 - note.get_delay() as i32).abs()))
                .min_by_key(|(_, dif)| *dif)
                .map(|(delay, _)| delay + generate_with_pi(1)[0])
                .unwrap_or(note.get_delay()),
        );

    new_note.clone_with_velocity(std::cmp::min(50 + new_note.get_velocity(), 100))
}

#[inline]
pub fn generate_lead_from_analyze(
    scale_notes: &Vec<Note>,
    analyzed_notes: &AnalyzedNotes,
    notes_to_data: &mut HashMap<Note, Vec<NoteData>>,
) -> Option<(impl BPM, Vec<NoteData>)> {
    let (bpm, lead) = try_generate_lead_from_analyze(scale_notes, analyzed_notes, notes_to_data)?;

    if lead.len() > 3 {
        Some((bpm, lead))
    } else {
        generate_lead_from_analyze(scale_notes, analyzed_notes, notes_to_data)
    }
}

#[inline]
fn try_generate_lead_from_analyze(
    scale_notes: &Vec<Note>,
    analyzed_notes: &AnalyzedNotes,
    notes_to_data: &mut HashMap<Note, Vec<NoteData>>,
) -> Option<(impl BPM, Vec<NoteData>)> {
    let mut rng = rand::thread_rng();
    let bpm = rng.gen_range(90..140);

    let bar_time = bpm.get_bar_time().as_millis() as DeltaTime;
    let melody_len = generate_melody_length();

    let mut first_notes = analyzed_notes
        .keys()
        .filter(|&note| scale_notes.contains(note))
        .map(|&note| note)
        .collect::<Vec<_>>();

    first_notes.shuffle(&mut rng);

    let first_note = *first_notes.iter().next()?;
    let first_note_datas = notes_to_data.get_mut(&first_note)?;
    first_note_datas.shuffle(&mut rng);

    let lengths = generate_with_pi(25)
        .into_iter()
        .filter(|&l| l >= 1 && l <= 8)
        .unique()
        .map(|l| get_bar_ratio(bar_time, l))
        .collect::<Vec<_>>();

    println!("LENGTHS: {:?}", lengths);

    let delays = generate_with_pi(25)
        .into_iter()
        .filter(|&d| d <= 4)
        .unique()
        .map(|d| get_bar_ratio(bar_time, d))
        .collect::<Vec<_>>();

    println!("DELAYS: {:?}", delays);

    let mut generated_lead = vec![fixed_to_tempo(
        *first_note_datas.iter().next()?,
        &lengths,
        &delays,
    )];

    generated_lead.extend(
        (1..melody_len)
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
            .filter_map(|note_opt| note_opt.map(|note| fixed_to_tempo(note, &lengths, &delays))),
    );

    Some((bpm, generated_lead))
}

#[inline]
pub fn generate_harmony_from_lead(generated_lead: &Vec<NoteData>) -> Vec<ChordData> {
    generated_lead
        .iter()
        .skip(1)
        .fold(vec![generated_lead[0]], |mut acc, &note| {
            match generate_with_pi(1)[0] % 6 {
                NOTE_TAKE => acc.push(note),

                _ => {
                    let last_note = *acc.last().unwrap();
                    *acc.last_mut().unwrap() = last_note.clone_with_new_length(
                        last_note.get_length() + note.get_delay() + note.get_length(),
                    )
                }
            }

            acc
        })
        .into_iter()
        .map(|note| {
            let mut notes = Chord::new(PitchClass::from(note), Quality::Minor, Number::Seventh)
                .notes()
                .into_iter()
                .map(Note::from)
                .map(|nt| note.clone_with_new_note(nt))
                .collect::<Vec<_>>();

            notes.iter_mut().skip(1).for_each(|n| {
                let zero_delay_note = n.clone_with_new_delay(0);
                *n = zero_delay_note;
            });

            notes
        })
        .collect()
}

#[inline]
fn generate_tonic_lead_note(
    key: PitchClass,
    notes_to_data: &HashMap<Note, Vec<NoteData>>,
    delay: DeltaTime,
) -> Option<NoteData> {
    let mut rng = rand::thread_rng();
    let mut note_datas = notes_to_data.get(&Note::from(MTNote::new(key, 5)))?.clone();
    note_datas.shuffle(&mut rng);
    Some(note_datas.iter().next()?.clone_with_new_delay(delay))
}

#[inline]
pub fn compose_note(note: NoteData) -> Vec<Message> {
    println!("Note: {:?}", note);

    vec![
        note.into_on_midi_event(note.get_delay()),
        note.into_off_midi_event(note.get_delay() + note.get_length()),
    ]
}

#[inline]
pub fn compose_chord(mut chord: ChordData) -> Vec<Message> {
    println!("Chord: {:?}", chord);

    let mut result = chord
        .iter()
        .map(|nd| nd.into_on_midi_event(nd.get_delay()))
        .collect::<Vec<_>>();

    chord.sort_by_key(|nd| nd.get_delay() + nd.get_length());

    result.extend(
        chord
            .iter()
            .map(|nd| nd.get_delay() + nd.get_length())
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
pub fn compose_from_generated<L, H>(
    bar_time: DeltaTime,
    generated_lead: Vec<NoteData>,
    scale_notes: &Vec<Note>,
    lead_composer: L,
    harmony_composer: H,
) -> (Vec<Message>, Vec<Message>)
where
    L: Fn(NoteData) -> Vec<Message>,
    H: Fn(ChordData) -> Vec<Message>,
{
    let mut rng = rand::thread_rng();

    let mut delays = generate_with_pi(25)
        .into_iter()
        .filter(|&l| l >= 4 && l <= 8)
        .unique()
        .collect::<Vec<_>>();

    delays.shuffle(&mut rng);

    let delay = generated_lead[0].get_delay() + get_bar_ratio(bar_time, *delays.first().unwrap());
    println!("DELAY: {}", delay);

    let (leads, harmonies): (Vec<Vec<Vec<Message>>>, Vec<Vec<Vec<Message>>>) = generate_with_pi(4)
        .into_iter()
        .map(|x| x % 3)
        .map(|direction| randomize_lead(generated_lead.clone(), scale_notes, direction))
        .map(|lead| {
            let harmony = generate_harmony_from_lead(&lead);
            (lead, harmony)
        })
        .enumerate()
        .map(|(ind, (mut lead, mut harmony))| match ind {
            0 => (lead, harmony),

            _ => {
                let first_note = lead[0];
                lead[0] = first_note.clone_with_new_delay(delay);

                let first_chord_note = harmony[0][0];
                harmony[0][0] = first_chord_note.clone_with_new_delay(delay);

                (lead, harmony)
            }
        })
        .map(|(leads, harmonies)| {
            (
                leads
                    .into_iter()
                    .map(|l| lead_composer(l))
                    .collect::<Vec<_>>(),
                harmonies
                    .into_iter()
                    .map(|c| harmony_composer(c))
                    .collect::<Vec<_>>(),
            )
        })
        .unzip();

    (
        leads.into_iter().flatten().flatten().collect(),
        harmonies.into_iter().flatten().flatten().collect(),
    )
}

#[inline]
fn randomize_lead(
    generated_lead: Vec<NoteData>,
    scale_notes: &Vec<Note>,
    direction: u32,
) -> Vec<NoteData> {
    let mut rng = rand::thread_rng();
    let mut semitones = vec![5, 7];
    semitones.shuffle(&mut rng);
    let semitones = semitones[0];

    generated_lead
        .into_iter()
        .map(|note| match direction {
            DIRECTION_UP => note.up(semitones).unwrap(),
            DIRECTION_DOWN => note.down(semitones).unwrap(),
            DIRECTION_STAY => note,
            _ => unreachable!(),
        })
        .map(|note| match scale_notes.contains(&note.get_note()) {
            true => note,
            false => scale_notes
                .iter()
                .filter(|&&scale_note| match direction {
                    DIRECTION_UP => scale_note > note.get_note(),
                    DIRECTION_DOWN => scale_note < note.get_note(),
                    _ => true,
                })
                .map(|&scale_note| (scale_note, (scale_note - note.get_note()).abs()))
                .min_by_key(|(_, dif)| *dif)
                .map(|(scale_note, _)| note.clone_with_new_note(scale_note))
                .unwrap_or(note),
        })
        .collect::<Vec<_>>()
}
