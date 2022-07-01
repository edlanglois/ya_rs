use bevy::prelude::*;

// todo:
// flight
// shoot
// scale
// eat

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Clone)]
pub enum YarDirection {
    Left,
    Right,
    Up,
    UpRight,
    UpLeft,
    Down,
    DownRight,
    DownLeft,
}

#[derive(Component)]
pub struct Yar {
    pub direction: YarDirection,
    anim_frame: usize,
}

impl Default for Yar {
    fn default() -> Self {
        Yar {
            direction: YarDirection::Up,
            anim_frame: 0,
        }
    }
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("yar_sprites.png");
    let texture_atlas = TextureAtlas::from_grid_with_padding(
        texture_handle,
        Vec2::new(16.0, 16.0),
        8,
        3,
        Vec2::new(2.0, 2.0),
    );

    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(3.0)),
            ..default()
        })
        .insert(Yar::default())
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)));
}

pub fn input(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Yar)>) {
    let mut yar_delta = Transform::identity();

    let speed = 3.0;
    let mut direction = YarDirection::Up;

    if keys.pressed(KeyCode::W) {
        yar_delta.translation.y += speed;
    }

    if keys.pressed(KeyCode::S) {
        yar_delta.translation.y -= speed;
    }

    if keys.pressed(KeyCode::A) {
        yar_delta.translation.x -= speed;
    }

    if keys.pressed(KeyCode::D) {
        yar_delta.translation.x += speed;
    }

    if yar_delta.translation.x > 0.0 {
        if yar_delta.translation.y > 0.0 {
            direction = YarDirection::UpRight;
        } else if yar_delta.translation.y < 0.0 {
            direction = YarDirection::DownRight;
        } else {
            direction = YarDirection::Right;
        }
    } else if yar_delta.translation.x < 0.0 {
        if yar_delta.translation.y > 0.0 {
            direction = YarDirection::UpLeft;
        } else if yar_delta.translation.y < 0.0 {
            direction = YarDirection::DownLeft;
        } else {
            direction = YarDirection::Left;
        }
    } else {
        if yar_delta.translation.y < 0.0 {
            direction = YarDirection::Down;
        }
    }

    for (mut transform, mut yar) in query.iter_mut() {
        transform.translation += yar_delta.translation;
        yar.direction = direction.clone();
    }
}

pub fn animate(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
        &mut Yar,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle, mut yar) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();

            let sprite_base = match yar.direction {
                YarDirection::Up => 0,
                YarDirection::UpRight => 2,
                YarDirection::Right => 4,
                YarDirection::DownRight => 6,
                YarDirection::Down => 8,
                YarDirection::DownLeft => 10,
                YarDirection::Left => 12,
                YarDirection::UpLeft => 14,
            };

            let anim_length = 2;

            yar.anim_frame = (yar.anim_frame + 1) % anim_length;

            sprite.index = sprite_base + yar.anim_frame;
        }
    }
}
