use geng::prelude::*;

/// A single GIF frame that has a texture and a duration.
pub struct GifFrame {
    pub texture: ugli::Texture,
    /// Duration of the frame in seconds.
    pub duration: f32,
}

/// GIF load options.
pub struct GifOptions {
    pub frame: geng::asset::TextureOptions,
}

pub async fn load_gif(
    manager: &geng::asset::Manager,
    path: impl AsRef<std::path::Path>,
    options: GifOptions,
) -> anyhow::Result<Vec<GifFrame>> {
    use image::AnimationDecoder;

    let path = path.as_ref();
    log::debug!("Loading gif at {:?}", path);

    let data = <Vec<u8> as geng::asset::Load>::load(manager, path, &())
        .await
        .context("when loading gif bytes")?;
    let gif = image::codecs::gif::GifDecoder::new(data.as_slice()).context("when decoding gif")?;
    let frames = gif
        .into_frames()
        .map(|frame| {
            let frame = frame.unwrap();
            let (n, d) = frame.delay().numer_denom_ms();
            let duration = n as f32 / d as f32 / 1000.0;

            let mut image = frame.into_buffer();
            if options.frame.premultiply_alpha {
                for pixel in image.pixels_mut() {
                    use image::Pixel;
                    *pixel = pixel.map_without_alpha(|x| {
                        (x as f32 * (pixel[3] as f32 / 0xff as f32)).round() as u8
                    });
                }
            }

            let mut texture = ugli::Texture::from_image_image(manager.ugli(), image);
            texture.set_filter(options.frame.filter);
            texture.set_wrap_mode(options.frame.wrap_mode);

            GifFrame { texture, duration }
        })
        .collect();

    Ok(frames)
}
