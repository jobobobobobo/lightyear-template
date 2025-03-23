use avian3d::prelude::Collider;
use bevy::prelude::*;
use lightyear::prelude::*;
use mygame_assets::{AssetState, LevelToLoad, assets::LevelAssets};
use mygame_protocol::component::Level;

pub struct LevelPlugin;

// A "Level" represents every non-replicated object in your environment.
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(load_level)
            .add_systems(OnEnter(AssetState::Loaded), level_loaded);

        app.init_resource::<CurrentLevel>();
    }
}

#[derive(Resource, Clone, Deref, DerefMut, Default)]
pub struct CurrentLevel(pub Level);

/// Fired by the server when a new level begins loading
/// Fired by the client when the server informs it of the new level
#[derive(Event, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct LoadLevelRequest {
    pub level: Level,
}

/// Inform the Asset system of what level to load, and trigger its loading
fn load_level(
    trigger: Trigger<LoadLevelRequest>,
    identity: NetworkIdentity,
    mut next_asset_state: ResMut<NextState<AssetState>>,
    mut level_to_load: ResMut<LevelToLoad>,
) {
    // todo: unload old level? show loading screen?
    level_to_load.0 = trigger.level;
    next_asset_state.set(AssetState::Loading);
}

/// In response to entering the "Loaded" AssetState, spawn the level assets
/// that were preloaded.
fn level_loaded(
    mut commands: Commands,
    mut current_level: ResMut<CurrentLevel>,
    level_to_load: Res<LevelToLoad>,
    level_assets: Res<LevelAssets>,
) {
    match **level_to_load {
        Level::Void => {
            warn!("Did not specify a level to load; loading nothing.")
        }
        Level::Example => {
            commands.spawn(SceneRoot(level_assets.example_level.clone()));
        }
    }

    current_level.0 = level_to_load.0;
}
