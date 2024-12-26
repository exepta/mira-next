use bevy::prelude::*;
use bevy_kira_audio::AudioInstance;
use crate::services::loading_service::LoadingServicePlugin;

pub mod loading_service;

pub struct ServicePlugin;

#[derive(Resource)]
#[allow(dead_code)]
pub struct AudioHandle(pub Handle<AudioInstance>);

impl Plugin for ServicePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LoadingServicePlugin);
    }
}