use crate::control::ControlEvent;
use crate::qotile::{Qotile, QotileDiedEvent, QOTILE_BOUNDS};
use crate::shield::{ShieldBlock, ShieldHealth, SHIELD_BLOCK_SPRITE_SIZE};
use crate::util;
use crate::yar::{Yar, YarDiedEvent, YAR_BOUNDS};
use crate::SCREEN_SCALE;
use crate::SCREEN_SIZE;
use bevy::math::const_vec2;
use bevy::prelude::*;

const ZORLON_CANNON_SPEED: f32 = 6.0;
const ZORLON_CANNON_BOUNDS: Vec2 = const_vec2!([16.0 * SCREEN_SCALE, 16.0 * SCREEN_SCALE]);

pub struct SpawnZorlonCannonEvent;
pub struct DespawnZorlonCannonEvent;

pub struct ZorlonCannonPlugin;

impl Plugin for ZorlonCannonPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnZorlonCannonEvent>()
            .add_event::<DespawnZorlonCannonEvent>()
            .add_event::<CannonCommandEvent>()
            .add_system(spawn)
            .add_system(despawn)
            // .add_system(track)
            // .add_system(shoot)
            .add_system(input)
            .add_system(fly)
            .add_system(leave_world)
            .add_system(collide_yar)
            .add_system(collide_qotile)
            .add_system(collide_shield);
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CannonDirection {
    Up,
    Down,
}

/// An input command to the cannon
#[derive(Debug, Default, Copy, Clone)]
pub struct CannonCommandEvent {
    pub direction: Option<CannonDirection>,
    pub shoot: bool,
}

impl ControlEvent for CannonCommandEvent {
    fn is_noop(&self) -> bool {
        self.direction.is_none() && !self.shoot
    }
}

impl From<&Input<KeyCode>> for CannonCommandEvent {
    fn from(keys: &Input<KeyCode>) -> Self {
        let direction = match (keys.pressed(KeyCode::W), keys.pressed(KeyCode::S)) {
            (true, false) => Some(CannonDirection::Up),
            (false, true) => Some(CannonDirection::Down),
            _ => None,
        };
        Self {
            direction,
            shoot: keys.pressed(KeyCode::Space),
        }
    }
}

#[derive(Component)]
pub struct ZorlonCannon {
    launched: bool,
}

pub fn spawn(
    mut commands: Commands,
    mut spawn_event: EventReader<SpawnZorlonCannonEvent>,
    game_state: Res<crate::GameState>,
    zc_query: Query<&Transform, With<ZorlonCannon>>,
) {
    if spawn_event.iter().next().is_none() || !zc_query.is_empty() {
        return;
    }

    let mut zorlon_transform = Transform::from_scale(Vec3::splat(SCREEN_SCALE));
    zorlon_transform.translation.x = -SCREEN_SIZE.x / 2.0;

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 23,
                ..default()
            },
            texture_atlas: game_state.sprite_atlas.clone(),
            transform: zorlon_transform,
            ..default()
        })
        .insert(ZorlonCannon { launched: false });
}

pub fn despawn(
    mut commands: Commands,
    mut despawn_event: EventReader<DespawnZorlonCannonEvent>,
    mut death_event: EventReader<YarDiedEvent>,
    query: Query<Entity, With<ZorlonCannon>>,
) {
    if (despawn_event.iter().next().is_none() && death_event.iter().next().is_none())
        || query.is_empty()
    {
        return;
    }

    let e = query.single();
    commands.entity(e).despawn();
}

pub fn input(
    mut cannon_commands: EventReader<CannonCommandEvent>,
    mut query: Query<(&mut Transform, &mut ZorlonCannon)>,
) {
    if query.is_empty() {
        return;
    }

    let (mut transform, mut zorlon_cannon) = query.single_mut();
    if zorlon_cannon.launched {
        return;
    }

    let speed = 3.0;
    for command in cannon_commands.iter() {
        if let Some(direction) = command.direction {
            transform.translation.y += match direction {
                CannonDirection::Up => speed,
                CannonDirection::Down => -speed,
            };
        }
        if command.shoot {
            zorlon_cannon.launched = true;
            // Don't allow more movement after shooting
            break;
        }
    }
}

