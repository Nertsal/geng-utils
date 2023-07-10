use geng::prelude::*;

/// A position on a torus.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct PositionTorus<T> {
    pos: vec2<T>,
    world_size: vec2<T>,
}

impl<T: Num> PositionTorus<T> {
    /// Normalize a position in the world to a torus.
    pub fn from_world(mut pos: vec2<T>, world_size: vec2<T>) -> Self {
        while pos.y < T::ZERO {
            pos.y += world_size.y;
        }
        while pos.y > world_size.y {
            pos.y -= world_size.y;
        }
        while pos.x < T::ZERO {
            pos.x += world_size.x;
            // pos.y = world_size.y - pos.y;
        }
        while pos.x > world_size.x {
            pos.x -= world_size.x;
            // pos.y = world_size.y - pos.y;
        }

        Self { pos, world_size }
    }

    /// The zero position.
    pub fn zero(world_size: vec2<T>) -> Self {
        Self::from_world(vec2::ZERO, world_size)
    }

    /// Get a random position on a torus of a given size.
    pub fn random(rng: &mut impl Rng, world_size: vec2<T>) -> Self {
        Self::from_world(
            vec2(
                rng.gen_range(T::ZERO..=world_size.x),
                rng.gen_range(T::ZERO..=world_size.y),
            ),
            world_size,
        )
    }

    pub fn to_world(self) -> vec2<T> {
        self.pos
    }

    pub fn world_size(self) -> vec2<T> {
        self.world_size
    }

    /// Returns a delta from zero to `self`.
    pub fn as_dir(self) -> vec2<T> {
        Self::zero(self.world_size).delta_to(self)
    }

    /// Shift a position in-place by the given delta.
    pub fn shift(&mut self, delta: vec2<T>) {
        *self = self.shifted(delta);
    }

    /// Return a position shifted by the given delta
    pub fn shifted(self, delta: vec2<T>) -> Self {
        Self::from_world(self.to_world() + delta, self.world_size)
    }

    /// Calculate the delta from `self` to `towards`.
    /// Practically a subtraction operator.
    ///
    /// Panics if two positions don't have the same `world_size`.
    pub fn delta_to(self, towards: Self) -> vec2<T> {
        assert_eq!(
            self.world_size, towards.world_size,
            "two positions are not from the same world"
        );

        let mut delta = towards.to_world() - self.to_world();

        // Normalize delta
        let two = T::ONE + T::ONE;
        if delta.x.abs() * two > self.world_size.x {
            let signum = delta.x.signum();
            delta.x -= self.world_size.x * signum;
        }
        if delta.y.abs() * two > self.world_size.y {
            let signum = delta.y.signum();
            delta.y -= self.world_size.y * signum;
        }

        delta
    }
}

impl<T: Float> PositionTorus<T> {
    pub fn to_world_f32(self) -> vec2<f32> {
        self.pos.map(T::as_f32)
    }

    /// Calculate the distance between two points.
    ///
    /// Panics if two positions don't have the same `world_size`.
    pub fn distance(self, other: Self) -> T {
        self.delta_to(other).len()
    }
}

#[test]
fn test_delta() {
    let world_size = vec2(20.0, 10.0);
    let a = PositionTorus::from_world(vec2(15.0, 1.0), world_size);
    let b = PositionTorus::from_world(vec2(10.0, 5.0), world_size);
    let c = PositionTorus::from_world(vec2(2.0, 7.0), world_size);
    assert_eq!(a.delta_to(b), vec2(-5.0, 4.0));
    assert_eq!(a.delta_to(c), vec2(7.0, -4.0));
    assert_eq!(b.delta_to(c), vec2(-8.0, 2.0));
}
