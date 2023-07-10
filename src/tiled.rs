use geng::prelude::*;

use crate::conversions::Vec2RealConversions;

/// Tiles the `area` with the texture uv's.
pub fn tile_area<T: Float>(
    tile_size: vec2<T>,
    offset: vec2<T>,
    area: Aabb2<T>,
) -> Vec<draw2d::TexturedVertex> {
    tile_area_subtexture(
        Aabb2::ZERO.extend_positive(vec2::splat(1.0)),
        tile_size,
        offset,
        area,
    )
}

/// Tiles the `area` with the subtexture uv's.
pub fn tile_area_subtexture<T: Float>(
    sub_texture: Aabb2<f32>,
    tile_size: vec2<T>,
    offset: vec2<T>,
    area: Aabb2<T>,
) -> Vec<draw2d::TexturedVertex> {
    let tile_size = tile_size.map(T::as_f32);
    let offset = offset.map(T::as_f32);
    let area = area.map(T::as_f32);

    let [a, b, c, d] = crate::geometry::unit_quad();
    let [va, vb, vc, vd] = sub_texture.corners();
    let set_uv = |v, uv| draw2d::TexturedVertex { a_vt: uv, ..v };
    let unit = [set_uv(a, va), set_uv(b, vb), set_uv(c, vc), set_uv(d, vd)];

    let tiles = (area.size() / tile_size).map(|x| x.ceil() as usize);
    (0..tiles.x)
        .flat_map(|x| {
            (0..tiles.y).flat_map(move |y| {
                let vs = unit.map(|v| draw2d::TexturedVertex {
                    a_pos: area.bottom_left() + offset + vec2(x, y).as_f32() * tile_size,
                    ..v
                });
                let indices = [0, 1, 2, 0, 2, 3];
                indices.map(|i| vs[i])
            })
        })
        .collect()
}
