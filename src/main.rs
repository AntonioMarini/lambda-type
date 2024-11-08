use bevy::prelude::*;
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
    .add_systems(PreStartup, (setup_assets, setup_camera))
    .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
    .add_plugins(MyGamepadPlugin)
    .add_plugins(GamePlugin)
    
    .run();
}
