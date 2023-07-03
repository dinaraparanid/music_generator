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
    /// Generates random arpeggio

    #[inline]
    pub fn random_arp() -> Self {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..=6);
        Self::iter().skip(index).next().unwrap()
    }

    /// Constructs notes based on tonic
    /// and current arpeggio state.
    /// Closest to tonic notes that
    /// match scale are taken

    #[inline]
    pub fn notes_from_tonic(
        &self,
        tonic_note: NoteData,
        scale_notes: &Vec<Note>,
    ) -> Option<Vec<NoteData>> {
        match self {
            ArpeggioTypes::SameSame => Some(vec![tonic_note; 2]),
            ArpeggioTypes::SameUp => notes_from_tonic(tonic_note, scale_notes, true, true, true),
            ArpeggioTypes::SameDown => notes_from_tonic(tonic_note, scale_notes, true, true, false),

            ArpeggioTypes::UpSame => notes_from_tonic(tonic_note, scale_notes, true, false, true),
            ArpeggioTypes::UpUp => notes_from_tonic(tonic_note, scale_notes, false, false, true),

            ArpeggioTypes::DownSame => {
                notes_from_tonic(tonic_note, scale_notes, true, false, false)
            }

            ArpeggioTypes::DownDown => {
                notes_from_tonic(tonic_note, scale_notes, false, false, false)
            }
        }
    }
}

/// Get closest note to the given one

#[inline]
fn get_closest_note(note: Note, scale_notes: &Vec<Note>, is_up: bool) -> Option<Note> {
    let filter = if is_up { up_filter } else { down_filter };

    let it = scale_notes
        .iter()
        .filter(|&&nt| filter(note, nt))
        .map(|&nt| nt);

    if is_up {
        it.min_by_key(|nt| nt.midi())
    } else {
        it.max_by_key(|nt| nt.midi())
    }
}

/// Constructs vector of 2 arpeggio notes by the given tonic note.
/// In case if tonic is note required (UpUp or DownDown),
/// position of tonic does note matter.

#[inline]
fn notes_from_tonic(
    tonic_note: NoteData,
    scale_notes: &Vec<Note>,
    is_tonic_required: bool,
    is_tonic_first: bool,
    is_up: bool,
) -> Option<Vec<NoteData>> {
    let second_note = get_closest_note(tonic_note.get_note(), scale_notes, is_up)?;
    let second_note = tonic_note.clone_with_new_note(second_note);

    if !is_tonic_required {
        return Some(vec![second_note; 2]);
    }

    Some(match is_tonic_first {
        true => vec![tonic_note, second_note],
        false => vec![second_note, tonic_note],
    })
}

/// Filters all notes with a higher pitch

#[inline]
fn up_filter(tonic: Note, other: Note) -> bool {
    tonic.midi() < other.midi()
}

/// Filters all notes with a lower pitch

#[inline]
fn down_filter(tonic: Note, other: Note) -> bool {
    tonic.midi() > other.midi()
}
