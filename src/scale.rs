/// How to scale images.  Based roughly off of the CSS3 [border-image-repeat] property values.
/// 
/// [border-image-repeat]:  https://www.w3.org/TR/css-backgrounds-3/#the-border-image-repeat
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Scale {
    /// The border image is used exactly once, scaled as far up/down as necessary.
    Stretch,

    /// The border image is used `floor(image_size / element_edge_size)` times.
    /// Additionally, a fractional image will be inserted in the middle.
    Repeat,

    /// The border image is used `max(1,round(image_size / element_edge_size))` times.
    Round,

    /// The border image is used `floor(image_size / element_edge_size)` times.
    /// Gaps in the border are left, so this really only makes sense for dashed borders.
    Space,
}

impl std::default::Default for Scale { fn default() -> Self { Scale::Stretch } }

impl Scale {
    // ...
}
