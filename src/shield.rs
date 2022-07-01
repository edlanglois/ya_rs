use bevy::prelude::*;

#[derive(Component)]
pub struct Shield;

#[derive(Component)]
pub struct ShieldNode;

pub fn setup(
    mut commands: Commands,
    game_state: Res<crate::game_state::GameState>,
) {
    // Spawn the shield nodes!
}

// move
// eat
