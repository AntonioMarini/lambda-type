use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, RigidBody, Velocity};

use crate::{game::common::components::{LifeTime, Orientation, Speed}, 
    resources::GameTextures
};

use super::{components::{Bullet, BulletBundle, BulletType}, events::BulletShotEvent};

pub fn handle_bullet_shot_event(
    mut bullet_shot_event_reader: EventReader<BulletShotEvent>,
    game_textures: Res<GameTextures>,
    texture_assets: Res<Assets<Image>>,
    asset_server: Res<AssetServer>, // Access the image assets
    mut commands: Commands
){
    let bullet_img = game_textures.bullet_base.clone();
    let scale = Vec3::new(3., 3., 1.);
    let collider_size = if let Some(image) = texture_assets.get(&bullet_img) {
        // The size is stored as a Vec2 in the image's texture descriptor
        Vec2::new(image.size_f32().x, image.size_f32().y)
    } else {
        // Default or fallback size if texture isn't loaded yet
        Vec2::new(50., 100.0)
    };

    // select the right asset for the bullet type.
    for bullet_shot in bullet_shot_event_reader.read() {
        let bullet_image = match bullet_shot.bullet_type {
            BulletType::BaseBullet => game_textures.bullet_base.clone(),
            BulletType::ChargedBullet => game_textures.bullet_base.clone(),
            BulletType::Beam => game_textures.bullet_base.clone(),
            BulletType::ChargedBeam => game_textures.bullet_base.clone(),
        };

        let transform = bullet_shot.transform.with_scale(
            scale
        );

        // bullet audio
        commands.spawn(AudioBundle {
            source: asset_server.load("audio/laserLarge_002.ogg"),
            settings: PlaybackSettings::ONCE,
        });

        spawn_bullet(&mut commands, BulletBundle {
            marker: Bullet,
            bullet_type: bullet_shot.bullet_type,
            shot_by: bullet_shot.shot_by,
            orientation: bullet_shot.orientation,
            speed: bullet_shot.bullet_speed,
            damage: bullet_shot.damage,
            lifetime: bullet_shot.lifetime,
            sprite: SpriteBundle {
                transform: transform,
                texture: bullet_image,
                ..Default::default()
            },
            ..Default::default()
        },
        Collider::capsule_x(collider_size.x/4., collider_size.x/8.),
    );
    }
}

pub fn spawn_bullet(
    commands: &mut Commands,
    bullet: BulletBundle,
    collider: Collider
) {
    commands.spawn(RigidBody::KinematicVelocityBased)
    .insert(Velocity{..Default::default()})
    .insert(bullet)
    .insert(collider);
}

pub fn move_bullet(
    mut query: Query<(&mut Velocity, &Orientation, &Speed), With<Bullet>>,
){
    for (mut velocity, orientation, speed) in query.iter_mut() {
        velocity.linvel = orientation.value * speed.value;
    }
}

pub fn bullet_lifetime_system(
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