use bevy::app::Plugin;
use bullet::BulletPlugin;
use enemy::EnemyPlugin;
use player::PlayerPlugin;

pub mod player;
pub mod common;
mod enemy;
mod bullet;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_plugins((PlayerPlugin, EnemyPlugin, BulletPlugin));
    }
}