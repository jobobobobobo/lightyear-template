use avian3d::prelude::{Collider, RigidBody};
use bevy::prelude::*;
use lightyear::prelude::*;
use assets::{CurrentLevel, Geometry, LevelState, assets::LevelAssets};
use protocol::message::Level;

pub struct LevelPlugin;

// A "Level" represents every non-replicated object in your environment.
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(LevelState::Loaded),
            (level_loaded, add_level_gameplay_components).chain(),
        );
    }
}

fn level_loaded(
    mut commands: Commands,
    current_level: Res<CurrentLevel>,
    level_assets: Res<LevelAssets>,
) {
    match **current_level {
        Level::Example => {
            commands.spawn(SceneRoot(level_assets.example_level.clone()));
        }
        Level::Void => {}
    }
}

fn add_level_gameplay_components(
    mut commands: Commands,
    q_geo: Query<Entity, (With<Geometry>, Without<RigidBody>)>,
) {
    for geo in &q_geo {
        commands.entity(geo).insert(RigidBody::Static);
    }
}
