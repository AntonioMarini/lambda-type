use bevy::prelude::*;

use crate::resources::{GameTextures, BULLET_TEXTURE_PATH};

pub fn setup_camera(mut commands: Commands){
    commands.spawn(Camera2dBundle::default());
}

pub fn setup_assets(mut commands: Commands, asset_server: Res<AssetServer>){
    let game_textures = GameTextures {
        player: asset_server.load("player.png"),
        bullet_base: asset_server.load(BULLET_TEXTURE_PATH),
        bullet_charged: asset_server.load(BULLET_TEXTURE_PATH),
        enemy1: asset_server.load("enemy_1.png")
    };
    commands.insert_resource(game_textures);
    println!("Game Textures loaded.");
}