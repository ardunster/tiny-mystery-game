use crate::rng::coin_flip::coin_flip;
use crate::rng::weighted_value::{choose_weighted_value, WeightedValue};
use crate::village::model::villager::Gender;

pub const HEAD_HOUSEHOLD_GENDER_WEIGHTS: [WeightedValue<Gender>; 2] = [
    WeightedValue {
        value: Gender::Male,
        weight: 7,
    },
    WeightedValue {
        value: Gender::Female,
        weight: 3,
    },
];

pub fn get_gender(villager_hash: u64) -> Gender {
    match coin_flip(&villager_hash) {
        true => Gender::Male,
        false => Gender::Female,
    }
}

pub fn get_head_of_household_gender(villager_hash: u64) -> Gender {
    choose_weighted_value(&HEAD_HOUSEHOLD_GENDER_WEIGHTS, villager_hash)
        .copied()
        .unwrap_or(Gender::Male)
}

pub fn get_spouse_gender(
    villager_hash: u64,
    head_of_household_gender: Gender,
) -> Gender {
    let opposite = match head_of_household_gender {
        Gender::Male => Gender::Female,
        Gender::Female => Gender::Male,
    };

    let weight_table = vec![
        WeightedValue {
            value: opposite,
            weight: 97,
        },
        WeightedValue {
            value: head_of_household_gender,
            weight: 1,
        },
    ];

    choose_weighted_value(&weight_table, villager_hash)
        .copied()
        .unwrap_or(opposite)
}
