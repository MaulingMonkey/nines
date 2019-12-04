use super::*;
use std::convert::TryFrom;
use std::fmt::Debug;
use std::ops::Deref;

/// Slice dimensions with non-negative & non-NAN dimensions (including borders.)
/// 
/// Do not base soundness assumptions on this definition of validity - `debug_assert_valid` can bypass checks.
#[derive(Clone, Copy, Debug, Default)]
pub struct ValidDimensions<S: Scalar>(Dimensions<S>);

impl<S: Scalar> Deref for ValidDimensions<S> { type Target = Dimensions<S>; fn deref(&self) -> &Self::Target { &self.0 } }
// NOTE:  Do *NOT* implement DerefMut!  That would allow validation to be bypassed, mooting the point of this type!

impl<S: Scalar> PartialEq<ValidDimensions<S>> for ValidDimensions<S> { fn eq(&self, other: &ValidDimensions<S>) -> bool { self.0 == other.0 } }
impl<S: Scalar> PartialEq<Dimensions<S>>      for ValidDimensions<S> { fn eq(&self, other: &Dimensions<S>     ) -> bool { self.0 == *other } }
impl<S: Scalar> PartialEq<ValidDimensions<S>> for Dimensions<S>      { fn eq(&self, other: &ValidDimensions<S>) -> bool { *self == other.0 } }

impl<S: Scalar> TryFrom< Dimensions<S>> for ValidDimensions<S> { type Error = Error; fn try_from(value:  Dimensions<S>) -> Result<Self, Error> { value.validate() } }
impl<S: Scalar> TryFrom<&Dimensions<S>> for ValidDimensions<S> { type Error = Error; fn try_from(value: &Dimensions<S>) -> Result<Self, Error> { value.validate() } }
impl<S: Scalar> From< ValidDimensions<S>> for Dimensions<S> { fn from(value:  ValidDimensions<S>) -> Self { value.0 } }
impl<S: Scalar> From<&ValidDimensions<S>> for Dimensions<S> { fn from(value: &ValidDimensions<S>) -> Self { value.0 } }
impl<S: Scalar> AsRef<Dimensions<S>> for ValidDimensions<S> { fn as_ref(&self) -> &Dimensions<S> { &self.0 } }
// NOTE:  Do *NOT* implement AsMut!  That would allow validation to be bypassed, mooting the point of this type!



/// The dimensions of a nine-square layout.  See also [ValidDimensions].
/// 
/// ```text
/// left             right   ┌──→ +x
///  ┊←──── outer ────→┊     │
///  ┊                 ┊     ↓
///  ┊  ┊←─ inner ─→┊  ┊    + y
///  ┊  ┊           ┊  ┊
///  ┌──┬───────────┬──┐┈┈┈┈┈┈┈┈ top
///  │  │           │  │          ↑
///  ├──┼───────────┼──┤┈┈┈┈      │
///  │  │           │  │   ↑      │
///  │  │           │  │ inner  outer
///  │  │           │  │   ↓      │
///  ├──┼───────────┼──┤┈┈┈┈      │
///  │  │           │  │          ↓
///  └──┴───────────┴──┘┈┈┈┈┈┈ bottom
/// ```
/// 
/// [ValidDimensions]:      struct.ValidDimensions.html
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Dimensions<S: Scalar> {
    pub outer: Rect<S>,
    pub inner: Rect<S>,
}

impl<S: Scalar> Dimensions<S> {
    /// Validate these dimensions are non-negative / non-NaN.  This means:
    /// 
    /// ```text
    /// outer.left ≤ inner.left ≤ inner.right ≤ outer.right
    /// outer.top ≤ inner.top ≤ inner.bottom ≤ outer.bottom
    /// ```
    #[must_use] pub fn validate(&self) -> Result<ValidDimensions<S>, Error> {
        if !(self.outer.left   <= self.inner.left  ) { return err("Expected outer.left ≤ inner.left"); }
        if !(self.inner.left   <= self.inner.right ) { return err("Expected inner.left ≤ inner.right"); }
        if !(self.inner.right  <= self.outer.right ) { return err("Expected inner.right ≤ outer.right"); }
        if !(self.outer.top    <= self.inner.top   ) { return err("Expected outer.top ≤ inner.top"); }
        if !(self.inner.top    <= self.inner.bottom) { return err("Expected inner.top ≤ inner.bottom"); }
        if !(self.inner.bottom <= self.outer.bottom) { return err("Expected inner.bottom ≤ outer.bottom"); }
        Ok(ValidDimensions(*self))
    }

    #[must_use] pub(crate) fn debug_assert_valid(&self) -> ValidDimensions<S> {
        if DEBUG {
            assert!(self.outer.left   <= self.inner.left,   "Expected outer.left ≤ inner.left");
            assert!(self.inner.left   <= self.inner.right,  "Expected inner.left ≤ inner.right");
            assert!(self.inner.right  <= self.outer.right,  "Expected inner.right ≤ outer.right");
            assert!(self.outer.top    <= self.inner.top,    "Expected outer.top ≤ inner.top");
            assert!(self.inner.top    <= self.inner.bottom, "Expected inner.top ≤ inner.bottom");
            assert!(self.inner.bottom <= self.outer.bottom, "Expected inner.bottom ≤ outer.bottom");
        }
        ValidDimensions(*self)
    }
}

