use rand::Rng;

/// Performs the selection of leads with the roulette algorithm
/// and the desired fitness to generate the next population.

#[inline]
pub fn select_from_population_with_roulette<T: Clone>(
    population: &Vec<T>,
    fitness_values: Vec<f32>,
) -> Vec<T> {
    let mut rng = rand::thread_rng();

    let fitness_sums = fitness_sums(fitness_values);
    let total_fitness = fitness_sums.last().unwrap().1;

    (0..population.len())
        .map(|_| {
            let roulette_value = rng.gen_range(0.0..total_fitness);

            fitness_sums
                .iter()
                .find(|(_, fitness)| *fitness > roulette_value)
                .map(|(i, _)| population[*i].clone())
        })
        .flatten()
        .collect()
}

/// Generates prefix sums vector from the given fitness values

#[inline]
fn fitness_sums(fitness_values: Vec<f32>) -> Vec<(usize, f32)> {
    fitness_values
        .into_iter()
        .scan(0.0, |acc, x| {
            *acc = *acc + x;
            Some(*acc)
        })
        .enumerate()
        .collect::<Vec<_>>()
}
