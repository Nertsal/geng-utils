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

/// Helper for rendering textures.
pub struct DrawTexture<'a> {
    /// The texture to render.
    pub texture: &'a ugli::Texture,
    /// World position to render at.
    pub target: Aabb2<f32>,
    /// Color to render the texture with.
    pub color: Rgba<f32>,
    /// Extra transformations applied before drawing.
    pub transform: mat3<f32>,
    // pub parameters: ugli::DrawParameters, // TODO: when geng allows passing parameters
}

impl<'a> DrawTexture<'a> {
    pub fn new(texture: &'a ugli::Texture) -> Self {
        Self {
            texture,
            target: Aabb2::ZERO.extend_positive(texture.size().as_f32()),
            color: Rgba::WHITE,
            transform: mat3::identity(),
            // parameters: ugli::DrawParameters::default(),
        }
    }

    pub fn colored(self, color: Rgba<f32>) -> Self {
        Self { color, ..self }
    }

    pub fn transformed(self, transform: mat3<f32>) -> Self {
        Self { transform, ..self }
    }

    /// Fit into the target [Aabb2] in world space and align.
    pub fn fit(self, target: Aabb2<f32>, align: vec2<f32>) -> Self {
        let target = crate::layout::fit_aabb(self.texture.size().as_f32(), target, align);
        Self { target, ..self }
    }

    /// Fit into the target [Aabb2] by width in world space and align vertically.
    pub fn fit_width(self, target: Aabb2<f32>, align: f32) -> Self {
        let target = crate::layout::fit_aabb_width(self.texture.size().as_f32(), target, align);
        Self { target, ..self }
    }

    /// Fit into the target [Aabb2] by height in world space and align horizontally.
    pub fn fit_height(self, target: Aabb2<f32>, align: f32) -> Self {
        let target = crate::layout::fit_aabb_height(self.texture.size().as_f32(), target, align);
        Self { target, ..self }
    }

    /// Fit to the whole framebuffer.
    pub fn fit_screen(self, align: vec2<f32>, framebuffer: &ugli::Framebuffer) -> Self {
        self.fit(
            Aabb2::ZERO.extend_positive(framebuffer.size().as_f32()),
            align,
        )
    }

    /// Fit to the width of the framebuffer.
    pub fn fit_screen_width(self, align: f32, framebuffer: &mut ugli::Framebuffer) -> Self {
        self.fit_width(
            Aabb2::ZERO.extend_positive(framebuffer.size().as_f32()),
            align,
        )
    }

    /// Fit to the height of the framebuffer.
    pub fn fit_screen_height(self, align: f32, framebuffer: &mut ugli::Framebuffer) -> Self {
        self.fit_height(
            Aabb2::ZERO.extend_positive(framebuffer.size().as_f32()),
            align,
        )
    }

    // /// Specifying the [DrawParameters](ugli::DrawParameters) to use when rendering.
    // pub fn with_parameters(self, parameters: ugli::DrawParameters) -> Self {
    //     Self { parameters, ..self }
    // }

    /// Align to pixel-perfect: such that pixels of the texture align with the pixels of the framebuffer.
    pub fn pixel_perfect(
        self,
        pos: vec2<f32>,
        align: vec2<f32>,
        pixel_scale: f32,
        camera: &impl geng::AbstractCamera2d,
        framebuffer: &mut ugli::Framebuffer,
    ) -> Self {
        let size = (self.texture.size().as_f32() * pixel_scale).map(|x| x.round() as usize);
        let target =
            crate::pixel::pixel_perfect_aabb(pos, align, size, camera, framebuffer.size().as_f32());
        Self { target, ..self }
    }

    pub fn as_textured_quad(self) -> draw2d::TexturedQuad<&'a ugli::Texture> {
        let transform = mat3::translate(self.target.center())
            * self.transform
            * mat3::scale(self.target.size() / 2.0);
        draw2d::TexturedQuad::unit_colored(self.texture, self.color).transform(transform)
    }

    pub fn draw(
        self,
        camera: &impl geng::AbstractCamera2d,
        geng: &Geng,
        framebuffer: &mut ugli::Framebuffer,
    ) {
        geng.draw2d()
            .draw2d(framebuffer, camera, &self.as_textured_quad());
    }
}
