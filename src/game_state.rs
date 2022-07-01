use bevy::prelude::*;

use crate::bullet::Bullet;

#[derive(Component, Default)]
pub struct GameState
{
    pub bullet: Option<Entity>,
}

