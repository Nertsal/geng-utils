use geng::prelude::*;

/// A value bounded by a closed interval.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Bounded<T> {
    value: T,
    min: T,
    max: T,
}

impl<T: PartialOrd + Copy> Bounded<T> {
    /// Construct a new value bounded by the given `range`.
    pub fn new(value: T, range: std::ops::RangeInclusive<T>) -> Self {
        Self {
            value,
            min: *range.start(),
            max: *range.end(),
        }
        .normalized()
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

    pub fn map<U>(&mut self, f: impl Fn(T) -> U) -> Bounded<U> {
        Bounded {
            value: f(self.value),
            min: f(self.min),
            max: f(self.max),
        }
    }

    fn normalized(self) -> Self {
        let value = self.value.clamp_range(self.min..=self.max);
        Self { value, ..self }
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
}

impl<T: UNum> Bounded<T> {
    /// Construct a new non-negative value with max set to be the initial value.
    pub fn new_max(value: T) -> Self {
        Self::new(value, T::ZERO..=value)
    }

    /// Construct a new non-negative value with a given max and initial value set to zero.
    pub fn new_zero(max: T) -> Self {
        Self::new(T::ZERO, T::ZERO..=max)
    }

    /// Changes the value by a `delta`, but keeps it in the interval.
    pub fn change(&mut self, delta: T) {
        self.value += delta;
        *self = self.normalized();
    }

    /// Sets the value after clamping it by the bounds.
    pub fn set(&mut self, value: T) {
        self.value = value;
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

    pub fn set_ratio(&mut self, ratio: T) {
        self.value = ratio.clamp_range(T::ZERO..=T::ONE) * (self.max - self.min) + self.min;
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
