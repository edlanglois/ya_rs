use bevy::prelude::*;

use crate::yar::Yar;
use crate::yar::YarDirection;
use crate::game_state::GameState;

#[derive(Component)]
pub struct Bullet {
    velocity: Vec3,
}

fn velocity_for_direction( direction: &YarDirection ) -> Vec3 {
    match direction {
        YarDirection::Left => Vec3::new(-1.0,0.0, 1.0),
        YarDirection::Right => Vec3::new(1.0,0.0, 1.0),
        YarDirection::Up => Vec3::new(0.0,1.0, 1.0),
        YarDirection::UpRight => Vec3::new(1.0,1.0, 1.0),
        YarDirection::UpLeft => Vec3::new(-1.0,1.0, 1.0),
        YarDirection::Down => Vec3::new(0.0,-1.0, 1.0),
        YarDirection::DownRight => Vec3::new(1.0,-1.0, 1.0),
        YarDirection::DownLeft => Vec3::new(-1.0,-1.0, 1.0),
        _ => Vec3::new(1.0, 0.0, 1.0)
    }
}

pub fn shoot(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &Handle<TextureAtlas>, &mut Yar)>
) {
    if game_state.bullet.is_some() {
        return;
    }

    for (mut transform, texture_atlas_handle, mut yar) in query.iter_mut() {
        if keys.pressed( KeyCode::Space) {
            game_state.bullet = Some( commands
                .spawn_bundle(SpriteSheetBundle {
                    sprite: TextureAtlasSprite {index: 21, ..default()},
                    texture_atlas: texture_atlas_handle.clone(),
                    transform: transform.clone(),
                    ..default()
                } )
                .insert(Bullet{velocity: velocity_for_direction(&yar.direction)}).id()
            );
        }
    }
}

pub fn fly(
    mut query: Query<(&mut Transform, &Bullet)>
) {
    for (mut transform, bullet) in query.iter_mut() {
        transform.translation += bullet.velocity;
    }
}