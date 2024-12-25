mod splash_screen;
mod title_screen;
mod menu_screen;
mod loading_screen;

use bevy::prelude::*;
use bevy::time::Stopwatch;
use crate::manager::{AppState, InGameState};
use crate::ui::loading_screen::LoadingScreenPlugin;
use crate::ui::menu_screen::MenuScreenPlugin;
use crate::ui::splash_screen::SplashScreenPlugin;
use crate::ui::title_screen::TitleScreenPlugin;

#[derive(Component)]
struct UIViewCamera;

#[derive(Component)]
pub struct UiElement;

#[derive(Component, Resource)]
pub struct ScreenState {
    pub timer: Stopwatch,
    pub step : usize
}

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::SplashScreen), ui_view_create);
        app.add_systems(OnEnter(AppState::InGame(InGameState::Playing)), ui_view_destroy);

        app.add_plugins((SplashScreenPlugin, LoadingScreenPlugin, TitleScreenPlugin, MenuScreenPlugin));
    }
}

pub fn ui_view_create(mut commands: Commands) {
    commands
        .spawn(Camera2d::default())
        .insert(Name::new("UI View Cam"))
        .insert(UIViewCamera);

    commands.spawn(Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        ..default()
    })
        .insert(Name::new("Overlay"))
        .insert(UiElement)
        .insert(BackgroundColor(Color::srgba_u8(0, 0, 0, 255)))
        .insert(ZIndex(1));
    info!("UI View Cam created");
}

fn ui_view_destroy(mut commands: Commands, query: Query<Entity, With<UIViewCamera>>) {
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).despawn_recursive();
        info!("UI View Cam destroyed");
    }
}

pub fn destroy_screen(mut commands: Commands, query: Query<(Entity, &Name), With<Name>>) {
    for (entity, name) in query.iter() {
        if name.as_str().eq("Container-Main") {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn play_life_cycle(In(entity): In<Entity>,
                       mut commands: Commands
) {
    commands.entity(entity).insert(Name::new("Container-Main"));
}

pub fn handle_delay_step(screen_state: &mut ResMut<ScreenState>, next_step: usize, wait: f32) {
    if screen_state.timer.elapsed_secs() >= wait {
        screen_state.step = next_step;
        screen_state.timer.reset();
    }
}

pub fn handle_fade_out_step(
    screen_state: &mut ResMut<ScreenState>,
    query: &mut Query<(&mut BackgroundColor, &Name), With<Name>>,
    time: Res<Time>,
    next_step: usize,
) {
    for (mut background_color, name) in query.iter_mut() {
        if name.as_str().starts_with("Overlay") {
            if let BackgroundColor(Color::Srgba(Srgba { red, green, blue, alpha })) = *background_color {
                let reduction_speed = 0.5;
                let delta_alpha = reduction_speed * time.delta_secs();
                let new_alpha = (alpha - delta_alpha).max(0.0);
                *background_color = BackgroundColor(Color::Srgba(Srgba { red, green, blue, alpha: new_alpha }));
                if new_alpha == 0.0 {
                    screen_state.step = next_step;
                }
            }
        }
    }
}

pub fn handle_fade_in_step(
    screen_state: &mut ResMut<ScreenState>,
    query: &mut Query<(&mut BackgroundColor, &Name), With<Name>>,
    time: Res<Time>,
    next_step: usize,
) {
    for (mut background_color, name) in query.iter_mut() {
        if name.as_str().starts_with("Overlay") {
            if let BackgroundColor(Color::Srgba(Srgba { red, green, blue, alpha })) = *background_color {
                let reduction_speed = 0.5;
                let delta_alpha = reduction_speed * time.delta_secs();
                let new_alpha = (alpha + delta_alpha).min(1.0);
                *background_color = BackgroundColor(Color::Srgba(Srgba { red, green, blue, alpha: new_alpha }));
                if new_alpha == 1.0 {
                    screen_state.step = next_step;
                }
            }
        }
    }
}