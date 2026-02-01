use bevy::prelude::{Component, Entity};

#[derive(Component)]
pub struct Family;

#[derive(Component, Default)]
pub struct FamilyMembers(pub Vec<Entity>);

#[derive(Component)]
pub struct Surname(pub String);

#[derive(Component)]
pub struct HouseholdSize(pub u64);
