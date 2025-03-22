use avian3d::prelude::{Collider, RigidBody, collider};
use bevy::{gltf::GltfMesh, prelude::*};
use lightyear::prelude::{
    client::{Interpolated, Predicted},
    server::ReplicationTarget,
};
use mygame_assets::{AssetState, assets::GlobalAssets};
use mygame_protocol::component::Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (player_physics).run_if(in_state(AssetState::Loaded)),
        );
    }
}

type NeedsPhysicsFilter = (
    Without<RigidBody>,
    Or<(
        With<Predicted>,
        With<ReplicationTarget>,
        (Without<Predicted>, Without<Interpolated>),
    )>,
);

fn player_physics(
    mut commands: Commands,
    q_player_without_physics: Query<Entity, (NeedsPhysicsFilter, With<Player>)>,
    global_assets: Res<GlobalAssets>,
) {
    if q_player_without_physics.is_empty() {
        return;
    }

    for player_entity in &q_player_without_physics {
        commands.entity(player_entity).insert((
            RigidBody::Kinematic,
            Collider::capsule(3.0, 4.0),
            SceneRoot(global_assets.character.clone())
        ));
    }
}
