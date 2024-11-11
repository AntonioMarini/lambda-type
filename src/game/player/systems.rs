use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::{ActiveCollisionTypes, ActiveEvents, Collider, RigidBody, Velocity};

use crate::{game::{bullet::{components::BulletType, 
constants::{BASE_BULLET_DAMAGE, BASE_BULLET_LIFETIME, BASE_BULLET_SPEED}, events::BulletShotEvent}, common::components::{Actors, Damage, LifeTime, Orientation, Speed}}, gamepad::MyGamepad, resources::GameTextures};

use super::components::*;

const PLAYER_BASE_SPEED : f32 = 500.0;

pub fn spawn_player(
    mut commands: Commands, 
    game_textures: Res<GameTextures>,
    texture_assets: Res<Assets<Image>>, // Access the image assets
) {
    let transform = Transform::from_xyz(25.0, 50.0, 0.0);
    let player_img = game_textures.player.clone();

     let collider_size = if let Some(image) = texture_assets.get(&player_img) {
        // The size is stored as a Vec2 in the image's texture descriptor
        Vec2::new(image.size_f32().x, image.size_f32().y)
    } else {
        // Default or fallback size if texture isn't loaded yet
        Vec2::new(75.0, 99.0)
    };


    commands.spawn((RigidBody::KinematicVelocityBased,PlayerBundle {
        marker: Player,
        sprite: SpriteBundle {
            texture: player_img,
            transform: transform,
            ..Default::default()
        },
        speed: Speed { value: PLAYER_BASE_SPEED},
        ..Default::default()
    }))
    .insert(Collider::cuboid(collider_size.x/2., collider_size.y/2.))
    .insert(ActiveEvents::COLLISION_EVENTS)
    .insert(ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_KINEMATIC);
}

pub fn move_player_system(
    axes: Res<Axis<GamepadAxis>>,
    my_gamepad: Option<Res<MyGamepad>>,
    keys: Res<ButtonInput<KeyCode>>, 
    mut query : Query<(&mut Velocity, &Speed), With<Player>>
){
    if let Ok((mut velocity, speed)) = query.get_single_mut() {

        let mut direction = Vec2::ZERO;
        // Keyboard
        
        if keys.pressed(KeyCode::KeyA){
            direction.x += -1.;
        } 
        if keys.pressed(KeyCode::KeyD){
            direction.x += 1.;
        } 
        if keys.pressed(KeyCode::KeyW){
            direction.y += 1.;
        }
        if keys.pressed(KeyCode::KeyS){
            direction.y += -1.;
        }

        // Controller

        if let Some(&MyGamepad(gamepad)) = my_gamepad.as_deref() {
            let axis_lx = GamepadAxis {
                gamepad, axis_type: GamepadAxisType::LeftStickX
            };
            let axis_ly = GamepadAxis {
                gamepad, axis_type: GamepadAxisType::LeftStickY
            };

            if let (Some(x), Some(y)) = (axes.get(axis_lx), axes.get(axis_ly)) {
                // combine X and Y into one vector
                direction.x += x;
                direction.y += y;    
            }
        }

        velocity.linvel = direction * speed.value;
    }
}

pub fn rotate_player_system(
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut query : Query<(&mut Transform, &mut Orientation), With<Player>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
){
    if let Ok((mut transform, mut orientation)) = query.get_single_mut() {
        let window = q_windows.single();

        if let Some(cursor_position) = window.cursor_position(){
            let (camera, camera_transform) = camera_query.single();
            let pos = transform.translation.truncate(); // player position
            let target = camera.viewport_to_world_2d(camera_transform, cursor_position).unwrap();
            orientation.value = target - pos;
            let angle = orientation.value.to_angle();
            orientation.value = orientation.value.normalize();

            transform.rotation = Quat::from_axis_angle(Vec3::new(0., 0., 1.), angle);
        }
    }
}

pub fn shoot_system(
    buttons: Res<ButtonInput<GamepadButton>>,
    my_gamepad: Option<Res<MyGamepad>>,
    mouse_input: Res<ButtonInput<MouseButton>>, 
    mut query: Query<(&mut Transform, &Orientation), With<Player>>,
    mut bullet_event_writer: EventWriter<BulletShotEvent>
){
    if let Ok((transform, orientation)) = query.get_single_mut(){
        let mut has_shoot: bool = false;

        if let Some(&MyGamepad(gamepad)) = my_gamepad.as_deref() {
            // In a real game, the buttons would be configurable, but here we hardcode them
            let shoot_button = GamepadButton {
                gamepad, button_type: GamepadButtonType::West
            };
            
            has_shoot = buttons.just_pressed(shoot_button);
        }

        if mouse_input.just_pressed(MouseButton::Left) {
            has_shoot = true;
        }

        if has_shoot {
            bullet_event_writer.send(BulletShotEvent {
                shot_by: Actors::Player,
                bullet_speed:  Speed{value: BASE_BULLET_SPEED},
                lifetime: LifeTime {lifetime_millis: BASE_BULLET_LIFETIME},
                bullet_type: BulletType::BaseBullet,
                transform: transform.clone(),
                damage: Damage {damage: BASE_BULLET_DAMAGE},
                orientation: orientation.clone()
            });
        }
    }
}