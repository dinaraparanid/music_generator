use crate::{
    midi::{
        bpm::BPM,
        generator::{get_bar_ratio, random_from_vec},
    },
    notes::{note::Note, note_data::*},
};

use rand::Rng;

use rust_music_theory::note::{Note as MTNote, Notes, PitchClass};

pub const DIRECTION_UP: u32 = 0;
pub const DIRECTION_DOWN: u32 = 1;
const DIRECTION_STAY: u32 = 2;

#[inline]
pub fn generate_bpm() -> impl BPM {
    let mut rng = rand::thread_rng();
    rng.gen_range(90..=120)
}

/// Generates number of notes in a synthwave melody.
/// Number is in the set of 4..=8

#[inline]
pub fn generate_synthwave_melody_length() -> usize {
    rand::thread_rng().gen_range(4..=8)
}

/// Tries to get a note in the scale list by the given note
/// and the index transformation function.
/// Transform function accepts current note's index and tries to
/// get note in scale list with transform(index) position

#[inline]
fn get_scaled_from_index<F>(tonic_note: Note, scale_notes: &Vec<Note>, transform: F) -> Option<Note>
where
    F: Fn(usize) -> usize,
{
    let pos = scale_notes.iter().position(|&nt| nt == tonic_note)?;
    scale_notes.get(transform(pos)).map(|&nt| nt)
}

/// Gets random note in scale which is close to the current one.
/// For the Up direction, notes allowed to be in position + 1..=3 from the current one,
/// for the Down direction, only note with position -1 can be used.
/// Direction is chosen as [DIRECTION_UP] or [DIRECTION_DOWN]

#[inline]
fn rand_close_note(tonic_note: Note, scale_notes: &Vec<Note>, up_down_direction: u32) -> Note {
    match up_down_direction {
        DIRECTION_UP => get_scaled_from_index(tonic_note, scale_notes, |pos| {
            let mut notes_dif = (0..=3).collect::<Vec<_>>();
            pos + random_from_vec(&mut notes_dif).unwrap()
        })
        .unwrap_or(tonic_note),

        DIRECTION_DOWN => {
            get_scaled_from_index(tonic_note, scale_notes, |pos| pos - 1).unwrap_or(tonic_note)
        }

        _ => unreachable!(),
    }
}

/// Constructs random MIDI note from the given tonic note.
/// Gets random note in scale which is close to the current one.
/// For the Up direction, notes allowed to be in position +1 or +2 from the current one,
/// for the Down direction, only note with position -1 can be used.
/// Note is always created with volume equal to 80. Direction is chosen randomly

#[inline]
fn take_rand_close_note(
    tonic_note: Note,
    scale_notes: &Vec<Note>,
    start_position: u32,
    len: DeltaTime,
    bpm: impl BPM,
    delay_ratio: u32,
) -> NoteData {
    NoteData::new(
        rand_close_note(tonic_note, scale_notes, rand::thread_rng().gen::<u32>() % 2),
        100,
        get_bar_ratio(bpm, start_position),
        len,
        get_bar_ratio(bpm, delay_ratio),
    )
}

/// Generates the lead melody from the given Key, scale and BPM.
/// For the lead melody, next algorithm is used:
/// Separates bar onto 16 parts, then for each
/// position either puts note with length 1/16 of bar,
/// or skips it. Only single pause with 2/16 length is allowed
/// Pause with 3/16 and greater are not allowed.
/// Chosen notes are close to the key and lie on scale

#[inline]
pub fn generate_lead_melody_with_bpm_and_len(
    key: PitchClass,
    scale_notes: &Vec<Note>,
    bpm: impl BPM,
    lead_len: usize,
) -> Vec<NoteData> {
    let mut even_lens = vec![1, 2];
    let mut odd_lens = vec![1, 3];

    let tonic_len = random_from_vec(&mut even_lens).unwrap();
    let tonic_time = get_bar_ratio(bpm, tonic_len);
    let tonic_note = generate_tonic_lead_note(key, 100, tonic_time, 0);

    let mut lead = vec![tonic_note];
    let mut cur_pos = tonic_len;

    while cur_pos < 16 && lead.len() < lead_len {
        let prev_note = *lead.last().unwrap();

        match cur_pos % 4 {
            0 => {
                let len = random_from_vec(&mut even_lens).unwrap();
                let note_time = get_bar_ratio(bpm, len);

                let next_note_mb =
                    next_note_or_skip(bpm, note_time, tonic_note, scale_notes, prev_note, cur_pos);

                match next_note_mb {
                    None => cur_pos += 1,

                    Some(next_note) => {
                        cur_pos += len;
                        lead.push(next_note)
                    }
                }
            }

            1 => {
                let len = random_from_vec(&mut odd_lens).unwrap();
                let note_time = get_bar_ratio(bpm, len);

                let next_note_mb =
                    next_note_or_skip(bpm, note_time, tonic_note, scale_notes, prev_note, cur_pos);

                match next_note_mb {
                    None => cur_pos += 1,

                    Some(next_note) => {
                        cur_pos += len;
                        lead.push(next_note)
                    }
                }
            }

            2 => {
                let len = random_from_vec(&mut even_lens).unwrap();
                let note_time = get_bar_ratio(bpm, len);

                let next_note_mb =
                    next_note_or_skip(bpm, note_time, tonic_note, scale_notes, prev_note, cur_pos);

                match next_note_mb {
                    None => cur_pos += 1,

                    Some(next_note) => {
                        cur_pos += len;
                        lead.push(next_note)
                    }
                }
            }

            3 => {
                let note_time = get_bar_ratio(bpm, 1);

                let next_note_mb =
                    next_note_or_skip(bpm, note_time, tonic_note, scale_notes, prev_note, cur_pos);

                cur_pos += 1;

                if let Some(next_note) = next_note_mb {
                    lead.push(next_note)
                }
            }

            _ => unreachable!(),
        }
    }

    lead
}

