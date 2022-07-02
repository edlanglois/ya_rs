use bevy::prelude::*;

use crate::yar::Yar;
use crate::yar::YarDirection;
use crate::game_state::GameState;
use crate::zorlon_cannon::ZorlonCannonState;
use crate::ShootEvent;
use crate::util::is_offscreen;

#[derive(Component)]
pub struct Bullet {
    velocity: Vec3,
}

const BULLET_SPEED: f32 = 6.0;

fn velocity_for_direction( direction: &YarDirection ) -> Vec3 {
    let vector = match direction {
        YarDirection::Left => Vec3::new(-1.0,0.0, 1.0),
        YarDirection::Right => Vec3::new(1.0,0.0, 1.0),
        YarDirection::Up => Vec3::new(0.0,1.0, 1.0),
        YarDirection::UpRight => Vec3::new(1.0,1.0, 1.0),
        YarDirection::UpLeft => Vec3::new(-1.0,1.0, 1.0),
        YarDirection::Down => Vec3::new(0.0,-1.0, 1.0),
        YarDirection::DownRight => Vec3::new(1.0,-1.0, 1.0),
        YarDirection::DownLeft => Vec3::new(-1.0,-1.0, 1.0),
    };
    vector * BULLET_SPEED
}

pub fn shoot(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    zc_state: ResMut<ZorlonCannonState>,
    mut shoot_event: EventReader<ShootEvent>,
    mut query: Query<(&Transform, &Handle<TextureAtlas>, &Yar)>
) {
    if zc_state.zorlon_cannon.is_some() {
        return;
    }

    if game_state.bullet.is_some() {
        return;
    }

    if query.is_empty() {
        return
    }

    let (transform, texture_atlas_handle, yar) = query.single_mut();

    if shoot_event.iter().next().is_some() {
        game_state.bullet = Some(commands
            .spawn_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite { index: 21, ..default() },
                texture_atlas: texture_atlas_handle.clone(),
                transform: transform.clone(),
                ..default()
            })
            .insert(Bullet { velocity: velocity_for_direction(&yar.direction) }).id()
        );
    }
}


pub fn fly(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut query: Query<(Entity, &mut Transform, &Bullet)>
) {
    if query.is_empty() {
        return
    }

    let ( e, mut transform, bullet ) = query.single_mut();

    if is_offscreen( transform.translation ) {
        commands.entity(e).despawn();
        game_state.bullet = None;
        return
    }

    transform.translation += bullet.velocity;
}