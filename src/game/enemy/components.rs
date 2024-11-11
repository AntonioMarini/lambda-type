use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

use crate::game::common::components::{Health, Speed};

#[derive(Component, Default)]
pub struct Enemy;

#[derive(Component, Default)]
pub enum EnemyType{
    #[default]
    Enemy1
}

#[derive(Bundle, Default)]
pub struct EnemyBundle{
    pub marker: Enemy,
    pub enemy_type: EnemyType,
    pub health: Health,
    pub speed: Speed,
    pub sprite: SpriteBundle,
    pub velocity: Velocity
}