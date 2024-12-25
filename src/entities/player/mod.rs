use bevy::core_pipeline::bloom::Bloom;
use bevy::prelude::*;
use bevy_atmosphere::prelude::{AtmosphereCamera, AtmospherePlugin};
use bevy_third_person_camera::{Offset, ThirdPersonCamera, ThirdPersonCameraPlugin, ThirdPersonCameraTarget, Zoom};
use crate::entities::EntityBase;
use crate::manager::{AppState, InGameState};
use crate::services::loading_service::LoadingData;

#[derive(Component, Resource, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct PlayerStats {
    pub base: EntityBase
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<PlayerStats>();
        app.add_plugins((
            ThirdPersonCameraPlugin,
            AtmospherePlugin
        ));
        app.add_systems(OnEnter(AppState::InGame(InGameState::Playing)), (spawn_player, create_player_camera));
    }
}

fn spawn_player(mut commands: Commands,
                asset_server: Res<AssetServer>,
                mut loading_data: ResMut<LoadingData>
) {
    let player_scene = asset_server
        .load(GltfAssetLabel::Scene(0).from_asset("entities/player.glb"));

    loading_data
        .loading_assets
        .push(player_scene.clone().into());

    commands.spawn(SceneRoot(player_scene))
        .insert(Name::new("Player"))
        .insert(Transform::from_xyz(0.0, 0.0, 0.0))
        .insert(ThirdPersonCameraTarget);
}

fn create_player_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("PlayerCamera"),
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, 10.0),
        GlobalTransform::default(),
        ThirdPersonCamera {
            sensitivity: Vec2::new(1.0, 1.0),
            zoom: Zoom::new(3.5, 12.75),
            cursor_lock_key: KeyCode::Escape,
            offset: Offset::new(0.0, 0.8),
            offset_enabled: true,
            ..default()
        },
        AtmosphereCamera::default(),
        Bloom::default(),
        DistanceFog {
            color: Color::srgb(0.3, 0.3, 0.3),
            falloff: FogFalloff::Linear {
                start: 600.0,
                end: 700.0
            },
            ..default()
        }
    ));
}

