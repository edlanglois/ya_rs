use bevy::prelude::*;
use crate::yar::{Yar, YarShootEvent};
use crate::zorlon_cannon::ZorlonCannon;
use crate::util::is_offscreen;

const BULLET_SPEED: f32 = 6.0;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(shoot)
            .add_system(fly)
        ;
    }
}

#[derive(Component)]
pub struct Bullet {
    velocity: Vec3,
}

pub fn shoot(
    mut commands: Commands,
    mut shoot_event: EventReader<YarShootEvent>,
    yar_query: Query<(&Transform, &Handle<TextureAtlas>, &Yar), (Without<ZorlonCannon>, Without<ZorlonCannon>)>,
    zc_query: Query<Entity, (With<ZorlonCannon>, Without<Yar>, Without<Bullet>)>,
    bullet_query: Query<&Bullet, (Without<Yar>, Without<ZorlonCannon>)>,
) {
    if shoot_event.iter().next().is_none() || !zc_query.is_empty() || yar_query.is_empty() || !bullet_query.is_empty() {
        return
    }

    let (transform, texture_atlas_handle, yar) = yar_query.single();

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite { index: 21, ..default() },
            texture_atlas: texture_atlas_handle.clone(),
            transform: transform.clone(),
            ..default()
        })
        .insert(Bullet { velocity: yar.direction_to_vector() * BULLET_SPEED });
}


pub fn fly(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &Bullet)>
) {
    if query.is_empty() {
        return
    }

    let ( e, mut transform, bullet ) = query.single_mut();

    if is_offscreen( transform.translation ) {
        commands.entity(e).despawn();
        return
    }

    transform.translation += bullet.velocity;
}