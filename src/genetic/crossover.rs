use crate::WithNextIterable;
use itertools::Itertools;
use rand::Rng;

#[inline]
pub fn crossover<T: Clone>(parent1: Vec<T>, parent2: Vec<T>) -> Vec<T> {
    let co_points = generate_co_points(parent1.len(), parent2.len());
    perform_crossover(parent1, parent2, co_points)
}

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
        .collect::<Vec<_>>()
}

#[inline]
fn perform_crossover<T: Clone>(parent1: Vec<T>, parent2: Vec<T>, co_points: Vec<usize>) -> Vec<T> {
    let first_part = parent1
        .iter()
        .take(co_points[0])
        .map(|x| x.clone())
        .collect::<Vec<_>>();

    let crossover = co_points.with_next().enumerate().fold(
        first_part,
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

#[inline]
fn next_part<T: Clone>(parent: &Vec<T>, next_co_point: usize, prev_co_point: usize) -> Vec<T> {
    parent
        .iter()
        .skip(prev_co_point)
        .take(next_co_point - prev_co_point)
        .map(move |x| x.clone())
        .collect()
}

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
