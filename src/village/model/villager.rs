use bevy::prelude::{Component, Entity};

#[derive(Component)]
pub struct Villager;

#[derive(Component)]
pub struct GivenName(pub String);

#[derive(Component, Debug, PartialEq, Clone, Copy)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Component)]
pub struct MemberOfFamily(pub Entity);

#[derive(Component)]
pub struct HeadOfHousehold;
