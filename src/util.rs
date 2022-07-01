use bevy::prelude::*;
use crate::SCREEN_SIZE;

pub fn is_offscreen(point: Vec3) -> bool {
    let half_width = SCREEN_SIZE.x / 2.0;
    let half_height = SCREEN_SIZE.y / 2.0;

    point.x < -half_width || point.x > half_width || point.y < -half_height || point.y > half_height
}
