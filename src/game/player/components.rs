use bevy::prelude::*;

use crate::game::common::components::{Acceleration, Speed, Velocity, Direction, Health, Boost};

#[derive(Component, Default)]
pub struct Player;

#[derive(Bundle, Default)]
pub struct PlayerBundle{
    pub marker: Player,
    pub velocity: Velocity,
    pub speed: Speed,
    pub acceleration: Acceleration,
    pub direction: Direction,
    pub health: Health,
    pub boost: Boost,
    pub sprite: SpriteBundle,
}