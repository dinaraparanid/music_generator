use rand::Rng;

#[inline]
pub fn crossover<T: Clone>(parent1: Vec<T>, parent2: Vec<T>) -> Vec<T> {
    let mut rng = rand::thread_rng();
    let max_co_point = std::cmp::min(parent1.len(), parent2.len());
    let co_point = rng.gen_range(0..max_co_point);

    let mut crossover = parent1.into_iter().take(co_point).collect::<Vec<_>>();
    crossover.extend(parent2.into_iter().skip(co_point));
    crossover
}
