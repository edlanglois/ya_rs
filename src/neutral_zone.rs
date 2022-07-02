use bevy::prelude::*;
use bevy::math::const_vec2;
use rand::prelude::*;
use crate::SCREEN_SIZE;
use crate::SCREEN_SCALE;

const NEUTRAL_ZONE_COLS: i32 = 7;
const NEUTRAL_ZONE_ROWS: i32 = 192;
const NEUTRAL_ZONE_SPRITE_SIZE: Vec2 = const_vec2!([8.0*SCREEN_SCALE, 1.0*SCREEN_SCALE]);
pub const NEUTRAL_ZONE_BOUNDS: Vec2 = const_vec2!([SCREEN_SCALE*8.0*NEUTRAL_ZONE_COLS as f32, SCREEN_SCALE*NEUTRAL_ZONE_ROWS as f32]);
const NEUTRAL_ZONE_SHIFT_TIME: f32 = 0.05;

#[derive(Component)]
pub struct ColorPalette
{
    pub neutral_zone_colors: Vec<Color>,
}

impl Default for ColorPalette
{
    fn default() -> Self {
        let neutral_zone_colors: Vec<Color> = vec![
            Color::rgb(0.00,0.00,0.00), Color::rgb(0.29,0.29,0.29),
            Color::rgb(0.29,0.29,0.00), Color::rgb(0.41,0.41,0.05),
            Color::rgb(0.48,0.16,0.00), Color::rgb(0.56,0.28,0.06),
            Color::rgb(0.56,0.11,0.00), Color::rgb(0.63,0.22,0.07),
            Color::rgb(0.58,0.00,0.00), Color::rgb(0.65,0.10,0.10),
            Color::rgb(0.51,0.00,0.39), Color::rgb(0.59,0.01,0.48),
            Color::rgb(0.30,0.00,0.51), Color::rgb(0.40,0.11,0.60),
            Color::rgb(0.04,0.00,0.56), Color::rgb(0.20,0.10,0.63),
            Color::rgb(0.00,0.11,0.53), Color::rgb(0.10,0.23,0.62),
            Color::rgb(0.00,0.19,0.39), Color::rgb(0.10,0.31,0.50),
            Color::rgb(0.00,0.25,0.18), Color::rgb(0.09,0.38,0.30),
            Color::rgb(0.00,0.26,0.00), Color::rgb(0.10,0.40,0.10),
            Color::rgb(0.08,0.23,0.00), Color::rgb(0.20,0.37,0.10),
            Color::rgb(0.19,0.19,0.00), Color::rgb(0.31,0.31,0.10),
            Color::rgb(0.28,0.18,0.00), Color::rgb(0.41,0.30,0.10),
        ];

        Self {
            neutral_zone_colors,
        }
    }
}

pub struct NeutralZonePlugin;

impl Plugin for NeutralZonePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ColorPalette>()
            .add_startup_system(spawn)
            //.add_system(despawn)
            .add_system(color_shift)
        ;
    }
}

#[derive(Component)]
pub struct NeutralZone;

#[derive(Component)]
pub struct NeutralZoneChunk;

#[derive(Component, Deref, DerefMut)]
pub struct ChunkShiftTimer(pub Timer);

pub fn spawn(
    mut commands: Commands,
) {
    let mut zone_origin = Transform::identity();
    zone_origin.translation.x -= 20.0 * SCREEN_SCALE;

    let mut zone_transform = Transform::identity();
    zone_transform.translation += zone_origin.translation;
    zone_transform.translation.x -= NEUTRAL_ZONE_BOUNDS.x / 2.0;
    zone_transform.translation.y -= SCREEN_SIZE.y / 2.0;

    let mut chunk_offset = Transform::identity();

    for _ in 0..NEUTRAL_ZONE_ROWS {
        for _ in 0..NEUTRAL_ZONE_COLS {
            let mut chunk_transform = zone_transform;
            chunk_transform.translation += chunk_offset.translation;

            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.25, 0.25, 0.75),
                        custom_size: Some(NEUTRAL_ZONE_SPRITE_SIZE),
                        ..default()
                    },
                    transform: chunk_transform,
                    ..default()
                })
                .insert(NeutralZoneChunk);

            chunk_offset.translation.x += NEUTRAL_ZONE_SPRITE_SIZE.x;
        }
        chunk_offset.translation.x = 0.0;
        chunk_offset.translation.y += NEUTRAL_ZONE_SPRITE_SIZE.y;
    }

    commands.spawn()
        .insert(ChunkShiftTimer(Timer::from_seconds(NEUTRAL_ZONE_SHIFT_TIME, true)))
        .insert(zone_origin )
        .insert(NeutralZone);
}
/*
pub fn despawn(
    mut commands: Commands,
    mut death_event: EventReader<YarDiedEvent>,
    nz_query: Query<Entity, With<NeutralZone>>,
    chunks_query: Query<Entity, With<NeutralZoneChunk>>,
) {
    if death_event.iter().next().is_none() {
        return;
    }

    for e in nz_query.iter() {
        commands.entity(e).despawn();
    }

    for e in chunks_query.iter() {
        commands.entity(e).despawn();
    }
}
*/
pub fn color_shift(
    color_palette: Res<ColorPalette>,
    time: Res<Time>,
    mut chunk_query: Query<&mut Sprite, With<NeutralZoneChunk>>,
    mut nz_query: Query<&mut ChunkShiftTimer, With<NeutralZone>>
) {
    if nz_query.is_empty() {
        return
    }

    let mut timer = nz_query.single_mut();
    timer.tick(time.delta());
    if timer.just_finished() {
        for mut sprite in chunk_query.iter_mut() {
            let mut rng = thread_rng();
            let black_chance: f32 = rng.gen();

            if black_chance < 0.35 {
                sprite.color = Color::rgb( 0.0, 0.0, 0.0 );
            } else {
                sprite.color = *color_palette.neutral_zone_colors.choose(&mut thread_rng()).unwrap();
            }
        }
    }
}
