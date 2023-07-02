use crate::{
    midi::{
        bpm::BPM,
        generator::{
            analyzer::AnalyzedNotes, arpeggio_types::*, fixed_to_tempo, get_bar_ratio,
            random_from_vec, randomize_with_pi,
        },
    },
    notes::{note::Note, note_data::*, ChordData},
};

use rand::{prelude::SliceRandom, Rng};

use rust_music_theory::{
    chord::{Chord, Number, Quality},
    note::{Note as MTNote, Notes, PitchClass},
};

use std::collections::HashMap;

const DIRECTION_UP: u32 = 0;
const DIRECTION_DOWN: u32 = 1;
const DIRECTION_STAY: u32 = 2;

const NOTE_TAKE: u32 = 0;
const NOTE_COMBINE: u32 = 1;

#[inline]
pub fn generate_key() -> PitchClass {
    let mut rng = rand::thread_rng();
    PitchClass::from_u8(rng.gen())
}

#[inline]
#[deprecated]
fn generate_melody_length() -> u32 {
    // even: arp (up/down | same/same | u/u | d/d)
    // odd: 3 + even
    rand::thread_rng().gen_range(3..=12)
}

#[inline]
#[deprecated]
fn probs_vec(up_down_stay_probs: (usize, usize, usize)) -> Vec<u32> {
    let mut probs_vec = vec![];
    probs_vec.extend(vec![0; up_down_stay_probs.0]);
    probs_vec.extend(vec![1; up_down_stay_probs.1]);
    probs_vec.extend(vec![2; up_down_stay_probs.2]);
    probs_vec
}

#[inline]
fn get_scaled<F>(tonic_note: Note, scale_notes: &Vec<Note>, change: F) -> Option<Note>
where
    F: Fn(usize) -> usize,
{
    let pos = scale_notes.iter().position(|&nt| nt == tonic_note).unwrap();
    scale_notes.get(change(pos)).map(|&nt| nt)
}

#[inline]
fn rand_close_note(tonic_note: Note, scale_notes: &Vec<Note>, up_down_direction: u32) -> Note {
    match up_down_direction {
        DIRECTION_UP => get_scaled(tonic_note, scale_notes, |pos| {
            let mut notes_dif = vec![1, 2];
            pos + random_from_vec(&mut notes_dif).unwrap()
        })
        .unwrap_or(tonic_note),

        DIRECTION_DOWN => get_scaled(tonic_note, scale_notes, |pos| {
            let mut notes_dif = vec![1, 2];
            pos - random_from_vec(&mut notes_dif).unwrap()
        })
        .unwrap_or(tonic_note),

        _ => unreachable!(),
    }
}

#[inline]
fn take_rand_close_note(
    tonic_note: Note,
    scale_notes: &Vec<Note>,
    start_position: u32,
    len: DeltaTime,
    bar_time: DeltaTime,
    delay_ratio: u32,
) -> NoteData {
    NoteData::new(
        rand_close_note(tonic_note, scale_notes, rand::thread_rng().gen::<u32>() % 2),
        100,
        start_position,
        len,
        get_bar_ratio(bar_time, delay_ratio),
    )
}

#[inline]
pub fn generate_lead_melody(key: PitchClass, scale_notes: &Vec<Note>) -> (impl BPM, Vec<NoteData>) {
    let mut rng = rand::thread_rng();
    let bpm = rng.gen_range(90..140);
    let bar_time = bpm.get_bar_time().as_millis() as DeltaTime;

    let single_len = get_bar_ratio(bar_time, 4);
    let tonic_note = generate_tonic_lead_note(key, 100, single_len, 0);
    let mut is_big_delay_used = false;

    let generated_lead = (4..32)
        .step_by(4)
        .fold(vec![tonic_note], |mut lead, position| {
            let prev_note = *lead.last().unwrap();
            let cur_delay = position - prev_note.get_start();

            match cur_delay {
                12 => {
                    // Distance is too big, must take note
                    is_big_delay_used = true;

                    lead.push(take_rand_close_note(
                        tonic_note.get_note(),
                        scale_notes,
                        position,
                        single_len,
                        bar_time,
                        cur_delay,
                    ))
                }

                8 => {
                    if is_big_delay_used {
                        lead.push(take_rand_close_note(
                            tonic_note.get_note(),
                            scale_notes,
                            position,
                            single_len,
                            bar_time,
                            cur_delay,
                        ))
                    }
                }

                _ => {
                    if rng.gen_bool(0.75) {
                        lead.push(take_rand_close_note(
                            tonic_note.get_note(),
                            scale_notes,
                            position,
                            single_len,
                            bar_time,
                            cur_delay,
                        ))
                    }
                }
            };

            lead
        });

    (bpm, generated_lead)
}

