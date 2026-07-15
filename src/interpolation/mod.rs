mod bezier;
mod sod;
mod spline;
mod uniform;

pub use self::{
    bezier::Bezier,
    sod::{SecondOrderDynamics, SecondOrderState},
    spline::Spline,
    uniform::calculate_uniform_transformation,
};

use geng::prelude::*;

pub fn smoothstep<T: Float>(t: T) -> T {
    T::from_f32(3.0) * t * t - T::from_f32(2.0) * t * t * t
}

/// A trait describing linearly interpolatable types.
pub trait Interpolatable: Clone {
    fn add(self, other: Self) -> Self;
    fn sub(self, other: Self) -> Self;
    fn scale(self, factor: f32) -> Self;
    fn length(self) -> f32 {
        self.length_sqr().sqrt()
    }
    fn length_sqr(self) -> f32;
}

impl<T: Float> Interpolatable for Angle<T> {
    fn add(self, other: Self) -> Self {
        self + other
    }
    fn sub(self, other: Self) -> Self {
        self - other
    }
    fn scale(self, factor: f32) -> Self {
        self * T::from_f32(factor)
    }
    fn length(self) -> f32 {
        self.as_radians().as_f32()
    }
    fn length_sqr(self) -> f32 {
        self.as_radians().as_f32().sqr()
    }
}

impl<A: Interpolatable, B: Interpolatable> Interpolatable for (A, B) {
    fn add(self, other: Self) -> Self {
        (self.0.add(other.0), self.1.add(other.1))
    }
    fn sub(self, other: Self) -> Self {
        (self.0.sub(other.0), self.1.sub(other.1))
    }
    fn scale(self, factor: f32) -> Self {
        (self.0.scale(factor), self.1.scale(factor))
    }
    fn length_sqr(self) -> f32 {
        self.0.length_sqr() + self.1.length_sqr()
    }
}

macro_rules! impl_interpolatable_for_float {
    ($T:ty) => {
        impl Interpolatable for $T {
            fn add(self, other: Self) -> Self {
                <$T as Add>::add(self, other)
            }
            fn sub(self, other: Self) -> Self {
                <$T as Sub>::sub(self, other)
            }
            fn scale(self, factor: f32) -> Self {
                <$T>::mul(self, <$T as Float>::from_f32(factor))
            }
            fn length(self) -> f32 {
                <$T as Float>::as_f32(self)
            }
            fn length_sqr(self) -> f32 {
                <$T as Float>::as_f32(self).sqr()
            }
        }
    };
}

macro_rules! impl_interpolatable_for_vec {
    ($T:ident) => {
        impl<T: Interpolatable + Copy> Interpolatable for $T<T> {
            fn add(self, other: Self) -> Self {
                $T::from(std::array::from_fn(|i| self[i].add(other[i])))
            }
            fn sub(self, other: Self) -> Self {
                $T::from(std::array::from_fn(|i| self[i].sub(other[i])))
            }
            fn scale(self, factor: f32) -> Self {
                self.map(|x| x.scale(factor))
            }
            fn length_sqr(self) -> f32 {
                self.iter().map(|x| x.length_sqr()).sum::<f32>()
            }
        }
    };
}

macro_rules! impl_interpolatable_for_mat {
    ($T:ident) => {
        impl<T: Interpolatable + Copy> Interpolatable for $T<T> {
            fn add(self, other: Self) -> Self {
                $T::new(std::array::from_fn(|i| {
                    std::array::from_fn(|j| self[(i, j)].add(other[(i, j)]))
                }))
            }
            fn sub(self, other: Self) -> Self {
                $T::new(std::array::from_fn(|i| {
                    std::array::from_fn(|j| self[(i, j)].sub(other[(i, j)]))
                }))
            }
            fn scale(self, factor: f32) -> Self {
                self.map(|x| x.scale(factor))
            }
            fn length_sqr(self) -> f32 {
                self.as_flat_array()
                    .iter()
                    .map(|x| x.length().sqr())
                    .sum::<f32>()
            }
        }
    };
}

impl_interpolatable_for_float!(f32);
impl_interpolatable_for_float!(f64);
impl_interpolatable_for_float!(R32);
impl_interpolatable_for_float!(R64);

impl_interpolatable_for_vec!(vec2);
impl_interpolatable_for_vec!(vec3);
impl_interpolatable_for_vec!(vec4);

impl_interpolatable_for_mat!(mat3);
impl_interpolatable_for_mat!(mat4);
