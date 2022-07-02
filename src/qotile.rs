use bevy::prelude::*;
use bevy::math::const_vec2;

use crate::SCREEN_SIZE;
use crate::SCREEN_SCALE;

const QOTILE_SPRITE_SIZE:Vec2 = const_vec2!([16.0, 18.0]);
const QOTILE_INSET:f32 = 16.0;
pub const QOTILE_BOUNDS:Vec2 = const_vec2!([16.0*SCREEN_SCALE, 18.0*SCREEN_SCALE]);

pub struct QotileDiedEvent;

pub struct QotilePlugin;

impl Plugin for QotilePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<QotileDiedEvent>()
            .add_startup_system(setup)
        ;
    }
}

#[derive(Component)]
pub struct Qotile;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let mut transform = Transform::from_scale( Vec3::splat( crate::SCREEN_SCALE ) );
    transform.translation.x += (SCREEN_SIZE.x/2.0) - (QOTILE_SPRITE_SIZE.x * crate::SCREEN_SCALE/2.0) - QOTILE_INSET;

    commands
        .spawn_bundle( SpriteBundle {
            texture: asset_server.load("qotile.png"),
            transform: transform,
            ..default()
        })
        .insert( Qotile );
}