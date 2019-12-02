use super::*;
use std::convert::TryFrom;

/// Describe a layout to render with.  You'll probably start with a:
/// 
/// 
/// ```rust
/// use nines::*;
/// 
/// let layout = Layout {
///     src: Dimensions {
///         outer: Rect::xywh(0, 0, 3, 3),
///         inner: Rect::xywh(1, 1, 1, 1),
///     },
///     dst: Dimensions {
///         outer: Rect::xywh(0, 0, 5, 4),
///         inner: Rect::xywh(1, 1, 3, 2),
///     },
///     style: Style::default(), // stretch
/// };
/// 
/// let mut rects = Vec::new();
/// layout.validate().unwrap().each_dst_src(|dst, src|{
///     rects.push((*dst, *src));
/// });
/// 
/// assert_eq!(rects[0].0, Rect::xywh(0, 0, 1, 1));
/// assert_eq!(rects[1].0, Rect::xywh(1, 0, 3, 1));
/// // ...
/// 
/// assert_eq!(rects[0].1, Rect::xywh(0, 0, 1, 1));
/// assert_eq!(rects[1].1, Rect::xywh(1, 0, 1, 1));
/// // ...
/// ```
pub struct Layout<Dst, Src> {
    pub dst:    Dst,
    pub src:    Src,
    pub style:  Style,
}

impl<S: Scalar> Layout<Dimensions<S>, Dimensions<S>> {
    /// Validate that dst and src contain valid (non-negative sized, non-NAN) dimensions.
    pub fn validate(&self) -> Result<Layout<ValidDimensions<S>, ValidDimensions<S>>, Error> {
        Ok(Layout {
            dst:    self.dst.validate()?,
            src:    self.src.validate()?,
            style:  self.style,
        })
    }
}

impl<S: Scalar> TryFrom<Layout<Dimensions<S>, Dimensions<S>>> for Layout<ValidDimensions<S>, ValidDimensions<S>> {
    type Error = Error;
    fn try_from(value: Layout<Dimensions<S>, Dimensions<S>>) -> Result<Layout<ValidDimensions<S>, ValidDimensions<S>>, Self::Error> {
        value.validate()
    }
}

impl<S: Scalar> Layout<ValidDimensions<S>, ValidDimensions<S>> {
    /// Enumerate the destination and source rectangles for a layout.
    pub fn each_dst_src(&self, mut each_dst_src: impl FnMut(&ValidRect<S>, &ValidRect<S>)) {
        do_layout_9(self.dst, self.src, self.style, &mut each_dst_src);
    }

    #[cfg(test)] fn collect_dst_src_vec(&self) -> Vec<(ValidRect<S>, ValidRect<S>)> {
        let mut v = Vec::new();
        self.each_dst_src(|dst, src| v.push((*dst, *src)));
        v
    }
}

fn do_layout_9<S: Scalar>(dst: ValidDimensions<S>, src: ValidDimensions<S>, style: Style, each_dst_src: &mut impl FnMut(&ValidRect<S>, &ValidRect<S>)) {
    let dstx = [dst.outer.left, dst.inner.left, dst.inner.right, dst.outer.right];
    let dsty = [dst.outer.top, dst.inner.top, dst.inner.bottom, dst.outer.bottom];
    let srcx = [src.outer.left, src.inner.left, src.inner.right, src.outer.right];
    let srcy = [src.outer.top, src.inner.top, src.inner.bottom, src.outer.bottom];

    for (x, y, horizontal,              vertical                ) in [
        (0, 0, Scale::Stretch,          Scale::Stretch          ), // Corner:   Top Left
        (1, 0, Scale::Stretch,          style.border.top        ), // Edge:     Top
        (2, 0, Scale::Stretch,          Scale::Stretch          ), // Corner:   Top Right
        (0, 1, style.border.left,       Scale::Stretch          ), // Edge:     Left
        (1, 1, style.center.horizontal, style.center.vertical   ), // Center
        (2, 1, style.border.right,      Scale::Stretch          ), // Edge:     Right
        (0, 2, Scale::Stretch,          Scale::Stretch          ), // Corner:   Bottom Left
        (1, 2, Scale::Stretch,          style.border.bottom     ), // Edge:     Bottom
        (2, 2, Scale::Stretch,          Scale::Stretch          ), // Corner:   Bottom Right
    ].iter().copied() {
        let (dx0, dx1, dy0, dy1) = (dstx[x+0], dstx[x+1], dsty[y+0], dsty[y+1]);
        let (sx0, sx1, sy0, sy1) = (srcx[x+0], srcx[x+1], srcy[y+0], srcy[y+1]);
        do_layout_1(
            Rect::<S>::from([dx0..dx1, dy0..dy1]).debug_assert_valid(),
            Rect::<S>::from([sx0..sx1, sy0..sy1]).debug_assert_valid(),
            horizontal,
            vertical,
            each_dst_src,
        );
    }
}

