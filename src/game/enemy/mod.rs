use bevy::prelude::*;
use systems::*;

pub mod systems;
pub mod components;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
        .add_systems(Startup, spawn_enemy)
        .add_systems(Update, enemy_move_system);
    }
}