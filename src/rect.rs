use super::*;
use std::convert::TryFrom;
use std::fmt::Debug;
use std::ops::{Deref, Range};

/// A rectangle with non-negative & non-NAN dimensions.
/// 
/// Do not base soundness assumptions on this definition of validity - `debug_assert_valid` can bypass checks.
#[derive(Clone, Copy, Debug, Default)]
pub struct ValidRect<S: Scalar>(Rect<S>);

impl<S: Scalar> Deref for ValidRect<S> { type Target = Rect<S>; fn deref(&self) -> &Self::Target { &self.0 } }
// NOTE:  Do *NOT* implement DerefMut!  That would allow validation to be bypassed, mooting the point of this type!

impl<S: Scalar> PartialEq<ValidRect<S>> for ValidRect<S> { fn eq(&self, other: &ValidRect<S>) -> bool { self.0 == other.0 } }
impl<S: Scalar> PartialEq<Rect<S>>      for ValidRect<S> { fn eq(&self, other: &Rect<S>     ) -> bool { self.0 == *other } }
impl<S: Scalar> PartialEq<ValidRect<S>> for Rect<S>      { fn eq(&self, other: &ValidRect<S>) -> bool { *self == other.0 } }

impl<S: Scalar> TryFrom< Rect<S>> for ValidRect<S> { type Error = Error; fn try_from(value:  Rect<S>) -> Result<Self, Error> { value.validate() } }
impl<S: Scalar> TryFrom<&Rect<S>> for ValidRect<S> { type Error = Error; fn try_from(value: &Rect<S>) -> Result<Self, Error> { value.validate() } }
impl<S: Scalar> From< ValidRect<S>> for Rect<S> { fn from(value:  ValidRect<S>) -> Self { value.0 } }
impl<S: Scalar> From<&ValidRect<S>> for Rect<S> { fn from(value: &ValidRect<S>) -> Self { value.0 } }
impl<S: Scalar> AsRef<Rect<S>> for ValidRect<S> { fn as_ref(&self) -> &Rect<S> { &self.0 } }
// NOTE:  Do *NOT* implement AsMut!  That would allow validation to be bypassed, mooting the point of this type!



/// A rectangle.  See also [ValidRect].  Generally not inclusive of the right/bottom edge.
/// 
/// [ValidRect]:    struct.ValidRect.html
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Rect<V: Debug> {
    pub left:   V,
    pub right:  V,
    pub top:    V,
    pub bottom: V,
}

impl<S: Scalar> ValidRect<S> {
    #[must_use] pub fn width(&self) -> S { self.right - self.left }
    #[must_use] pub fn height(&self) -> S { self.bottom - self.top }
    #[must_use] pub fn size(&self) -> [S; 2] { [self.width(), self.height()] }
}

impl<S: Scalar> Rect<S> {
    #[must_use] pub fn xywh(x: S, y: S, w: S, h: S) -> Self { Self { left: x, top: y, right: x + w, bottom: y + h } }

    #[must_use] pub fn grow(&self, borders: &Self) -> Self {
        Self {
            left:   self.left   - borders.left,
            right:  self.right  + borders.right,
            top:    self.top    - borders.top,
            bottom: self.bottom + borders.bottom,
        }
    }

    #[must_use] pub fn shrink(&self, borders: &Self) -> Self {
        Self {
            left:   self.left   + borders.left,
            right:  self.right  - borders.right,
            top:    self.top    + borders.top,
            bottom: self.bottom - borders.bottom,
        }
    }

    /// Validate this rectangle has non-negative / non-NaN dimensions.  This means:
    /// 
    /// ```text
    /// left ≤ right
    /// top ≤ bottom
    /// ```
    #[must_use] pub fn validate(&self) -> Result<ValidRect<S>, Error> {
        if !(self.left <= self.right) { return err("Expected left ≤ right"); }
        if !(self.top <= self.bottom) { return err("Expected top ≤ bottom"); }
        Ok(ValidRect(*self))
    }

    #[must_use] pub(crate) fn debug_assert_valid(&self) -> ValidRect<S> {
        if DEBUG {
            assert!(self.left <= self.right, "Expected left ≤ right");
            assert!(self.top <= self.bottom, "Expected top ≤ bottom");
        }
        ValidRect(*self)
    }
}

impl<S: Scalar> From<Range<[S; 2]>> for Rect<S> {
    fn from(value: Range<[S; 2]>) -> Self {
        Self { left: value.start[0], right: value.end[0], top: value.start[1], bottom: value.end[1] }
    }
}

impl<S: Scalar> From<[Range<S>; 2]> for Rect<S> {
    fn from(value: [Range<S>; 2]) -> Self {
        Self { left: value[0].start, right: value[0].end, top: value[1].start, bottom: value[1].end }
    }
}

impl<S: Scalar> From<Range<(S, S)>> for Rect<S> {
    fn from(value: Range<(S, S)>) -> Self {
        Self { left: value.start.0, right: value.end.0, top: value.start.1, bottom: value.end.1 }
    }
}

impl<S: Scalar> From<(Range<S>, Range<S>)> for Rect<S> {
    fn from(value: (Range<S>, Range<S>)) -> Self {
        Self { left: value.0.start, right: value.0.end, top: value.1.start, bottom: value.1.end }
    }
}

#[test] fn rect_test() {
    use std::f32::NAN;

    let r = Rect::xywh(10, 20, 30, 40).validate().unwrap();
    assert_eq!(r.left,      10);
    assert_eq!(r.right,     40);
    assert_eq!(r.top,       20);
    assert_eq!(r.bottom,    60);
    assert_eq!(r.width(),   30);
    assert_eq!(r.height(),  40);
    assert_eq!(r.size(),    [30, 40]);

    assert_eq!(r, Rect::from([10..40, 20..60]));
    assert_eq!(r, Rect::from([10,20]..[40,60]));
    assert_eq!(r, Rect::from((10..40, 20..60)));
    assert_eq!(r, Rect::from((10,20)..(40,60)));

    assert!(r.validate().is_ok());
    assert!(Rect::xywh(0, 0,  0,  0).validate().is_ok());
    assert!(Rect::xywh(0, 0,  0, -1).validate().is_err());
    assert!(Rect::xywh(0, 0, -1,  0).validate().is_err());
    assert!(Rect::xywh(0, 0, -1, -1).validate().is_err());

    assert!(Rect::xywh(0.0, 0.0, 0.0, 0.0).validate().is_ok());
    assert!(Rect::xywh(NAN, 0.0, 0.0, 0.0).validate().is_err());
    assert!(Rect::xywh(0.0, NAN, 0.0, 0.0).validate().is_err());
    assert!(Rect::xywh(0.0, 0.0, NAN, 0.0).validate().is_err());
    assert!(Rect::xywh(0.0, 0.0, 0.0, NAN).validate().is_err());
}
