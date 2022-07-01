use bevy::prelude::*;
use bevy::math::const_vec2;

// todo:
// flight
// shoot
// scale
// eat

use crate::{ShootBullet, SpawnZorlonCannon};
use crate::SCREEN_SIZE;
use crate::SCREEN_SCALE;
use crate::qotile::{Qotile, QOTILE_BOUNDS};
use crate::util;

const YAR_BOUNDS:Vec2 = const_vec2!([16.0*SCREEN_SCALE, 16.0*SCREEN_SCALE]);

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

#[derive(Component)]
pub struct Yar {
    pub direction: YarDirection,
    anim_frame: usize,
}

impl Default for Yar {
    fn default() -> Self {
        Yar {
            direction: YarDirection::Up,
            anim_frame: 0,
        }
    }
}

pub fn setup(
    mut commands: Commands,
    game_state: Res<crate::game_state::GameState>,
) {
//    let texture_atlas_handle = texture_atlases.add(texture_atlas);
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
    mut shoot_event: EventWriter<ShootBullet>,
    mut query: Query<(&mut Transform, &mut Yar)>) {
    if query.is_empty() {
        return
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

    let (mut transform, mut yar) = query.single_mut();

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
        shoot_event.send(ShootBullet);
    }
}

pub fn animate(
    time: Res<Time>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &mut Yar,
    )>,
) {
    if query.is_empty() {
        return
    }

    let (mut timer, mut sprite, mut yar) = query.single_mut();

    timer.tick(time.delta());
    if timer.just_finished() {
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
}

pub fn collide_qotile(
    mut spawn_event: EventWriter<SpawnZorlonCannon>,
    query: Query<(&Transform, Option<&Yar>, Option<&Qotile>,
    )>,
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