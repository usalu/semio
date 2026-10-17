//! 🔧️ Kurbo boundary of the scene: the conversions from the first-party geometry to the kurbo affine, point, path element and shape, compiled only for the host and browser targets. See [`super`].

use super::*;

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub(super) fn affine_to_kurbo(value: Affine) -> kurbo::Affine {
    kurbo::Affine::new(value.as_coeffs())
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn point_to_kurbo(value: Point) -> kurbo::Point {
    kurbo::Point::new(value.x, value.y)
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn path_element_to_kurbo(value: &PathEl) -> kurbo::PathEl {
    match value {
        PathEl::MoveTo(point) => kurbo::PathEl::MoveTo(point_to_kurbo(*point)),
        PathEl::LineTo(point) => kurbo::PathEl::LineTo(point_to_kurbo(*point)),
        PathEl::QuadTo(control, point) => kurbo::PathEl::QuadTo(point_to_kurbo(*control), point_to_kurbo(*point)),
        PathEl::CurveTo(control1, control2, point) => kurbo::PathEl::CurveTo(point_to_kurbo(*control1), point_to_kurbo(*control2), point_to_kurbo(*point)),
        PathEl::ClosePath => kurbo::PathEl::ClosePath,
    }
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn shape_path_elements(shape: ShapeRef<'_>, tolerance: f64) -> Vec<PathEl> {
    match shape {
        ShapeRef::Rect(shape) => shape.path_elements(tolerance),
        ShapeRef::RoundedRect(shape) => shape.path_elements(tolerance),
        ShapeRef::Circle(shape) => shape.path_elements(tolerance),
        ShapeRef::Line(shape) => shape.path_elements(tolerance),
        ShapeRef::Arc(shape) => shape.path_elements(tolerance),
        ShapeRef::CubicBez(shape) => shape.path_elements(tolerance),
        ShapeRef::BezPath(shape) => shape.elements(),
    }
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub(super) struct KurboShapeAdapter<'a>(pub(super) ShapeRef<'a>);

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
impl kurbo::Shape for KurboShapeAdapter<'_> {
    type PathElementsIter<'iter>
        = std::vec::IntoIter<kurbo::PathEl>
    where
        Self: 'iter;

    fn path_elements(&self, tolerance: f64) -> Self::PathElementsIter<'_> {
        shape_path_elements(self.0, tolerance).iter().map(path_element_to_kurbo).collect::<Vec<_>>().into_iter()
    }
    fn area(&self) -> f64 {
        kurbo::Shape::area(&self.to_path(0.1))
    }

    fn perimeter(&self, accuracy: f64) -> f64 {
        kurbo::Shape::perimeter(&self.to_path(accuracy), accuracy)
    }

    fn winding(&self, point: kurbo::Point) -> i32 {
        kurbo::Shape::winding(&self.to_path(0.1), point)
    }

    fn bounding_box(&self) -> kurbo::Rect {
        kurbo::Shape::bounding_box(&self.to_path(0.1))
    }
}
