//! 9-Slice scaling math.
//! 
//! ```text
//! left             right   ┌──→ +x
//!  ┊←──── outer ────→┊     │
//!  ┊                 ┊     ↓
//!  ┊  ┊←─ inner ─→┊  ┊    + y
//!  ┊  ┊           ┊  ┊
//!  ┌──┬───────────┬──┐┈┈┈┈┈┈┈┈ top
//!  │  │           │  │          ↑
//!  ├──┼───────────┼──┤┈┈┈┈      │
//!  │  │           │  │   ↑      │
//!  │  │           │  │ inner  outer
//!  │  │           │  │   ↓      │
//!  ├──┼───────────┼──┤┈┈┈┈      │
//!  │  │           │  │          ↓
//!  └──┴───────────┴──┘┈┈┈┈┈┈ bottom
//! ```
//! 
//! ### References
//! 
//! * [Wikipedia: 9-slice scaling](https://en.wikipedia.org/wiki/9-slice_scaling)
//! * [Unity: 9-slicing Sprites](https://docs.unity3d.com/Manual/9SliceSprites.html)
//! * [CSS Backgrounds and Borders ML3: &para;6 Border Images](https://www.w3.org/TR/css-backgrounds-3/#border-images)
//! 
//! [Scalar]:   trait.Scalar.html
//! [uNN]:      https://doc.rust-lang.org/std/primitive.u32.html
//! 
//! ### Crate Features
//! 
//! | feature           | Overview |
//! | ----------------- | -------- |
//! | debug             | Enable extra asserts for debugging nines itself.
//! | unsigned-scalar   | Allow [Scalar] to use underflow-prone [uNN] types.



const DEBUG : bool = cfg!(feature = "debug");

mod dimensions;
mod error;
mod layout;
mod rect;
mod scalar;
mod scale;
mod style;

pub use dimensions::{Dimensions, ValidDimensions};
pub use error::Error;
pub use layout::Layout;
pub use rect::{Rect, ValidRect};
pub use scalar::Scalar;
pub use scale::Scale;
pub use style::Style;

pub(crate) use error::err;



/// A { horizontal, vertical } pair.
#[derive(Clone, Copy, Debug, Default)]
pub struct Axises<V: std::fmt::Debug> {
    /// A horizontal (x-axis) value
    pub horizontal: V,

    /// A vertical (y-axis) value
    pub vertical:   V,
}
