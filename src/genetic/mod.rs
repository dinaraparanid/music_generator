use crate::{
    genetic::{
        crossover::crossover, fitness::fitness, mutation::mutate,
        selection::select_from_population_with_roulette,
    },
    midi::{
        bpm::BPM,
        generator::{generator::generate_lead_melody_with_bpm, random_from_vec},
        parser::midi_file_manager::extract_notes,
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

type LeadPopulation = Vec<Vec<NoteData>>;

#[inline]
pub async fn generate_lead_with_genetic_algorithm(
    key: PitchClass,
    scale_notes: &Vec<Note>,
    desired_fitness: f32,
    mutation_rate: f32,
) -> (impl BPM, Vec<NoteData>) {
    loop {
        let generated = try_generate_lead_with_genetic_algorithm(
            key,
            scale_notes,
            desired_fitness,
            mutation_rate,
        )
        .await;

        if let Some(success) = generated {
            break success;
        }
    }
}

#[inline]
async fn try_generate_lead_with_genetic_algorithm(
    key: PitchClass,
    scale_notes: &Vec<Note>,
    desired_fitness: f32,
    mutation_rate: f32,
) -> Option<(impl BPM, Vec<NoteData>)> {
    let mut rng = rand::thread_rng();
    let bpm = rng.gen_range(75..=110);

    let mut ideal_leads = extract_notes().await.ok()?;
    let (path, ideal_lead) = random_from_vec(&mut ideal_leads)?;

    let population = (0..)
        .map(|_| generate_lead_melody_with_bpm(key, scale_notes, bpm))
        .filter(|lead| lead.len() == ideal_lead.len())
        .take(1000)
        .collect::<Vec<_>>();

    let fitness_values = next_fitness(bpm, &population, &ideal_lead);
    let max_fit = max_fitness(&fitness_values);

    let population_size = population.len();

    println!("Chosen lead: {:?}", path);
    println!("IDEAL: {:?}", ideal_lead);

    (0..=MAX_GENERATIONS)
        .scan(
            (population, fitness_values, max_fit),
            |(population, fitness_values, max_fit), _| {
                let mut selected =
                    select_from_population_with_roulette(&population, fitness_values.clone());

                *population =
                    next_population(&mut selected, scale_notes, mutation_rate, population_size);

                *fitness_values = next_fitness(bpm, population, &ideal_lead);

                *max_fit = max_fitness(fitness_values);

                Some((population.clone(), fitness_values.clone(), *max_fit))
            },
        )
        .find(|(_, _, max_fitness)| *max_fitness >= desired_fitness)
        .map(|(population, fitness_values, max_fitness)| {
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

#[inline]
fn next_population(
    selected: &mut LeadPopulation,
    scale_notes: &Vec<Note>,
    mutation_rate: f32,
    population_size: usize,
) -> LeadPopulation {
    (0..)
        .map(|_| next_child_with_mb_parent(selected, scale_notes, mutation_rate))
        .filter_map(|population| population)
        .flatten()
        .take(population_size)
        .collect::<Vec<_>>()
}

#[inline]
fn next_child_with_mb_parent(
    selected: &mut LeadPopulation,
    scale_notes: &Vec<Note>,
    mutation_rate: f32,
) -> Option<LeadPopulation> {
    let mut population = Vec::with_capacity(2);
    let parent1 = random_from_vec(selected)?;
    let parent2 = random_from_vec(selected)?;

    let child = crossover(parent1.clone(), parent2.clone());
    let child = mutate(child, scale_notes, mutation_rate);
    population.push(child);

    if rand::thread_rng().gen_bool(0.25) {
        population.push(random_from_vec(&mut vec![parent1, parent2]).unwrap())
    }

    Some(population)
}

#[inline]
fn next_fitness(
    bpm: impl BPM,
    population: &LeadPopulation,
    ideal_lead: &Vec<NoteData>,
) -> Vec<f32> {
    population
        .iter()
        .map(|lead| fitness(bpm, lead, &ideal_lead))
        .collect::<Vec<_>>()
}

#[inline]
fn max_fitness(fitness_values: &Vec<f32>) -> f32 {
    fitness_values
        .iter()
        .map(|&x| x)
        .max_by(|x, y| x.partial_cmp(y).unwrap())
        .unwrap_or(0.0)
}
