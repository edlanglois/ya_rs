use crate::SCREEN_SCALE;
use crate::SCREEN_SIZE;
use bevy::math::const_vec2;
use bevy::prelude::*;

pub struct SpawnShieldEvent;

pub struct ShieldPlugin;

impl Plugin for ShieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnShieldEvent>()
            .add_startup_system(setup)
            .add_system(spawn)
            //.add_system(despawn)
            .add_system(monitor_health);
    }
}

#[derive(Component)]
pub struct Shield;

#[derive(Component)]
pub struct ShieldBlock {
    pub position: Vec2,
}

#[derive(Component)]
pub struct ShieldHealth {
    pub health: i32,
}

static SHIELD_SHAPE_CURVED: [usize; 128] = [
    0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0,
    1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0,
    1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0,
    0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1,
];

const SHIELD_HEIGHT_IN_BLOCKS: u32 = 16;
const SHIELD_WIDTH_IN_BLOCKS: u32 = 8;
pub const SHIELD_BLOCK_SPRITE_SIZE: Vec2 = const_vec2!([8.0 * SCREEN_SCALE, 8.0 * SCREEN_SCALE]);
const SHIELD_HEIGHT: f32 = SHIELD_HEIGHT_IN_BLOCKS as f32 * 8.0 * SCREEN_SCALE;
const SHIELD_WIDTH: f32 = SHIELD_WIDTH_IN_BLOCKS as f32 * 8.0 * SCREEN_SCALE;
const SHIELD_BLOCK_INITIAL_HEALTH: i32 = 5;

pub fn setup(mut spawn_event: EventWriter<SpawnShieldEvent>) {
    spawn_event.send(SpawnShieldEvent);
}

pub fn spawn(mut commands: Commands, mut spawn_event: EventReader<SpawnShieldEvent>) {
    if spawn_event.iter().next().is_none() {
        return;
    }

    let mut shield_origin = Transform::identity();
    shield_origin.translation.x = SCREEN_SIZE.x / 2.0 - SHIELD_WIDTH;
    shield_origin.translation.y -= SHIELD_HEIGHT / 2.0;

    let mut block_offset = Transform::identity();

    for y in 0..SHIELD_HEIGHT_IN_BLOCKS {
        for x in 0..SHIELD_WIDTH_IN_BLOCKS {
            let block_index = (x + y * SHIELD_WIDTH_IN_BLOCKS) as usize;
            if SHIELD_SHAPE_CURVED[block_index] == 1 {
                let block_transform = Transform::from_translation(
                    shield_origin.translation + block_offset.translation,
                );

                commands
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgb(0.34, 0.18, 0.05),
                            custom_size: Some(SHIELD_BLOCK_SPRITE_SIZE),
                            ..default()
                        },
                        transform: block_transform,
                        ..default()
                    })
                    .insert(ShieldBlock {
                        position: Vec2::new(x as f32, y as f32),
                    })
                    .insert(ShieldHealth {
                        health: SHIELD_BLOCK_INITIAL_HEALTH,
                    });
            }

            block_offset.translation.x += SHIELD_BLOCK_SPRITE_SIZE.x;
        }
        block_offset.translation.x = 0.0;
        block_offset.translation.y += SHIELD_BLOCK_SPRITE_SIZE.y;
    }
}

/*
pub fn despawn(
    mut commands: Commands,
    mut death_event: EventReader<YarDiedEvent>,
    blocks_query: Query<Entity, With<ShieldBlock>>,
) {
    if death_event.iter().next().is_none() {
        return;
    }

    for e in blocks_query.iter() {
        commands.entity(e).despawn();
    }
}
 */

pub fn monitor_health(mut commands: Commands, query: Query<(Entity, &ShieldHealth)>) {
    for (e, shield_health) in query.iter() {
        if shield_health.health <= 0 {
            commands.entity(e).despawn();
        }
    }
}
