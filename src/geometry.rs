use geng::prelude::*;

/// An axis-aligned square with width and height of 2.
pub fn unit_quad_geometry(ugli: &Ugli) -> ugli::VertexBuffer<draw2d::TexturedVertex> {
    ugli::VertexBuffer::new_dynamic(ugli, unit_quad().to_vec())
}

/// An axis-aligned square with width and height of 2.
pub fn unit_quad() -> [draw2d::TexturedVertex; 4] {
    [
        draw2d::TexturedVertex {
            a_pos: vec2(-1.0, -1.0),
            a_color: Rgba::WHITE,
            a_vt: vec2(0.0, 0.0),
        },
        draw2d::TexturedVertex {
            a_pos: vec2(1.0, -1.0),
            a_color: Rgba::WHITE,
            a_vt: vec2(1.0, 0.0),
        },
        draw2d::TexturedVertex {
            a_pos: vec2(1.0, 1.0),
            a_color: Rgba::WHITE,
            a_vt: vec2(1.0, 1.0),
        },
        draw2d::TexturedVertex {
            a_pos: vec2(-1.0, 1.0),
            a_color: Rgba::WHITE,
            a_vt: vec2(0.0, 1.0),
        },
    ]
}
