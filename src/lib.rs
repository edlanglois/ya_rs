use bevy::prelude::*;

mod animation;
use animation::SpriteAnimationPlugin;

mod yar;

// Todo:
// Yar
// Field
// Qotile
// Destroyer
// Neutral Zone
// Bullet

pub struct YarsPlugin;

impl Plugin for YarsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_camera)
            .add_startup_system(yar::setup);
    }
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(YarsPlugin)
        .add_plugin(SpriteAnimationPlugin)
        .run();
}
