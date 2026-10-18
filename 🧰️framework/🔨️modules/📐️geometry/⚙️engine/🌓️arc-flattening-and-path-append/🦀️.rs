//! 🌓️ Arc flattening and path append of the geometry primitives: the elliptical arc and circle segment generation, the ellipse sampling helpers, the tuple conversions and the shape append to a path. See [`super`].

use super::*;

// #region 🔖️Shapes
/// 🌓️ Adaptive cubic approximation used by [`Arc`] and [`RoundedRect`]. The sixth-root segment
/// count and tangent-arm construction preserve the former `kurbo::Arc::append_iter` behavior for
/// every positive tolerance while remaining first-party at runtime.
pub(super) fn elliptical_arc_segments(center: Point, radii: (f64, f64), x_rotation: f64, start_angle: f64, sweep: f64, tolerance: f64) -> Vec<PathSeg> {
    let tolerance = positive_tolerance(tolerance);
    let sign = sweep.signum();
    let scaled_error = radii.0.max(radii.1) / tolerance;
    let subdivisions_per_ellipse = (1.1163 * scaled_error).powf(1.0 / 6.0).max(3.999_999);
    let segment_count = (subdivisions_per_ellipse * sweep.abs() / std::f64::consts::TAU).ceil() as usize;
    if segment_count == 0 {
        return Vec::new();
    }
    let step = sweep / segment_count as f64;
    let arm_length = (4.0 / 3.0) * (0.25 * step).abs().tan() * sign;
    let mut angle = start_angle;
    let mut start = sample_ellipse(radii, x_rotation, angle);
    let mut segments = Vec::with_capacity(segment_count);
    for _ in 0..segment_count {
        let end_angle = angle + step;
        let start_tangent = sample_ellipse(radii, x_rotation, angle + std::f64::consts::FRAC_PI_2);
        let end = sample_ellipse(radii, x_rotation, end_angle);
        let end_tangent = sample_ellipse(radii, x_rotation, end_angle + std::f64::consts::FRAC_PI_2);
        segments.push(PathSeg::Cubic(
            point_from_offset(center, start),
            point_from_offset(center, (start.0 + arm_length * start_tangent.0, start.1 + arm_length * start_tangent.1)),
            point_from_offset(center, (end.0 - arm_length * end_tangent.0, end.1 - arm_length * end_tangent.1)),
            point_from_offset(center, end),
        ));
        angle = end_angle;
        start = end;
    }
    segments
}

/// ⭕️ Adaptive cubic approximation preserving the former `kurbo::Circle::path_elements`
/// segment-count and minimum-error four-segment arm length.
pub(super) fn circle_path_elements(center: Point, radius: f64, tolerance: f64) -> Vec<PathEl> {
    let scaled_error = radius.abs() / positive_tolerance(tolerance);
    let (segment_count, arm_length) = if scaled_error < 1.0 / 1.9608e-4 {
        (4, 0.551_915_024_494)
    } else {
        let segment_count = (1.1163 * scaled_error).powf(1.0 / 6.0).ceil() as usize;
        (segment_count, (4.0 / 3.0) * (std::f64::consts::FRAC_PI_2 / segment_count as f64).tan())
    };
    let step = std::f64::consts::TAU / segment_count as f64;
    let mut elements = Vec::with_capacity(segment_count + 2);
    elements.push(PathEl::MoveTo(Point::new(center.x + radius, center.y)));
    for index in 1..=segment_count {
        let end_angle = step * index as f64;
        let start_angle = end_angle - step;
        let (start_sin, start_cos) = start_angle.sin_cos();
        let (end_sin, end_cos) = if index == segment_count { (0.0, 1.0) } else { end_angle.sin_cos() };
        elements.push(PathEl::CurveTo(
            Point::new(center.x + radius * (start_cos - arm_length * start_sin), center.y + radius * (start_sin + arm_length * start_cos)),
            Point::new(center.x + radius * (end_cos + arm_length * end_sin), center.y + radius * (end_sin - arm_length * end_cos)),
            Point::new(center.x + radius * end_cos, center.y + radius * end_sin),
        ));
    }
    elements.push(PathEl::ClosePath);
    elements
}

fn positive_tolerance(tolerance: f64) -> f64 {
    if tolerance.is_finite() && tolerance > 0.0 { tolerance } else { f64::EPSILON }
}

fn point_from_offset(center: Point, offset: (f64, f64)) -> Point {
    Point::new(center.x + offset.0, center.y + offset.1)
}

fn sample_ellipse(radii: (f64, f64), x_rotation: f64, angle: f64) -> (f64, f64) {
    let (angle_sin, angle_cos) = angle.sin_cos();
    let x = radii.0 * angle_cos;
    let y = radii.1 * angle_sin;
    let (sin_r, cos_r) = x_rotation.sin_cos();
    (x * cos_r - y * sin_r, x * sin_r + y * cos_r)
}

/// 🌓️ The point at `angle` on the elliptical arc described by `radii`/`x_rotation`/`center` —
/// same parametrization [`Arc::eval`] uses, and the one [`elliptical_arc_segments`] samples.
pub(super) fn elliptical_point(center: Point, radii: (f64, f64), x_rotation: f64, angle: f64) -> Point {
    point_from_offset(center, sample_ellipse(radii, x_rotation, angle))
}

impl From<Point> for (f64, f64) {
    fn from(value: Point) -> Self {
        (value.x, value.y)
    }
}

impl From<Vec2> for (f64, f64) {
    fn from(value: Vec2) -> Self {
        (value.x, value.y)
    }
}

/// 📐️ Appends flattened path elements from a shape into a path buffer.
pub fn append_shape_to_path<'a>(path: &mut BezPath, shape: impl Into<ShapeRef<'a>>, tolerance: f64) {
    let shape = shape.into();
    let elements = match shape {
        ShapeRef::Rect(s) => s.path_elements(tolerance),
        ShapeRef::RoundedRect(s) => s.path_elements(tolerance),
        ShapeRef::Circle(s) => s.path_elements(tolerance),
        ShapeRef::Line(s) => s.path_elements(tolerance),
        ShapeRef::Arc(s) => s.path_elements(tolerance),
        ShapeRef::CubicBez(s) => s.path_elements(tolerance),
        ShapeRef::BezPath(s) => s.elements(),
    };
    for el in elements {
        path.push(el);
    }
}
// #endregion 🔖️Shapes
