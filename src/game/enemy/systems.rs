use bevy::prelude::*;

use crate::{game::common::components::{Speed, Health}, resources::GameTextures};

use super::components::*;

pub fn spawn_enemy(
    mut commands: Commands,
    game_textures: Res<GameTextures>
){
    let mut trans = Transform::from_xyz(100.0, 50.0, 0.0);
    trans.scale = Vec3{x: 3.0, y: 3.0, z: 1.0};
    commands.spawn(EnemyBundle{
        marker: Enemy,
        enemy_type: EnemyType::Enemy1,
        speed: Speed{base_speed: 100., max_speed: 100.},
        health: Health{hp: 50, max_hp: 50},
        sprite: SpriteBundle{
            texture: game_textures.enemy1.clone(),
            transform: trans.clone(),
            ..Default::default()
        },
        ..Default::default()
    });
}

pub fn enemy_move_system(){

}