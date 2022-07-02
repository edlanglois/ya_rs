use bevy::prelude::*;
use crate::SCREEN_SIZE;

pub fn is_offscreen(point: Vec3) -> bool {
    let half_width = SCREEN_SIZE.x / 2.0;
    let half_height = SCREEN_SIZE.y / 2.0;

    point.x < -half_width || point.x > half_width || point.y < -half_height || point.y > half_height
}

// intersect 2d aabb
// This is a Valve interview question. Sort of like asking "have you ever written a game before?"
pub fn intersect_rect(p1:&Vec3, b1:&Vec2, p2:&Vec3, b2:&Vec2) -> bool {
    let y1min = p1.y - b1.y/2.0;
    let y1max = p1.y + b1.y/2.0;
    let x1min = p1.x - b1.x/2.0;
    let x1max = p1.x + b1.x/2.0;

    let y2min = p2.y - b2.y/2.0;
    let y2max = p2.y + b2.y/2.0;
    let x2min = p2.x - b2.x/2.0;
    let x2max = p2.x + b2.x/2.0;

    let intersect_y = y1min < y2max && y1max > y2min;
    let intersect_x = x1min < x2max && x1max > x2min;

    intersect_y && intersect_x
}
