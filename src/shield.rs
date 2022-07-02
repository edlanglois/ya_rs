use bevy::prelude::*;

#[derive(Component)]
pub struct Shield;

#[derive(Component)]
pub struct ShieldNode;
//.add_startup_system(shield::setup.after(setup_sprites))

pub fn setup(
    mut _commands: Commands,
    _game_state: Res<crate::GameState>,
) {
    // Spawn the shield nodes!
}

// move
// eat