impl<S: Scalar> ValidDimensions<S> {
    #[must_use] pub fn outer(&self) -> ValidRect<S> { self.outer.debug_assert_valid() }
    #[must_use] pub fn inner(&self) -> ValidRect<S> { self.inner.debug_assert_valid() }

    /// Get the sizes of the borders - that is, the spacing between the outer and inner rects.
    /// 
    /// ```text
    ///  left   right
    ///  ┊←→┊    ┊←→┊
    ///  ┊  ┊    ┊  ┊
    ///  ┌──┬────┬──┐┈┈
    ///  │  │    │  │ ↕ top
    ///  ├──┼────┼──┤┈┈
    ///  │  │    │  │
    ///  │  │    │  │
    ///  ├──┼────┼──┤┈┈
    ///  │  │    │  │ ↕ bottom
    ///  └──┴────┴──┘┈┈
    /// ```
    #[must_use] pub fn borders(&self) -> Rect<S> {
        Rect {
            left:   self.inner.left     - self.outer.left,
            right:  self.outer.right    - self.inner.right,
            top:    self.inner.top      - self.outer.top,
            bottom: self.outer.bottom   - self.inner.bottom,
        }
    }

    /// Create a new 9-slice with specified outer dimensions, keeping the border sizes the same.
    /// 
    /// Can return Err if:
    /// * `rect` is invalid
    /// * The center would have negative bounds
    /// 
    /// May panic on overflow/underflow.
    #[must_use] pub fn with_outer(&self, outer: impl Into<ValidRect<S>>) -> Result<Self, Error> {
        let borders = self.borders();
        let outer = outer.into().validate()?;
        if borders.left + borders.right > outer.width()  { return err("Resulting dimensions would have a negative center width"); }
        if borders.top + borders.bottom > outer.height() { return err("Resulting dimensions would have a negative center height"); }
        Ok(Dimensions {
            inner: outer.shrink(&borders),
            outer: *outer,
        }.debug_assert_valid())
    }

    /// Create a new 9-slice with specified inner dimensions, keeping the border sizes the same.
    /// 
    /// May panic on overflow/underflow.
    #[must_use] pub fn with_inner(&self, inner: impl Into<ValidRect<S>>) -> Self {
        let inner = inner.into();
        let borders = self.borders();
        Dimensions {
            inner: *inner,
            outer: inner.grow(&borders),
        }.debug_assert_valid()
    }
}



#[test] fn dims_int_test() {
    use std::mem::swap;

    let mut slice = Dimensions {
        outer: [0..10, 0..100].into(),
        inner: [1.. 8, 10..80].into(),
    };

    assert_eq!(slice.validate().unwrap().borders(), Rect { left: 1, right: 2, top: 10, bottom: 20 });
    assert_eq!(slice.validate().unwrap().outer(),   Rect::from([0..10, 0..100]));
    assert_eq!(slice.validate().unwrap().inner(),   Rect::from([1.. 8, 10..80]));

    assert!(slice.validate().is_ok());
    swap(&mut slice.outer, &mut slice.inner);
    assert!(slice.validate().is_err());
    swap(&mut slice.outer, &mut slice.inner);
    assert!(slice.validate().is_ok());
}

#[test] fn dims_f32_test() {
    use std::mem::swap;
    use std::f32::NAN;
    
    let mut slice = Dimensions {
        outer: [0.0..10.0, 0.0..100.0].into(),
        inner: [1.0.. 8.0, 10.0..80.0].into(),
    };

    assert_eq!(slice.validate().unwrap().borders(), Rect { left: 1.0, right: 2.0, top: 10.0, bottom: 20.0 });
    assert_eq!(slice.validate().unwrap().outer(),   Rect::from([0.0..10.0, 0.0..100.0]));
    assert_eq!(slice.validate().unwrap().inner(),   Rect::from([1.0.. 8.0, 10.0..80.0]));

    assert!(slice.validate().is_ok());
    swap(&mut slice.outer, &mut slice.inner);
    assert!(slice.validate().is_err());
    swap(&mut slice.outer, &mut slice.inner);
    assert!(slice.validate().is_ok());

    slice.outer.left  = NAN; assert!(slice.validate().is_err()); slice.outer.left  =  0.0; assert!(slice.validate().is_ok());
    slice.inner.left  = NAN; assert!(slice.validate().is_err()); slice.inner.left  =  1.0; assert!(slice.validate().is_ok());
    slice.inner.right = NAN; assert!(slice.validate().is_err()); slice.inner.right =  8.0; assert!(slice.validate().is_ok());
    slice.outer.right = NAN; assert!(slice.validate().is_err()); slice.outer.right = 10.0; assert!(slice.validate().is_ok());

    slice.outer.top    = NAN; assert!(slice.validate().is_err()); slice.outer.top    =   0.0; assert!(slice.validate().is_ok());
    slice.inner.top    = NAN; assert!(slice.validate().is_err()); slice.inner.top    =  10.0; assert!(slice.validate().is_ok());
    slice.inner.bottom = NAN; assert!(slice.validate().is_err()); slice.inner.bottom =  80.0; assert!(slice.validate().is_ok());
    slice.outer.bottom = NAN; assert!(slice.validate().is_err()); slice.outer.bottom = 100.0; assert!(slice.validate().is_ok());
}
