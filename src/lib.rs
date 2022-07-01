use bevy::prelude::*;

mod yar;
mod bullet;
mod game_state;

// Plan:
// Hour 1
//  * Plan
//  * Basic app setup.
//  * Yar idle.
// Hour 2
//  * Yar flight w/ anims.
//  * Yar bullet.
// Hour 3
//  - Destroy bullet if offscreen.
//  - Wrap yar if fly offscreen.
//  - Get screen size and sprite size correct.


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
// ...
// Hour ?
//   - Sound FX
//   - Throb
//   - Qotile Death
//   - Swirl Powerup / Swirl Shoot
//   - Eat
//   - Death
// Hour ?
//   - WASM
//   - React Web

pub struct YarsPlugin;

impl Plugin for YarsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<game_state::GameState>()
            .add_startup_system(setup_camera)
            .add_startup_system(yar::setup)
            .add_system(yar::input)
            .add_system(yar::animate)
            .add_system(bullet::shoot)
            .add_system(bullet::fly)
        ;
    }
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(YarsPlugin)
        .run();
}
