use std::ops::{Add, Sub};
use std::fmt::{Debug};

/// [iNN] or [fNN] intrinsics.  Opt-in to underflow-prone [uNN] support via `"unsigned-scalars"` feature.
/// 
/// [iNN]:      https://doc.rust-lang.org/std/primitive.i32.html
/// [uNN]:      https://doc.rust-lang.org/std/primitive.u32.html
/// [fNN]:      https://doc.rust-lang.org/std/primitive.f32.html
pub trait Scalar : Copy + Add<Output = Self> + Sub<Output = Self> + Debug + Default + PartialOrd {}
//impl<S: Copy + Add<Output = S> + Sub<Output = S> + Debug + Default + PartialOrd> Scalar for S {}

impl Scalar for i8    {}
impl Scalar for i16   {}
impl Scalar for i32   {}
impl Scalar for i64   {}
impl Scalar for i128  {}
impl Scalar for isize {}

// XXX: These are trivial to underflow in UI layout, so I've chosen to discourage these.
#[cfg(feature = "unsigned-scalar")] impl Scalar for u8    {}
#[cfg(feature = "unsigned-scalar")] impl Scalar for u16   {}
#[cfg(feature = "unsigned-scalar")] impl Scalar for u32   {}
#[cfg(feature = "unsigned-scalar")] impl Scalar for u64   {}
#[cfg(feature = "unsigned-scalar")] impl Scalar for u128  {}
#[cfg(feature = "unsigned-scalar")] impl Scalar for usize {}

impl Scalar for f32   {}
impl Scalar for f64   {}
