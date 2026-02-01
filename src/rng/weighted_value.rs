use super::position_in_range::position_in_range;

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

    let mut threshold = position_in_range(&0, &(total_weight - 1), &hash);
    for item in options {
        let weight = item.weight;
        if threshold < weight {
            return Some(&item.value);
        }
        threshold -= weight;
    }

    None
}

#[test]
fn choose_weighted_value_total_weight_zero_is_none() {
    let options = vec![
        WeightedValue {
            value: "A",
            weight: 0,
        },
        WeightedValue {
            value: "B",
            weight: 0,
        },
    ];

    assert_eq!(choose_weighted_value(&options, 0), None);
    assert_eq!(choose_weighted_value(&options, 123), None);
}

#[test]
fn choose_weighted_value_weight_array_empty_is_none() {
    let options: Vec<WeightedValue<&str>> = vec![];

    assert_eq!(choose_weighted_value(&options, 0), None);
    assert_eq!(choose_weighted_value(&options, 123), None);
}

#[test]
fn choose_weighted_value_never_none_when_total_weight_positive() {
    let options = vec![
        WeightedValue {
            value: "A",
            weight: 2,
        },
        WeightedValue {
            value: "B",
            weight: 3,
        },
    ];

    for hash in 0..10_000u64 {
        let picked = choose_weighted_value(&options, hash);
        assert!(picked.is_some(), "hash={hash} unexpectedly returned None");
    }
}

#[test]
fn choose_weighted_value_respects_boundaries() {
    let options = vec![
        WeightedValue {
            value: "A",
            weight: 2,
        },
        WeightedValue {
            value: "B",
            weight: 3,
        },
    ];

    assert_eq!(choose_weighted_value(&options, 0).copied(), Some("A"));
    assert_eq!(choose_weighted_value(&options, 1).copied(), Some("A"));

    assert_eq!(choose_weighted_value(&options, 2).copied(), Some("B"));
    assert_eq!(choose_weighted_value(&options, 3).copied(), Some("B"));
    assert_eq!(choose_weighted_value(&options, 4).copied(), Some("B"));
}

#[test]
fn choose_weighted_value_wraps_correctly() {
    let options = vec![
        WeightedValue {
            value: "A",
            weight: 2,
        },
        WeightedValue {
            value: "B",
            weight: 3,
        },
    ];

    assert_eq!(choose_weighted_value(&options, 5).copied(), Some("A"));
    assert_eq!(choose_weighted_value(&options, 6).copied(), Some("A"));

    assert_eq!(choose_weighted_value(&options, 7).copied(), Some("B"));
    assert_eq!(choose_weighted_value(&options, 8).copied(), Some("B"));
    assert_eq!(choose_weighted_value(&options, 9).copied(), Some("B"));
}

#[test]
fn choose_weighted_value_distribution_is_reasonable() {
    let options = vec![
        WeightedValue {
            value: "A",
            weight: 2,
        },
        WeightedValue {
            value: "B",
            weight: 3,
        },
    ];

    let mut a = 0u64;
    let mut b = 0u64;

    for hash in 0..10_000u64 {
        match *choose_weighted_value(&options, hash).unwrap() {
            "A" => a += 1,
            "B" => b += 1,
            _ => unreachable!(),
        }
    }

    // Expected ratio A:B is 2:3.
    // Allow some tolerance.
    let ratio = a as f64 / b as f64;
    assert!(ratio > 0.60 && ratio < 0.75, "a={a} b={b} ratio={ratio}");
}
