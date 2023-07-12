use rand::Rng;

#[inline]
pub fn select_from_population_with_roulette<T: Clone>(
    population: &Vec<T>,
    fitness_values: Vec<f32>,
) -> Vec<T> {
    let mut rng = rand::thread_rng();

    let fitness_sums = fitness_values
        .into_iter()
        .scan(0.0, |acc, x| {
            *acc = *acc + x;
            Some(*acc)
        })
        .enumerate()
        .collect::<Vec<_>>();

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
