use crate::rng::weighted_value::{choose_weighted_value, WeightedValue};
use crate::village::model::villager::Gender;

const MALE_HAS_SPOUSE_TABLE: [WeightedValue<bool>; 2] = [
    WeightedValue {
        value: true,
        weight: 85,
    },
    WeightedValue {
        value: false,
        weight: 15,
    },
];

const FEMALE_HAS_SPOUSE_TABLE: [WeightedValue<bool>; 2] = [
    WeightedValue {
        value: true,
        weight: 45,
    },
    WeightedValue {
        value: false,
        weight: 55,
    },
];

pub fn check_family_head_has_spouse(
    family_hash: u64,
    family_size: u64,
    head_of_household_gender: Gender,
) -> bool {
    if family_size == 1 {
        return false;
    };

    let head_is_male = match head_of_household_gender {
        Gender::Male => true,
        Gender::Female => false,
    };

    let has_spouse_weight_table = if head_is_male {
        MALE_HAS_SPOUSE_TABLE
    } else {
        FEMALE_HAS_SPOUSE_TABLE
    };

    choose_weighted_value(&has_spouse_weight_table, family_hash)
        .copied()
        .unwrap_or(true)
}

// fn build_family_relationships(family_member: Entity) {}
