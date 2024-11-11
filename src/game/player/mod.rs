pub mod systems;
pub mod components;

use bevy::prelude::*;
use systems::*;

use crate::AppState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(AppState::Game), spawn_player)
        .add_systems(Update, 
            (
                rotate_player_system,
                move_player_system,
                shoot_system
            ).run_if(in_state(AppState::Game)));
    }
}