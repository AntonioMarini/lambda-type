use bevy::prelude::*;
use systems::*;

use crate::AppState;

pub mod systems;
pub mod components;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
        .add_systems(OnEnter(AppState::Game), spawn_enemy)
        .add_systems(Update, (enemy_move_system).run_if(in_state(AppState::Game)));
    }
}