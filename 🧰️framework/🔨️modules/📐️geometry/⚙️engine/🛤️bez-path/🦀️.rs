//! 🛤️ Bez path of the geometry primitives: the path elements, the bezier path with its builders, bounds and segment iteration and the point bounds and quadratic solver helpers. See [`super`].

use super::*;

// #region 🔖️Shapes
#[derive(Clone, Debug, PartialEq)]
pub enum PathEl {
    MoveTo(Point),
    LineTo(Point),
    QuadTo(Point, Point),
    CurveTo(Point, Point, Point),
    ClosePath,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct BezPath {
    pub(crate) elements: Vec<PathEl>,
}

impl BezPath {
    pub fn new() -> Self {
        Self { elements: Vec::new() }
    }
    pub fn move_to(&mut self, p: impl Into<(f64, f64)>) {
        let (x, y) = p.into();
        self.elements.push(PathEl::MoveTo(Point::new(x, y)));
    }
    pub fn line_to(&mut self, p: impl Into<(f64, f64)>) {
        let (x, y) = p.into();
        self.elements.push(PathEl::LineTo(Point::new(x, y)));
    }
    pub fn quad_to(&mut self, p1: Point, p2: Point) {
        self.elements.push(PathEl::QuadTo(p1, p2));
    }
    pub fn curve_to(&mut self, p1: Point, p2: Point, p3: Point) {
        self.elements.push(PathEl::CurveTo(p1, p2, p3));
    }
    pub fn close_path(&mut self) {
        self.elements.push(PathEl::ClosePath);
    }
    pub fn push(&mut self, el: PathEl) {
        self.elements.push(el);
    }
    pub fn elements(&self) -> Vec<PathEl> {
        self.elements.clone()
    }
    /// 🫴️ Borrows the original path storage without allocating an element copy.
    pub fn elements_slice(&self) -> &[PathEl] { &self.elements }
    /// 📥️ Retains the supplied original element allocation without rebuilding it.
    pub fn from_elements(elements: Vec<PathEl>) -> Self { Self { elements } }
    /// 📤️ Transfers the exact original allocation with its pointer, capacity and order intact.
    pub fn into_elements(self) -> Vec<PathEl> { self.elements }
    /// 🧮️ Tight bounding box — the union of every [`PathSeg::tight_bounds`] (analytic per-axis
    /// extrema, not the loose control-point box), matching `kurbo::Shape::bounding_box`'s own
    /// exactness for the element kinds [`PathEl`] can hold. See `bezpath_tests`'s differential
    /// proof against `kurbo::Shape::bounding_box`.
    pub fn bounding_box(&self) -> Rect {
        let segments = self.path_segments();
        if segments.is_empty() {
            return Rect::new(0.0, 0.0, 0.0, 0.0);
        }
        let mut bounds = (f64::INFINITY, f64::INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);
        for seg in &segments {
            let (x0, y0, x1, y1) = seg.tight_bounds();
            bounds = (bounds.0.min(x0), bounds.1.min(y0), bounds.2.max(x1), bounds.3.max(y1));
        }
        Rect::new(bounds.0, bounds.1, bounds.2, bounds.3)
    }
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
    /// 🧹️ Retires at most one owned path element and witnesses when only vector backing remains.
    pub fn retirement_step(&mut self) -> bool {
        self.elements.pop().is_none()
    }
    /// 📏️ Exact element-vector backing retained after all path elements are retired.
    pub fn retirement_backing_bytes(&self) -> usize {
        self.elements.capacity().saturating_mul(size_of::<PathEl>())
    }
    /// ✏️ Line/quad/cubic segments in path order — a `ClosePath` element becomes an implicit
    /// `Line` back to the most recent `MoveTo` (skipped when the current point is already there),
    /// same semantics as `kurbo::BezPath::path_segments` for the element kinds [`PathEl`] can hold.
    /// No arc-flattening tolerance parameter (unlike kurbo's own `path_segments`) because our
    /// [`PathEl`] has no `Arc` variant to flatten.
    pub fn path_segments(&self) -> Vec<PathSeg> {
        let mut segments = Vec::new();
        let mut subpath_start: Option<Point> = None;
        let mut current: Option<Point> = None;
        for el in self.elements() {
            match el {
                PathEl::MoveTo(p) => {
                    subpath_start = Some(p);
                    current = Some(p);
                }
                PathEl::LineTo(p) => {
                    if let Some(c) = current {
                        segments.push(PathSeg::Line(c, p));
                    }
                    current = Some(p);
                }
                PathEl::QuadTo(ctrl, p) => {
                    if let Some(c) = current {
                        segments.push(PathSeg::Quad(c, ctrl, p));
                    }
                    current = Some(p);
                }
                PathEl::CurveTo(c1, c2, p) => {
                    if let Some(c) = current {
                        segments.push(PathSeg::Cubic(c, c1, c2, p));
                    }
                    current = Some(p);
                }
                PathEl::ClosePath => {
                    if let (Some(start), Some(c)) = (subpath_start, current) {
                        if distance_between(c, start) > f64::EPSILON {
                            segments.push(PathSeg::Line(c, start));
                        }
                        current = Some(start);
                    }
                }
            }
        }
        segments
    }
    /// 🔧️ Maps `affine` over every point of every element — the first-party replacement for a
    /// caller round-tripping through a renderer-specific path representation just to move a path.
    pub fn apply_affine(&self, affine: Affine) -> Self {
        let mut out = Self::new();
        for el in self.elements() {
            out.push(match el {
                PathEl::MoveTo(p) => PathEl::MoveTo(affine * p),
                PathEl::LineTo(p) => PathEl::LineTo(affine * p),
                PathEl::QuadTo(c, p) => PathEl::QuadTo(affine * c, affine * p),
                PathEl::CurveTo(c1, c2, p) => PathEl::CurveTo(affine * c1, affine * c2, affine * p),
                PathEl::ClosePath => PathEl::ClosePath,
            });
        }
        out
    }
}

pub(super) fn bounds_of_points(points: impl Iterator<Item = Point>) -> (f64, f64, f64, f64) {
    let mut bounds = (f64::INFINITY, f64::INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);
    let mut any = false;
    for p in points {
        any = true;
        bounds = (bounds.0.min(p.x), bounds.1.min(p.y), bounds.2.max(p.x), bounds.3.max(p.y));
    }
    if any {
        bounds
    } else {
        (0.0, 0.0, 0.0, 0.0)
    }
}

pub(super) fn solve_quadratic(c0: f64, c1: f64, c2: f64) -> Vec<f64> {
    let scaled_constant = c0 * c2.recip();
    let scaled_linear = c1 * c2.recip();
    if !scaled_constant.is_finite() || !scaled_linear.is_finite() {
        let root = -c0 / c1;
        if root.is_finite() {
            return vec![root];
        }
        return if c0 == 0.0 && c1 == 0.0 { vec![0.0] } else { Vec::new() };
    }
    let discriminant = scaled_linear * scaled_linear - 4.0 * scaled_constant;
    if discriminant < 0.0 {
        return Vec::new();
    }
    if discriminant == 0.0 {
        return vec![-0.5 * scaled_linear];
    }
    let root1 = if discriminant.is_finite() { -0.5 * (scaled_linear + discriminant.sqrt().copysign(scaled_linear)) } else { -scaled_linear };
    let root2 = scaled_constant / root1;
    if !root2.is_finite() {
        return vec![root1];
    }
    if root2 > root1 { vec![root1, root2] } else { vec![root2, root1] }
}
// #endregion 🔖️Shapes
