use bevy::prelude::*;

use crate::{game::common::components::{Direction, LifeTime, Speed}, resources::GameTextures};

use super::{components::{Bullet, BulletBundle, BulletType}, events::BulletShotEvent};

pub fn handle_bullet_shot_event(
    mut bullet_shot_event_reader: EventReader<BulletShotEvent>,
    game_textures: Res<GameTextures>,
    mut commands: Commands
){

    // select the right asset for the bullet type.
    for bullet_shot in bullet_shot_event_reader.read() {
        let bullet_image = match bullet_shot.bullet_type {
            BulletType::BaseBullet => game_textures.bullet_base.clone(),
            BulletType::ChargedBullet => game_textures.bullet_base.clone(),
            BulletType::Beam => game_textures.bullet_base.clone(),
            BulletType::ChargedBeam => game_textures.bullet_base.clone(),
        };

        spawn_bullet(&mut commands, BulletBundle {
            marker: Bullet,
            bullet_type: bullet_shot.bullet_type,
            shot_by: bullet_shot.shot_by,
            direction: bullet_shot.direction,
            speed: bullet_shot.bullet_speed,
            damage: bullet_shot.damage,
            lifetime: bullet_shot.lifetime,
            sprite: SpriteBundle {
                transform: bullet_shot.transform,
                texture: bullet_image,
                ..Default::default()
            }
        });
    }
}

pub fn spawn_bullet(
    commands: &mut Commands,
    bullet: BulletBundle
) {
    commands.spawn(bullet);
}

pub fn move_bullet(
    mut query: Query<(&Direction, &Speed, &mut Transform), With<Bullet>>,
    time: Res<Time>
){
    for (direction, speed, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += direction.direction.x * speed.base_speed *time.delta_seconds();
        translation.y += direction.direction.y * speed.base_speed *time.delta_seconds();
    }
}

pub fn despawn_bullet_system(
    mut query: Query<(Entity, &mut LifeTime), With<Bullet>>,
    time: Res<Time>,
    mut commands: Commands
){
    for (entity, mut lifetime) in query.iter_mut(){
        if lifetime.lifetime_millis <= 0 {
            println!("Bullet id: {} despawn", entity.index());
            commands.entity(entity).despawn();
        }
        lifetime.lifetime_millis -= time.delta().as_millis() as i128;
    }
}