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

pub const DIRECTION_UP: u32 = 0;
pub const DIRECTION_DOWN: u32 = 1;
const DIRECTION_STAY: u32 = 2;

const NOTE_TAKE: u32 = 0;

/// Generates random key from
/// the set of [D#, E, F, F#, G, G#]

#[inline]
pub fn generate_key() -> PitchClass {
    let mut keys = vec![
        PitchClass::D, // +
        PitchClass::E,
        PitchClass::F,
        PitchClass::G,
        PitchClass::A, // +
    ];

    random_from_vec(&mut keys).unwrap()
}

#[inline]
pub fn generate_bpm() -> impl BPM {
    let mut rng = rand::thread_rng();
    rng.gen_range(90..=110)
}

/// Generates number of notes in a melody.
/// Number is in the set of 3..=12
///
/// # Deprecated
/// Currently, melody is generated as an arpeggio,
/// where number of notes is decided in the generation process.
/// Was used in previous solutions, may be used in the future

#[inline]
#[deprecated]
fn generate_melody_length() -> u32 {
    // even: arpeggio
    // odd: shuffle $ arp + 3 + arp
    rand::thread_rng().gen_range(3..=12)
}

/// Constructs probabilities vector from the triple,
/// where all elements are probabilities of one
/// of the events from the triple
///
/// # Deprecated
/// Probabilities were used in Markov chain based solutions.
/// They still may be used in the future
///
/// # Example
/// ```
/// use music_generator::midi::generator::generator::probs_vec;
/// let triple = (2, 3, 4);
/// assert_eq!(probs_vec(triple), vec![0, 0, 1, 1, 1, 2, 2, 2, 2])
/// ```

#[inline]
#[deprecated]
pub fn probs_vec(up_down_stay_probs: (usize, usize, usize)) -> Vec<u32> {
    let mut probs_vec = vec![];
    probs_vec.extend(vec![0; up_down_stay_probs.0]);
    probs_vec.extend(vec![1; up_down_stay_probs.1]);
    probs_vec.extend(vec![2; up_down_stay_probs.2]);
    probs_vec
}

/// Tries to get a note in the scale list by the given note.
/// Change function accepts current note's index and tries to
/// get note in scale list with change(index) position

#[inline]
fn get_scaled<F>(tonic_note: Note, scale_notes: &Vec<Note>, change: F) -> Option<Note>
where
    F: Fn(usize) -> usize,
{
    let pos = scale_notes.iter().position(|&nt| nt == tonic_note)?;
    scale_notes.get(change(pos)).map(|&nt| nt)
}

/// Gets random note in scale which is close to the current one.
/// For the Up direction, notes allowed to be in position +1 or +2 from the current one,
/// for the Down direction, only note with position -1 can be used.
/// Direction is chosen as [DIRECTION_UP] or [DIRECTION_DOWN]

