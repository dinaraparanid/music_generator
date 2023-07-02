use crate::notes::{note::Note, note_data::NoteData};
use rand::Rng;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Copy, Clone, Eq, PartialEq, Debug)]
pub enum ArpeggioTypes {
    SameSame,
    SameUp,
    SameDown,
    UpSame,
    UpUp,
    DownSame,
    DownDown,
}

impl ArpeggioTypes {
    #[inline]
    pub fn random_arp() -> Self {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..=6);
        Self::iter().skip(index).next().unwrap()
    }

    #[inline]
    pub fn next_part(
        &self,
        tonic_note: NoteData,
        scale_notes: &Vec<Note>,
    ) -> Option<Vec<NoteData>> {
        match self {
            ArpeggioTypes::SameSame => Some(vec![tonic_note; 2]),

            ArpeggioTypes::SameUp => {
                notes_from_tonic(tonic_note, scale_notes, true, true, true, up_filter)
            }

            ArpeggioTypes::SameDown => {
                notes_from_tonic(tonic_note, scale_notes, true, true, false, down_filter)
            }

            ArpeggioTypes::UpSame => {
                notes_from_tonic(tonic_note, scale_notes, true, false, true, up_filter)
            }

            ArpeggioTypes::UpUp => {
                notes_from_tonic(tonic_note, scale_notes, false, false, true, up_filter)
            }

            ArpeggioTypes::DownSame => {
                notes_from_tonic(tonic_note, scale_notes, true, false, false, down_filter)
            }

            ArpeggioTypes::DownDown => {
                notes_from_tonic(tonic_note, scale_notes, false, false, false, down_filter)
            }
        }
    }
}

#[inline]
pub fn get_closest_note<C: Fn(Note, Note) -> bool>(
    note: Note,
    scale_notes: &Vec<Note>,
    is_up: bool,
    cmp: C,
) -> Option<Note> {
    let it = scale_notes
        .iter()
        .filter(|&&nt| cmp(note, nt))
        .map(|&nt| nt);

    if is_up {
        it.min_by_key(|nt| nt.midi())
    } else {
        it.max_by_key(|nt| nt.midi())
    }
}

#[inline]
fn notes_from_tonic<C: Fn(Note, Note) -> bool>(
    tonic_note: NoteData,
    scale_notes: &Vec<Note>,
    is_tonic_required: bool,
    is_tonic_first: bool,
    is_up: bool,
    cmp: C,
) -> Option<Vec<NoteData>> {
    let second_note = get_closest_note(tonic_note.get_note(), scale_notes, is_up, cmp)?;
    let second_note = tonic_note.clone_with_new_note(second_note);

    if !is_tonic_required {
        return Some(vec![second_note; 2]);
    }

    Some(match is_tonic_first {
        true => vec![tonic_note, second_note],
        false => vec![second_note, tonic_note],
    })
}

#[inline]
pub fn up_filter(tonic: Note, other: Note) -> bool {
    tonic.midi() < other.midi()
}

#[inline]
pub fn down_filter(tonic: Note, other: Note) -> bool {
    tonic.midi() > other.midi()
}
