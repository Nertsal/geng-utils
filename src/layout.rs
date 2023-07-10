use geng::prelude::*;

/// Get a point inside the aabb.
/// (0.0, 0.0) corresponds to min.
/// (1.0, 1.0) corresponds to max.
pub fn aabb_pos<T: Float>(aabb: Aabb2<T>, align: vec2<T>) -> vec2<T> {
    aabb.min + aabb.size() * align
}

/// Align an aabb of the given size inside another.
pub fn align_aabb<T: Float>(size: vec2<T>, aabb: Aabb2<T>, align: vec2<T>) -> Aabb2<T> {
    let half = T::from_f32(0.5);
    let pos_aabb = aabb.extend_symmetric(-size * half);
    let pos = aabb_pos(pos_aabb, align);
    Aabb2::point(pos).extend_symmetric(size * half)
}

/// Fit an aabb of the given size into the given aabb.
pub fn fit_aabb<T: Float>(size: vec2<T>, aabb: Aabb2<T>, align: vec2<T>) -> Aabb2<T> {
    let ratio = aabb.size() / size;
    let ratio = if ratio.x < ratio.y { ratio.x } else { ratio.y };
    let fit_size = size * ratio;
    align_aabb(fit_size, aabb, align)
}

/// Fit an aabb of the given size by width into the given aabb.
pub fn fit_aabb_width<T: Float>(size: vec2<T>, aabb: Aabb2<T>, align: T) -> Aabb2<T> {
    let ratio = aabb.width() / size.x;
    let fit_size = size * ratio;
    align_aabb(fit_size, aabb, vec2(T::ZERO, align))
}

/// Fit an aabb of the given size by height into the given aabb.
pub fn fit_aabb_height<T: Float>(size: vec2<T>, aabb: Aabb2<T>, align: T) -> Aabb2<T> {
    let ratio = aabb.height() / size.y;
    let fit_size = size * ratio;
    align_aabb(fit_size, aabb, vec2(align, T::ZERO))
}

#[test]
fn test_fit_aabb() {
    let size = vec2(1.0, 1.0);

    let aabb = Aabb2 {
        min: vec2(0.0, 0.0),
        max: vec2(10.0, 5.0),
    };
    assert_eq!(
        fit_aabb(size, aabb, vec2::splat(0.0)),
        Aabb2 {
            min: vec2(0.0, 0.0),
            max: vec2(5.0, 5.0),
        }
    );
    assert_eq!(
        fit_aabb(size, aabb, vec2::splat(0.5)),
        Aabb2 {
            min: vec2(2.5, 0.0),
            max: vec2(7.5, 5.0),
        }
    );
    assert_eq!(
        fit_aabb(size, aabb, vec2::splat(1.0)),
        Aabb2 {
            min: vec2(5.0, 0.0),
            max: vec2(10.0, 5.0),
        }
    );

    let aabb = Aabb2 {
        min: vec2(0.0, 0.0),
        max: vec2(5.0, 10.0),
    };
    assert_eq!(
        fit_aabb(size, aabb, vec2::splat(0.0)),
        Aabb2 {
            min: vec2(0.0, 0.0),
            max: vec2(5.0, 5.0),
        }
    );
    assert_eq!(
        fit_aabb(size, aabb, vec2::splat(0.5)),
        Aabb2 {
            min: vec2(0.0, 2.5),
            max: vec2(5.0, 7.5),
        }
    );
    assert_eq!(
        fit_aabb(size, aabb, vec2::splat(1.0)),
        Aabb2 {
            min: vec2(0.0, 5.0),
            max: vec2(5.0, 10.0),
        }
    );
}