#[inline]
fn rand_close_note(tonic_note: Note, scale_notes: &Vec<Note>, up_down_direction: u32) -> Note {
    match up_down_direction {
        DIRECTION_UP => get_scaled(tonic_note, scale_notes, |pos| {
            let mut notes_dif = (0..=2).collect::<Vec<_>>();
            pos + random_from_vec(&mut notes_dif).unwrap()
        })
        .unwrap_or(tonic_note),

        DIRECTION_DOWN => get_scaled(tonic_note, scale_notes, |pos| pos - 1).unwrap_or(tonic_note),

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
    bar_time: DeltaTime,
    delay_ratio: u32,
) -> NoteData {
    NoteData::new(
        rand_close_note(tonic_note, scale_notes, rand::thread_rng().gen::<u32>() % 2),
        45,
        start_position,
        len,
        get_bar_ratio(bar_time, delay_ratio),
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
pub fn generate_lead_melody_with_bpm(
    key: PitchClass,
    scale_notes: &Vec<Note>,
    bpm: impl BPM,
) -> Vec<NoteData> {
    let bar_time = bpm.get_bar_time().as_millis() as DeltaTime;
    let single_len = get_bar_ratio(bar_time, 4);
    let tonic_note = generate_tonic_lead_note(key, 45, single_len, 0);

    let mut generated_lead = (4..64).step_by(4).fold(vec![tonic_note], |lead, position| {
        push_next_note_or_skip(
            bar_time,
            single_len,
            tonic_note,
            scale_notes,
            lead,
            position,
        )
    });

    generated_lead.push(tonic_note);
    generated_lead
}

#[inline]
fn push_next_note_or_skip(
    bar_time: DeltaTime,
    single_len: DeltaTime,
    tonic_note: NoteData,
    scale_notes: &Vec<Note>,
    mut lead: Vec<NoteData>,
    position: DeltaTime,
) -> Vec<NoteData> {
    let mut rng = rand::thread_rng();
    let prev_note = *lead.last().unwrap();
    let cur_delay = position - prev_note.get_start() - 4;

    let mut push_next = || {
        push_next_note(
            bar_time,
            single_len,
            tonic_note,
            scale_notes,
            &mut lead,
            position,
            cur_delay,
        )
    };

    match cur_delay {
        12 => push_next(),

        _ => {
            if rng.gen_bool(0.25) {
                push_next()
            }
        }
    };

    lead.into_iter().collect()
}

#[inline]
fn push_next_note(
    bar_time: DeltaTime,
    single_len: DeltaTime,
    tonic_note: NoteData,
    scale_notes: &Vec<Note>,
    lead: &mut Vec<NoteData>,
    position: DeltaTime,
    cur_delay: DeltaTime,
) {
    lead.push(take_rand_close_note(
        tonic_note.get_note(),
        scale_notes,
        position,
        single_len,
        bar_time,
        cur_delay,
    ))
}

/// Generates both BPM (95..=115) and the lead melody.
/// For the lead melody, next algorithm is used:
/// Separates bar onto 16 parts, then for each
/// position either puts note with length 1/16 of bar,
/// or skips it. Only single pause with 2/16 length is allowed
/// Pause with 3/16 and greater are not allowed.
/// Chosen notes are close to the key and lie on scale

#[inline]
pub fn generate_lead_melody(key: PitchClass, scale_notes: &Vec<Note>) -> (impl BPM, Vec<NoteData>) {
    let mut rng = rand::thread_rng();
    let bpm = rng.gen_range(95..=115);
    (bpm, generate_lead_melody_with_bpm(key, scale_notes, bpm))
}

/// Generates BPM, melody len and the melody itself with even/odd algorithm.
/// If melody length is even, generates either fully arpeggio melody,
/// or if melody is dividable by 3, creates 3x melody. For odd melodies
/// creates arpeggio + 3 + arpeggio melody, than shuffles it.
///
/// # Deprecated
/// Algorithm have produced too controversial and unstable results.
/// Currently, abandoned, but may be integrated in the future

#[inline]
#[deprecated]
fn generate_even_odd_melody(key: PitchClass, scale_notes: &Vec<Note>) -> (impl BPM, Vec<NoteData>) {
    let mut rng = rand::thread_rng();
    let bpm = rng.gen_range(95..=115);

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
        80,
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

/// Generates event melody from the given tonic note and the melody length.
/// Generates either fully arpeggio melody,
/// or if melody is dividable by 3, creates 3x melody.
///
/// # Deprecated
/// Algorithm have produced too controversial and unstable results.
/// Currently, abandoned, but may be integrated in the future

#[inline]
#[deprecated]
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
        .map(|x| x % 2)
        .map(|dir| randomize_lead(melody_3.clone(), scale_notes, dir))
        .flatten()
        .collect()
}

/// Generates repeating arpeggio melody by the given tonic note.
/// Generated notes are placed closely to the tonic and match the scale
///
/// # Deprecated
/// Algorithm have produced too controversial and unstable results.
/// Currently, abandoned, but may be integrated in the future

#[inline]
#[deprecated]
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
            .map(|arp| (arp, arp.notes_from_tonic(tonic_note, scale_notes)))
            .skip_while(|(_, part)| part.is_none())
            .next()
            .map(|(arp, part)| (arp, part.unwrap()))
            .unwrap();

        let _ = last_arp.insert(arp);

        if i != 0 {
            let first_note = part[0];
            part[0] = first_note.clone_with_new_delay(random_from_vec(&mut delays).unwrap())
        }

        arp_lead.extend(part);
        arp_lead
    })
}

/// Generates melody with and odd length.
/// Creates arpeggio + 3 + arpeggio melody, than shuffles it.
/// Melody with length 3, as well as arpeggios,
/// are generated randomly by the given tonic note
///
/// # Deprecated
/// Algorithm have produced too controversial and unstable results.
/// Currently, abandoned, but may be integrated in the future

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

/// Generates melody with length 3. Next algorithm is used for generation:
/// First note is chosen randomly, then Markov chains are applied to generate
/// pitch, length and the delay. If note with pitch upper than tonic is generated,
/// then probability for the note with same/lower pitch to be used is increased
/// (and that is true for all 3 parameters)
///
/// # Deprecated
/// Algorithm have produced too controversial and unstable results.
/// Currently, abandoned, but may be integrated in the future