fn do_layout_1<S: Scalar>(dst: ValidRect<S>, src: ValidRect<S>, _horizontal: Scale, _vertical: Scale, each_dst_src: &mut impl FnMut(&ValidRect<S>, &ValidRect<S>)) {
    // XXX: This is wrong, need to not ignore horizontal / vertical
    assert_eq!(_horizontal, Scale::Stretch, "Non-default horizontal scale not yet implemented");
    assert_eq!(_vertical,   Scale::Stretch, "Non-default vertical scale not yet implemented");
    each_dst_src(&dst, &src);
}

/// Expect a basic stretched Z pattern.
/// 
/// ### src
/// 
/// ```text
///   1   1   1
/// ┊←─→┊←─→┊←─→┊
/// ┌───┬───┬───┐┈┈
/// │ 0 │ 1 │ 2 │ ↕ 1
/// ├───┼───┼───┤┈┈
/// │ 3 │ 4 │ 5 │ ↕ 1
/// ├───┼───┼───┤┈┈
/// │ 6 │ 7 │ 8 │ ↕ 1
/// └───┴───┴───┘┈┈
/// ```
/// 
/// ### dst
/// 
/// ```text
///   1    2    1
/// ┊←─→┊←───→┊←─→┊
/// ┌───┬─────┬───┐┈┈
/// │ 0 │  1  │ 2 │ ↕ 1
/// ├───┼─────┼───┤┈┈
/// │   │     │   │
/// │ 3 │  4  │ 5 │  2
/// │   │     │   │
/// ├───┼─────┼───┤┈┈
/// │ 6 │  7  │ 8 │ ↕ 1
/// └───┴─────┴───┘┈┈
/// ```
#[test] fn layout_basic_z_test() {
    let layout = Layout {
        src: Dimensions {
            outer: Rect::xywh(0, 0, 3, 3),
            inner: Rect::xywh(1, 1, 1, 1),
        },
        dst: Dimensions {
            outer: Rect::xywh(0, 0, 5, 4),
            inner: Rect::xywh(1, 1, 3, 2),
        },
        style: Style::default(), // stretch
    };
    let rects = layout.validate().unwrap().collect_dst_src_vec();

    assert_eq!(*rects[0].0, Rect::xywh(0, 0, 1, 1));
    assert_eq!(*rects[1].0, Rect::xywh(1, 0, 3, 1));
    assert_eq!(*rects[2].0, Rect::xywh(4, 0, 1, 1));
    assert_eq!(*rects[3].0, Rect::xywh(0, 1, 1, 2));
    assert_eq!(*rects[4].0, Rect::xywh(1, 1, 3, 2));
    assert_eq!(*rects[5].0, Rect::xywh(4, 1, 1, 2));
    assert_eq!(*rects[6].0, Rect::xywh(0, 3, 1, 1));
    assert_eq!(*rects[7].0, Rect::xywh(1, 3, 3, 1));
    assert_eq!(*rects[8].0, Rect::xywh(4, 3, 1, 1));

    assert_eq!(*rects[0].1, Rect::xywh(0, 0, 1, 1));
    assert_eq!(*rects[1].1, Rect::xywh(1, 0, 1, 1));
    assert_eq!(*rects[2].1, Rect::xywh(2, 0, 1, 1));
    assert_eq!(*rects[3].1, Rect::xywh(0, 1, 1, 1));
    assert_eq!(*rects[4].1, Rect::xywh(1, 1, 1, 1));
    assert_eq!(*rects[5].1, Rect::xywh(2, 1, 1, 1));
    assert_eq!(*rects[6].1, Rect::xywh(0, 2, 1, 1));
    assert_eq!(*rects[7].1, Rect::xywh(1, 2, 1, 1));
    assert_eq!(*rects[8].1, Rect::xywh(2, 2, 1, 1));
}
