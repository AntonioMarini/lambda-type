pub mod systems;
pub mod components;

use bevy::prelude::*;
use systems::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, spawn_player)
        .add_systems(Update, 
            (move_player_kb_system, 
            move_player_pad_system, 
            move_player_system,
            fire_player_pad_system).chain());
    }
}