#[inline]
#[deprecated]
fn generate_3_melody(tonic_note: NoteData, scale_notes: &Vec<Note>) -> Vec<NoteData> {
    let note_velocity = tonic_note.get_velocity();
    let note_length = tonic_note.get_length();
    let note_delay = tonic_note.get_delay();
    let mut note_up_down_stay_probs = (1, 1, 1);

    // Constructs lead, starting from the tonic note

    (1..3)
        .scan(tonic_note, |prev_note, _| {
            let mut note_probs_vec = probs_vec(note_up_down_stay_probs);

            let mut next_notes = match random_from_vec(&mut note_probs_vec).unwrap() {
                // If up, picks note with upper pitch that matches scale
                DIRECTION_UP => scale_notes
                    .iter()
                    .filter(|&note| note.midi() > prev_note.get_note().midi())
                    .map(|&note| (note, DIRECTION_UP))
                    .collect::<Vec<_>>(),

                // If down, picks note with lower pitch that matches scale
                DIRECTION_DOWN => scale_notes
                    .iter()
                    .filter(|&note| note.midi() < prev_note.get_note().midi())
                    .map(|&note| (note, DIRECTION_DOWN))
                    .collect::<Vec<_>>(),

                // If stay, produces the same note
                DIRECTION_STAY => vec![(prev_note.get_note(), DIRECTION_STAY)],

                _ => unreachable!(),
            };

            let (next_note, next_note_direction) = random_from_vec(&mut next_notes)?;

            // Updates probabilities according to the event:
            // if `event 1` was chosen, increments all other events

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

/// Generates lead from the parsed MIDI notes, using Markov chains.
/// Markov chains are used to get the next note given the current one.
///
/// # Deprecated
/// There were a lot of issues with parsing lead notes,
/// because it is not easy to contrast lead and harmony in the single MIDI file.
/// As a result, melodies were to random and the result is too unstable.
/// Currently, abandoned, but may be integrated in the future

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

/// Tries to generate appropriate melody with parsed notes using Markov chains.
/// Produces both BPM and the generated lead melody from the analyzed
/// dataset of notes and their probabilities. First note is taken randomly,
/// then next notes are taken from the analyzed dataset.
///
/// # Deprecated
/// There were a lot of issues with parsing lead notes,
/// because it is not easy to contrast lead and harmony in the single MIDI file.
/// As a result, melodies were to random and the result is too unstable.
/// Currently, abandoned, but may be integrated in the future

#[inline]
#[deprecated]
fn try_generate_lead_from_analyze(
    scale_notes: &Vec<Note>,
    analyzed_notes: &AnalyzedNotes,
    notes_to_data: &mut HashMap<Note, Vec<NoteData>>,
) -> Option<(impl BPM, Vec<NoteData>)> {
    let mut rng = rand::thread_rng();
    let bpm = rng.gen_range(95..=115);

    let bar_time = bpm.get_bar_time().as_millis() as DeltaTime;
    let melody_len = generate_melody_length();

    // First note is taken randomly from the dataset
    let mut first_notes = analyzed_notes
        .keys()
        .filter(|&note| scale_notes.contains(note))
        .map(|&note| note)
        .collect::<Vec<_>>();

    first_notes.shuffle(&mut rng);

    let first_note = *first_notes.iter().next()?;
    let first_note_datas = notes_to_data.get_mut(&first_note)?;
    first_note_datas.shuffle(&mut rng);

    // Allowed lengths for notes
    let lengths = (4..=32)
        .map(|l| get_bar_ratio(bar_time, l))
        .collect::<Vec<_>>();

    println!("LENGTHS: {:?}", lengths);

    // Allowed delays for notes
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
                // Getting second note from the previous one
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
                // Taking random MIDI note from the produced note
                let mut rng = rand::thread_rng();
                let mut second_note_datas = notes_to_data.get(&second_note)?.clone();
                second_note_datas.shuffle(&mut rng);

                let next_note = second_note_datas.into_iter().next()?;
                Some(next_note)
            })
            .take_while(|note_opt| note_opt.is_some()) // generating melody while we can
            .filter_map(|note_opt| note_opt.map(|note| fixed_to_tempo(note, &lengths, &delays))), // fixing everything to match tempo
    );

    Some((bpm, generated_lead))
}

type ChordProgression = HashMap<String, (Quality, Number)>;

/// Generates harmony from the given lead.
/// Chords in harmony are generated according
/// to the Aeolian mode and Melodic Minor scale:
/// 1. Minor triad (tonic note)
/// 2. Diminished triad (T)
/// 3. Augmented triad (TS)
/// 4. Minor triad (TST)
/// 5. Major triad (TSTT)
/// 6. Minor triad (TSTTS)
/// 7. Dominant seventh (TSTTSTS)
///
/// However, harmony is composed to match the scale,
/// not chords, so fixing algorithm is also used in generation process.
///
/// Notes for harmonies are generated with the next algorithm:
/// First note copies the first note in the lead,
/// then it produces a fixed chord according to the picked note,
/// then it either picks the next note (low probability) or
/// extends current chord with the length + delay of the next note

