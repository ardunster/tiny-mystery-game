use crate::names::get_surname;
use crate::resources::WorldSeed;
use crate::rng::{calculate_hash, position_in_range};
use crate::villagers::{self, VillagerPlugin};
use bevy::prelude::*;
use bevy::window::WindowRef::Entity;

pub struct VillageGenerationPlugin;

impl Plugin for VillageGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(VillagerPlugin)
            .insert_resource(VillageGenConfig::default())
            .add_event::<GenerateVillage>()
            // .add_systems(Startup, request_generate_village)
            .add_systems(Update, generate_village_on_request);
    }
}

#[derive(Resource)]
pub struct VillageGenConfig {
    pub families_min: u64,
    pub families_max: u64,
    pub family_size_min: u64,
    pub family_size_max: u64,
}

impl Default for VillageGenConfig {
    fn default() -> Self {
        Self {
            families_min: 4,
            families_max: 8,
            family_size_min: 1,
            family_size_max: 6,
        }
    }
}

#[derive(Component)]
pub struct Family;

#[derive(Component, Default)]
pub struct FamilyMembers(pub Vec<Entity>);

#[derive(Component)]
pub struct Surname(pub String);

#[derive(Component)]
pub struct HouseholdSize(pub u64);

#[derive(Event)]
pub struct GenerateVillage;

pub fn request_generate_village(
    mut event_writer: EventWriter<GenerateVillage>,
) {
    event_writer.write(GenerateVillage);
}

fn generate_village_on_request(
    mut commands: Commands,
    mut generate_village_event: EventReader<GenerateVillage>,
    world_seed_resource: Res<WorldSeed>,
    config: Res<VillageGenConfig>,
) {
    if generate_village_event.is_empty() {
        // debug!(target: "Village::Generate", "Generate Village Event is empty");
        return;
    }
    debug!(target: "Village::Generate", "Received Generate Village Event.");
    generate_village_event.clear();

    let world_seed = world_seed_resource.as_str();

    debug!(target: "Village::Generate", "World Seed: {}", world_seed);
    let village_hash = calculate_hash(&world_seed);

    let family_count = position_in_range(
        &config.families_min,
        &config.families_max,
        &village_hash,
    );

    debug!(target: "Village::Generate", "Family Count = {}", family_count);

    for family_index in 0..family_count {
        debug!(target: "Family::Generate", "Generating Family Index {}", family_index);

        let family_seed = format!("{}:family:{}", world_seed, family_index);
        debug!(target: "Family::Generate", "Family Seed: {}", family_seed);

        let family_hash = calculate_hash(&family_seed);

        let surname = get_surname(&family_hash);
        debug!(target: "Family::Generate", "Surname: {}", surname);

        let family_size = position_in_range(
            &config.family_size_min,
            &config.family_size_max,
            &family_hash,
        );
        debug!(target: "Family::Generate", "Family Size: {}", family_size);

        let family_entity = commands
            .spawn((
                Family,
                Surname(surname),
                HouseholdSize(family_size),
                FamilyMembers::default(),
            ))
            .id();

        debug!(target: "Family::Generate", "Family Entity: {}", family_entity);

        let mut family_member_entities: Vec<Entity> =
            Vec::with_capacity(family_size as usize);

        for family_member_index in 0..family_size {
            let is_head = family_member_index == 0;

            // let mut is_spouse = false;
            // if (family_member_index == 1) {
            //
            // }

            let new_villager = villagers::generate_villager(
                &mut commands,
                &family_seed,
                family_member_index,
                family_entity,
                is_head,
            );

            family_member_entities.push(new_villager);
        }

        commands
            .entity(family_entity)
            .insert(FamilyMembers(family_member_entities));
    }
}
