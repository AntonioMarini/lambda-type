use bevy::prelude::*;

use crate::{game::{bullet::{components::BulletType, constants::{BASE_BULLET_DAMAGE, BASE_BULLET_LIFETIME, BASE_BULLET_SPEED}, events::BulletShotEvent}, common::components::{Actors, Damage, Direction, LifeTime, Speed, Velocity}}, gamepad::MyGamepad, resources::GameTextures};

use super::components::*;

const PLAYER_BASE_SPEED : f32 = 500.0;
const PLAYER_MAX_SPEED : f32 = 700.0;

pub fn spawn_player(mut commands: Commands, game_textures: Res<GameTextures>) {

    let mut trans = Transform::from_xyz(25.0, 50.0, 0.0);
    trans.scale = Vec3{x: 3.0, y: 3.0, z: 3.0};
    commands.spawn(PlayerBundle {
        marker: Player,
        speed: Speed { base_speed: PLAYER_BASE_SPEED, max_speed: PLAYER_MAX_SPEED},
        sprite: SpriteBundle {
            texture: game_textures.player.clone(),
            transform: trans.clone(),
            ..Default::default()
        },
        ..Default::default()
    });
}

pub fn move_player_kb_system(
    keys: Res<ButtonInput<KeyCode>>, 
    time: Res<Time>, 
    mut query : Query<(&Speed, &mut Direction, &mut Velocity), With<Player>>
){
    if let Ok((speed, mut direction,mut velocity)) = query.get_single_mut() {
        
        if keys.pressed(KeyCode::KeyA){
            direction.direction.x = -1.;
            direction.direction.y = 0.;
        }else if keys.pressed(KeyCode::KeyD){
            direction.direction.x = 1.;
            direction.direction.y = 0.;
        }else if keys.pressed(KeyCode::KeyW){
            direction.direction.y = 1.;
            direction.direction.x = 0.;
        }else if keys.pressed(KeyCode::KeyS){
            direction.direction.y = -1.;
            direction.direction.x = 0.;
        }else {
            direction.direction.x = 0.;
            direction.direction.y = 0.;
        }

        velocity.velocity.x = direction.direction.x * speed.base_speed * time.delta_seconds(); 
        velocity.velocity.y = direction.direction.y * speed.base_speed * time.delta_seconds();
    }
}

pub fn move_player_pad_system(
    axes: Res<Axis<GamepadAxis>>,
    my_gamepad: Option<Res<MyGamepad>>,
    time: Res<Time>, 
    mut query: Query<(&Speed, &mut Direction, &mut Velocity), With<Player>>
) {
    let Some(&MyGamepad(gamepad)) = my_gamepad.as_deref() else {
        // no gamepad is connected
        return;
    };

    // The joysticks are represented using a separate axis for X and Y
    let axis_lx = GamepadAxis {
        gamepad, axis_type: GamepadAxisType::LeftStickX
    };
    let axis_ly = GamepadAxis {
        gamepad, axis_type: GamepadAxisType::LeftStickY
    };

    if let (Some(x), Some(y)) = (axes.get(axis_lx), axes.get(axis_ly)) {
        // combine X and Y into one vector
        if let Ok((speed, mut direction, mut velocity)) = query.get_single_mut(){
           direction.direction.x = x;
           direction.direction.y = y;
           velocity.velocity.x = direction.direction.x * speed.base_speed * time.delta_seconds(); 
           velocity.velocity.y = direction.direction.y * speed.base_speed * time.delta_seconds();
        }
    }
}

pub fn move_player_system(mut player_pos: Query<(&Velocity, &mut Transform), With<Player>>){
    for (velocity, mut transform) in player_pos.iter_mut(){
        let translation = &mut transform.translation;
        translation.x += velocity.velocity.x;
        translation.y += velocity.velocity.y;
    }
}

pub fn fire_player_pad_system(
    buttons: Res<ButtonInput<GamepadButton>>,
    my_gamepad: Option<Res<MyGamepad>>,
    mut query: Query<&mut Transform, With<Player>>,
    mut bullet_event_writer: EventWriter<BulletShotEvent>
){

    let Some(&MyGamepad(gamepad)) = my_gamepad.as_deref() else {
        // no gamepad is connected
        return;
    };

    // In a real game, the buttons would be configurable, but here we hardcode them
    let shoot_button = GamepadButton {
        gamepad, button_type: GamepadButtonType::West
    };
    
    if buttons.just_pressed(shoot_button) {
        if let Ok(transform) = query.get_single_mut(){
            // fire an event that spawns a bullet
            bullet_event_writer.send(BulletShotEvent {
                shot_by: Actors::Player,
                bullet_speed:  Speed{base_speed: BASE_BULLET_SPEED, max_speed: BASE_BULLET_SPEED},
                lifetime: LifeTime {lifetime_millis: BASE_BULLET_LIFETIME},
                direction: Direction { direction: Vec2::new(1., 0.) },
                bullet_type: BulletType::BaseBullet,
                transform: transform.clone(),
                damage: Damage {damage: BASE_BULLET_DAMAGE}
            });
        }
    }
}