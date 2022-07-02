use bevy::prelude::*;
use bevy::math::const_vec2;
use crate::{ShootEvent, SpawnZorlonCannon};
use crate::SCREEN_SIZE;
use crate::SCREEN_SCALE;
use crate::qotile::{Qotile, QOTILE_BOUNDS};
use crate::util;

pub const YAR_BOUNDS:Vec2 = const_vec2!([16.0*SCREEN_SCALE, 16.0*SCREEN_SCALE]);

pub struct YarDiedEvent;
pub struct YarRespawnEvent;

pub struct YarPlugin;

impl Plugin for YarPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<YarDiedEvent>()
            .add_event::<YarRespawnEvent>()
            .add_startup_system(setup.after(crate::setup_sprites))
            .add_system(input)
            .add_system(animate)
            .add_system(collide_qotile)
            .add_system(death)
            .add_system(respawn)
        ;
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Clone)]
pub enum YarDirection {
    Left,
    Right,
    Up,
    UpRight,
    UpLeft,
    Down,
    DownRight,
    DownLeft,
}

#[derive(Clone, PartialEq, Eq)]
enum YarAnim {
    Fly,
    Death,
}

#[derive(Component)]
pub struct Yar {
    pub direction: YarDirection,
    anim_frame: usize,
    anim: YarAnim,
}

impl Default for Yar {
    fn default() -> Self {
        Yar {
            direction: YarDirection::Up,
            anim_frame: 0,
            anim: YarAnim::Fly,
        }
    }
}

pub fn setup(
    commands: Commands,
    game_state: Res<crate::game_state::GameState>,
) {
    spawn( commands, game_state );
}

pub fn spawn(
    mut commands: Commands,
    game_state: Res<crate::game_state::GameState>,
) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: game_state.sprite_atlas.clone(),
            transform: Transform::from_scale(Vec3::splat(crate::SCREEN_SCALE)),
            ..default()
        })
        .insert(Yar::default())
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)));
}

pub fn input(
    keys: Res<Input<KeyCode>>,
    mut shoot_event: EventWriter<ShootEvent>,
    mut query: Query<(&mut Transform, &mut Yar)>) {
    if query.is_empty() {
        return
    }

    let (mut transform, mut yar) = query.single_mut();

    if matches!(yar.anim, YarAnim::Death) {
        return;
    }

    let mut yar_delta = Transform::identity();

    let speed = 3.0;
    let mut direction:Option<YarDirection> = None;

    if keys.pressed(KeyCode::W) {
        yar_delta.translation.y += speed;
    }

    if keys.pressed(KeyCode::S) {
        yar_delta.translation.y -= speed;
    }

    if keys.pressed(KeyCode::A) {
        yar_delta.translation.x -= speed;
    }

    if keys.pressed(KeyCode::D) {
        yar_delta.translation.x += speed;
    }

    if yar_delta.translation.x > 0.0 {
        if yar_delta.translation.y > 0.0 {
            direction = Some(YarDirection::UpRight);
        } else if yar_delta.translation.y < 0.0 {
            direction = Some(YarDirection::DownRight);
        } else {
            direction = Some(YarDirection::Right);
        }
    } else if yar_delta.translation.x < 0.0 {
        if yar_delta.translation.y > 0.0 {
            direction = Some(YarDirection::UpLeft);
        } else if yar_delta.translation.y < 0.0 {
            direction = Some(YarDirection::DownLeft);
        } else {
            direction = Some(YarDirection::Left);
        }
    } else {
        if yar_delta.translation.y > 0.0 {
            direction = Some(YarDirection::Up);
        } else if yar_delta.translation.y < 0.0 {
            direction = Some(YarDirection::Down);
        }
    }

    // If Yar moves offscreen in the horizontal direction, correct the move to bound Yar.
    {
        let x_pos = transform.translation.x + yar_delta.translation.x;

        let x_underflow = (x_pos - YAR_BOUNDS.x / 2.0) - (-SCREEN_SIZE.x / 2.0);
        if x_underflow < 0.0 {
            yar_delta.translation.x -= x_underflow;
        }

        let x_overflow = (x_pos + YAR_BOUNDS.x / 2.0) - (SCREEN_SIZE.x / 2.0);
        if x_overflow > 0.0 {
            yar_delta.translation.x -= x_overflow;
        }
    }

    // If Yar's centerpoint moves offscreen in the vertical direction, wrap Yar to the other side.
    {
        let y_pos = transform.translation.y + yar_delta.translation.y;

        if y_pos < -SCREEN_SIZE.y / 2.0 {
            yar_delta.translation.y += SCREEN_SIZE.y;
        } else if y_pos > SCREEN_SIZE.y / 2.0 {
            yar_delta.translation.y -= SCREEN_SIZE.y;
        }
    }

    transform.translation += yar_delta.translation;
    if let Some(dir) = direction.clone() {
        yar.direction = dir;
    }

    if keys.pressed( KeyCode::Space) {
        shoot_event.send(ShootEvent);
    }
}

