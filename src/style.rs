use super::*;

/// The scaling style of a nine-square layout.
/// 
/// Corners, and the short axis of borders, are hardcoded against `Scale::Stretch`.
/// 
/// ```text
///    ┈┬───────────┬┈
///     │           │ 
///    ┈┼───────────┼┈
///     ┊           ┊
/// ```
/// 
/// Will use the values:
/// 
/// | axis | direction  | value |
/// | ---- | ---------- | ----- |
/// | `x` | `horizontal`    | `border.top`
/// | `y` | `vertical`      | `Scale::Stretch`
/// 
/// ```text
///  ┌──┬┈
///  │  │ 
///  ├──┼┈
///  ┊  ┊ 
/// ```
/// 
/// Will use the values:
/// 
/// | axis | direction  | value |
/// | ---- | ---------- | ----- |
/// | `x` | `horizontal`    | `Scale::Stretch`
/// | `y` | `vertical`      | `Scale::Stretch`
/// 
/// The center gets its own values:
/// 
/// ```text
///     ┊           ┊
///    ┈┼───────────┼┈
///     │           │
///     │           │
///     │           │
///    ┈┼───────────┼┈
///     ┊           ┊
/// ```
/// 
/// Will use the values:
/// 
/// | axis | direction  | value |
/// | ---- | ---------- | ----- |
/// | `x` | `horizontal`    | `center.horizontal`
/// | `y` | `vertical`      | `center.vertical`
#[derive(Clone, Copy, Debug, Default)]
pub struct Style {
    pub border:     Rect<Scale>,
    pub center:     Axises<Scale>,
}

impl Style {
    /// Create a new style with the same scaling on all axises.
    /// 
    /// ```text
    ///  ┊←→┊←─ scale ─→┊←→┊
    ///  ┌──┬───────────┬──┐┈┈┈┈
    ///  │  │           │  │   ↕
    ///  ├──┼───────────┼──┤┈┈┈┈
    ///  │  │           │  │   ↑
    ///  │  │           │  │ scale
    ///  │  │           │  │   ↓
    ///  ├──┼───────────┼──┤┈┈┈┈
    ///  │  │           │  │   ↕
    ///  └──┴───────────┴──┘┈┈┈┈
    /// ```
    pub const fn new(scale: Scale) -> Self {
        Self::new_horizontal_vertical(scale, scale)
    }

    /// Create a new style with uniform scaling along each axis.
    /// 
    /// ```text
    ///  ┊←→┊←horizontal→┊←→┊
    ///  ┌──┬────────────┬──┐┈┈┈┈┈
    ///  │  │            │  │    ↕
    ///  ├──┼────────────┼──┤┈┈┈┈┈
    ///  │  │            │  │    ↑
    ///  │  │            │  │ vertical
    ///  │  │            │  │    ↓
    ///  ├──┼────────────┼──┤┈┈┈┈┈
    ///  │  │            │  │    ↕
    ///  └──┴────────────┴──┘┈┈┈┈┈
    /// ```
    pub const fn new_horizontal_vertical(horizontal: Scale, vertical: Scale) -> Self {
        Self {
            border: Rect {
                left:   vertical,
                right:  vertical,
                top:    horizontal,
                bottom: horizontal,
            },
            center: Axises {
                horizontal,
                vertical,
            },
        }
    }
}
