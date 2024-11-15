use bevy::prelude::*;

pub const PLAYER_TEXTURE_PATH: &str = "sprites/playerShip1_red.png";
pub const BULLET_TEXTURE_PATH: &str = "bullet_1.png";
pub const ENEMY_1_PATH: &str = "sprites/enemy1.png";

#[derive(Resource)]
pub struct GameTextures{
    pub player: Handle<Image>,
    pub bullet_base: Handle<Image>,
    pub bullet_charged: Handle<Image>,
    pub enemy1: Handle<Image>
}