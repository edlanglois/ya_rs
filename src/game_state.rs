use bevy::prelude::*;

#[derive(Component, Default)]
pub struct GameState
{
    pub bullet: Option<Entity>,
    pub zorlon_cannon: Option<Entity>,
    pub sprite_atlas: Handle<TextureAtlas>,
}

