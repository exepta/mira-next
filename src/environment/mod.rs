use std::f32::consts::PI;
use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::prelude::*;
use bevy_rapier3d::geometry::ComputedColliderShape;
use bevy_rapier3d::prelude::{AsyncSceneCollider, RigidBody, TriMeshFlags};
use crate::manager::{AppState, InGameState};

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame(InGameState::Playing)), dev_plate);
    }
}

fn dev_plate(mut commands: Commands,
             asset_server: Res<AssetServer>
) {
    let prolog_example = asset_server.load(GltfAssetLabel::Scene(0).from_asset("map/test-map.glb"));

    commands.spawn(SceneRoot(prolog_example.clone()))
        .insert(Name::new("Prolog-Example"))
        .insert(Transform::from_xyz(0.0, 0.0, 0.0))
        .insert(RigidBody::Fixed)
        .insert(AsyncSceneCollider {
            shape: Some(ComputedColliderShape::TriMesh(TriMeshFlags::MERGE_DUPLICATE_VERTICES)),
            ..default()
        });



    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 200.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
            .build(),
    ));
}