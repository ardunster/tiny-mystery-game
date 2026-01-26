use crate::names::get_first_name;
use crate::rng::{calculate_hash, coin_flip};
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

#[derive(Component, PartialEq, Clone, Copy)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Component)]
pub struct MemberOfFamily(pub Entity);

#[derive(Component)]
pub struct HeadOfHousehold;

fn generate_villager(
    mut commands: Commands,
    seed: &str,
    position_key: u32,
    family: Entity,
    is_head: bool,
) -> Entity {
    let seed_with_pos = seed.to_owned() + &position_key.to_string();
    debug!(target: "Villager::Generate", "seed with position: {}", seed_with_pos);

    let hash = calculate_hash(&seed_with_pos);
    debug!(target: "Villager::Generate", "hash: {}", hash);

    let gender = match coin_flip(&hash) {
        true => Gender::Male,
        false => Gender::Female,
    };

    let given_name = get_first_name(&hash, &gender);

    debug!(target: "Villager::Generate", "Name and gender: {} {}", given_name, gender);
    let mut villager_entity = commands.spawn((
        Villager,
        GivenName(given_name.to_string()),
        gender,
        MemberOfFamily(family),
    ));

    if (is_head) {
        villager_entity.insert(HeadOfHousehold);
    }

    debug!(target: "Villager::Generate", "villager_entity: {}", villager_entity);

    villager_entity.id()
}