#[inline]
fn next_note_or_skip(
    bpm: impl BPM,
    len: DeltaTime,
    tonic_note: NoteData,
    scale_notes: &Vec<Note>,
    prev_note: NoteData,
    position: DeltaTime,
) -> Option<NoteData> {
    let bar_time = bpm.bar_time().as_millis();

    let prev_note_start = prev_note.start() as f64 * 16.0 / bar_time as f64;
    let prev_note_start = prev_note_start.round() as DeltaTime;

    let prev_note_len = prev_note.length() as f64 * 16.0 / bar_time as f64;
    let prev_note_len = prev_note_len.round() as DeltaTime;

    let cur_delay = position - prev_note_start - prev_note_len;
    let next = || next_note(bpm, len, tonic_note, scale_notes, position, cur_delay);

    match cur_delay {
        3 => Some(next()),

        _ => match rand::thread_rng().gen_bool(0.75) {
            true => Some(next()),
            false => None,
        },
    }
}

#[inline]
fn next_note(
    bpm: impl BPM,
    len: DeltaTime,
    tonic_note: NoteData,
    scale_notes: &Vec<Note>,
    position: DeltaTime,
    cur_delay: DeltaTime,
) -> NoteData {
    take_rand_close_note(
        tonic_note.note(),
        scale_notes,
        position,
        len,
        bpm,
        cur_delay,
    )
}

/// Generates tonic lead note with the given key.
/// All produced notes lie on the 4-th octave

#[inline]
fn generate_tonic_lead_note(
    key: PitchClass,
    velocity: Velocity,
    length: DeltaTime,
    delay: DeltaTime,
) -> NoteData {
    NoteData::new(Note::from(MTNote::new(key, 5)), velocity, 0, length, delay)
}

/// Fixes note's pitch to lie on the scale.
/// Note with closest pitch on scale is chosen and produced.
/// If note is already on the scale, returns the same note

#[inline]
fn fix_note_to_closest_scaled(note: NoteData, scale_notes: &Vec<Note>) -> NoteData {
    match scale_notes.contains(&note.note()) {
        true => note,
        false => scale_notes
            .iter()
            .map(|&scale_note| (scale_note, (scale_note - note.note()).abs()))
            .min_by_key(|(_, dif)| *dif)
            .map(|(scale_note, _)| note.clone_with_new_note(scale_note))
            .unwrap_or(note),
    }
}

#[inline]
fn randomize_note_with_given_diff(
    note: NoteData,
    scale_notes: &Vec<Note>,
    direction: u32,
    diff: usize,
) -> NoteData {
    let note = match direction {
        DIRECTION_UP => note.clone_with_new_note(
            get_scaled_from_index(note.note(), scale_notes, |pos| pos + diff)
                .unwrap_or(note.note()),
        ),

        DIRECTION_DOWN => note.clone_with_new_note(
            get_scaled_from_index(note.note(), scale_notes, |pos| pos - 1).unwrap_or(note.note()),
        ),

        _ => unreachable!(),
    };

    match scale_notes.contains(&note.note()) {
        true => note,
        false => fix_note_to_closest_scaled(note, scale_notes),
    }
}

/// Randomizes note by increasing or decreasing pitch
/// (up or down by 0..=3 notes from scale).
/// All produced notes lie on the scale

#[inline]
pub fn randomize_note(note: NoteData, scale_notes: &Vec<Note>) -> NoteData {
    let mut diffs = (0..=6).collect::<Vec<_>>();
    let diff = random_from_vec(&mut diffs).unwrap();
    let direction = random_from_vec(&mut vec![DIRECTION_UP, DIRECTION_DOWN]).unwrap();
    randomize_note_with_given_diff(note, scale_notes, direction, diff)
}

/// Randomizes lead by increasing or decreasing
/// pitches for all notes in the lead
/// (up or down by 0..=3 notes from scale).
/// All produced notes lie on the scale.
/// All notes from the lead either upped or downed altogether

#[inline]
pub fn randomize_lead(
    generated_lead: Vec<NoteData>,
    scale_notes: &Vec<Note>,
    direction: u32,
) -> Vec<NoteData> {
    let mut diffs = (0..=2).collect::<Vec<_>>();
    let diff = random_from_vec(&mut diffs).unwrap();

    generated_lead
        .into_iter()
        .map(|note| randomize_note_with_given_diff(note, scale_notes, direction, diff))
        .collect::<Vec<_>>()
}
