use bevy::prelude::*;
use bevy::math::const_vec2;
use crate::SCREEN_SCALE;
use crate::qotile::Qotile;
use crate::yar::{Yar, YAR_BOUNDS, YarDiedEvent};
use crate::util;

// Gameplay Note: Not sure if the destroyer missile spawns instantly in all difficulty modes.
// Need to check...

const DESTROYER_MISSILE_SPEED: f32 = 0.5;
pub const DESTROYER_MISSILE_BOUNDS:Vec2 = const_vec2!([16.0*SCREEN_SCALE, 16.0*SCREEN_SCALE]);

// Events
pub struct SpawnDestroyerMissile;
pub struct DespawnDestroyerMissile;

#[derive(Component)]
pub struct DestroyerMissile;

pub struct DestroyerMissilePlugin;

impl Plugin for DestroyerMissilePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<DespawnDestroyerMissile>()
            .add_system(spawn)
            .add_system(despawn)
            .add_system(track)
            .add_system(collide_yar)
        ;
    }
}

pub fn spawn(
    mut commands: Commands,
    game_state: Res<crate::game_state::GameState>,
    query: Query<(&Transform, Option<&DestroyerMissile>, Option<&Qotile>)>,
)
{
    let mut qotile_transform = Transform::identity();

    for (transform, destroyer_missile, qotile) in query.iter() {
        if destroyer_missile.is_some() {
            return
        }

        if qotile.is_some() {
            qotile_transform = transform.clone();
        }
    }

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite { index: 28, ..default() },
            texture_atlas: game_state.sprite_atlas.clone(),
            transform: qotile_transform,
            ..default()
        })
        .insert(DestroyerMissile);
}

pub fn despawn(
    mut commands: Commands,
    mut despawn_event: EventReader<DespawnDestroyerMissile>,
    query: Query<(Entity, &DestroyerMissile)>
) {
    if query.is_empty() {
        return;
    }

    if despawn_event.iter().next().is_none() {
        return;
    }

    let (e, _) = query.single();

    commands.entity(e).despawn();
}

pub fn track(
    mut query: Query<(&mut Transform, Option<&DestroyerMissile>, Option<&Yar>)>
)
{
    let mut yar_transform = Transform::identity();

    for (transform, _destroyer_missile, yar) in query.iter_mut() {
        if yar.is_some() {
            yar_transform = transform.clone();
        }
    }

    for (mut transform, destroyer_missile, _yar) in query.iter_mut() {
        if destroyer_missile.is_some() {
            let mut direction = (yar_transform.translation - transform.translation).normalize();
            transform.translation += direction * DESTROYER_MISSILE_SPEED;
        }
    }
}

pub fn collide_yar(
    mut death_event: EventWriter<YarDiedEvent>,
    mut despawn_event: EventWriter<DespawnDestroyerMissile>,
    mut yar_query: Query<(&mut Transform), (With<DestroyerMissile>, Without<Yar>)>,
    mut dm_query: Query<(&mut Transform), (With<Yar>, Without<DestroyerMissile>)>
)
{
    if dm_query.is_empty() || yar_query.is_empty() {
        return;
    }

    let mut yar_transform = yar_query.single();
    let mut dm_transform = dm_query.single();

    if util::intersect_rect(
        &yar_transform.translation,
        &YAR_BOUNDS,
        &dm_transform.translation,
        &DESTROYER_MISSILE_BOUNDS) {
        death_event.send(YarDiedEvent);
        despawn_event.send(DespawnDestroyerMissile);
    }
}