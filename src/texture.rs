use geng::prelude::*;

use crate::conversions::Vec2RealConversions;

/// Construct a new black texture of the given size.
pub fn new_texture(ugli: &Ugli, size: vec2<usize>) -> ugli::Texture {
    ugli::Texture::new_with(ugli, size, |_| Rgba::BLACK)
}

/// Ensure the texture's size is equal to `size`.
/// If the size is the same, the texture doesn't change.
pub fn update_texture_size(texture: &mut ugli::Texture, size: vec2<usize>, ugli: &Ugli) {
    if texture.size() != size {
        *texture = ugli::Texture::new_with(ugli, size, |_| Rgba::BLACK);
        texture.set_filter(ugli::Filter::Nearest);
    }
}

/// Attach a texture to a color-only framebuffer.
pub fn attach_texture<'a>(texture: &'a mut ugli::Texture, ugli: &Ugli) -> ugli::Framebuffer<'a> {
    ugli::Framebuffer::new_color(ugli, ugli::ColorAttachment::Texture(texture))
}

/// Fit a texture into the target [Aabb2] in world space.
pub fn draw_texture_fit(
    texture: &ugli::Texture,
    target: Aabb2<f32>,
    align: vec2<f32>,
    camera: &impl geng::AbstractCamera2d,
    geng: &Geng,
    framebuffer: &mut ugli::Framebuffer,
) {
    let target = crate::layout::fit_aabb(texture.size().as_f32(), target, align);
    geng.draw2d()
        .textured_quad(framebuffer, camera, target, texture, Rgba::WHITE);
}

/// Fit a texture into the target [Aabb2] by width in world space.
pub fn draw_texture_fit_width(
    texture: &ugli::Texture,
    target: Aabb2<f32>,
    align: f32,
    camera: &impl geng::AbstractCamera2d,
    geng: &Geng,
    framebuffer: &mut ugli::Framebuffer,
) {
    let target = crate::layout::fit_aabb_width(texture.size().as_f32(), target, align);
    geng.draw2d()
        .textured_quad(framebuffer, camera, target, texture, Rgba::WHITE);
}

/// Fit a texture into the target [Aabb2] by height in world space.
pub fn draw_texture_fit_height(
    texture: &ugli::Texture,
    target: Aabb2<f32>,
    align: f32,
    camera: &impl geng::AbstractCamera2d,
    geng: &Geng,
    framebuffer: &mut ugli::Framebuffer,
) {
    let target = crate::layout::fit_aabb_height(texture.size().as_f32(), target, align);
    geng.draw2d()
        .textured_quad(framebuffer, camera, target, texture, Rgba::WHITE);
}

/// Fit a texture to the whole framebuffer.
pub fn draw_texture_fit_screen(
    texture: &ugli::Texture,
    align: vec2<f32>,
    geng: &Geng,
    framebuffer: &mut ugli::Framebuffer,
) {
    draw_texture_fit(
        texture,
        Aabb2::ZERO.extend_positive(framebuffer.size().as_f32()),
        align,
        &geng::PixelPerfectCamera,
        geng,
        framebuffer,
    );
}

/// Fit a texture to the width of the framebuffer.
pub fn draw_texture_fit_screen_width(
    texture: &ugli::Texture,
    align: f32,
    geng: &Geng,
    framebuffer: &mut ugli::Framebuffer,
) {
    draw_texture_fit_width(
        texture,
        Aabb2::ZERO.extend_positive(framebuffer.size().as_f32()),
        align,
        &geng::PixelPerfectCamera,
        geng,
        framebuffer,
    );
}

/// Fit a texture to the height of the framebuffer.
pub fn draw_texture_fit_screen_height(
    texture: &ugli::Texture,
    align: f32,
    geng: &Geng,
    framebuffer: &mut ugli::Framebuffer,
) {
    draw_texture_fit_height(
        texture,
        Aabb2::ZERO.extend_positive(framebuffer.size().as_f32()),
        align,
        &geng::PixelPerfectCamera,
        geng,
        framebuffer,
    );
}