pub fn animate(
    mut commands: Commands,
    time: Res<Time>,
    mut respawn_event: EventWriter<YarRespawnEvent>,
    mut query: Query<(
        Entity,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &mut Yar,
    )>,
) {
    if query.is_empty() {
        return
    }

    let (e, mut timer, mut sprite, mut yar) = query.single_mut();

    timer.tick(time.delta());
    if timer.just_finished() {
        match yar.anim {
            YarAnim::Fly => {
                let sprite_base = match yar.direction {
                    YarDirection::Up => 0,
                    YarDirection::UpRight => 2,
                    YarDirection::Right => 4,
                    YarDirection::DownRight => 6,
                    YarDirection::Down => 8,
                    YarDirection::DownLeft => 10,
                    YarDirection::Left => 12,
                    YarDirection::UpLeft => 14,
                };

                let anim_length = 2;

                yar.anim_frame = (yar.anim_frame + 1) % anim_length;

                sprite.index = sprite_base + yar.anim_frame;
            }
            YarAnim::Death => {
                let death_anim: Vec<usize> = vec![5,7,9,11,13,15,1,1,1,1,16,16,16,17,17,17,18,19,20,22,22,22];
                yar.anim_frame += 1;

                if yar.anim_frame >= death_anim.len() {
                    // todo: move this out of here
                    // probably a despawn yar event, then respawn...
                    commands.entity(e).despawn();

                    respawn_event.send(YarRespawnEvent);
                } else {
                    sprite.index = death_anim[yar.anim_frame];
                }
            }
        }
    }
}

pub fn collide_qotile(
    mut spawn_event: EventWriter<SpawnZorlonCannon>,
    query: Query<(&Transform, Option<&Yar>, Option<&Qotile>)>,
) {
    let mut yar_transform = Transform::identity();
    let mut qotile_transform = Transform::identity();

    for (transform, yar, qotile) in query.iter() {
        if yar.is_some() {
            yar_transform = transform.clone();
        }
        if qotile.is_some() {
            qotile_transform = transform.clone();
        }
    }

    if util::intersect_rect(
        &yar_transform.translation,
        &YAR_BOUNDS,
        &qotile_transform.translation,
        &QOTILE_BOUNDS) {
        spawn_event.send(SpawnZorlonCannon);
    }
}

pub fn death(
    mut death_event: EventReader<YarDiedEvent>,
    mut query: Query<&mut Yar>,
) {
    if death_event.iter().next().is_none() {
        return;
    }

    let mut yar = query.single_mut();
    yar.anim = YarAnim::Death;
    yar.anim_frame = 0;
}

pub fn respawn(
    commands: Commands,
    game_state: Res<crate::game_state::GameState>,
    mut respawn_event: EventReader<YarRespawnEvent>,
) {
    if respawn_event.iter().next().is_none() {
        return;
    }

    spawn( commands, game_state );
}