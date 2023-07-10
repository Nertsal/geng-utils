use geng::prelude::*;

/// Get a point inside the aabb.
/// (0.0, 0.0) corresponds to min.
/// (1.0, 1.0) corresponds to max.
pub fn aabb_pos<T: Float>(aabb: Aabb2<T>, align: vec2<T>) -> vec2<T> {
    aabb.min + aabb.size() * align
}

/// Fit an aabb of the given size into the given aabb.
pub fn fit_aabb<T: Float>(size: vec2<T>, aabb: Aabb2<T>, align: vec2<T>) -> Aabb2<T> {
    let half = T::from_f32(0.5);
    let target_size = aabb.size();

    let ratio = target_size / size;
    let ratio = if ratio.x < ratio.y { ratio.x } else { ratio.y };
    let fit_size = size * ratio;

    let pos_aabb = aabb.extend_symmetric(-fit_size * half);
    let pos = aabb_pos(pos_aabb, align);

    Aabb2::point(pos).extend_symmetric(fit_size * half)
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
