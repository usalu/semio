//! 🧩️ Shape reference of the geometry primitives: the shape dispatch macro, the borrowed shape enum and its conversions from every first party shape. See [`super`].

use super::*;

// #region 🔖️Shapes
#[macro_export]
macro_rules! with_shape_ref {
    ($shape:expr, |$s:ident| $body:expr) => {
        match $shape {
            $crate::ShapeRef::Rect($s) => $body,
            $crate::ShapeRef::RoundedRect($s) => $body,
            $crate::ShapeRef::Circle($s) => $body,
            $crate::ShapeRef::Line($s) => $body,
            $crate::ShapeRef::Arc($s) => $body,
            $crate::ShapeRef::CubicBez($s) => $body,
            $crate::ShapeRef::BezPath($s) => $body,
        }
    };
}

#[derive(Clone, Copy, Debug)]
pub enum ShapeRef<'a> {
    Rect(&'a Rect),
    RoundedRect(&'a RoundedRect),
    Circle(&'a Circle),
    Line(&'a Line),
    Arc(&'a Arc),
    CubicBez(&'a CubicBez),
    BezPath(&'a BezPath),
}

impl<'a> From<&'a Rect> for ShapeRef<'a> {
    fn from(value: &'a Rect) -> Self {
        Self::Rect(value)
    }
}

impl<'a> From<&'a RoundedRect> for ShapeRef<'a> {
    fn from(value: &'a RoundedRect) -> Self {
        Self::RoundedRect(value)
    }
}

impl<'a> From<&'a Circle> for ShapeRef<'a> {
    fn from(value: &'a Circle) -> Self {
        Self::Circle(value)
    }
}

impl<'a> From<&'a Line> for ShapeRef<'a> {
    fn from(value: &'a Line) -> Self {
        Self::Line(value)
    }
}

impl<'a> From<&'a Arc> for ShapeRef<'a> {
    fn from(value: &'a Arc) -> Self {
        Self::Arc(value)
    }
}

impl<'a> From<&'a CubicBez> for ShapeRef<'a> {
    fn from(value: &'a CubicBez) -> Self {
        Self::CubicBez(value)
    }
}

impl<'a> From<&'a BezPath> for ShapeRef<'a> {
    fn from(value: &'a BezPath) -> Self {
        Self::BezPath(value)
    }
}
// #endregion 🔖️Shapes
