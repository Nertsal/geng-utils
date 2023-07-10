use geng::prelude::*;

/// A value bounded by a closed interval.
#[derive(Debug, Clone, Copy)]
pub struct Bounded<T> {
    value: T,
    min: T,
    max: T,
}

impl<T: Num> Bounded<T> {
    /// Construct a new non-zero value with max set to be the initial value.
    pub fn new_max(value: T) -> Self {
        Self::new(value, T::ZERO..=value)
    }

    /// Construct a new value bounded by the given `range`.
    pub fn new(value: T, range: std::ops::RangeInclusive<T>) -> Self {
        Self {
            value,
            min: *range.start(),
            max: *range.end(),
        }
        .normalized()
    }

    fn normalized(self) -> Self {
        let value = self.value.clamp_range(self.min..=self.max);
        Self { value, ..self }
    }

    pub fn value(&self) -> T {
        self.value
    }

    pub fn min(&self) -> T {
        self.min
    }

    pub fn max(&self) -> T {
        self.max
    }

    /// Whether the value is at a maximum.
    pub fn is_max(&self) -> bool {
        self.value >= self.max
    }

    /// Whether the value is at a minimum.
    pub fn is_min(&self) -> bool {
        self.value <= self.min
    }

    /// Whether the value is above minimum.
    pub fn is_above_min(&self) -> bool {
        self.value > self.min
    }

    /// Changes the value by a `delta`, but keeps it in the interval.
    pub fn change(&mut self, delta: T) {
        self.value += delta;
        *self = self.normalized();
    }
}

impl<T: Float> Bounded<T> {
    /// Returns a number in range 0..=1 representing the value in the interval
    /// where `0 = min`, and `1 = max`.
    pub fn get_ratio(&self) -> T {
        let len = self.max - self.min;
        if len.approx_eq(&T::ZERO) {
            T::ZERO
        } else {
            (self.value - self.min) / len
        }
    }
}

#[test]
fn test_bounded() {
    let mut bounded = Bounded::new(0.0, 0.0..=10.0);
    assert_eq!(bounded.value(), 0.0);

    bounded.change(5.0);
    assert_eq!(bounded.value(), 5.0);

    bounded.change(6.0);
    assert_eq!(bounded.value(), 10.0);

    bounded.change(-11.0);
    assert_eq!(bounded.value(), 0.0);
}
