mod player;

use bevy::prelude::*;
use crate::entities::player::PlayerPlugin;

#[derive(Component, Resource, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct EntityBase {
    pub id: usize,
    pub max_health: f32,
    pub health: f32,
    pub speed: f32,
    pub movement_speed_multi: f32,
    pub sprinting_speed_multi: f32,
    pub crouching_speed_multi: f32,
}

impl Default for EntityBase {
    fn default() -> Self {
        Self {
            id: 1,
            max_health: 320.0,
            health: 320.0,
            speed: 20.0,
            movement_speed_multi: 1.2,
            sprinting_speed_multi: 3.0,
            crouching_speed_multi: 0.75,
        }
    }
}

pub struct EntitiesPlugin;

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<EntityBase>();
        app.add_plugins(PlayerPlugin);
    }
}