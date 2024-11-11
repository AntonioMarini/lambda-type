use bevy::prelude::{Event, Transform};

use crate::game::common::components::{Actors, Damage, LifeTime, Orientation, Speed};

use super::components::BulletType;

#[derive(Event)]
pub struct BulletShotEvent {
    pub shot_by: Actors,
    pub orientation: Orientation,
    pub bullet_speed: Speed,
    pub lifetime: LifeTime,
    pub transform: Transform,
    pub bullet_type: BulletType,
    pub damage: Damage
}