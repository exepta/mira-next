mod splash_screen;

use bevy::prelude::*;
use crate::manager::{AppState, InGameState};
use crate::ui::splash_screen::SplashScreenPlugin;

#[derive(Component)]
struct UIViewCamera;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::SplashScreen), ui_view_create);
        app.add_systems(OnEnter(AppState::InGame(InGameState::Playing)), ui_view_destroy);

        app.add_plugins(SplashScreenPlugin);
    }
}

fn ui_view_create(mut commands: Commands) {
    commands
        .spawn(Camera2d::default())
        .insert(Name::new("UI View Cam"))
        .insert(UIViewCamera);
    info!("UI View Cam created");
}

fn ui_view_destroy(mut commands: Commands, query: Query<Entity, With<UIViewCamera>>) {
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).despawn_recursive();
        info!("UI View Cam destroyed");
    }
}