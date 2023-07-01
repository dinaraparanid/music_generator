use crate::{
    midi::generator::{
        generator::{generate_harmony_from_lead, randomize_lead},
        get_bar_ratio, randomize_with_pi,
    },
    notes::{note::Note, note_data::*, ChordData},
};

use ghakuf::messages::Message;
use itertools::Itertools;
use rand::{prelude::SliceRandom, Rng};

#[inline]
pub fn compose_note(note: NoteData) -> Vec<Message> {
    println!("Note: {:?}", note);

    vec![
        note.into_on_midi_event(note.get_delay()),
        note.into_off_midi_event(note.get_length()),
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

    let mut delays = (0..20)
        .map(|_| rng.gen_range(16..=32))
        .unique()
        .collect::<Vec<_>>();

    delays.shuffle(&mut rng);

    let delay = generated_lead[0].get_delay() + get_bar_ratio(bar_time, *delays.first().unwrap());
    println!("DELAY BETWEEN PARTS: {}", delay);

    let (leads, harmonies): (Vec<Vec<Vec<Message>>>, Vec<Vec<Vec<Message>>>) = randomize_with_pi(4)
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
