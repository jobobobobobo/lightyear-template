use bevy::prelude::*;

pub struct RenderPlugin;

mod camera;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(camera:: CameraPlugin);
    }
}
