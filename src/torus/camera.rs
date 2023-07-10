use geng::prelude::*;

use super::position::PositionTorus;

/// A camera that exists on a torus space.
///
/// Use the [project](CameraTorus::project) method to get a normalized position of objects
/// relative to the camera.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraTorus<T: Float> {
    pub center: PositionTorus<T>,
    pub fov: T,
    pub rotation: Angle<T>,
}

impl<T: Float> CameraTorus<T> {
    /// Construct a new camera on a torus with the given size.
    pub fn new(fov: T, world_size: vec2<T>) -> Self {
        Self {
            center: PositionTorus::zero(world_size),
            fov,
            rotation: Angle::ZERO,
        }
    }

    fn to_camera2d(&self) -> geng::Camera2d {
        geng::Camera2d {
            center: self.center.to_world().map(T::as_f32),
            rotation: Angle::ZERO,
            fov: self.fov.as_f32(),
        }
    }

    /// Project a world position to a position relative to the camera.
    /// The resulting position can be used to render the object.
    pub fn project(&self, position: PositionTorus<T>) -> vec2<T> {
        let center = self.center.to_world();
        center + self.center.delta_to(position)
    }

    /// Project a world position to a position relative to the camera.
    /// The resulting position can be used to render the object.
    pub fn project_f32(&self, position: PositionTorus<T>) -> vec2<f32> {
        self.project(position).map(T::as_f32)
    }
}

impl<T: Float> geng::AbstractCamera2d for CameraTorus<T> {
    fn view_matrix(&self) -> mat3<f32> {
        self.to_camera2d().view_matrix()
    }

    fn projection_matrix(&self, framebuffer_size: vec2<f32>) -> mat3<f32> {
        self.to_camera2d().projection_matrix(framebuffer_size)
    }
}