#[inline]
pub fn generate_harmony_from_lead(
    key: PitchClass,
    generated_lead: &Vec<NoteData>,
    scale_notes: &Vec<Note>,
) -> Vec<ChordData> {
    // Map of chords for Aeolian mode
    let aeolian_chord_progression = aeolian_chord_progression(key);

    generated_lead
        .iter()
        .skip(1)
        .fold(vec![generated_lead[0]], new_chord_note_or_extend)
        .into_iter()
        .map(|note| build_chord_from(note, &aeolian_chord_progression, scale_notes))
        .collect()
}

/// Map of chords for Aeolian mode

#[inline]
fn aeolian_chord_progression(key: PitchClass) -> ChordProgression {
    let tonic_note = Note::from(MTNote::new(key, 5));

    HashMap::from([
        (format!("{}", key), (Quality::Minor, Number::Triad)),
        (
            format!("{}", PitchClass::from(tonic_note.up(5).unwrap())),
            (Quality::Minor, Number::Triad),
        ),
        (
            format!("{}", PitchClass::from(tonic_note.up(7).unwrap())),
            (Quality::Minor, Number::Triad),
        ),
        (
            format!("{}", PitchClass::from(tonic_note.up(8).unwrap())),
            (Quality::Major, Number::Triad),
        ),
        (
            format!("{}", PitchClass::from(tonic_note.up(10).unwrap())),
            (Quality::Major, Number::Triad),
        ),
        (
            format!("{}", PitchClass::from(tonic_note.up(14).unwrap())),
            (Quality::Minor, Number::Triad),
        ),
        (
            format!("{}", PitchClass::from(tonic_note.up(15).unwrap())),
            (Quality::Diminished, Number::Triad),
        ),
    ])
}

/// With probability 1 : 2 picks the note or extends chords

#[inline]
fn new_chord_note_or_extend(mut acc: Vec<NoteData>, note: &NoteData) -> Vec<NoteData> {
    match rand::thread_rng().gen::<u32>() % 3 {
        NOTE_TAKE => acc.push(*note),

        _ => {
            // Extend current chord from the current note
            let last_note = *acc.last().unwrap();

            *acc.last_mut().unwrap() = last_note.clone_with_new_length(
                last_note.get_length() + note.get_delay() + note.get_length(),
            )
        }
    }

    acc.into_iter().collect()
}

/// Constructs and fixes chords from the picked notes

#[inline]
fn build_chord_from(
    note: NoteData,
    aeolian_chord_progression: &ChordProgression,
    scale_notes: &Vec<Note>,
) -> Vec<NoteData> {
    let (quality, number) = *aeolian_chord_progression
        .get(&format!("{}", PitchClass::from(note.get_note())))
        .unwrap_or(&(Quality::Minor, Number::Triad));

    let mut notes = Chord::new(PitchClass::from(note), quality, number)
        .notes()
        .into_iter()
        .map(Note::from)
        .map(|nt| nt.octave_down().unwrap())
        .map(|nt| note.clone_with_new_note(nt).clone_with_velocity(35))
        .map(|note| fix_note_to_closest_scaled(note, scale_notes))
        .collect::<Vec<_>>();

    println!("CHORD: {:?}", notes);

    // Removes the delays from the notes in the chord,
    // so all notes will play altogether

    notes.iter_mut().skip(1).for_each(|n| {
        let zero_delay_note = n.clone_with_new_delay(0);
        *n = zero_delay_note;
    });

    notes
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
    NoteData::new(Note::from(MTNote::new(key, 4)), velocity, 0, length, delay)
}

/// Fixes note's pitch to lie on the scale.
/// Note with closest pitch on scale is chosen and produced.
/// If note is already on the scale, returns the same note

#[inline]
fn fix_note_to_closest_scaled(note: NoteData, scale_notes: &Vec<Note>) -> NoteData {
    match scale_notes.contains(&note.get_note()) {
        true => note,
        false => scale_notes
            .iter()
            .map(|&scale_note| (scale_note, (scale_note - note.get_note()).abs()))
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
            get_scaled(note.get_note(), scale_notes, |pos| pos + diff).unwrap_or(note.get_note()),
        ),

        DIRECTION_DOWN => note.clone_with_new_note(
            get_scaled(note.get_note(), scale_notes, |pos| pos - 1).unwrap_or(note.get_note()),
        ),

        _ => unreachable!(),
    };

    match scale_notes.contains(&note.get_note()) {
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
    let mut diffs = (1..=3).collect::<Vec<_>>();
    let diff = random_from_vec(&mut diffs).unwrap();

    generated_lead
        .into_iter()
        .map(|note| randomize_note_with_given_diff(note, scale_notes, direction, diff))
        .collect::<Vec<_>>()
}
