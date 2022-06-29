use bevy::prelude::*;

mod animation;
use animation::SpriteAnimationPlugin;

mod yar;

// Plan:
// Hour 1
//  - Plan
//  - Basic app setup.
//  - Yar idle.
// Hour 2
//  - Yar flight w/ anims.
//  - Yar bullet.
// Hour 3
//   - Qotile's Shield
//   - Eat
// Hour 4
//   - Neutral Zone
// Hour 5
//   - Qotile
//   - Destroyer Missile
// Hour 6
//   - Swirl
// ...

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
