use bevy::prelude::*;

use crate::game::common::components::{Speed, Health, Direction, Velocity};

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
    pub speed: Speed,
    pub direction: Direction,
    pub velocity: Velocity,
    pub health: Health,
    pub sprite: SpriteBundle
}