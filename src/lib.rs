use bevy::math::const_vec2;
use bevy::prelude::*;

mod bullet;
mod destroyer_missile;
mod neutral_zone;
mod qotile;
mod shield;
mod util;
mod yar;
mod zorlon_cannon;

// Not really, but close enough for this project.
// We have no concept of playfield memory and sprite memory so...
const ATARI_RES_X: f32 = 160.0 * 2.0;
const ATARI_RES_Y: f32 = 192.0;
const SCREEN_SCALE: f32 = 4.0;
const SCREEN_SIZE: Vec2 = const_vec2!([ATARI_RES_X * SCREEN_SCALE, ATARI_RES_Y * SCREEN_SCALE]);
const SPRITE_SIZE: Vec2 = const_vec2!([16.0, 16.0]);

#[derive(Component, Default)]
pub struct GameState {
    pub sprite_atlas: Handle<TextureAtlas>,
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

pub fn setup_sprites(
    mut game_state: ResMut<GameState>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("yar_sprites.png");
    let texture_atlas = TextureAtlas::from_grid_with_padding(
        texture_handle,
        SPRITE_SIZE,
        8,
        4,
        Vec2::new(2.0, 2.0),
    );

    game_state.sprite_atlas = texture_atlases.add(texture_atlas);
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
        .init_resource::<GameState>()
        .add_plugins(DefaultPlugins)
        .add_plugin(yar::YarPlugin)
        .add_plugin(bullet::BulletPlugin)
        .add_plugin(zorlon_cannon::ZorlonCannonPlugin)
        .add_plugin(destroyer_missile::DestroyerMissilePlugin)
        .add_plugin(qotile::QotilePlugin)
        .add_plugin(neutral_zone::NeutralZonePlugin)
        .add_plugin(shield::ShieldPlugin)
        .add_startup_system(setup_camera)
        .add_startup_system(setup_sprites)
        .run();
}
