use crate::WithNextIterable;
use itertools::Itertools;
use rand::Rng;

/// Generates lead from two given leads by mixing them.
/// Firstly, random number of crossover points (1..=4)
/// are generated, then concrete parts between CO points
/// are taken from the leads alternately.
/// Generated lead is not guaranteed to have
/// length equal to either first or second parents' lengths

#[inline]
pub fn crossover<T: Clone>(parent1: Vec<T>, parent2: Vec<T>) -> Vec<T> {
    let co_points = generate_co_points(parent1.len(), parent2.len());
    perform_crossover(parent1, parent2, co_points)
}

/// Generates random number of crossover points (1..=4) as indexes.
/// Smallest lead's length is taken as the generation bound

#[inline]
fn generate_co_points(parent1_len: usize, parent2_len: usize) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    let max_co_point = std::cmp::min(parent1_len, parent2_len);
    let co_points = rng.gen_range(1..=4);

    (0..)
        .map(|_| rng.gen_range(0..max_co_point))
        .dedup()
        .take(co_points)
        .sorted()
        .collect()
}

/// Performs crossover from two parent leads and given crossover points.
/// Generated leads are started with the first parent's parts,
/// then mixed as 2, 1, 2, 1, 2 ... Generated lead is not guaranteed to have
/// length equal to either first or second parents' lengths

#[inline]
fn perform_crossover<T: Clone>(parent1: Vec<T>, parent2: Vec<T>, co_points: Vec<usize>) -> Vec<T> {
    let beginning = parent1
        .iter()
        .take(co_points[0])
        .map(|x| x.clone())
        .collect();

    let crossover = co_points.with_next().enumerate().fold(
        beginning,
        |acc, (i, (&next_co_point, &prev_co_point))| {
            extended_with_next_part(acc, &parent1, &parent2, i, next_co_point, prev_co_point)
        },
    );

    extend_with_last_part(
        crossover,
        parent1,
        parent2,
        co_points.len(),
        *co_points.last().unwrap(),
    )
}

/// Extends currently generating lead with parts
/// from either first or second parent.
/// Chose is made considering the given index

#[inline]
fn extended_with_next_part<T: Clone>(
    mut acc: Vec<T>,
    parent1: &Vec<T>,
    parent2: &Vec<T>,
    i: usize,
    next_co_point: usize,
    prev_co_point: usize,
) -> Vec<T> {
    acc.extend(match i % 2 {
        0 => next_part(&parent1, next_co_point, prev_co_point),
        1 => next_part(&parent2, next_co_point, prev_co_point),
        _ => unreachable!(),
    });

    acc
}

/// Takes slice from the given parent

#[inline]
fn next_part<T: Clone>(parent: &Vec<T>, next_co_point: usize, prev_co_point: usize) -> Vec<T> {
    parent
        .iter()
        .skip(prev_co_point)
        .take(next_co_point - prev_co_point)
        .map(move |x| x.clone())
        .collect()
}

/// Extends currently generating crossover
/// with the last part from either first or second parts.
/// Chose is made considering the number of co points
/// (even -> parent1, odd -> parent2)

#[inline]
fn extend_with_last_part<T: Clone>(
    mut crossover: Vec<T>,
    parent1: Vec<T>,
    parent2: Vec<T>,
    co_points_num: usize,
    last_co_point: usize,
) -> Vec<T> {
    match co_points_num % 2 {
        0 => crossover.extend(parent1.into_iter().skip(last_co_point)),
        1 => crossover.extend(parent2.into_iter().skip(last_co_point)),
        _ => unreachable!(),
    };

    crossover
}
