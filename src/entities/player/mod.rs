mod input;

use bevy::core_pipeline::bloom::Bloom;
use bevy::prelude::*;
use bevy_atmosphere::prelude::{AtmosphereCamera, AtmospherePlugin};
use bevy_rapier3d::prelude::{Collider, Damping, LockedAxes, RigidBody, Velocity};
use bevy_third_person_camera::{Offset, ThirdPersonCamera, ThirdPersonCameraPlugin, ThirdPersonCameraTarget, Zoom};
use crate::entities::EntityBase;
use crate::entities::player::input::PlayerInputPlugin;
use crate::manager::{AppState, InGameState};
use crate::services::loading_service::LoadingData;

#[derive(Component, Resource, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct PlayerStats {
    pub base: EntityBase,
    pub stamina: f32,
    pub stamina_fill_count: f32,
    pub mana: f32,
    pub state: PlayerState,
    pub jump_height: f32,
    pub timers: StatsTimer
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            base: EntityBase::default(),
            stamina: 270.0,
            stamina_fill_count: 70.0,
            mana: 145.0,
            state: PlayerState::default(),
            jump_height: 2.0,
            timers: StatsTimer::default()
        }
    }
}

#[derive(Component, Reflect, Debug)]
pub struct Grounded(pub bool);


#[derive(Component, Reflect, Debug, Clone, Copy, PartialEq, Eq)]
#[reflect(Component)]
pub enum PlayerState {
    Idling,
    Moving,
    Jumping,
    Attacking,
    Sprinting,
    Sneaking,
    Dodging,
    Grounded,
    Climbing,
    Blocking,
    Dead,
}

impl Default for PlayerState {
    fn default() -> Self {
        PlayerState::Idling
    }
}

#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct StatsTimer {
    pub sprint_timer: f32,
    pub stamina_fill_timer: f32,
    pub stamina_fill_delay: f32,
}

impl Default for StatsTimer {
    fn default() -> Self {
        Self {
            sprint_timer: 0.0,
            stamina_fill_timer: 0.0,
            stamina_fill_delay: 0.8,
        }
    }
}

#[derive(Component, Reflect, Debug, Clone)]
pub struct PlayerCamera;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<PlayerStats>();
        app.add_plugins((
            ThirdPersonCameraPlugin,
            AtmospherePlugin,
        ));
        app.add_systems(OnEnter(AppState::InGame(InGameState::Playing)), (spawn_player, create_player_camera));

        app.add_plugins(PlayerInputPlugin);
    }
}

fn spawn_player(mut commands: Commands,
                asset_server: Res<AssetServer>,
                mut loading_data: ResMut<LoadingData>
) {
    let player_scene = asset_server
        .load(GltfAssetLabel::Scene(0).from_asset("entities/test-bot.glb"));

    loading_data
        .loading_assets
        .push(player_scene.clone().into());

    commands.spawn(SceneRoot(player_scene))
        .insert(Name::new("Player"))
        .insert(Transform::from_xyz(0.0, 50.0, 0.0))
        .insert(ThirdPersonCameraTarget)
        .insert(PlayerStats::default())
        .insert(RigidBody::Dynamic)
        .insert(Velocity::default())
        .insert(Grounded(true))
        .insert(Collider::capsule(Vec3::new(0.0, 0.25, 0.0), Vec3::new(0.0, 3.2, 0.0), 0.3))
        .insert(Damping {
            angular_damping: 0.5,
            linear_damping: 0.2
        })
        .insert(LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z);
}

fn create_player_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("PlayerCamera"),
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, 10.0),
        GlobalTransform::default(),
        PlayerCamera,
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
            color: Color::srgb(0.3, 0.3, 0.32),
            falloff: FogFalloff::Linear {
                start: 500.0,
                end: 600.0
            },
            ..default()
        }
    ));
}

