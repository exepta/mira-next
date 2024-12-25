use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy::remote::http::RemoteHttpPlugin;
use bevy::remote::RemotePlugin;
use bevy::winit::WinitWindows;
use bevy_hui::HuiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_kira_audio::AudioPlugin;
use bevy_rapier3d::prelude::{DebugRenderStyle, NoUserData, RapierDebugRenderPlugin, RapierPhysicsPlugin};
use winit::window::Icon;
use crate::entities::EntitiesPlugin;
use crate::services::ServicePlugin;
use crate::ui::UIPlugin;

#[derive(Component, Resource, States, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[allow(dead_code)]
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
#[allow(dead_code)]
pub enum MenuState {
    #[default]
    MainUi,
    SettingsUi,
    AccountUi,
    CreditsUI
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum InGameState {
    #[default]
    Playing,
    Paused,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
#[allow(dead_code)]
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
        app.add_plugins(AudioPlugin);
        app.add_plugins((HuiPlugin, RemotePlugin::default(), RemoteHttpPlugin::default()));

        app
            .add_plugins((
                RapierPhysicsPlugin::<NoUserData>::default(),
                plugin_init_rapier3d_debug()
            ));

        app.add_plugins((ServicePlugin, UIPlugin, EntitiesPlugin));

        app.add_systems(Startup, load_window_icon);
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

fn load_window_icon(windows: NonSend<WinitWindows>) {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("assets/logos/mira-icon.png")
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

    for window in windows.windows.values() {
        window.set_window_icon(Some(icon.clone()));
    }
}