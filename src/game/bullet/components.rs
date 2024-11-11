use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

use crate::game::common::components::{Actors, Damage, LifeTime, Orientation, Speed};

#[derive(Component, Clone, Copy, Default)]
#[allow(dead_code)]
pub enum BulletType {
    #[default]
    BaseBullet,
    ChargedBullet,
    Beam,
    ChargedBeam
}

#[derive(Component, Default)]
pub struct Bullet;

#[derive(Bundle, Default)]
pub struct BulletBundle{
    pub marker: Bullet,
    pub bullet_type: BulletType,
    pub shot_by: Actors,
    pub orientation: Orientation,
    pub velocity: Velocity,
    pub speed: Speed,
    pub damage: Damage,
    pub sprite: SpriteBundle,
    pub lifetime: LifeTime,
}