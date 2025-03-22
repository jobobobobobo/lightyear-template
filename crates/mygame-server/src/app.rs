use std::time::Duration;

use bevy::{
    app::{PanicHandlerPlugin, ScheduleRunnerPlugin},
    asset::AssetPlugin,
    diagnostic::DiagnosticsPlugin,
    gltf::GltfPlugin,
    input::InputPlugin,
    log::LogPlugin,
    pbr::PbrPlugin,
    prelude::*,
    render::{
        camera::CameraPlugin, mesh::skinning::SkinnedMeshInverseBindposes, settings::{RenderCreation, WgpuSettings}, RenderPlugin as BevyRenderPlugin
    },
    scene::ScenePlugin,
    state::app::StatesPlugin, window::ExitCondition,
};
use lightyear::{
    prelude::*,
    server::{config::ServerConfig, plugin::ServerPlugins},
};
use mygame_common::CommonPlugin;
use mygame_render::RenderPlugin;

use crate::{network::NetworkPlugin, replication::ReplicationPlugin};

pub fn build_server_app(server_config: ServerConfig, asset_path: String, headless: bool) -> App {
    let mut app = App::new();

    let asset_plugin = AssetPlugin {
        file_path: asset_path.clone(),
        ..default()
    };

    if headless {
        // GltfPlugin relies on RenderPlugin, so we've got a mess here.
        app.add_plugins((
            MinimalPlugins
                .build()
                .set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                    1.0 / 100.0,
                ))),
            asset_plugin,
            WindowPlugin {
                primary_window: None,
                exit_condition: ExitCondition::DontExit,
                ..default()
            },
            BevyRenderPlugin {
                render_creation: RenderCreation::Automatic(WgpuSettings {
                    backends: None,
                    ..default()
                }),
                ..default()
            },
            PanicHandlerPlugin::default(),
            LogPlugin::default(),
            TransformPlugin::default(),
            HierarchyPlugin::default(),
            DiagnosticsPlugin::default(),
            StatesPlugin::default(),
            ScenePlugin::default(),
            GltfPlugin::default(),
            PbrPlugin::default(),
        ));

        app.init_asset::<Image>(); // or add ImagePlugin
    } else {
        app.add_plugins(DefaultPlugins.build().set(asset_plugin));
    }

    app.add_plugins(ServerPlugins {
        config: server_config,
    })
    .add_plugins((CommonPlugin, NetworkPlugin, ReplicationPlugin));

    app
}
