mod player;

use bevy::prelude::*;
use crate::entities::player::PlayerPlugin;

#[derive(Component, Resource, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct EntityBase {
    pub id: usize,
    pub max_health: f64,
    pub health: f64,
    pub movement_speed: f64,
    pub sprinting_speed: f64,
    pub crouching_speed: f64,
}

pub struct EntitiesPlugin;

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<EntityBase>();
        app.add_plugins(PlayerPlugin);
    }
}