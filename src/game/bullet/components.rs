use bevy::prelude::*;

use crate::game::common::components::{LifeTime, Speed, Direction, Damage, Actors};

#[derive(Component, Clone, Copy)]
#[allow(dead_code)]
pub enum BulletType {
    BaseBullet,
    ChargedBullet,
    Beam,
    ChargedBeam
}

#[derive(Component, Default)]
pub struct Bullet;

#[derive(Bundle)]
pub struct BulletBundle{
    pub marker: Bullet,
    pub bullet_type: BulletType,
    pub shot_by: Actors,
    pub direction: Direction,
    pub speed: Speed,
    pub damage: Damage,
    pub sprite: SpriteBundle,
    pub lifetime: LifeTime
}