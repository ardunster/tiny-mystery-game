use crate::rng;

#[derive(Clone)]
pub struct WeightedValue<T> {
    pub value: T,
    pub weight: u64,
}

pub fn choose_weighted_value<T>(
    options: &[WeightedValue<T>],
    hash: u64,
) -> Option<&T> {
    let total_weight: u64 = options
        .iter()
        .map(|weighted_value| weighted_value.weight)
        .sum();
    if total_weight == 0 {
        return None;
    }

    let mut threshold = rng::position_in_range(&0, &(total_weight - 1), &hash);
    for item in options {
        let weight = item.weight;
        if threshold < weight {
            return Some(&item.value);
        }
        threshold -= weight;
    }

    None
}
