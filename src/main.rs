mod manager;
mod ui;
mod entities;
mod services;
mod environment;

use bevy::image::ImageSamplerDescriptor;
use bevy::prelude::*;
use bevy::render::RenderPlugin;
use bevy::render::settings::{Backends, RenderCreation, WgpuFeatures, WgpuSettings};
use bevy::window::WindowResolution;
use crate::manager::{AppState, ManagerPlugin};

fn main() -> AppExit {
    let mut app = App::new();
    at_initialize_bevy(&mut app).run()
}

fn at_start_up(mut next_state: ResMut<NextState<AppState>>) {
    info!("Starting app...");
    next_state.set(AppState::SplashScreen);
}

fn at_initialize_bevy(app: &mut App) -> &mut App {
    info!("Initializing Bevy App...");
    app
        .add_plugins(DefaultPlugins.set(
            WindowPlugin {
                primary_window: Some(Window {
                    title: "Mira Game - 0.15 bevy".to_string(),
                    resolution: WindowResolution::new(1270.0, 720.0),
                    ..default()
                }),
                ..default()
            }
        ).set(
            RenderPlugin {
                render_creation: RenderCreation::Automatic(create_gpu_settings()),
                ..default()
            }
        ).set(
            ImagePlugin {
                default_sampler: ImageSamplerDescriptor::nearest(),
            }
        ))
        .add_systems(OnEnter(AppState::Startup), at_start_up)
        .add_plugins(ManagerPlugin)
}

fn create_gpu_settings() -> WgpuSettings {
    WgpuSettings {
        features: WgpuFeatures::POLYGON_MODE_LINE,
        backends: Some(Backends::VULKAN),
        ..default()
    }
}

