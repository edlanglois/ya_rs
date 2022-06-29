use bevy::prelude::*;
use bevy::sprite::Rect;

// todo:
// flight
// shoot
// scale
// eat

use crate::animation::AnimationTimer;

#[derive(Component)]
pub struct Yar;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("yars_sprites.png");
    let mut texture_atlas = TextureAtlas::new_empty(texture_handle, Vec2::new(8.0, 8.0));

    // Yar Idle 1
    texture_atlas.add_texture(Rect {
        min: Vec2::new(0.0, 8.0),
        max: Vec2::new(16.0, 24.0),
    });

    // Yar Idle 2
    texture_atlas.add_texture(Rect {
        min: Vec2::new(18.0, 8.0),
        max: Vec2::new(34.0, 24.0),
    });

    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..default()
        })
        .insert(Yar)
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)));
}
