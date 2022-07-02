use bevy::prelude::*;
use bevy::math::const_vec2;
use crate::yar::{Yar, YAR_BOUNDS, YarDiedEvent};
use crate::qotile::{Qotile, QOTILE_BOUNDS};
use crate::SCREEN_SIZE;
use crate::SCREEN_SCALE;
use crate::{ShootEvent, QotileDiedEvent, SpawnZorlonCannon, DespawnZorlonCannon};
use crate::util;

const ZORLON_CANNON_SPEED:f32 = 6.0;
const ZORLON_CANNON_BOUNDS:Vec2 = const_vec2!([16.0*SCREEN_SCALE, 16.0*SCREEN_SCALE]);

#[derive(Component, Default)]
pub struct ZorlonCannonState
{
    pub zorlon_cannon: Option<Entity>,
    pub zorlon_launched: bool,
}

pub struct ZorlonCannonPlugin;

impl Plugin for ZorlonCannonPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ZorlonCannonState>()
            .add_system(spawn)
            .add_system(despawn)
            .add_system(track)
            .add_system(shoot)
            .add_system(fly)
            .add_system(leave_world)
            .add_system(collide_yar)
            .add_system(collide_qotile)
        ;
    }
}

#[derive(Component)]
pub struct ZorlonCannon;

pub fn spawn(
    mut commands: Commands,
    mut zc_state: ResMut<ZorlonCannonState>,
    mut spawn_event: EventReader<SpawnZorlonCannon>,
    query: Query<(&Transform, &Handle<TextureAtlas>, &Yar)>
) {
    if query.is_empty() {
        return
    }

    let (yar_transform, texture_atlas_handle, _) = query.single();

    let mut zorlon_transform = yar_transform.clone();
    zorlon_transform.translation.x = -SCREEN_SIZE.x / 2.0;

    for _e in spawn_event.iter() {
        if zc_state.zorlon_cannon.is_some() {
            return;
        }

        zc_state.zorlon_cannon = Some(commands
            .spawn_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite { index: 23, ..default() },
                texture_atlas: texture_atlas_handle.clone(),
                transform: zorlon_transform.clone(),
                ..default()
            })
            .insert(ZorlonCannon).id()
        );
    }
}

pub fn despawn(
    mut commands: Commands,
    mut zc_state: ResMut<ZorlonCannonState>,
    mut despawn_event: EventReader<DespawnZorlonCannon>,
    query: Query<(Entity, &ZorlonCannon)>
) {
    if query.is_empty() {
        return;
    }

    if despawn_event.iter().next().is_none() {
        return;
    }

    let (e, _zorlon_cannon) = query.single();

    commands.entity(e).despawn();
    zc_state.zorlon_cannon = None;
    zc_state.zorlon_launched = false;
}

pub fn track(
    zc_state: ResMut<ZorlonCannonState>,
    mut query: Query<(&mut Transform, Option<&Yar>, Option<&ZorlonCannon>)>
) {
    if zc_state.zorlon_cannon.is_none() {
        return;
    }

    if zc_state.zorlon_launched {
        return;
    }

    let mut yar_transform = Transform::identity();

    for (transform, yar, _zorlon_cannon) in query.iter_mut() {
        if yar.is_some() {
            yar_transform = transform.clone();
        }

    }

   for (mut transform, _yar, zorlon_cannon) in query.iter_mut() {
        if zorlon_cannon.is_some() {
            transform.translation.y = yar_transform.translation.y;
            return
        }
    }
}

pub fn shoot(
    mut zc_state: ResMut<ZorlonCannonState>,
    mut shoot_event: EventReader<ShootEvent>,
) {
    if zc_state.zorlon_cannon.is_none() {
        return;
    }

    if shoot_event.iter().next().is_some() {
        zc_state.zorlon_launched = true;
    }
}

pub fn fly(
    zc_state: ResMut<ZorlonCannonState>,
    mut query: Query<(&mut Transform, &ZorlonCannon)>
) {
    if zc_state.zorlon_cannon.is_none() {
        return;
    }

    if !zc_state.zorlon_launched {
        return;
    }

    if query.is_empty() {
        return;
    }

    let (mut transform, _zorlon_cannon ) = query.single_mut();

    transform.translation.x += ZORLON_CANNON_SPEED;
}

pub fn leave_world(
    mut despawn_event: EventWriter<DespawnZorlonCannon>,
    mut query: Query<(&Transform, &ZorlonCannon)>
) {
    if query.is_empty() {
        return;
    }

    let (transform, _zorlon_cannon ) = query.single_mut();

    if util::is_offscreen( transform.translation ) {
        despawn_event.send(DespawnZorlonCannon);
    }
}

pub fn collide_yar(
    zc_state: ResMut<ZorlonCannonState>,
    mut death_event: EventWriter<YarDiedEvent>,
    mut despawn_event: EventWriter<DespawnZorlonCannon>,
    query: Query<(&Transform, Option<&Yar>, Option<&ZorlonCannon>)>,
) {
    if zc_state.zorlon_cannon.is_none() {
        return;
    }

    if !zc_state.zorlon_launched {
        return;
    }

    let mut yar_transform = Transform::identity();
    let mut zc_transform = Transform::identity();

    for (transform, yar, zc) in query.iter() {
        if yar.is_some() {
            yar_transform = transform.clone();
        }
        if zc.is_some() {
            zc_transform = transform.clone();
        }
    }

    if util::intersect_rect(
        &yar_transform.translation,
        &YAR_BOUNDS,
        &zc_transform.translation,
        &ZORLON_CANNON_BOUNDS) {
        death_event.send(YarDiedEvent);
        despawn_event.send(DespawnZorlonCannon);
    }
}

pub fn collide_qotile(
    zc_state: ResMut<ZorlonCannonState>,
    mut death_event: EventWriter<QotileDiedEvent>,
    mut despawn_event: EventWriter<DespawnZorlonCannon>,
    query: Query<(&Transform, Option<&Qotile>, Option<&ZorlonCannon>)>,
) {
    if zc_state.zorlon_cannon.is_none() {
        return;
    }

    if !zc_state.zorlon_launched {
        return;
    }

    let mut q_transform = Transform::identity();
    let mut zc_transform = Transform::identity();

    for (transform, yar, zc) in query.iter() {
        if yar.is_some() {
            q_transform = transform.clone();
        }
        if zc.is_some() {
            zc_transform = transform.clone();
        }
    }

    if util::intersect_rect(
        &q_transform.translation,
        &QOTILE_BOUNDS,
        &zc_transform.translation,
        &ZORLON_CANNON_BOUNDS) {
        death_event.send(QotileDiedEvent);
        despawn_event.send(DespawnZorlonCannon);
    }
}