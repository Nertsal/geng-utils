use geng::prelude::{Aabb2, Angle, Float, R32, R64, mat3, mat4, r32, vec2, vec3, vec4};

pub trait ResultUnwrapEitherExt {
    type Value;
    fn unwrap_either(self) -> Self::Value;
}

impl<T> ResultUnwrapEitherExt for Result<T, T> {
    type Value = T;
    fn unwrap_either(self) -> Self::Value {
        match self {
            Ok(t) | Err(t) => t,
        }
    }
}

/// A trait for converting into an [R32].
pub trait RealConversions {
    fn as_r32(&self) -> R32;
}

impl<T: Float> RealConversions for T {
    fn as_r32(&self) -> R32 {
        r32(self.as_f32())
    }
}

/// A trait for converting [Angle]'s inner types between [f32] and [R32].
pub trait AngleRealConversions {
    fn as_f32(&self) -> Angle<f32>;
    fn as_r32(&self) -> Angle<R32>;
}

/// A trait for converting [vec2]'s inner types between [f32] and [R32].
pub trait Vec2RealConversions {
    fn as_f32(&self) -> vec2<f32>;
    fn as_r32(&self) -> vec2<R32>;
}

/// A trait for converting [vec3]'s inner types between [f32] and [R32].
pub trait Vec3RealConversions {
    fn as_f32(&self) -> vec3<f32>;
    fn as_r32(&self) -> vec3<R32>;
}

/// A trait for converting [vec4]'s inner types between [f32] and [R32].
pub trait Vec4RealConversions {
    fn as_f32(&self) -> vec4<f32>;
    fn as_r32(&self) -> vec4<R32>;
}

/// A trait for converting [Aabb2]'s inner types between [f32] and [R32].
pub trait Aabb2RealConversions {
    fn as_f32(&self) -> Aabb2<f32>;
    fn as_r32(&self) -> Aabb2<R32>;
}

/// A trait for converting [mat3]'s inner types between [f32] and [R32].
pub trait Mat3RealConversions {
    fn as_f32(&self) -> mat3<f32>;
    fn as_r32(&self) -> mat3<R32>;
}

/// A trait for converting [mat4]'s inner types between [f32] and [R32].
pub trait Mat4RealConversions {
    fn as_f32(&self) -> mat4<f32>;
    fn as_r32(&self) -> mat4<R32>;
}

macro_rules! impl_lossy {
    ($trait:ident, $typ:tt, $t:ident) => {
        impl $trait for $typ<$t> {
            fn as_f32(&self) -> $typ<f32> {
                self.map(|x| x as f32)
            }
            fn as_r32(&self) -> $typ<R32> {
                self.map(|x| r32(x as f32))
            }
        }
    };
}

macro_rules! impl_float {
    ($trait:ident, $typ:tt, $t:ident) => {
        impl $trait for $typ<$t> {
            fn as_f32(&self) -> $typ<f32> {
                self.map(|x| x.as_f32())
            }
            fn as_r32(&self) -> $typ<R32> {
                self.map(|x| r32(x.as_f32()))
            }
        }
    };
}

macro_rules! impl_all_lossy {
    ($trait:ident, $typ:tt) => {
        impl_lossy!($trait, $typ, usize);
        impl_lossy!($trait, $typ, u8);
        impl_lossy!($trait, $typ, u16);
        impl_lossy!($trait, $typ, u32);
        impl_lossy!($trait, $typ, u64);
        // impl_lossy!($trait, $typ, u128); // TODO: Aabb2 does not fully support u128
        impl_lossy!($trait, $typ, isize);
        impl_lossy!($trait, $typ, i8);
        impl_lossy!($trait, $typ, i16);
        impl_lossy!($trait, $typ, i32);
        impl_lossy!($trait, $typ, i64);
        // impl_lossy!($trait, $typ, i128);
    };
}

macro_rules! impl_all_float {
    ($trait:ident, $typ:tt) => {
        impl_float!($trait, $typ, f32);
        impl_float!($trait, $typ, f64);
        impl_float!($trait, $typ, R32);
        impl_float!($trait, $typ, R64);
    };
}

impl_all_float!(AngleRealConversions, Angle);

impl_all_lossy!(Vec2RealConversions, vec2);
impl_all_float!(Vec2RealConversions, vec2);
impl_all_lossy!(Vec3RealConversions, vec3);
impl_all_float!(Vec3RealConversions, vec3);
impl_all_lossy!(Vec4RealConversions, vec4);
impl_all_float!(Vec4RealConversions, vec4);

impl_all_lossy!(Aabb2RealConversions, Aabb2);
impl_all_float!(Aabb2RealConversions, Aabb2);

impl<T: Float> Mat3RealConversions for mat3<T> {
    fn as_f32(&self) -> mat3<f32> {
        self.map(|x| x.as_f32())
    }
    fn as_r32(&self) -> mat3<R32> {
        self.map(|x| r32(x.as_f32()))
    }
}

impl<T: Float> Mat4RealConversions for mat4<T> {
    fn as_f32(&self) -> mat4<f32> {
        self.map(|x| x.as_f32())
    }
    fn as_r32(&self) -> mat4<R32> {
        self.map(|x| r32(x.as_f32()))
    }
}

#[test]
fn test_unwrap_either() {
    let result: Result<i32, i32> = Ok(1);
    assert_eq!(result.unwrap_either(), 1);
    let result: Result<i32, i32> = Err(2);
    assert_eq!(result.unwrap_either(), 2);
}