pub fn fly(mut zc_query: Query<(&mut Transform, &ZorlonCannon)>) {
    if zc_query.is_empty() {
        return;
    }

    let (mut transform, zorlon_cannon) = zc_query.single_mut();
    if !zorlon_cannon.launched {
        return;
    }

    transform.translation.x += ZORLON_CANNON_SPEED;
}

pub fn leave_world(
    mut despawn_event: EventWriter<DespawnZorlonCannonEvent>,
    mut query: Query<&Transform, With<ZorlonCannon>>,
) {
    if query.is_empty() {
        return;
    }

    let transform = query.single_mut();

    if util::is_offscreen(transform.translation) {
        despawn_event.send(DespawnZorlonCannonEvent);
    }
}

pub fn collide_yar(
    mut death_event: EventWriter<YarDiedEvent>,
    mut despawn_event: EventWriter<DespawnZorlonCannonEvent>,
    yar_query: Query<&Transform, (With<Yar>, Without<ZorlonCannon>)>,
    zc_query: Query<(&Transform, &ZorlonCannon), Without<Yar>>,
) {
    if yar_query.is_empty() || zc_query.is_empty() {
        return;
    }

    let (zc_transform, zorlon_cannon) = zc_query.single();
    if !zorlon_cannon.launched {
        return;
    }

    let yar_transform = yar_query.single();

    if util::intersect_rect(
        &yar_transform.translation,
        &YAR_BOUNDS,
        &zc_transform.translation,
        &ZORLON_CANNON_BOUNDS,
    ) {
        death_event.send(YarDiedEvent);
        despawn_event.send(DespawnZorlonCannonEvent);
    }
}

pub fn collide_qotile(
    mut death_event: EventWriter<QotileDiedEvent>,
    mut despawn_event: EventWriter<DespawnZorlonCannonEvent>,
    qotile_query: Query<&Transform, (With<Qotile>, Without<ZorlonCannon>)>,
    zc_query: Query<(&Transform, &ZorlonCannon), Without<Qotile>>,
) {
    if qotile_query.is_empty() || zc_query.is_empty() {
        return;
    }

    let (zc_transform, zorlon_cannon) = zc_query.single();
    if !zorlon_cannon.launched {
        return;
    }

    let q_transform = qotile_query.single();

    if util::intersect_rect(
        &q_transform.translation,
        &QOTILE_BOUNDS,
        &zc_transform.translation,
        &ZORLON_CANNON_BOUNDS,
    ) {
        death_event.send(QotileDiedEvent);
        despawn_event.send(DespawnZorlonCannonEvent);
    }
}

pub fn collide_shield(
    mut despawn_event: EventWriter<DespawnZorlonCannonEvent>,
    mut shield_query: Query<
        (&Transform, &mut ShieldHealth),
        (With<ShieldBlock>, Without<ZorlonCannon>),
    >,
    zc_query: Query<(&Transform, &ZorlonCannon), Without<ShieldBlock>>,
) {
    if shield_query.is_empty() || zc_query.is_empty() {
        return;
    }

    let (zc_transform, zorlon_cannon) = zc_query.single();
    if !zorlon_cannon.launched {
        return;
    }

    for (shield_transform, mut shield_health) in shield_query.iter_mut() {
        if util::intersect_rect(
            &shield_transform.translation,
            &YAR_BOUNDS,
            &zc_transform.translation,
            &SHIELD_BLOCK_SPRITE_SIZE,
        ) {
            shield_health.health -= 5;
            despawn_event.send(DespawnZorlonCannonEvent);
            return; // Can only break one shield block at a time. Awful, really.
        }
    }
}
