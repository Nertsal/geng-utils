use geng::prelude::*;

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
