use std::env;

use bevy::prelude::*;
use bevy_rapier2d::{plugin::{NoUserData, RapierPhysicsPlugin}, render::RapierDebugRenderPlugin};
use game::GamePlugin;
use gamepad::MyGamepadPlugin;
use systems::*;

pub mod resources;
pub mod systems;
pub mod gamepad;

mod game;
mod main_menu;

fn main() {    
    App::new()
    .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
    .init_state::<AppState>()
    .add_systems(Startup, (setup_assets, setup_camera))
    .add_systems(Update, check_assets_ready.run_if(in_state(AppState::AssetLoading)))
    .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
    .add_plugins(RapierDebugRenderPlugin::default())
    .add_plugins(MyGamepadPlugin)
    .add_plugins(GamePlugin)
    
    .run();
}

#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Default)]
pub enum AppState {
    #[default]
    AssetLoading,
    Game,
}