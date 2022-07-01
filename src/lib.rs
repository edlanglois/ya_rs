use bevy::prelude::*;
use bevy::math::const_vec2;

mod yar;
mod qotile;
mod bullet;
mod game_state;
mod shield;
mod util;
mod zorlon_cannon;

// Not really, but close enough for this project.
// We have no concept of playfield memory and sprite memory so...
const ATARI_RES_X: f32 = 160.0 * 2.0;
const ATARI_RES_Y: f32 = 192.0;
const SCREEN_SCALE: f32 = 4.0;
const SCREEN_SIZE:Vec2 = const_vec2!([ATARI_RES_X * SCREEN_SCALE, ATARI_RES_Y * SCREEN_SCALE]);
const SPRITE_SIZE:Vec2 = const_vec2!([16.0, 16.0]);

pub struct ShootBullet;
pub struct SpawnZorlonCannon;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

pub fn setup_sprites(
    mut game_state: ResMut<game_state::GameState>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("yar_sprites.png");
    let texture_atlas = TextureAtlas::from_grid_with_padding(
        texture_handle,
        SPRITE_SIZE,
        8,
        3,
        Vec2::new(2.0, 2.0),
    );

    game_state.sprite_atlas =  texture_atlases.add(texture_atlas);
}

pub fn run() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "ya_rs' revenge".to_string(),
            width: SCREEN_SIZE.x,
            height: SCREEN_SIZE.y,
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .init_resource::<game_state::GameState>()
        .add_event::<ShootBullet>()
        .add_event::<SpawnZorlonCannon>()
        .add_startup_system(setup_camera)
        .add_startup_system(setup_sprites)
        .add_startup_system(yar::setup.after(setup_sprites))
        .add_startup_system(shield::setup.after(setup_sprites))
        .add_startup_system(qotile::setup)
        .add_system(yar::input)
        .add_system(yar::animate)
        .add_system(yar::collide_qotile)
        .add_system(bullet::shoot)
        .add_system(bullet::fly)
        .add_system(zorlon_cannon::spawn)
        .run();
}


