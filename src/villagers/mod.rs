use crate::names::get_first_name;
use crate::rng::calculate_hash;
use bevy::app::App;
use bevy::prelude::*;

struct VillagerPlugin;

impl Plugin for VillagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, generate_villager);
    }
}

#[derive(Component)]
pub struct Villager;

#[derive(Component)]
pub struct Name(String);

#[derive(Component, PartialEq)]
pub enum Gender {
    Male,
    Female,
}

fn generate_villager(mut commands: Commands) {
    // TODO: Put seed in world and get it from world somehow later
    let stringy_seed = "some_seed".to_string();

    let hash = calculate_hash(&stringy_seed);

    let name = get_first_name(hash, Gender::Male);
    commands.spawn((Villager, Name(name.to_string())));
}