/// even: arpeggio
/// odd: shuffle $ arp + 3 + arp

#[inline]
#[deprecated]
fn generate_melody(key: PitchClass, scale_notes: &Vec<Note>) -> (impl BPM, Vec<NoteData>) {
    let mut rng = rand::thread_rng();
    let bpm = rng.gen_range(90..140);

    let bar_time = bpm.get_bar_time().as_millis() as DeltaTime;
    let melody_len = generate_melody_length();

    println!("BAR TIME: {}", bar_time);
    println!("MELODY LEN: {}", melody_len);

    let mut lengths = (2..=8)
        .step_by(2)
        .map(|l| get_bar_ratio(bar_time, l))
        .collect::<Vec<_>>();

    let mut delays = (0..=4)
        .step_by(2)
        .map(|d| get_bar_ratio(bar_time, d))
        .collect::<Vec<_>>();

    let tonic_note = generate_tonic_lead_note(
        key,
        100,
        random_from_vec(&mut lengths).unwrap(),
        random_from_vec(&mut delays).unwrap(),
    );

    let generated_melody = match melody_len % 2 {
        0 => generate_even_melody(tonic_note, scale_notes, melody_len, bar_time),
        1 => generate_odd_melody(tonic_note, scale_notes, melody_len, bar_time),
        _ => unreachable!(),
    };

    (bpm, generated_melody)
}

#[inline]
fn generate_even_melody(
    tonic_note: NoteData,
    scale_notes: &Vec<Note>,
    melody_len: u32,
    bar_time: DeltaTime,
) -> Vec<NoteData> {
    if melody_len % 3 != 0 || rand::thread_rng().gen_bool(0.5) {
        return generate_arpeggio_melody(tonic_note, scale_notes, melody_len, bar_time);
    }

    let melody_3 = generate_3_melody(tonic_note, scale_notes);

    randomize_with_pi(melody_len as usize / 3)
        .into_iter()
        .map(|x| x % 3)
        .map(|dir| randomize_lead(melody_3.clone(), scale_notes, dir))
        .flatten()
        .collect()
}

#[inline]
fn generate_arpeggio_melody(
    tonic_note: NoteData,
    scale_notes: &Vec<Note>,
    arp_len: u32,
    bar_time: DeltaTime,
) -> Vec<NoteData> {
    let mut delays = (0..=4)
        .step_by(2)
        .map(|d| get_bar_ratio(bar_time, d))
        .collect::<Vec<_>>();

    let mut last_arp = None;

    (0..arp_len).step_by(2).fold(vec![], |mut arp_lead, i| {
        let (arp, mut part) = (0..)
            .map(|_| ArpeggioTypes::random_arp())
            .filter(|arp| last_arp.map(|last| *arp != last).unwrap_or(true))
            .map(|arp| (arp, arp.next_part(tonic_note, scale_notes)))
            .skip_while(|(_, part)| part.is_none())
            .next()
            .map(|(arp, part)| (arp, part.unwrap()))
            .unwrap();

        let _ = last_arp.insert(arp);

        if i != 0 {
            let first_note = part[0];
            part[0] = first_note.clone_with_new_delay(random_from_vec(&mut delays).unwrap())
        }

        if rand::thread_rng().gen_bool(0.25) {
            part.push(tonic_to_arp_note(tonic_note))
        }

        arp_lead.extend(part);
        arp_lead
    })
}

#[inline]
#[deprecated]
fn generate_odd_melody(
    tonic_note: NoteData,
    scale_notes: &Vec<Note>,
    melody_len: u32,
    bar_time: DeltaTime,
) -> Vec<NoteData> {
    let odd_part = 3;
    let event_part = melody_len - odd_part;

    let mut arp_variations = (0..event_part)
        .step_by(2)
        .map(|first_arp_len| (first_arp_len, event_part - first_arp_len))
        .collect::<Vec<_>>();

    let three_melody = generate_3_melody(tonic_note, scale_notes);

    if event_part == 0 {
        return three_melody;
    }

    let (first_arp_len, second_arp_len) = random_from_vec(&mut arp_variations).unwrap();

    let first_arp_melody =
        generate_arpeggio_melody(tonic_note, scale_notes, first_arp_len, bar_time);

    let second_arp_melody =
        generate_arpeggio_melody(tonic_note, scale_notes, second_arp_len, bar_time);

    let mut odd_melody = vec![first_arp_melody, second_arp_melody, three_melody];
    odd_melody.shuffle(&mut rand::thread_rng());
    odd_melody.into_iter().flatten().collect()
}

