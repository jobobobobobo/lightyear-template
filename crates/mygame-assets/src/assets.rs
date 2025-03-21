use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct GlobalAssets {
    pub character: Handle<Gltf>,
}

#[derive(Resource, Default)]
pub struct LevelAssets {
    pub example_level: ExampleLevelAssets,
}

#[derive(Default)]
pub struct ExampleLevelAssets {
    pub environment: Handle<Gltf>,
}

