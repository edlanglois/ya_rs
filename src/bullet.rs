use crate::neutral_zone::{NeutralZone, NEUTRAL_ZONE_BOUNDS};
use crate::shield::{ShieldBlock, ShieldHealth, SHIELD_BLOCK_SPRITE_SIZE};
use crate::util;
use crate::yar::{Yar, YarDiedEvent, YarShootEvent, YAR_BOUNDS};
use crate::zorlon_cannon::ZorlonCannon;
use crate::SCREEN_SCALE;
use bevy::math::const_vec2;
use bevy::prelude::*;

const BULLET_SPEED: f32 = 6.0;
const BULLET_BOUNDS: Vec2 = const_vec2!([2.0 * SCREEN_SCALE, 2.0 * SCREEN_SCALE]);

pub struct DespawnBulletEvent;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DespawnBulletEvent>()
            .add_system(despawn)
            .add_system(shoot)
            .add_system(fly)
            .add_system(collide_shield);
    }
}

#[derive(Component)]
pub struct Bullet {
    velocity: Vec3,
}

pub fn despawn(
    mut commands: Commands,
    mut despawn_event: EventReader<DespawnBulletEvent>,
    mut death_event: EventReader<YarDiedEvent>,
    query: Query<Entity, With<Bullet>>,
) {
    if (despawn_event.iter().next().is_none() && death_event.iter().next().is_none())
        || query.is_empty()
    {
        return;
    }

    let e = query.single();
    commands.entity(e).despawn();
}

pub fn shoot(
    mut commands: Commands,
    mut shoot_event: EventReader<YarShootEvent>,
    yar_query: Query<
        (&Transform, &Handle<TextureAtlas>, &Yar),
        (Without<ZorlonCannon>, Without<ZorlonCannon>),
    >,
    zc_query: Query<Entity, (With<ZorlonCannon>, Without<Yar>, Without<Bullet>)>,
    bullet_query: Query<&Bullet, (Without<Yar>, Without<ZorlonCannon>)>,
    nz_query: Query<&Transform, With<NeutralZone>>,
) {
    if shoot_event.iter().next().is_none()
        || !zc_query.is_empty()
        || yar_query.is_empty()
        || !bullet_query.is_empty()
    {
        return;
    }

    let (transform, texture_atlas_handle, yar) = yar_query.single();

    if !nz_query.is_empty() {
        let nz_transform = nz_query.single();

        // Yar cannot shoot while in the Neutral Zone
        if util::intersect_rect(
            &transform.translation,
            &YAR_BOUNDS,
            &nz_transform.translation,
            &NEUTRAL_ZONE_BOUNDS,
        ) {
            return;
        }
    }

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 21,
                ..default()
            },
            texture_atlas: texture_atlas_handle.clone(),
            transform: transform.clone(),
            ..default()
        })
        .insert(Bullet {
            velocity: yar.direction_to_vector() * BULLET_SPEED,
        });
}

pub fn fly(
    mut despawn_event: EventWriter<DespawnBulletEvent>,
    mut query: Query<(&mut Transform, &Bullet)>,
) {
    if query.is_empty() {
        return;
    }

    let (mut transform, bullet) = query.single_mut();

    if util::is_offscreen(transform.translation) {
        despawn_event.send(DespawnBulletEvent);
        return;
    }

    transform.translation += bullet.velocity;
}

pub fn collide_shield(
    mut despawn_event: EventWriter<DespawnBulletEvent>,
    mut shield_query: Query<(&Transform, &mut ShieldHealth, &ShieldBlock), Without<Bullet>>,
    bullet_query: Query<&Transform, (With<Bullet>, Without<ShieldBlock>)>,
) {
    if shield_query.is_empty() || bullet_query.is_empty() {
        return;
    }

    let bullet_transform = bullet_query.single();

    let mut struck_block_position = Vec2::default();

    for (shield_transform, _, shield_block) in shield_query.iter_mut() {
        if util::intersect_rect(
            &shield_transform.translation,
            &BULLET_BOUNDS,
            &bullet_transform.translation,
            &SHIELD_BLOCK_SPRITE_SIZE,
        ) {
            despawn_event.send(DespawnBulletEvent);
            struck_block_position = shield_block.position;
            break;
        }
    }

    // Bullet kills blocks in a cross shape.
    for (_, mut shield_health, shield_block) in shield_query.iter_mut() {
        if shield_block.position == struck_block_position
            || (shield_block.position + Vec2::new(1.0, 0.0)) == struck_block_position
            || (shield_block.position + Vec2::new(0.0, 1.0)) == struck_block_position
            || (shield_block.position + Vec2::new(-1.0, 0.0)) == struck_block_position
            || (shield_block.position + Vec2::new(0.0, -1.0)) == struck_block_position
        {
            shield_health.health -= 5;
        }
    }
}
