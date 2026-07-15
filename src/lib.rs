//! A collection of utilities to use with [geng](https://github.com/geng-engine/geng) engine.

/// Bounded value.
pub mod bounded;
/// Handy conversion between types.
pub mod conversions;
/// Some primitive geometric shapes.
pub mod geometry;
/// GIF loader.
pub mod gif;
/// Different interpolation algorithms.
pub mod interpolation;
/// A possibly more convenient key enum.
pub mod key;
/// Layout utils.
pub mod layout;
/// Operations useful for pixel art.
pub mod pixel;
/// Asynchronous tasks wrapped in a sync API.
pub mod task;
/// Common operations with textures.
pub mod texture;
/// Generating tiled geometry.
pub mod tiled;
