use crate::util;
use crate::yar::Yar;
use bevy::math::const_vec2;
use bevy::prelude::*;
use rand::prelude::*;
use std::time::Duration;

use crate::SCREEN_SCALE;
use crate::SCREEN_SIZE;

const QOTILE_SPRITE_SIZE: Vec2 = const_vec2!([16.0, 18.0]);
const QOTILE_INSET: f32 = 16.0;
pub const QOTILE_BOUNDS: Vec2 = const_vec2!([16.0 * SCREEN_SCALE, 18.0 * SCREEN_SCALE]);
const SWIRL_SPEED: f32 = 6.0;

const SWIRL_DELAY_BASE: f32 = 3.0;
const SWIRL_DELAY_VARIANCE: f32 = 5.0;

const LAUNCH_DELAY_BASE: f32 = 1.0;
const LAUNCH_DELAY_VARIANCE: f32 = 3.0;

pub struct QotileDiedEvent;
pub struct SpawnQotileEvent;
pub struct DespawnQotileEvent;

pub struct QotilePlugin;

impl Plugin for QotilePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<QotileDiedEvent>()
            .add_event::<SpawnQotileEvent>()
            .add_event::<DespawnQotileEvent>()
            .add_startup_system(setup)
            .add_system(spawn)
            .add_system(despawn)
            .add_system(animate)
            .add_system(timer)
            .add_system(fly)
            .add_system(leave_world)
            .add_system(died);
    }
}

pub enum SwirlState {
    NotSwirl,
    SwirlIdle,
    SwirlFly,
}

pub enum QotileAnim {
    Idle,
    Swirl,
}

#[derive(Component)]
pub struct Qotile {
    pub swirl_state: SwirlState,
    anim: QotileAnim,
    anim_frame: usize,
    flight_vector: Vec3,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(pub Timer);

#[derive(Component, Deref, DerefMut)]
pub struct SwirlTimer(pub Timer);

fn swirl_delay() -> f32 {
    SWIRL_DELAY_BASE + SWIRL_DELAY_VARIANCE * thread_rng().gen::<f32>()
}

fn launch_delay() -> f32 {
    LAUNCH_DELAY_BASE + LAUNCH_DELAY_VARIANCE * thread_rng().gen::<f32>()
}

fn setup(mut spawn_event: EventWriter<SpawnQotileEvent>) {
    spawn_event.send(SpawnQotileEvent);
}

fn spawn(
    mut commands: Commands,
    mut spawn_event: EventReader<SpawnQotileEvent>,
    asset_server: Res<AssetServer>,
) {
    if spawn_event.iter().next().is_none() {
        return;
    }

    let mut transform = Transform::from_scale(Vec3::splat(crate::SCREEN_SCALE));
    transform.translation.x +=
        (SCREEN_SIZE.x / 2.0) - (QOTILE_SPRITE_SIZE.x * crate::SCREEN_SCALE / 2.0) - QOTILE_INSET;

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("qotile.png"),
            transform: transform,
            ..default()
        })
        .insert(SwirlTimer(Timer::from_seconds(swirl_delay(), false)))
        .insert(AnimationTimer(Timer::from_seconds(0.05, true)))
        .insert(Qotile {
            swirl_state: SwirlState::NotSwirl,
            anim: QotileAnim::Idle,
            anim_frame: 0,
            flight_vector: Vec3::default(),
        });
}

pub fn despawn(
    mut commands: Commands,
    mut despawn_event: EventReader<DespawnQotileEvent>,
    //mut death_event: EventReader<YarDiedEvent>,
    mut spawn_event: EventWriter<SpawnQotileEvent>,
    query: Query<Entity, With<Qotile>>,
) {
    if despawn_event.iter().next().is_none() || query.is_empty() {
        return;
    }
    /*
        if (despawn_event.iter().next().is_none() && death_event.iter().next().is_none()) || query.is_empty() {
            return;
        }
    */
    let e = query.single();
    commands.entity(e).despawn();

    spawn_event.send(SpawnQotileEvent);
}

fn animate(
    time: Res<Time>,
    mut query: Query<(&mut Qotile, &mut AnimationTimer, &mut TextureAtlasSprite)>,
) {
    if query.is_empty() {
        return;
    }

    let (mut qotile, mut timer, mut sprite) = query.single_mut();

    timer.tick(time.delta());
    if timer.just_finished() {
        match qotile.anim {
            QotileAnim::Idle => {}
            QotileAnim::Swirl => {
                let anim_length = 3;
                qotile.anim_frame = (qotile.anim_frame + 1) % anim_length;

                let sprite_base = 25;
                sprite.index = sprite_base + qotile.anim_frame;
            }
        }
    }
}

fn timer(
    mut commands: Commands,
    game_state: Res<crate::GameState>,
    time: Res<Time>,
    mut qotile_query: Query<(Entity, &Transform, &mut SwirlTimer, &mut Qotile), Without<Yar>>,
    yar_query: Query<&Transform, With<Yar>>,
) {
    if qotile_query.is_empty() || yar_query.is_empty() {
        return;
    }

    let (e, transform, mut timer, mut qotile) = qotile_query.single_mut();

    timer.tick(time.delta());
    if timer.just_finished() {
        match qotile.swirl_state {
            SwirlState::NotSwirl => {
                qotile.anim = QotileAnim::Swirl;
                qotile.swirl_state = SwirlState::SwirlIdle;
                timer.set_duration(Duration::from_secs_f32(launch_delay()));
                timer.reset();

                commands
                    .entity(e)
                    .remove_bundle::<SpriteBundle>()
                    .insert_bundle(SpriteSheetBundle {
                        texture_atlas: game_state.sprite_atlas.clone(),
                        transform: transform.clone(),
                        sprite: TextureAtlasSprite {
                            index: 25,
                            ..default()
                        },
                        ..default()
                    });
            }
            SwirlState::SwirlIdle => {
                qotile.swirl_state = SwirlState::SwirlFly;
                let yar_transform = yar_query.single();
                qotile.flight_vector =
                    (yar_transform.translation - transform.translation).normalize();
                qotile.flight_vector.z = 0.0;
                commands.entity(e).remove::<SwirlTimer>();
            }
            SwirlState::SwirlFly => {}
        }
    }
}

fn fly(mut query: Query<(&mut Transform, &Qotile)>) {
    if query.is_empty() {
        return;
    }

    let (mut transform, qotile) = query.single_mut();

    if matches!(qotile.swirl_state, SwirlState::SwirlFly) {
        transform.translation += qotile.flight_vector * SWIRL_SPEED;
    }
}

pub fn leave_world(
    mut despawn_event: EventWriter<DespawnQotileEvent>,
    mut query: Query<&Transform, With<Qotile>>,
) {
    if query.is_empty() {
        return;
    }

    let transform = query.single_mut();

    if util::is_offscreen(transform.translation) {
        despawn_event.send(DespawnQotileEvent);
    }
}

pub fn died(
    mut death_event: EventReader<QotileDiedEvent>,
    mut despawn_event: EventWriter<DespawnQotileEvent>,
) {
    if death_event.iter().next().is_none() {
        return;
    }

    // todo: victory stuffs

    println!("QOTILE DEAD, YOU WIN");

    despawn_event.send(DespawnQotileEvent);
}
