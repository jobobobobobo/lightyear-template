use avian3d::prelude::Collider;
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct GlobalAssets {
    pub character: Handle<Scene>,
}

#[derive(Resource, Default)]
pub struct LevelAssets {
    pub example_level: Handle<Scene>,
}
