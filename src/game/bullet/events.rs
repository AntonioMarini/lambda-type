use bevy::prelude::{Event, Transform};

use crate::game::common::components::{Actors, Damage, Direction, LifeTime, Speed};

use super::components::BulletType;

#[derive(Event)]
pub struct BulletShotEvent {
    pub shot_by: Actors,
    pub bullet_speed: Speed,
    pub lifetime: LifeTime,
    pub transform: Transform,
    pub direction: Direction,
    pub bullet_type: BulletType,
    pub damage: Damage
}