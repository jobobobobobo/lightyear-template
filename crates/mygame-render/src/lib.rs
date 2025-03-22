use avian3d::prelude::PhysicsDebugPlugin;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct RenderPlugin;

mod camera;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                camera:: CameraPlugin,
                PhysicsDebugPlugin::default(),
                WorldInspectorPlugin::default(),
            ));
    }
}
