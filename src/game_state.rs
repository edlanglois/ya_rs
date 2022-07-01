use bevy::prelude::*;

#[derive(Component, Default)]
pub struct GameState
{
    pub bullet: Option<Entity>,
    pub sprite_atlas: Handle<TextureAtlas>,
}

