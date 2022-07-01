use bevy::prelude::*;
use crate::SpawnZorlonCannon;
use crate::yar::Yar;
use crate::game_state::GameState;
use crate::SCREEN_SIZE;

#[derive(Component)]
pub struct ZorlonCannon;

pub fn spawn(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut spawn_event: EventReader<SpawnZorlonCannon>,
    query: Query<(&Transform, &Handle<TextureAtlas>, &Yar)>
) {
    if query.is_empty() {
        return
    }

    let (yar_transform, texture_atlas_handle, _) = query.single();

    let mut zorlon_transform = yar_transform.clone();
    zorlon_transform.translation.x = -SCREEN_SIZE.x / 2.0;

    for _e in spawn_event.iter() {
        if game_state.zorlon_cannon.is_some() {
            return;
        }

        game_state.zorlon_cannon = Some(commands
            .spawn_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite { index: 23, ..default() },
                texture_atlas: texture_atlas_handle.clone(),
                transform: zorlon_transform.clone(),
                ..default()
            })
            .insert(ZorlonCannon).id()
        );
    }
}