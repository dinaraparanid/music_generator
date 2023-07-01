use crate::notes::note_data::*;
use astro_float::{ctx::Context, Consts, RoundingMode};
use rand::{prelude::SliceRandom, Rng};

pub mod analyzer;
pub mod arpeggio_types;
pub mod composer;
pub mod generator;

#[inline]
fn random_from_vec<T: Clone>(data: &mut Vec<T>) -> Option<T> {
    data.shuffle(&mut rand::thread_rng());
    data.first().map(|t| t.clone())
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
fn randomize_with_pi(len: usize) -> Vec<u32> {
    pi_numbers(rand::thread_rng().gen::<usize>() % 50, len)
        .into_iter()
        .collect()
}

#[inline]
fn get_bar_ratio(bar_time: DeltaTime, ratio: u32) -> DeltaTime {
    bar_time / 64 * ratio
}

#[inline]
fn fixed_to_tempo(note: NoteData, lengths: &Vec<DeltaTime>, delays: &Vec<DeltaTime>) -> NoteData {
    let new_note = note
        .clone_with_new_length(
            lengths
                .iter()
                .map(|&len| (len, (len as i32 - note.get_length() as i32).abs()))
                .min_by_key(|(_, dif)| *dif)
                .map(|(len, _)| len + randomize_with_pi(1)[0])
                .unwrap_or(note.get_length()),
        )
        .clone_with_new_delay(
            delays
                .iter()
                .map(|&delay| (delay, (delay as i32 - note.get_delay() as i32).abs()))
                .min_by_key(|(_, dif)| *dif)
                .map(|(delay, _)| delay + randomize_with_pi(1)[0])
                .unwrap_or(note.get_delay()),
        );

    new_note.clone_with_velocity(std::cmp::min(60 + new_note.get_velocity(), 100))
}
