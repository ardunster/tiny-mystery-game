use crate::names::get_surname;
use crate::rng::hash::calculate_hash;
use crate::rng::position_in_range::position_in_range;
use crate::village::generation::village::VillageGenConfig;
use crate::village::generation::villager;
use crate::village::model::family::{
    Family, FamilyMembers, HouseholdSize, Surname,
};
use bevy::log::debug;
use bevy::prelude::{Commands, Entity, Res};

pub fn generate_family(
    mut commands: &mut Commands,
    config: &Res<VillageGenConfig>,
    world_seed: &str,
    family_index: u64,
) {
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

        let new_villager = villager::generate_villager(
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
