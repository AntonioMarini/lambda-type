use bevy::{math::NormedVectorSpace, prelude::*};
use bevy_rapier2d::prelude::{Collider, RigidBody, Sensor, Velocity};

use crate::{
    game::{
        common::components::{Health, Orientation, Speed},
        player::components::Player,
    },
    resources::GameTextures,
};

use super::components::*;

pub fn spawn_enemy(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    texture_assets: Res<Assets<Image>>, // Access the image assets
) {
    let enemy_img = game_textures.enemy1.clone();
    let collider_size = if let Some(image) = texture_assets.get(&enemy_img) {
        // The size is stored as a Vec2 in the image's texture descriptor
        Vec2::new(image.size_f32().x, image.size_f32().y)
    } else {
        // Default or fallback size if texture isn't loaded yet
        Vec2::new(100., 100.0)
    };

    let trans = Transform::from_xyz(100.0, 50.0, 0.0);
    commands
        .spawn((
            EnemyBundle {
                marker: Enemy,
                enemy_type: EnemyType::Enemy1,
                health: Health { hp: 50, max_hp: 50 },
                speed: Speed { value: 100. },
                sprite: SpriteBundle {
                    texture: game_textures.enemy1.clone(),
                    transform: trans.clone(),
                    ..Default::default()
                },
                ..Default::default()
            },
            RigidBody::KinematicVelocityBased,
            Collider::cuboid(collider_size.x / 2., collider_size.y / 2.),
        ))
        .insert(Sensor);
}

pub fn follow_player(
    player_query: Query<&Transform, With<Player>>,
    mut enemies_query: Query<(&Transform, &mut Velocity, &Speed), With<Enemy>>,
) {
    if let Ok(player_trans) = player_query.get_single() {
        for (enemy_trans, mut enemy_velocity, speed) in enemies_query.iter_mut() {
            if enemy_trans
                .translation
                .truncate()
                .distance(player_trans.translation.truncate())
                < 50.
            {
                break;
            }
            let target = player_trans.translation.truncate() - enemy_trans.translation.truncate();
            enemy_velocity.linvel = target.normalize() * speed.value;
        }
    }
}

pub fn rotate_enemy(mut enemies_query: Query<(&mut Transform, &Velocity), With<Enemy>>) {
    for (mut enemy_trans, enemy_velocity) in enemies_query.iter_mut() {
        let angle = enemy_velocity.linvel.to_angle();
        enemy_trans.rotation = Quat::from_axis_angle(Vec3::new(0., 0., 1.), angle);
    }
}
