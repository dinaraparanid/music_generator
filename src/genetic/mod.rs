use crate::{
    genetic::{
        crossover::crossover, fitness::fitness, mutation::mutate,
        selection::select_from_population_with_roulette,
    },
    midi::{
        bpm::BPM,
        generator::{generator::generate_lead_melody_with_bpm, random_from_vec},
    },
    notes::{note::Note, note_data::NoteData},
};

use rand::Rng;
use rust_music_theory::note::PitchClass;

mod crossover;
mod fitness;
mod mutation;
mod selection;

const MAX_GENERATIONS: usize = 100;

#[inline]
pub fn generate_lead_with_genetic_algorithm(
    key: PitchClass,
    scale_notes: &Vec<Note>,
    desired_fitness: f32,
    mutation_rate: f32,
) -> Option<(impl BPM, Vec<NoteData>)> {
    let mut rng = rand::thread_rng();
    let bpm = rng.gen_range(95..=115);

    let population = (0..rng.gen_range(20..=40))
        .map(|_| generate_lead_melody_with_bpm(key, scale_notes, bpm))
        .collect::<Vec<_>>();

    let population_size = population.len();

    (0..=MAX_GENERATIONS)
        .map(|_| {
            let fitness_values = population.iter().map(fitness).collect::<Vec<_>>();

            let max_fitness = fitness_values
                .iter()
                .map(|&x| x)
                .max_by(|x, y| x.partial_cmp(y).unwrap())
                .unwrap_or(0.0);

            let mut selected =
                select_from_population_with_roulette(&population, fitness_values.clone());

            let population = (0..)
                .map(|_| {
                    let mut population = Vec::with_capacity(2);
                    let parent1 = random_from_vec(&mut selected)?;
                    let parent2 = random_from_vec(&mut selected)?;

                    let child = crossover(parent1.clone(), parent2.clone());
                    let child = mutate(child, scale_notes, mutation_rate);
                    population.push(child);

                    if rng.gen_bool(0.2) {
                        population.push(random_from_vec(&mut vec![parent1, parent2]).unwrap())
                    }

                    Some(population)
                })
                .filter_map(|population| population)
                .flatten()
                .take(population_size)
                .collect::<Vec<_>>();

            Some(((population, fitness_values), max_fitness))
        })
        .filter_map(|population| population)
        .find(|(_, max_fitness)| *max_fitness >= desired_fitness)
        .map(|((population, fitness_values), max_fitness)| {
            let first_ok_fitness_ind = fitness_values
                .into_iter()
                .position(|x| x == max_fitness)
                .unwrap();

            let ok_lead = population
                .into_iter()
                .skip(first_ok_fitness_ind)
                .next()
                .unwrap();

            (bpm, ok_lead)
        })
}
