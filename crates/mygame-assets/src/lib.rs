use assets::{GlobalAssets, LevelAssets};
use avian3d::prelude::{Collider, ColliderConstructor};
use bevy::{
    asset::{AssetPlugin as BevyAssetPlugin, LoadState},
    gltf::{GltfMesh, GltfPlugin},
    prelude::*,
};
use mygame_protocol::component::Level;

pub mod assets;

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AssetState::Loading), on_enter_load_level)
            .add_systems(
                Update,
                check_asset_loading.run_if(in_state(AssetState::Loading)),
            )
            .add_systems(OnEnter(AssetState::Postprocess), postprocess_assets)
            .init_state::<AssetState>()
            .init_resource::<LoadingAssets>()
            .init_resource::<LevelToLoad>()
            .init_resource::<LevelAssets>()
            .init_resource::<GlobalAssets>();
    }
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AssetState {
    #[default]
    Idle,
    Loading,
    Postprocess,
    Loaded,
}

/// Resource to track the current handles being loaded
#[derive(Resource, Default)]
pub struct LoadingAssets {
    pub handles: Vec<UntypedHandle>,
}

/// External systems are responsible for setting LevelToLoad
#[derive(Resource, Clone, Deref, DerefMut, Default)]
pub struct LevelToLoad(pub Level);

/// When entering the "Loading" AssetState, load the assets required
/// for the LevelToLoad. Queue the resultant Handles to be polled for
/// completion in `check_asset_loading`
fn on_enter_load_level(
    asset_server: Res<AssetServer>,
    level_to_load: Res<LevelToLoad>,
    mut loading_assets: ResMut<LoadingAssets>,
    mut level_assets: ResMut<LevelAssets>,
    mut global_assets: ResMut<GlobalAssets>,
) {
    global_assets.character = asset_server.load(GltfAssetLabel::Scene(0).from_asset("scenes/example_character.glb"));

    match **level_to_load {
        Level::Example => {
            level_assets.example_level = asset_server.load(GltfAssetLabel::Scene(0).from_asset("scenes/example_environment.glb"));

            loading_assets
                .handles
                .push(level_assets.example_level.clone().untyped());
        }
        Level::Void => {}
    }
}

/// Sets the AssetState to Loaded once all queued Handles have finished loading
/// Downstream systems should consume this state change as part of their loading process
fn check_asset_loading(
    asset_server: Res<AssetServer>,
    mut loading_assets: ResMut<LoadingAssets>,
    mut next_state: ResMut<NextState<AssetState>>,
) {
    let all_loaded =
        loading_assets
            .handles
            .iter()
            .all(|handle| match asset_server.get_load_state(handle) {
                Some(LoadState::Loaded) => true,
                _ => false,
            });

    if all_loaded {
        info!("All assets loaded successfully");
        next_state.set(AssetState::Postprocess);
        loading_assets.handles.clear();
    }
}

/// Just adds colliders, but other postprocessing on the Scene could be done here
/// In the future, when Avian3d's Collision component is #[reflect], it would be nice
///  to actually construct the colliders here, rather than defer them with ColliderConstructor
fn postprocess_assets(
    mut commands: Commands,
    level_to_load: Res<LevelToLoad>,
    mut scenes: ResMut<Assets<Scene>>,
    level_assets: ResMut<LevelAssets>,
    meshes: Res<Assets<Mesh>>,
) {
    match **level_to_load {
        Level::Example => {
            if let Some(scene) = scenes.get_mut(&level_assets.example_level) {
                let mut entities_to_process = Vec::new();
                
                for entity_ref in scene.world.iter_entities() {
                    let entity = entity_ref.id();
                    if let Some(mesh_handle) = scene.world.get::<Mesh3d>(entity) {
                        entities_to_process.push((entity, mesh_handle.clone()));
                    }
                }
                
                for (entity, mesh_handle) in entities_to_process {
                    if let Some(mesh) = meshes.get(&mesh_handle) {
                        scene.world.entity_mut(entity).insert(
                            ColliderConstructor::TrimeshFromMesh,
                        );
                    }
                }
            }
        }
        Level::Void => {
            // Handle void level if needed
        }
    }
    commands.set_state(AssetState::Loaded);
}
