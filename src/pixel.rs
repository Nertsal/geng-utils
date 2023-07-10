use geng::prelude::*;

use crate::conversions::Vec2RealConversions;

/// Align a world position with a framebuffer pixel.
pub fn pixel_perfect_pos(
    pos: vec2<f32>,
    camera: &impl geng::AbstractCamera2d,
    framebuffer_size: vec2<f32>,
) -> vec2<f32> {
    // Transform to screen space
    let pos = camera_world_to_screen(pos, camera, framebuffer_size);
    let pos = pos.map(f32::floor);
    // Transform back to world
    camera.screen_to_world(framebuffer_size, pos)
}

/// Align a rectangle of the given pixel size with the framebuffer's pixels.
pub fn pixel_perfect_aabb(
    pos: vec2<f32>,
    align: vec2<f32>,
    size: vec2<usize>,
    camera: &impl geng::AbstractCamera2d,
    framebuffer_size: vec2<f32>,
) -> Aabb2<f32> {
    // Transform to screen space
    let pos = camera_world_to_screen(pos, camera, framebuffer_size);
    let align_size = (size.as_f32() * align).map(f32::fract);
    let pos = pos.map(f32::floor) + align_size;
    let screen_aabb = Aabb2::point(pos).extend_symmetric(size.as_f32() / 2.0);
    // Transform back to world
    screen_aabb.map_bounds(|pos| camera.screen_to_world(framebuffer_size, pos))
}

fn camera_world_to_screen(
    pos: vec2<f32>,
    camera: &impl geng::AbstractCamera2d,
    framebuffer_size: vec2<f32>,
) -> vec2<f32> {
    let pos = (camera.projection_matrix(framebuffer_size) * camera.view_matrix()) * pos.extend(1.0);
    let pos = pos.xy() / pos.z;
    // if pos.x.abs() > 1.0 || pos.y.abs() > 1.0 {
    //     return None;
    // }
    vec2(
        (pos.x + 1.0) / 2.0 * framebuffer_size.x,
        (pos.y + 1.0) / 2.0 * framebuffer_size.y,
    )
}
