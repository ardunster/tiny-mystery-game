use crate::names::get_surname;
use crate::rng::hash::calculate_hash;
use crate::rng::position_in_range::position_in_range;
use crate::village::generation::family_relationships::check_family_head_has_spouse;
use crate::village::generation::gender::{
    get_head_of_household_gender, get_spouse_gender,
};
use crate::village::generation::village::VillageGenConfig;
use crate::village::generation::villager;
use crate::village::model::family::{
    Family, FamilyMembers, HouseholdSize, Surname,
};
use crate::village::model::villager::{Gender, HeadOfHousehold, SpouseOf};
use bevy::log::debug;
use bevy::prelude::{Commands, Entity, Res};

pub fn generate_family(
    mut commands: &mut Commands,
    config: &Res<VillageGenConfig>,
    world_seed: &str,
    family_index: u64,
) {
    debug!(target: "Generate::Family", "Generating Family Index {}", family_index);

    let family_seed = format!("{}:family:{}", world_seed, family_index);
    debug!(target: "Generate::Seed::Family", "Family Seed: {}", family_seed);

    let family_hash = calculate_hash(&family_seed);

    let surname = get_surname(&family_hash);
    debug!(target: "Generate::Family", "Surname: {}", surname);

    let family_size = position_in_range(
        &config.family_size_min,
        &config.family_size_max,
        &family_hash,
    );
    debug!(target: "Generate::Family", "Family Size: {}", family_size);

    let family_entity = commands
        .spawn((
            Family,
            &family_seed,
            Surname(surname),
            HouseholdSize(family_size),
            FamilyMembers::default(),
        ))
        .id();

    debug!(target: "Generate::Family", "Family Entity: {}", family_entity);

    let mut family_member_entities: Vec<Entity> =
        Vec::with_capacity(family_size as usize);

    let mut head_of_household: Option<Entity> = None;
    let mut head_of_household_gender: Gender = Gender::Male;
    let mut has_spouse: bool = false;

    for family_member_index in 0..family_size {
        let villager_seed =
            format!("{}:member:{}", family_seed, family_member_index);
        debug!(target: "Generate::Seed::Villager", "Villager Seed: {}", villager_seed);

        let villager_hash = calculate_hash(&villager_seed);
        debug!(target: "Generate::Hash::Villager", "Villager Hash: {}", villager_hash);

        let is_head = family_member_index == 0;

        let new_family_member = villager::create_villager(
            &mut commands,
            &villager_seed,
            family_entity,
        );

        if is_head {
            debug!(target: "Generate::Family", "Family Member {} is Head of Household", new_family_member);
            commands.entity(new_family_member).insert(HeadOfHousehold);
            head_of_household = Option::from(new_family_member);

            let gender = get_head_of_household_gender(villager_hash);
            head_of_household_gender = gender;
            debug!(target: "Generate::Family", "Family Member {} is {:?}", new_family_member, gender);
            commands.entity(new_family_member).insert(gender);

            has_spouse = check_family_head_has_spouse(
                family_hash,
                family_size,
                head_of_household_gender,
            );
        } else if has_spouse && family_member_index == 1 {
            debug!(target: "Generate::Family", "Family Member {} is Spouse", new_family_member);
            let spouse_gender =
                get_spouse_gender(villager_hash, head_of_household_gender);
            debug!(target: "Generate::Family", "Family Member {} is {:?}", new_family_member, spouse_gender);
            commands.entity(new_family_member).insert(spouse_gender);
            commands
                .entity(new_family_member)
                .insert(SpouseOf(head_of_household.unwrap()));
            commands
                .entity(head_of_household.unwrap())
                .insert(SpouseOf(new_family_member));
        } else {
            // todo: non head, non spouse logic
        }

        family_member_entities.push(new_family_member);
    }

    commands
        .entity(family_entity)
        .insert(FamilyMembers(family_member_entities));
}