#[inline]
#[deprecated]
fn generate_3_melody(tonic_note: NoteData, scale_notes: &Vec<Note>) -> Vec<NoteData> {
    let note_velocity = tonic_note.get_velocity();
    let note_length = tonic_note.get_length();
    let note_delay = tonic_note.get_delay();
    let mut note_up_down_stay_probs = (1, 1, 1);

    (1..3)
        .scan(tonic_note, |prev_note, _| {
            let mut note_probs_vec = probs_vec(note_up_down_stay_probs);

            let mut next_notes = match random_from_vec(&mut note_probs_vec).unwrap() {
                DIRECTION_UP => scale_notes
                    .iter()
                    .filter(|&note| note.midi() > prev_note.get_note().midi())
                    .map(|&note| (note, DIRECTION_UP))
                    .collect::<Vec<_>>(),

                DIRECTION_DOWN => scale_notes
                    .iter()
                    .filter(|&note| note.midi() < prev_note.get_note().midi())
                    .map(|&note| (note, DIRECTION_DOWN))
                    .collect::<Vec<_>>(),

                DIRECTION_STAY => vec![(prev_note.get_note(), DIRECTION_STAY)],

                _ => unreachable!(),
            };

            let (next_note, next_note_direction) = random_from_vec(&mut next_notes)?;

            match next_note_direction {
                DIRECTION_UP => {
                    note_up_down_stay_probs.1 += 2;
                    note_up_down_stay_probs.2 += 2;
                }

                DIRECTION_DOWN => {
                    note_up_down_stay_probs.0 += 2;
                    note_up_down_stay_probs.2 += 2;
                }

                DIRECTION_STAY => {
                    note_up_down_stay_probs.0 += 2;
                    note_up_down_stay_probs.1 += 2;
                }

                _ => unreachable!(),
            };

            *prev_note = NoteData::new(next_note, note_velocity, 0, note_length, note_delay);
            Some(*prev_note)
        })
        .collect()
}

#[inline]
#[deprecated]
fn generate_lead_from_analyze(
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
#[deprecated]
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

    let lengths = (4..=32)
        .map(|l| get_bar_ratio(bar_time, l))
        .collect::<Vec<_>>();

    println!("LENGTHS: {:?}", lengths);

    let delays = (0..=16)
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
                    .filter(|&(note, _)| prev_note.midi().abs_diff(note.midi()) <= 12)
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
        .fold(
            vec![generated_lead[0].octave_down().unwrap()],
            |mut acc, &note| {
                match rand::thread_rng().gen::<u32>() % 10 {
                    NOTE_TAKE => acc.push(note),

                    _ => {
                        let last_note = *acc.last().unwrap();
                        *acc.last_mut().unwrap() = last_note.clone_with_new_length(
                            last_note.get_length() + note.get_delay() + note.get_length(),
                        )
                    }
                }

                acc
            },
        )
        .into_iter()
        .map(|note| {
            let mut notes = Chord::new(PitchClass::from(note), Quality::Minor, Number::Seventh)
                .notes()
                .into_iter()
                .filter(|n| n.octave < 5)
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
    velocity: Velocity,
    length: DeltaTime,
    delay: DeltaTime,
) -> NoteData {
    NoteData::new(Note::from(MTNote::new(key, 5)), velocity, 0, length, delay)
}

#[inline]
pub fn randomize_lead(
    generated_lead: Vec<NoteData>,
    scale_notes: &Vec<Note>,
    direction: u32,
) -> Vec<NoteData> {
    let mut diffs = (0..=3).collect::<Vec<_>>();
    let diff = random_from_vec(&mut diffs).unwrap();

    generated_lead
        .into_iter()
        .map(|note| match direction {
            DIRECTION_UP => note.clone_with_new_note(
                get_scaled(note.get_note(), scale_notes, |pos| pos + diff)
                    .unwrap_or(note.get_note()),
            ),

            DIRECTION_DOWN => note.clone_with_new_note(
                get_scaled(note.get_note(), scale_notes, |pos| pos - diff)
                    .unwrap_or(note.get_note()),
            ),

            DIRECTION_STAY => note,

            _ => unreachable!(),
        })
        .map(|note| match scale_notes.contains(&note.get_note()) {
            true => note,

            false => scale_notes
                .iter()
                .map(|&scale_note| (scale_note, (scale_note - note.get_note()).abs()))
                .min_by_key(|(_, dif)| *dif)
                .map(|(scale_note, _)| note.clone_with_new_note(scale_note))
                .unwrap_or(note),
        })
        .collect::<Vec<_>>()
}
