use crate::{
    midi::generator::{
        generator::{generate_harmony_from_lead, randomize_lead},
        randomize_with_pi,
    },
    notes::{note::Note, note_data::*, ChordData},
};

use ghakuf::messages::Message;
use rust_music_theory::note::PitchClass;

/// Constructs vector of ON and OFF MIDI events.
/// ON event is happened after the note's delay,
/// OFF event is happened after the note's end
/// (when length is reached)

#[inline]
pub fn compose_note(note: NoteData) -> Vec<Message> {
    vec![
        note.into_on_midi_event(note.get_delay()),
        note.into_off_midi_event(note.get_length()),
    ]
}

/// Constructs vector of ON/OFF MIDI events for all notes in a chord.
/// It is assumed that all notes in a chord have the same delay.
/// However, notes length may differ, so OFF events are sorted by the length

#[inline]
pub fn compose_chord(mut chord: ChordData) -> Vec<Message> {
    // Constructing on events

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

/// Generates harmony from the given lead.
/// Then constructs vectors of MIDI messages
/// from both lead and generated harmony

#[inline]
pub fn compose_from_generated<L, H>(
    key: PitchClass,
    generated_lead: Vec<NoteData>,
    scale_notes: &Vec<Note>,
    lead_composer: L,
    harmony_composer: H,
) -> (Vec<Message>, Vec<Message>)
where
    L: Fn(NoteData) -> Vec<Message>,
    H: Fn(ChordData) -> Vec<Message>,
{
    let (leads, harmonies): (Vec<Vec<Vec<Message>>>, Vec<Vec<Vec<Message>>>) = randomize_with_pi(4)
        .into_iter()
        .map(|x| x % 2)
        .map(|direction| randomize_lead(generated_lead.clone(), scale_notes, direction))
        .map(|lead| {
            let harmony = generate_harmony_from_lead(key, &lead, scale_notes);
            (lead, harmony)
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

    let leads = leads.into_iter().flatten().flatten().collect::<Vec<_>>();

    let harmonies = harmonies
        .into_iter()
        .flatten()
        .flatten()
        .collect::<Vec<_>>();

    (
        vec![leads.clone(); 2].into_iter().flatten().collect(),
        vec![harmonies.clone(); 2].into_iter().flatten().collect(),
    )
}
