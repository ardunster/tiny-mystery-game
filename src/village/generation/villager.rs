use crate::village::model::villager::{MemberOfFamily, Villager};
use bevy::app::{App, Plugin};
use bevy::log::debug;
use bevy::prelude::{Commands, Entity};

pub struct VillagerGenerationPlugin;

impl Plugin for VillagerGenerationPlugin {
    fn build(&self, _app: &mut App) {
        // app.add_systems(Startup, generate_villager);
    }
}

pub fn create_villager(commands: &mut Commands, family: Entity) -> Entity {
    let villager_entity = commands.spawn((Villager, MemberOfFamily(family)));

    debug!(target: "Generate::Villager", "Created Villager for Family {}: {}", family, villager_entity.id());

    villager_entity.id()
}

// let given_name = get_first_name(&villager_hash, &gender);

// debug!(target: "Generate::Villager", "Name and gender: {} {:?}", given_name, gender);

// GivenName(given_name.to_string()),
