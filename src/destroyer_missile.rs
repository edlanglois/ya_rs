use bevy::prelude::*;
use bevy::math::const_vec2;
use crate::SCREEN_SCALE;
use crate::qotile::Qotile;
use crate::yar::{Yar, YAR_BOUNDS, YarDiedEvent};
use crate::util;

// Gameplay Note: Not sure if the destroyer missile spawns instantly in all difficulty modes.
// Need to check...

const DESTROYER_MISSILE_SPEED: f32 = 0.5;
pub const DESTROYER_MISSILE_BOUNDS:Vec2 = const_vec2!([8.0*SCREEN_SCALE, 2.0*SCREEN_SCALE]);

//pub struct SpawnDestroyerMissileEvent;
pub struct DespawnDestroyerMissileEvent;

#[derive(Component)]
pub struct DestroyerMissile;

pub struct DestroyerMissilePlugin;

impl Plugin for DestroyerMissilePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<DespawnDestroyerMissileEvent>()
            .add_system(spawn)
            .add_system(despawn)
            .add_system(track)
            .add_system(collide_yar)
        ;
    }
}

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    missile_query: Query<Entity, (With<DestroyerMissile>, Without<Yar>, Without<Qotile>)>,
    qotile_query: Query<&Transform, (With<Qotile>, Without<Yar>, Without<DestroyerMissile>)>,
    yar_query: Query<&Yar, (Without<DestroyerMissile>, Without<Qotile>)>
)
{
    if !missile_query.is_empty() || yar_query.is_empty() || qotile_query.is_empty() {
        return
    }

    let yar = yar_query.single();
    if yar.is_dead() {
        return
    }

    let qotile_transform = qotile_query.single();

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("destroyer_missile.png"),
            transform: qotile_transform.clone(),
            ..default()
        })
        .insert(DestroyerMissile);
}

pub fn despawn(
    mut commands: Commands,
    mut despawn_event: EventReader<DespawnDestroyerMissileEvent>,
    query: Query<Entity, With<DestroyerMissile>>
) {
    if query.is_empty() {
        return;
    }

    if despawn_event.iter().next().is_none() {
        return;
    }

    let e = query.single();
    commands.entity(e).despawn();
}

pub fn track(
    mut missile_query: Query<&mut Transform, (With<DestroyerMissile>, Without<Yar>)>,
    yar_query: Query<&Transform, (With<Yar>, Without<DestroyerMissile>)>
)
{
    if missile_query.is_empty() || yar_query.is_empty() {
        return
    }

    let yar_transform = yar_query.single();
    let mut missile_transform = missile_query.single_mut();
    let direction = (yar_transform.translation - missile_transform.translation).normalize();

    missile_transform.translation += direction * DESTROYER_MISSILE_SPEED;
}

pub fn collide_yar(
    mut death_event: EventWriter<YarDiedEvent>,
    mut despawn_event: EventWriter<DespawnDestroyerMissileEvent>,
    yar_query: Query<&Transform, (With<DestroyerMissile>, Without<Yar>)>,
    dm_query: Query<&Transform, (With<Yar>, Without<DestroyerMissile>)>
)
{
    if dm_query.is_empty() || yar_query.is_empty() {
        return;
    }

    let yar_transform = yar_query.single();
    let dm_transform = dm_query.single();

    if util::intersect_rect(
        &yar_transform.translation,
        &YAR_BOUNDS,
        &dm_transform.translation,
        &DESTROYER_MISSILE_BOUNDS) {
        death_event.send(YarDiedEvent);
        despawn_event.send(DespawnDestroyerMissileEvent);
    }
}