use bevy::{app::{Plugin, Update}, prelude::{in_state, IntoSystemConfigs}};
use systems::*;

use crate::AppState;

mod systems;

pub struct CollisionsPlugin;

impl Plugin for CollisionsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, check_collisions_system.run_if(in_state(AppState::Game)));
    }
}