use bevy::prelude::{Component, Resource};

#[derive(Resource)]
pub struct EnvArgsResource {
    pub args: Vec<String>,
}

#[derive(Resource)]
pub struct WorldSeed(pub String);

impl WorldSeed {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Component)]
pub struct EntitySeed(pub String);
