use bevy::app::{Plugin, Update};
use events::BulletShotEvent;
use systems::*;

mod systems;
pub mod components;
pub mod events;
pub mod constants;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
        .add_event::<BulletShotEvent>()
        .add_systems(Update, (handle_bullet_shot_event, move_bullet, despawn_bullet_system));
    }
}