use crate::names::get_first_name;
use crate::rng::{
    calculate_hash, choose_weighted_value, coin_flip, WeightedValue,
};
use bevy::app::App;
use bevy::prelude::*;

pub struct VillagerPlugin;

impl Plugin for VillagerPlugin {
    fn build(&self, _app: &mut App) {
        // app.add_systems(Startup, generate_villager);
    }
}

#[derive(Component)]
pub struct Villager;

#[derive(Component)]
pub struct GivenName(String);

#[derive(Component, Debug, PartialEq, Clone, Copy)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Component)]
pub struct MemberOfFamily(pub Entity);

#[derive(Component)]
pub struct HeadOfHousehold;

pub const HEAD_HOUSEHOLD_GENDER_WEIGHTS: [WeightedValue<Gender>; 2] = [
    WeightedValue {
        value: Gender::Male,
        weight: 9,
    },
    WeightedValue {
        value: Gender::Female,
        weight: 1,
    },
];

pub fn generate_villager(
    mut commands: &mut Commands,
    family_seed: &str,
    family_member_index: u64,
    family: Entity,
    is_head: bool,
) -> Entity {
    let villager_seed =
        format!("{}:member:{}", family_seed, family_member_index);

    debug!(target: "Villager::Generate", "Villager Seed: {}", villager_seed);

    let hash = calculate_hash(&villager_seed);
    debug!(target: "Villager::Generate", "hash: {}", hash);

    let gender = if is_head {
        choose_weighted_value(&HEAD_HOUSEHOLD_GENDER_WEIGHTS, hash)
            .copied()
            .unwrap_or(Gender::Male)
    } else {
        match coin_flip(&hash) {
            true => Gender::Male,
            false => Gender::Female,
        }
    };

    let given_name = get_first_name(&hash, &gender);

    debug!(target: "Villager::Generate", "Name and gender: {} {:?}", given_name, gender);
    let mut villager_entity = commands.spawn((
        Villager,
        GivenName(given_name.to_string()),
        gender,
        MemberOfFamily(family),
    ));

    if is_head {
        villager_entity.insert(HeadOfHousehold);
    }

    debug!(target: "Villager::Generate", "villager_entity: {}", villager_entity.id());

    villager_entity.id()
}
