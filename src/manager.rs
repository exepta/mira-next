use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::{DebugRenderStyle, NoUserData, RapierDebugRenderPlugin, RapierPhysicsPlugin};
use crate::ui::UIPlugin;

#[derive(Component, Resource, States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    #[default]
    Startup,
    SplashScreen,
    TitleScreen,
    MainMenu(MenuState),
    InGame(InGameState),
    Loading(LoadingState),
    Quit,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum MenuState {
    #[default]
    MainUi,
    SettingsUi,
    AccountUi,
    CreditsUI
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum InGameState {
    #[default]
    Playing,
    Paused,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum LoadingState {
    #[default]
    Ready,
    Progressing
}

pub struct ManagerPlugin;

impl Plugin for ManagerPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>();

        app.add_plugins(WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::F3)));

        app
            .add_plugins((
                RapierPhysicsPlugin::<NoUserData>::default(),
                plugin_init_rapier3d_debug()
            ));

        app.add_plugins(UIPlugin);
    }
}

fn plugin_init_rapier3d_debug() -> RapierDebugRenderPlugin {
    RapierDebugRenderPlugin {
        enabled: true,
        style: DebugRenderStyle {
            collider_parentless_color: [0.0, 1.0, 1.0, 1.0],
            collider_dynamic_color: [305.0, 1.0, 0.5, 1.0],
            collider_fixed_color: [65.0, 1.0, 0.5, 1.0],
            collider_kinematic_color: [140.0, 1.0, 0.5, 1.0],
            sleep_color_multiplier: [0.0, 0.5, 0.5, 1.0],
            ..default()
        },
        ..default()
    }
}