use crate::notes::note_data::*;
use astro_float::{ctx::Context, Consts, RoundingMode};
use rand::{prelude::SliceRandom, Rng};

#[deprecated]
pub mod analyzer;

pub mod arpeggio_types;
pub mod composer;
pub mod generator;

/// Gets random element from the vector.
/// If vector is empty, returns
///
/// # Example
/// ```
/// use music_generator::midi::generator::random_from_vec;
///
/// let mut v = vec![1];
/// assert_eq!(random_from_vec(&mut v), Some(1));
///
/// let mut empty = Vec::<u32>::new();
/// assert_eq!(random_from_vec(&mut empty), None)
/// ```

#[inline]
pub fn random_from_vec<T: Clone>(data: &mut Vec<T>) -> Option<T> {
    data.shuffle(&mut rand::thread_rng());
    data.first().map(|t| t.clone())
}

/// Gets digits from PI starting from
/// the given position in mantissa
///
/// # Example
/// ```
/// use music_generator::midi::generator::pi_numbers;
/// assert_eq!(pi_numbers(0, 2), vec![1, 4])
/// ```

#[inline]
pub fn pi_numbers(from: usize, len: usize) -> Vec<u32> {
    let pi = Context::new(1024, RoundingMode::ToEven, Consts::new().unwrap()).const_pi();
    let pi = format!("{}", pi);

    pi.chars()
        .skip(2 + from)
        .take(len)
        .map(|x| x.to_digit(10).unwrap())
        .collect()
}

/// Gets random sequence of digits
/// from PI with a given length

#[inline]
fn randomize_with_pi(len: usize) -> Vec<u32> {
    pi_numbers(rand::thread_rng().gen::<usize>() % 50, len)
        .into_iter()
        .collect()
}

/// Gets time for a given ratio in terms of bar's time.
/// Note that bar is divided into 16 parts

#[inline]
fn get_bar_ratio(bar_time: DeltaTime, ratio: u32) -> DeltaTime {
    bar_time * ratio / 16
}

trait FixedToTempoNoteData {
    fn with_fixed_to_tempo_length(self, lengths: &Vec<DeltaTime>) -> Self;
    fn with_fixed_to_tempo_delay(self, delays: &Vec<DeltaTime>) -> Self;
}

impl FixedToTempoNoteData for NoteData {
    #[inline]
    fn with_fixed_to_tempo_length(self, lengths: &Vec<DeltaTime>) -> Self {
        self.clone_with_new_length(
            lengths
                .iter()
                .map(|&len| (len, (len as i32 - self.length() as i32).abs()))
                .min_by_key(|(_, dif)| *dif)
                .map(|(len, _)| len + randomize_with_pi(1)[0])
                .unwrap_or(self.length()),
        )
    }

    #[inline]
    fn with_fixed_to_tempo_delay(self, delays: &Vec<DeltaTime>) -> Self {
        self.clone_with_new_delay(
            delays
                .iter()
                .map(|&delay| (delay, (delay as i32 - self.delay() as i32).abs()))
                .min_by_key(|(_, dif)| *dif)
                .map(|(delay, _)| delay + randomize_with_pi(1)[0])
                .unwrap_or(self.delay()),
        )
    }
}

/// Fixing parsed MIDI notes to match
/// the  appropriate lengths and delays,
/// generated from the BPM

#[inline]
fn fixed_to_tempo(note: NoteData, lengths: &Vec<DeltaTime>, delays: &Vec<DeltaTime>) -> NoteData {
    let fixed_velocity = std::cmp::min(60 + note.velocity(), 100);

    note.with_fixed_to_tempo_length(lengths)
        .with_fixed_to_tempo_delay(delays)
        .clone_with_velocity(fixed_velocity)
}
