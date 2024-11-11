use bevy::prelude::*;

use crate::{resources::{GameTextures, BULLET_TEXTURE_PATH, ENEMY_1_PATH, PLAYER_TEXTURE_PATH}, AppState};

pub fn setup_camera(mut commands: Commands){
    commands.spawn(Camera2dBundle::default());
}

pub fn setup_assets(mut commands: Commands, asset_server: Res<AssetServer>){
    let game_textures = GameTextures {
        player: asset_server.load(PLAYER_TEXTURE_PATH),
        bullet_base: asset_server.load(BULLET_TEXTURE_PATH),
        bullet_charged: asset_server.load(BULLET_TEXTURE_PATH),
        enemy1: asset_server.load(ENEMY_1_PATH)
    };
    commands.insert_resource(game_textures);
    println!("Game Textures loaded.");
}

pub fn check_assets_ready(
    server: Res<AssetServer>,
    game_textures: Res<GameTextures>,
    mut next_state: ResMut<NextState<AppState>>
) {
    use bevy::asset::LoadState;

    match server.get_load_state(&game_textures.bullet_base).unwrap() {
        LoadState::NotLoaded => {return;},
        LoadState::Loading => {},
        LoadState::Loaded => {},
        LoadState::Failed(asset_load_error) => {
            println!("Error: {}", asset_load_error);
        },
    }

    match server.get_load_state(&game_textures.player).unwrap() {
        LoadState::NotLoaded => {return;},
        LoadState::Loading => {},
        LoadState::Loaded => {},
        LoadState::Failed(asset_load_error) => {println!("Error: {}", asset_load_error);},
    }

    match server.get_load_state(&game_textures.enemy1).unwrap() {
        LoadState::NotLoaded => {return;},
        LoadState::Loading => {},
        LoadState::Loaded => {},
        LoadState::Failed(asset_load_error) => {println!("Error: {}", asset_load_error);},
    }

    match server.get_load_state(&game_textures.bullet_charged).unwrap() {
        LoadState::NotLoaded => {return;},
        LoadState::Loading => {},
        LoadState::Loaded => {},
        LoadState::Failed(asset_load_error) => {println!("Error: {}", asset_load_error);},
    }

    next_state.set(AppState::Game);
}