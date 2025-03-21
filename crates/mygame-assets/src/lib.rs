use bevy::{asset::{AssetPlugin as BevyAssetPlugin, LoadState}, gltf::GltfPlugin, prelude::*};
use level_assets::{ExampleLevelAssets, LevelAssets};
use mygame_protocol::component::Level;

pub mod level_assets;

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                OnEnter(AssetState::Loading), on_enter_load_level
            )
            .add_systems(
                Update, check_asset_loading.run_if(in_state(AssetState::Loading)),
            )
            .init_state::<AssetState>()
            .init_resource::<LoadingAssets>()
            .init_resource::<LevelAssets>()
            .init_resource::<CurrentLevel>();
    }
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash,)]
pub enum AssetState {
    #[default]
    Idle,
    Loading,
    Loaded
}

// Resource to track the current handles being loaded
#[derive(Resource, Default)]
pub struct LoadingAssets {
    pub handles: Vec<UntypedHandle>,
}

// Resource to indicate the level to load
// TODO: Belongs elsewhere?
#[derive(Resource, Clone, Deref, DerefMut, Default)]
pub struct CurrentLevel(pub Level);


fn on_enter_load_level(
    asset_server: Res<AssetServer>,
    current_level: Res<CurrentLevel>,
    mut loading_assets: ResMut<LoadingAssets>,
    mut level_assets: ResMut<LevelAssets>,
) {
    match **current_level {
        Level::Example => {
            let environment: Handle<Gltf> = asset_server.load("scenes/example_environment.glb");

            level_assets.example_level = ExampleLevelAssets {
                environment: environment.clone(),
            };

            loading_assets.handles.push(environment.untyped());
        },
        Level::Void => {
            
        }
    }
}

fn check_asset_loading(
    asset_server: Res<AssetServer>,
    loading_assets: Res<LoadingAssets>,
    mut next_state: ResMut<NextState<AssetState>>,
) {
    let all_loaded = loading_assets.handles.iter()
        .all(|handle| {
            match asset_server.get_load_state(handle) {
                Some(LoadState::Loaded) => true,
                _ => false,
            }
        });
    
    if all_loaded {
        info!("All assets loaded successfully");
        next_state.set(AssetState::Loaded);
    }
}
