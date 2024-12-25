use bevy::prelude::*;
use crate::services::loading_service::LoadingServicePlugin;

pub mod loading_service;

pub struct ServicePlugin;

impl Plugin for ServicePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LoadingServicePlugin);
    }
}