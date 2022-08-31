use crate::qotile::{DespawnQotileEvent, Qotile, SwirlState, QOTILE_BOUNDS};
use crate::shield::{ShieldBlock, ShieldHealth, SHIELD_BLOCK_SPRITE_SIZE};
use crate::util;
use crate::zorlon_cannon::SpawnZorlonCannonEvent;
use crate::SCREEN_SCALE;
use crate::SCREEN_SIZE;
use bevy::math::const_vec2;
use bevy::prelude::*;

pub const YAR_BOUNDS: Vec2 = const_vec2!([16.0 * SCREEN_SCALE, 16.0 * SCREEN_SCALE]);
const YAR_EAT_KNOCKBACK: f32 = 8.0 * SCREEN_SCALE;

pub struct YarShootEvent;
pub struct YarDiedEvent;
pub struct YarRespawnEvent;

pub struct YarPlugin;

impl Plugin for YarPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<YarShootEvent>()
            .add_event::<YarDiedEvent>()
            .add_event::<YarRespawnEvent>()
            .add_startup_system(setup.after(crate::setup_sprites))
            .add_system(input)
            .add_system(animate)
            .add_system(collide_qotile)
            .add_system(collide_shield)
            .add_system(death)
            .add_system(respawn);
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
pub enum YarAnim {
    Fly,
    Death,
}

#[derive(Component)]
pub struct Yar {
    pub direction: YarDirection,
    anim_frame: usize,
    pub anim: YarAnim,
}

impl Yar {
    pub fn is_dead(&self) -> bool {
        matches!(self.anim, YarAnim::Death)
    }

    pub fn direction_to_vector(&self) -> Vec3 {
        match self.direction {
            YarDirection::Left => Vec3::new(-1.0, 0.0, 1.0),
            YarDirection::Right => Vec3::new(1.0, 0.0, 1.0),
            YarDirection::Up => Vec3::new(0.0, 1.0, 1.0),
            YarDirection::UpRight => Vec3::new(1.0, 1.0, 1.0),
            YarDirection::UpLeft => Vec3::new(-1.0, 1.0, 1.0),
            YarDirection::Down => Vec3::new(0.0, -1.0, 1.0),
            YarDirection::DownRight => Vec3::new(1.0, -1.0, 1.0),
            YarDirection::DownLeft => Vec3::new(-1.0, -1.0, 1.0),
        }
    }
}

impl Default for Yar {
    fn default() -> Self {
        Yar {
            direction: YarDirection::Down,
            anim_frame: 0,
            anim: YarAnim::Fly,
        }
    }
}

pub fn setup(commands: Commands, game_state: Res<crate::GameState>) {
    spawn(commands, game_state);
}

pub fn spawn(mut commands: Commands, game_state: Res<crate::GameState>) {
    let mut transform = Transform::from_scale(Vec3::splat(crate::SCREEN_SCALE));
    transform.translation.x -= (SCREEN_SIZE.x / 2.0) - (YAR_BOUNDS.x * 2.0);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: game_state.sprite_atlas.clone(),
            transform: transform,
            ..default()
        })
        .insert(Yar::default())
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)));
}

pub fn input(
    keys: Res<Input<KeyCode>>,
    mut shoot_event: EventWriter<YarShootEvent>,
    mut query: Query<(&mut Transform, &mut Yar)>,
) {
    if query.is_empty() {
        return;
    }

    let (mut transform, mut yar) = query.single_mut();

    if matches!(yar.anim, YarAnim::Death) {
        return;
    }

    let mut yar_delta = Transform::identity();

    let speed = 3.0;
    let mut direction: Option<YarDirection> = None;

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

    if keys.pressed(KeyCode::Space) {
        shoot_event.send(YarShootEvent);
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
        return;
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
                let death_anim: Vec<usize> = vec![
                    5, 7, 9, 11, 13, 15, 1, 1, 1, 1, 16, 16, 16, 17, 17, 17, 18, 19, 20, 22, 22, 22,
                ];
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
    mut spawn_event: EventWriter<SpawnZorlonCannonEvent>,
    mut death_event: EventWriter<YarDiedEvent>,
    mut despawn_event: EventWriter<DespawnQotileEvent>,
    yar_query: Query<&Transform, (With<Yar>, Without<Qotile>)>,
    qotile_query: Query<(&Transform, &Qotile), Without<Yar>>,
) {
    if yar_query.is_empty() || qotile_query.is_empty() {
        return;
    }

    let yar_transform = yar_query.single();
    let (qotile_transform, qotile) = qotile_query.single();

    if util::intersect_rect(
        &yar_transform.translation,
        &YAR_BOUNDS,
        &qotile_transform.translation,
        &QOTILE_BOUNDS,
    ) {
        if matches!(qotile.swirl_state, SwirlState::NotSwirl) {
            spawn_event.send(SpawnZorlonCannonEvent);
        } else {
            death_event.send(YarDiedEvent);
            despawn_event.send(DespawnQotileEvent);
        }
    }
}

pub fn collide_shield(
    mut spawn_event: EventWriter<SpawnZorlonCannonEvent>,
    mut yar_query: Query<(&mut Transform, &Yar), Without<ShieldBlock>>,
    mut shield_query: Query<(&Transform, &mut ShieldHealth), With<ShieldBlock>>,
) {
    if yar_query.is_empty() || shield_query.is_empty() {
        return;
    }

    let (mut yar_transform, yar) = yar_query.single_mut();
    for (shield_transform, mut shield_health) in shield_query.iter_mut() {
        if util::intersect_rect(
            &yar_transform.translation,
            &YAR_BOUNDS,
            &shield_transform.translation,
            &SHIELD_BLOCK_SPRITE_SIZE,
        ) {
            shield_health.health -= 1;

            let mut knockback = yar.direction_to_vector();
            knockback.z = 0.0;
            yar_transform.translation -= knockback * YAR_EAT_KNOCKBACK;

            spawn_event.send(SpawnZorlonCannonEvent);
        }
    }
}

pub fn death(mut death_event: EventReader<YarDiedEvent>, mut query: Query<&mut Yar>) {
    if death_event.iter().next().is_none() || query.is_empty() {
        return;
    }

    let mut yar = query.single_mut();
    yar.anim = YarAnim::Death;
    yar.anim_frame = 0;
}

pub fn respawn(
    commands: Commands,
    game_state: Res<crate::GameState>,
    mut respawn_event: EventReader<YarRespawnEvent>,
) {
    if respawn_event.iter().next().is_none() {
        return;
    }

    spawn(commands, game_state);
}
