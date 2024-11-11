use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

use crate::game::common::components::{ Boost, Health, Orientation, Speed};

#[derive(Component, Default)]
pub struct Player;

#[derive(Bundle, Default)]
pub struct PlayerBundle{
    pub marker: Player,
    pub health: Health,
    pub speed: Speed,
    pub boost: Boost,
    pub orientation: Orientation,
    pub sprite: SpriteBundle, 
    pub velocity: Velocity,
}