use avian3d::prelude::Collider;
use bevy::{prelude::*};
use lightyear::prelude::*;
use mygame_assets::{level_assets::LevelAssets, AssetState, CurrentLevel};
use mygame_protocol::component::Level;


pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_observer(load_level)
            .add_systems(OnEnter(AssetState::Loaded), level_loaded);
    }
}

#[derive(Event, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct LoadLevelRequest {
    pub level: Level,
}

// A "Level" in this template represents every non-replicated object in your environment.
fn load_level(
    trigger: Trigger<LoadLevelRequest>,
    identity: NetworkIdentity,
    mut next_asset_state: ResMut<NextState<AssetState>>,
    mut level_to_load: ResMut<CurrentLevel>,
) {
    // todo: unload old level? 
    level_to_load.0 = trigger.level;
    next_asset_state.set(AssetState::Loading);
}

fn level_loaded(
    mut commands: Commands,
    current_level: Res<CurrentLevel>,
    level_assets: Res<LevelAssets>,
    gltfs: Res<Assets<Gltf>>,
) {
    match **current_level {
        Level::Void => {
            warn!("Did not specify a level to load; loading nothing.")
        },
        Level::Example => {
            let environment = gltfs.get(&level_assets.example_level.environment).expect("Shouldve been preloaded");

            commands.spawn(
                SceneRoot(environment.scenes[0].clone())
            );
        },
    }
}
