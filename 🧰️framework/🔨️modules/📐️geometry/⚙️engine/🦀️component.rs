//! 📐️ 2D geometry: single-source Point/Vec2/Affine/shape wrappers over `kurbo`, selection math, and curve/polygon algorithms.

// #region 🔖️Shapes
//! @emoji 📦️ Thin wrappers over `kurbo` primitives — the one interface boundary the rest of the codebase depends on.

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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point(pub(crate) kurbo::Point);

impl Point {
    pub const ZERO: Self = Self(kurbo::Point::ZERO);
    pub fn new(x: f64, y: f64) -> Self {
        Self(kurbo::Point::new(x, y))
    }
    pub fn distance(self, other: Self) -> f64 {
        self.0.distance(other.0)
    }
    pub fn x(&self) -> f64 {
        self.0.x
    }
    pub fn y(&self) -> f64 {
        self.0.y
    }
    /// 🔓️ Escape hatch for the renderer bridge crate to interop with `kurbo`/`vello`.
    pub fn to_kurbo(&self) -> kurbo::Point {
        self.0
    }
}

impl std::ops::Add<Vec2> for Point {
    type Output = Self;
    fn add(self, rhs: Vec2) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::Sub<Vec2> for Point {
    type Output = Self;
    fn sub(self, rhs: Vec2) -> Self {
        Self(self.0 - rhs.0)
    }
}

impl std::ops::Sub for Point {
    type Output = Vec2;
    fn sub(self, rhs: Self) -> Vec2 {
        Vec2(self.0 - rhs.0)
    }
}

impl std::ops::Deref for Point {
    type Target = kurbo::Point;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec2(pub(crate) kurbo::Vec2);

impl std::ops::Deref for Vec2 {
    type Target = kurbo::Vec2;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Vec2 {
    pub const ZERO: Self = Self(kurbo::Vec2::ZERO);
    pub fn new(x: f64, y: f64) -> Self {
        Self(kurbo::Vec2::new(x, y))
    }
    pub fn hypot(self) -> f64 {
        self.0.hypot()
    }
    pub fn dot(self, other: Self) -> f64 {
        self.0.dot(other.0)
    }
    pub fn x(&self) -> f64 {
        self.0.x
    }
    pub fn y(&self) -> f64 {
        self.0.y
    }
    /// 🔓️ Escape hatch for the renderer bridge crate to interop with `kurbo`/`vello`.
    pub fn to_kurbo(&self) -> kurbo::Vec2 {
        self.0
    }
}

impl From<(f64, f64)> for Vec2 {
    fn from((x, y): (f64, f64)) -> Self {
        Self::new(x, y)
    }
}

impl std::ops::Add for Vec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}

impl std::ops::SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl std::ops::Div<f64> for Vec2 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Self(self.0 / rhs)
    }
}

impl std::ops::Mul<f64> for Vec2 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self(self.0 * rhs)
    }
}

impl std::ops::Mul<Vec2> for f64 {
    type Output = Vec2;
    fn mul(self, rhs: Vec2) -> Vec2 {
        rhs * self
    }
}

impl std::ops::MulAssign<f64> for Vec2 {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
    }
}

impl std::ops::Neg for Vec2 {
    type Output = Self;
    fn neg(self) -> Self {
        Self(-self.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Affine(pub(crate) kurbo::Affine);

impl Affine {
    pub const IDENTITY: Self = Self(kurbo::Affine::IDENTITY);
    pub fn new(coeffs: [f64; 6]) -> Self {
        Self(kurbo::Affine::new(coeffs))
    }
    pub fn translate(self, offset: impl Into<Vec2>) -> Self {
        let offset = offset.into();
        Self(self.0 * kurbo::Affine::translate(offset.0))
    }
    pub fn scale(self, s: f64) -> Self {
        Self(self.0 * kurbo::Affine::scale(s))
    }
    pub fn rotate(self, angle: f64) -> Self {
        Self(self.0 * kurbo::Affine::rotate(angle))
    }
    /// 🔓️ Escape hatch for the renderer bridge crate to interop with `kurbo`/`vello`.
    pub fn to_kurbo(&self) -> kurbo::Affine {
        self.0
    }
    /// 🔢️ Raw `[a, b, c, d, e, f]` matrix coefficients, for callers that need direct numeric access (e.g. north/south orientation checks) without importing `kurbo` themselves.
    pub fn as_coeffs(&self) -> [f64; 6] {
        self.0.as_coeffs()
    }
}

impl std::ops::Mul for Affine {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self(self.0 * rhs.0)
    }
}

impl std::ops::Mul<Point> for Affine {
    type Output = Point;
    fn mul(self, rhs: Point) -> Point {
        Point(self.0 * rhs.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rect(pub(crate) kurbo::Rect);

impl Rect {
    pub fn new(x0: f64, y0: f64, x1: f64, y1: f64) -> Self {
        Self(kurbo::Rect::new(x0, y0, x1, y1))
    }
    pub fn from_points(p0: Point, p1: Point) -> Self {
        Self(kurbo::Rect::from_points(p0.0, p1.0))
    }
    pub fn inflate(self, dx: f64, dy: f64) -> Self {
        Self(self.0.inflate(dx, dy))
    }
    pub fn x0(self) -> f64 {
        self.0.x0
    }
    pub fn y0(self) -> f64 {
        self.0.y0
    }
    pub fn x1(self) -> f64 {
        self.0.x1
    }
    pub fn y1(self) -> f64 {
        self.0.y1
    }
    pub fn width(self) -> f64 {
        self.0.width()
    }
    pub fn height(self) -> f64 {
        self.0.height()
    }
    /// 🔓️ Escape hatch for the renderer bridge crate to interop with `kurbo`/`vello`.
    pub fn to_kurbo(&self) -> kurbo::Rect {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RoundedRectRadii(pub(crate) kurbo::RoundedRectRadii);

impl RoundedRectRadii {
    pub fn new(top_left: f64, top_right: f64, bottom_right: f64, bottom_left: f64) -> Self {
        Self(kurbo::RoundedRectRadii::new(top_left, top_right, bottom_right, bottom_left))
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RoundedRect(pub(crate) kurbo::RoundedRect);

impl RoundedRect {
    pub fn new(rect: Rect, radii: RoundedRectRadii) -> Self {
        let r = rect.0;
        Self(kurbo::RoundedRect::new(r.x0, r.y0, r.x1, r.y1, radii.0))
    }
    /// 🔓️ Escape hatch for the renderer bridge crate to interop with `kurbo`/`vello`.
    pub fn to_kurbo(&self) -> kurbo::RoundedRect {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Circle(pub(crate) kurbo::Circle);

impl Circle {
    pub fn new(center: Point, radius: f64) -> Self {
        Self(kurbo::Circle::new(center.0, radius))
    }
    /// 🔓️ Escape hatch for the renderer bridge crate to interop with `kurbo`/`vello`.
    pub fn to_kurbo(&self) -> kurbo::Circle {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Line(pub(crate) kurbo::Line);

impl Line {
    pub fn new(p0: Point, p1: Point) -> Self {
        Self(kurbo::Line::new(p0.0, p1.0))
    }
    /// 🔓️ Escape hatch for the renderer bridge crate to interop with `kurbo`/`vello`.
    pub fn to_kurbo(&self) -> kurbo::Line {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Arc(pub(crate) kurbo::Arc);

impl Arc {
    pub fn new(center: Point, radii: (f64, f64), start_angle: f64, sweep: f64, x_rotation: f64) -> Self {
        Self(kurbo::Arc::new(center.0, radii, start_angle, sweep, x_rotation))
    }
    pub fn eval(self, t: f64) -> Point {
        Point(kurbo::ParamCurve::eval(&self.0, t))
    }
    /// 🔓️ Escape hatch for the renderer bridge crate to interop with `kurbo`/`vello`.
    pub fn to_kurbo(&self) -> kurbo::Arc {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CubicBez(pub(crate) kurbo::CubicBez);

impl std::ops::Deref for CubicBez {
    type Target = kurbo::CubicBez;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl CubicBez {
    pub fn new(p0: Point, p1: Point, p2: Point, p3: Point) -> Self {
        Self(kurbo::CubicBez::new(p0.0, p1.0, p2.0, p3.0))
    }
    pub fn eval(self, t: f64) -> Point {
        Point(kurbo::ParamCurve::eval(&self.0, t))
    }
    pub fn p0(self) -> Point {
        Point(self.0.p0)
    }
    pub fn p1(self) -> Point {
        Point(self.0.p1)
    }
    pub fn p2(self) -> Point {
        Point(self.0.p2)
    }
    pub fn p3(self) -> Point {
        Point(self.0.p3)
    }
    /// 🔓️ Escape hatch for the renderer bridge crate to interop with `kurbo`/`vello`.
    pub fn to_kurbo(&self) -> kurbo::CubicBez {
        self.0
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PathEl {
    MoveTo(Point),
    LineTo(Point),
    QuadTo(Point, Point),
    CurveTo(Point, Point, Point),
    ClosePath,
}

impl From<kurbo::PathEl> for PathEl {
    fn from(value: kurbo::PathEl) -> Self {
        match value {
            kurbo::PathEl::MoveTo(p) => Self::MoveTo(Point(p)),
            kurbo::PathEl::LineTo(p) => Self::LineTo(Point(p)),
            kurbo::PathEl::QuadTo(p0, p1) => Self::QuadTo(Point(p0), Point(p1)),
            kurbo::PathEl::CurveTo(p0, p1, p2) => Self::CurveTo(Point(p0), Point(p1), Point(p2)),
            kurbo::PathEl::ClosePath => Self::ClosePath,
        }
    }
}

impl From<PathEl> for kurbo::PathEl {
    fn from(value: PathEl) -> Self {
        match value {
            PathEl::MoveTo(p) => Self::MoveTo(p.0),
            PathEl::LineTo(p) => Self::LineTo(p.0),
            PathEl::QuadTo(p0, p1) => Self::QuadTo(p0.0, p1.0),
            PathEl::CurveTo(p0, p1, p2) => Self::CurveTo(p0.0, p1.0, p2.0),
            PathEl::ClosePath => Self::ClosePath,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct BezPath(pub(crate) kurbo::BezPath);

impl BezPath {
    pub fn new() -> Self {
        Self(kurbo::BezPath::new())
    }
    pub fn move_to(&mut self, p: impl Into<(f64, f64)>) {
        let (x, y) = p.into();
        self.0.move_to((x, y));
    }
    pub fn line_to(&mut self, p: impl Into<(f64, f64)>) {
        let (x, y) = p.into();
        self.0.line_to((x, y));
    }
    pub fn quad_to(&mut self, p1: Point, p2: Point) {
        self.0.quad_to(p1.0, p2.0);
    }
    pub fn curve_to(&mut self, p1: Point, p2: Point, p3: Point) {
        self.0.curve_to(p1.0, p2.0, p3.0);
    }
    pub fn close_path(&mut self) {
        self.0.close_path();
    }
    pub fn push(&mut self, el: PathEl) {
        self.0.push(el.into());
    }
    pub fn elements(&self) -> Vec<PathEl> {
        self.0.elements().iter().copied().map(Into::into).collect()
    }
    pub fn bounding_box(&self) -> Rect {
        Rect(kurbo::Shape::bounding_box(&self.0))
    }
    /// 🔓️ Escape hatch for the renderer bridge crate to interop with `kurbo`/`vello`.
    pub fn to_kurbo(&self) -> kurbo::BezPath {
        self.0.clone()
    }
}

impl From<Point> for (f64, f64) {
    fn from(value: Point) -> Self {
        (value.0.x, value.0.y)
    }
}

impl From<Vec2> for (f64, f64) {
    fn from(value: Vec2) -> Self {
        (value.0.x, value.0.y)
    }
}

/// @emoji 📐️ Appends flattened path elements from a shape into a path buffer.
pub fn append_shape_to_path<'a>(path: &mut BezPath, shape: impl Into<ShapeRef<'a>>, tolerance: f64) {
    let shape = shape.into();
    with_shape_ref!(shape, |s| {
        for el in kurbo::Shape::path_elements(&s.0, tolerance) {
            path.push(el.into());
        }
    });
}
// #endregion 🔖️Shapes

// #region 🔖️GenericGeometry
// @emoji 📏️ Distance/normalize/clamp primitives shared by every geometry consumer.

#[inline]
pub fn clamp_f64(value: f64, min: f64, max: f64) -> f64 {
    value.max(min).min(max)
}

#[inline]
pub fn distance_between(left: Point, right: Point) -> f64 {
    (right - left).hypot()
}

#[inline]
pub fn normalize_or_zero(vector: Vec2) -> Vec2 {
    let len = vector.hypot();
    if len <= f64::EPSILON {
        return Vec2::new(0.0, 0.0);
    }
    vector / len
}

/// 📡️ Point where a ray from the origin along unit direction `(ux, uy)` exits an axis-aligned rectangle of half-extents `(hw, hh)`.
pub fn ray_from_origin_to_axis_aligned_rectangle_edge(hw: f64, hh: f64, ux: f64, uy: f64) -> Point {
    let mut t_best = f64::INFINITY;
    if ux.abs() > 1e-12 {
        let tx = ux.signum() * hw / ux;
        let y_at = uy * tx;
        if tx > 0.0 && y_at.abs() <= hh + 1e-9 {
            t_best = t_best.min(tx);
        }
    }
    if uy.abs() > 1e-12 {
        let ty = uy.signum() * hh / uy;
        let x_at = ux * ty;
        if ty > 0.0 && x_at.abs() <= hw + 1e-9 {
            t_best = t_best.min(ty);
        }
    }
    if !t_best.is_finite() || t_best <= 0.0 || t_best == f64::INFINITY {
        return Point::new(hw, 0.0);
    }
    Point::new(ux * t_best, uy * t_best)
}

pub fn distance_point_to_polyline(point: Point, path: &BezPath, _segments: usize) -> f64 {
    let mut smallest = f64::INFINITY;
    let mut start: Option<Point> = None;
    let mut previous: Option<Point> = None;
    for el in path.elements() {
        match el {
            PathEl::MoveTo(p) => {
                start = Some(p);
                previous = Some(p);
            }
            PathEl::LineTo(p) => {
                if let Some(prev) = previous {
                    smallest = smallest.min(distance_to_segment(point, prev, p));
                }
                previous = Some(p);
            }
            PathEl::ClosePath => {
                if let (Some(first), Some(prev)) = (start, previous) {
                    smallest = smallest.min(distance_to_segment(point, prev, first));
                }
            }
            _ => {}
        }
    }
    smallest
}

pub fn distance_point_to_cubic_bezier(point: Point, curve: CubicBez, segments: usize) -> f64 {
    let mut smallest = f64::INFINITY;
    let mut previous = curve.eval(0.0);
    let n = segments.max(1);
    for index in 1..=n {
        let t = index as f64 / n as f64;
        let next = curve.eval(t);
        smallest = smallest.min(distance_to_segment(point, previous, next));
        previous = next;
    }
    smallest
}

fn distance_to_segment(point: Point, start: Point, end: Point) -> f64 {
    let segment = end - start;
    let segment_len_squared = segment.dot(segment);
    if segment_len_squared <= f64::EPSILON {
        return distance_between(point, start);
    }
    let projection = clamp_f64((point - start).dot(segment) / segment_len_squared, 0.0, 1.0);
    let closest = start + segment * projection;
    distance_between(point, closest)
}
// #endregion 🔖️GenericGeometry

// #region 🔖️CurveExtensions
// @emoji 🧮️ Cubic-bezier subdivision, length, nearest-point, and segment/circle intersection.

/// 📐️ De Casteljau evaluation of a cubic bezier at parameter `t`.
pub fn cubic_point_at(c: CubicBez, t: f64) -> Point {
    c.eval(clamp_f64(t, 0.0, 1.0))
}

fn lerp_point(a: Point, b: Point, t: f64) -> Point {
    Point::new(a.x() + (b.x() - a.x()) * t, a.y() + (b.y() - a.y()) * t)
}

/// ✂️ Splits a cubic bezier at parameter `t` via De Casteljau subdivision.
pub fn cubic_split(c: CubicBez, t: f64) -> (CubicBez, CubicBez) {
    let (p0, p1, p2, p3) = (c.p0(), c.p1(), c.p2(), c.p3());
    let a = lerp_point(p0, p1, t);
    let b = lerp_point(p1, p2, t);
    let cc = lerp_point(p2, p3, t);
    let d = lerp_point(a, b, t);
    let e = lerp_point(b, cc, t);
    let f = lerp_point(d, e, t);
    (CubicBez::new(p0, a, d, f), CubicBez::new(f, e, cc, p3))
}

/// 📏️ Polyline-approximated arc length of a cubic bezier.
pub fn cubic_arc_length(c: CubicBez, segments: usize) -> f64 {
    let n = segments.max(1);
    let mut total = 0.0;
    let mut previous = c.eval(0.0);
    for i in 1..=n {
        let t = i as f64 / n as f64;
        let next = c.eval(t);
        total += distance_between(previous, next);
        previous = next;
    }
    total
}

/// 🎯️ Parameter `t` of the sampled point on a cubic bezier nearest to `point`.
pub fn cubic_nearest_t(point: Point, c: CubicBez, segments: usize) -> f64 {
    let n = segments.max(1);
    let mut best_t = 0.0;
    let mut best_dist = f64::INFINITY;
    for i in 0..=n {
        let t = i as f64 / n as f64;
        let d = distance_between(point, c.eval(t));
        if d < best_dist {
            best_dist = d;
            best_t = t;
        }
    }
    best_t
}

/// ✂️ Intersection point of two line segments, if any (parametric line-line solve).
pub fn segment_intersection(a0: Point, a1: Point, b0: Point, b1: Point) -> Option<Point> {
    let (x1, y1, x2, y2) = (a0.x(), a0.y(), a1.x(), a1.y());
    let (x3, y3, x4, y4) = (b0.x(), b0.y(), b1.x(), b1.y());
    let denom = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
    if denom.abs() <= f64::EPSILON {
        return None;
    }
    let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / denom;
    let u = ((x1 - x3) * (y1 - y2) - (y1 - y3) * (x1 - x2)) / denom;
    if !(0.0..=1.0).contains(&t) || !(0.0..=1.0).contains(&u) {
        return None;
    }
    Some(Point::new(x1 + t * (x2 - x1), y1 + t * (y2 - y1)))
}

/// ⭕️ Up to two intersection points of a circle and a line segment.
pub fn circle_line_intersections(center: Point, r: f64, p0: Point, p1: Point) -> Vec<Point> {
    let d = Vec2::new(p1.x() - p0.x(), p1.y() - p0.y());
    let f = Vec2::new(p0.x() - center.x(), p0.y() - center.y());
    let a = d.dot(d);
    if a <= f64::EPSILON {
        return Vec::new();
    }
    let b = 2.0 * f.dot(d);
    let c = f.dot(f) - r * r;
    let disc = b * b - 4.0 * a * c;
    if disc < 0.0 {
        return Vec::new();
    }
    let sq = disc.sqrt();
    let mut out = Vec::new();
    for t in [(-b - sq) / (2.0 * a), (-b + sq) / (2.0 * a)] {
        if (0.0..=1.0).contains(&t) {
            out.push(Point::new(p0.x() + t * d.x(), p0.y() + t * d.y()));
        }
    }
    out
}
// #endregion 🔖️CurveExtensions

// #region 🔖️PolygonExtensions
// @emoji 🔺️ Convex hull, area, centroid, and point-set bounding box.

/// 🐚️ Convex hull via Andrew's monotone chain (returned counter-clockwise, no duplicate closing point).
pub fn convex_hull(points: &[Point]) -> Vec<Point> {
    if points.len() < 3 {
        return points.to_vec();
    }
    let mut sorted: Vec<Point> = points.to_vec();
    sorted.sort_by(|a, b| a.x().total_cmp(&b.x()).then(a.y().total_cmp(&b.y())));
    sorted.dedup_by(|a, b| (a.x() - b.x()).abs() < f64::EPSILON && (a.y() - b.y()).abs() < f64::EPSILON);
    if sorted.len() < 3 {
        return sorted;
    }
    let cross = |o: Point, a: Point, b: Point| (a.x() - o.x()) * (b.y() - o.y()) - (a.y() - o.y()) * (b.x() - o.x());
    let mut lower: Vec<Point> = Vec::new();
    for &p in &sorted {
        while lower.len() >= 2 && cross(lower[lower.len() - 2], lower[lower.len() - 1], p) <= 0.0 {
            lower.pop();
        }
        lower.push(p);
    }
    let mut upper: Vec<Point> = Vec::new();
    for &p in sorted.iter().rev() {
        while upper.len() >= 2 && cross(upper[upper.len() - 2], upper[upper.len() - 1], p) <= 0.0 {
            upper.pop();
        }
        upper.push(p);
    }
    lower.pop();
    upper.pop();
    lower.extend(upper);
    lower
}

/// 🔺️ Signed polygon area via the shoelace formula (positive when counter-clockwise).
pub fn polygon_area(points: &[Point]) -> f64 {
    if points.len() < 3 {
        return 0.0;
    }
    let mut sum = 0.0;
    for i in 0..points.len() {
        let a = points[i];
        let b = points[(i + 1) % points.len()];
        sum += a.x() * b.y() - b.x() * a.y();
    }
    sum * 0.5
}

/// ⚖️ Area-weighted polygon centroid (falls back to the vertex average for degenerate/zero-area polygons).
pub fn polygon_centroid(points: &[Point]) -> Point {
    if points.is_empty() {
        return Point::ZERO;
    }
    let area = polygon_area(points);
    if area.abs() <= f64::EPSILON {
        let n = points.len() as f64;
        let sx: f64 = points.iter().map(Point::x).sum();
        let sy: f64 = points.iter().map(Point::y).sum();
        return Point::new(sx / n, sy / n);
    }
    let mut cx = 0.0;
    let mut cy = 0.0;
    for i in 0..points.len() {
        let a = points[i];
        let b = points[(i + 1) % points.len()];
        let cross = a.x() * b.y() - b.x() * a.y();
        cx += (a.x() + b.x()) * cross;
        cy += (a.y() + b.y()) * cross;
    }
    let factor = 1.0 / (6.0 * area);
    Point::new(cx * factor, cy * factor)
}

/// 🧮️ Axis-aligned bounding box of a point set.
pub fn bounding_box(points: &[Point]) -> Option<geom_sel::WorldBox> {
    geom_sel::world_box_from_points(points)
}
// #endregion 🔖️PolygonExtensions

// #region 🔖️GeomSel
pub mod geom_sel {
    use crate::{CubicBez, Point};

    #[derive(Clone, Copy, Debug)]
    pub struct WorldBox {
        pub min_x: f64,
        pub min_y: f64,
        pub max_x: f64,
        pub max_y: f64,
    }

    pub fn inflate_world_box(b: WorldBox, pad: f64) -> WorldBox {
        WorldBox { min_x: b.min_x - pad, min_y: b.min_y - pad, max_x: b.max_x + pad, max_y: b.max_y + pad }
    }

    pub fn world_boxes_overlap(a: WorldBox, b: WorldBox) -> bool {
        a.min_x <= b.max_x && a.max_x >= b.min_x && a.min_y <= b.max_y && a.max_y >= b.min_y
    }

    pub fn world_box_contains_point(b: WorldBox, p: Point) -> bool {
        p.x >= b.min_x && p.x <= b.max_x && p.y >= b.min_y && p.y <= b.max_y
    }

    pub fn world_box_contains_box(outer: WorldBox, inner: WorldBox) -> bool {
        inner.min_x >= outer.min_x && inner.max_x <= outer.max_x && inner.min_y >= outer.min_y && inner.max_y <= outer.max_y
    }

    fn world_box_corners(b: WorldBox) -> [Point; 4] {
        [Point::new(b.min_x, b.min_y), Point::new(b.max_x, b.min_y), Point::new(b.max_x, b.max_y), Point::new(b.min_x, b.max_y)]
    }

    pub fn world_box_from_points(points: &[Point]) -> Option<WorldBox> {
        if points.is_empty() {
            return None;
        }
        let mut min_x = f64::INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut max_y = f64::NEG_INFINITY;
        for p in points {
            min_x = min_x.min(p.x);
            min_y = min_y.min(p.y);
            max_x = max_x.max(p.x);
            max_y = max_y.max(p.y);
        }
        Some(WorldBox { min_x, min_y, max_x, max_y })
    }

    pub fn point_in_polygon(point: Point, polygon: &[Point]) -> bool {
        if polygon.len() < 3 {
            return false;
        }
        let mut inside = false;
        let mut j = polygon.len() - 1;
        for i in 0..polygon.len() {
            let a = polygon[i];
            let b = polygon[j];
            let crosses = (a.y > point.y) != (b.y > point.y);
            if crosses && point.x < (b.x - a.x) * (point.y - a.y) / (b.y - a.y) + a.x {
                inside = !inside;
            }
            j = i;
        }
        inside
    }

    fn point_on_segment(point: Point, start: Point, end: Point) -> bool {
        const EPS: f64 = 1e-9;
        point.x >= start.x.min(end.x) - EPS
            && point.x <= start.x.max(end.x) + EPS
            && point.y >= start.y.min(end.y) - EPS
            && point.y <= start.y.max(end.y) + EPS
            && ((end.x - start.x) * (point.y - start.y) - (end.y - start.y) * (point.x - start.x)).abs() <= EPS
    }

    fn orientation(a: Point, b: Point, c: Point) -> i8 {
        let v = (b.y - a.y) * (c.x - b.x) - (b.x - a.x) * (c.y - b.y);
        if v > 1e-9 {
            1
        } else if v < -1e-9 {
            -1
        } else {
            0
        }
    }

    fn segments_intersect(a0: Point, a1: Point, b0: Point, b1: Point) -> bool {
        let o1 = orientation(a0, a1, b0);
        let o2 = orientation(a0, a1, b1);
        let o3 = orientation(b0, b1, a0);
        let o4 = orientation(b0, b1, a1);
        if o1 != o2 && o3 != o4 {
            return true;
        }
        point_on_segment(b0, a0, a1) || point_on_segment(b1, a0, a1) || point_on_segment(a0, b0, b1) || point_on_segment(a1, b0, b1)
    }

    fn world_box_edges(box_: WorldBox) -> [(Point, Point); 4] {
        let [a, b, c, d] = world_box_corners(box_);
        [(a, b), (b, c), (c, d), (d, a)]
    }

    pub fn segment_intersects_world_box(start: Point, end: Point, box_: WorldBox) -> bool {
        if world_box_contains_point(box_, start) || world_box_contains_point(box_, end) {
            return true;
        }
        world_box_edges(box_).iter().any(|&(a, b)| segments_intersect(start, end, a, b))
    }

    fn polygon_segments(polygon: &[Point]) -> Vec<(Point, Point)> {
        if polygon.is_empty() {
            return Vec::new();
        }
        let mut out = Vec::with_capacity(polygon.len());
        for i in 0..polygon.len() {
            out.push((polygon[i], polygon[(i + 1) % polygon.len()]));
        }
        out
    }

    pub fn polygon_contains_world_box(polygon: &[Point], box_: WorldBox) -> bool {
        world_box_corners(box_).iter().all(|&p| point_in_polygon(p, polygon))
    }

    pub fn polygon_intersects_world_box(polygon: &[Point], box_: WorldBox) -> bool {
        if world_box_corners(box_).iter().any(|&p| point_in_polygon(p, polygon)) {
            return true;
        }
        if polygon.iter().any(|&p| world_box_contains_point(box_, p)) {
            return true;
        }
        polygon_segments(polygon).iter().any(|&(s, e)| segment_intersects_world_box(s, e, box_))
    }

    pub fn segment_intersects_polygon(start: Point, end: Point, polygon: &[Point]) -> bool {
        if point_in_polygon(start, polygon) || point_in_polygon(end, polygon) {
            return true;
        }
        polygon_segments(polygon).iter().any(|&(a, b)| segments_intersect(start, end, a, b))
    }

    pub fn cubic_bezier_axis_bounds(c: CubicBez) -> WorldBox {
        let xs = [c.p0.x, c.p1.x, c.p2.x, c.p3.x];
        let ys = [c.p0.y, c.p1.y, c.p2.y, c.p3.y];
        WorldBox {
            min_x: xs.iter().copied().fold(f64::INFINITY, f64::min),
            max_x: xs.iter().copied().fold(f64::NEG_INFINITY, f64::max),
            min_y: ys.iter().copied().fold(f64::INFINITY, f64::min),
            max_y: ys.iter().copied().fold(f64::NEG_INFINITY, f64::max),
        }
    }

    pub fn cubic_bezier_point(c: CubicBez, t: f64) -> Point {
        c.eval(t.clamp(0.0, 1.0))
    }
}
// #endregion 🔖️GeomSel

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance_between_matches_pythagoras() {
        let a = Point::new(0.0, 0.0);
        let b = Point::new(3.0, 4.0);
        assert!((distance_between(a, b) - 5.0).abs() < 1e-9);
    }

    #[test]
    fn normalize_or_zero_handles_zero_vector() {
        let v = normalize_or_zero(Vec2::new(0.0, 0.0));
        assert_eq!(v.x(), 0.0);
        assert_eq!(v.y(), 0.0);
    }

    #[test]
    fn cubic_split_endpoints_match_source_and_split_point() {
        let c = CubicBez::new(Point::new(0.0, 0.0), Point::new(0.0, 10.0), Point::new(10.0, 10.0), Point::new(10.0, 0.0));
        let mid = cubic_point_at(c, 0.5);
        let (left, right) = cubic_split(c, 0.5);
        assert!(distance_between(left.p0(), c.p0()) < 1e-9);
        assert!(distance_between(right.p3(), c.p3()) < 1e-9);
        assert!(distance_between(left.p3(), mid) < 1e-9);
        assert!(distance_between(right.p0(), mid) < 1e-9);
    }

    #[test]
    fn cubic_nearest_t_finds_endpoint_for_endpoint_query() {
        let c = CubicBez::new(Point::new(0.0, 0.0), Point::new(0.0, 10.0), Point::new(10.0, 10.0), Point::new(10.0, 0.0));
        let t = cubic_nearest_t(c.p0(), c, 64);
        assert!(t < 0.05);
    }

    #[test]
    fn segment_intersection_finds_crossing_point() {
        let hit = segment_intersection(Point::new(0.0, 0.0), Point::new(10.0, 10.0), Point::new(0.0, 10.0), Point::new(10.0, 0.0));
        let hit = hit.expect("segments cross");
        assert!(distance_between(hit, Point::new(5.0, 5.0)) < 1e-9);
    }

    #[test]
    fn segment_intersection_none_for_parallel_lines() {
        let hit = segment_intersection(Point::new(0.0, 0.0), Point::new(10.0, 0.0), Point::new(0.0, 5.0), Point::new(10.0, 5.0));
        assert!(hit.is_none());
    }

    #[test]
    fn circle_line_intersections_finds_two_points_through_center() {
        let hits = circle_line_intersections(Point::new(0.0, 0.0), 5.0, Point::new(-10.0, 0.0), Point::new(10.0, 0.0));
        assert_eq!(hits.len(), 2);
        assert!(distance_between(hits[0], Point::new(-5.0, 0.0)) < 1e-9);
        assert!(distance_between(hits[1], Point::new(5.0, 0.0)) < 1e-9);
    }

    #[test]
    fn convex_hull_of_square_with_interior_point_drops_interior() {
        let points = vec![Point::new(0.0, 0.0), Point::new(10.0, 0.0), Point::new(10.0, 10.0), Point::new(0.0, 10.0), Point::new(5.0, 5.0)];
        let hull = convex_hull(&points);
        assert_eq!(hull.len(), 4);
    }

    #[test]
    fn polygon_area_of_unit_square_is_one() {
        let square = vec![Point::new(0.0, 0.0), Point::new(1.0, 0.0), Point::new(1.0, 1.0), Point::new(0.0, 1.0)];
        assert!((polygon_area(&square).abs() - 1.0).abs() < 1e-9);
    }

    #[test]
    fn polygon_centroid_of_square_is_center() {
        let square = vec![Point::new(0.0, 0.0), Point::new(10.0, 0.0), Point::new(10.0, 10.0), Point::new(0.0, 10.0)];
        let centroid = polygon_centroid(&square);
        assert!(distance_between(centroid, Point::new(5.0, 5.0)) < 1e-9);
    }

    #[test]
    fn bounding_box_covers_all_points() {
        let points = vec![Point::new(-2.0, 3.0), Point::new(5.0, -1.0), Point::new(1.0, 8.0)];
        let bb = bounding_box(&points).expect("non-empty");
        assert_eq!(bb.min_x, -2.0);
        assert_eq!(bb.max_x, 5.0);
        assert_eq!(bb.min_y, -1.0);
        assert_eq!(bb.max_y, 8.0);
    }

    #[test]
    fn point_in_polygon_detects_interior_and_exterior() {
        let square = [Point::new(0.0, 0.0), Point::new(10.0, 0.0), Point::new(10.0, 10.0), Point::new(0.0, 10.0)];
        assert!(geom_sel::point_in_polygon(Point::new(5.0, 5.0), &square));
        assert!(!geom_sel::point_in_polygon(Point::new(15.0, 5.0), &square));
    }
}
// #endregion 🔖️Tests

// #region 🔖️Algebra
// #region 🔖️Vec3
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0, z: 0.0 };

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn from_array(v: [f32; 3]) -> Self {
        Self { x: v[0], y: v[1], z: v[2] }
    }

    pub fn to_array(self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }

    #[allow(clippy::should_implement_trait, reason = "value-semantics add/sub used pervasively as plain methods (not operator overloads) by dependent crates outside this campaign wave's scope; renaming is a breaking API change")]
    pub fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    #[allow(clippy::should_implement_trait, reason = "value-semantics add/sub used pervasively as plain methods (not operator overloads) by dependent crates outside this campaign wave's scope; renaming is a breaking API change")]
    pub fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    pub fn scale(self, s: f32) -> Self {
        Self::new(self.x * s, self.y * s, self.z * s)
    }

    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Self) -> Self {
        Self::new(self.y * other.z - self.z * other.y, self.z * other.x - self.x * other.z, self.x * other.y - self.y * other.x)
    }

    pub fn length(self) -> f32 {
        self.dot(self).sqrt()
    }

    pub fn normalize(self) -> Self {
        let len = self.length();
        if len < 1e-8 {
            return Self::ZERO;
        }
        self.scale(1.0 / len)
    }
}
// #endregion 🔖️Vec3

// #region 🔖️Mat4
#[derive(Clone, Copy, Debug)]
pub struct Mat4 {
    pub cols: [[f32; 4]; 4],
}

impl Mat4 {
    pub fn identity() -> Self {
        Self { cols: [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]] }
    }

    pub fn perspective(fov_y: f32, aspect: f32, near: f32, far: f32) -> Self {
        let f = 1.0 / (fov_y * 0.5).tan();
        let gl_z = (far + near) / (near - far);
        let gl_w = (2.0 * far * near) / (near - far);
        Self { cols: [[f / aspect, 0.0, 0.0, 0.0], [0.0, f, 0.0, 0.0], [0.0, 0.0, 0.5 * gl_z - 0.5, -1.0], [0.0, 0.0, 0.5 * gl_w, 0.0]] }
    }

    pub fn look_at(eye: Vec3, target: Vec3, up: Vec3) -> Self {
        let f = target.sub(eye).normalize();
        let s = f.cross(up).normalize();
        let u = s.cross(f);
        Self { cols: [[s.x, u.x, -f.x, 0.0], [s.y, u.y, -f.y, 0.0], [s.z, u.z, -f.z, 0.0], [-s.dot(eye), -u.dot(eye), f.dot(eye), 1.0]] }
    }

    #[allow(clippy::should_implement_trait, reason = "value-semantics mul used pervasively as a plain method (not operator overload) by dependent crates outside this campaign wave's scope; renaming is a breaking API change")]
    pub fn mul(self, other: Self) -> Self {
        let mut out = Self::identity();
        for col in 0..4 {
            for row in 0..4 {
                out.cols[col][row] = self.cols[0][row] * other.cols[col][0] + self.cols[1][row] * other.cols[col][1] + self.cols[2][row] * other.cols[col][2] + self.cols[3][row] * other.cols[col][3];
            }
        }
        out
    }

    pub fn transform_point(self, p: Vec3) -> Vec3 {
        let x = p.x * self.cols[0][0] + p.y * self.cols[1][0] + p.z * self.cols[2][0] + self.cols[3][0];
        let y = p.x * self.cols[0][1] + p.y * self.cols[1][1] + p.z * self.cols[2][1] + self.cols[3][1];
        let z = p.x * self.cols[0][2] + p.y * self.cols[1][2] + p.z * self.cols[2][2] + self.cols[3][2];
        let w = p.x * self.cols[0][3] + p.y * self.cols[1][3] + p.z * self.cols[2][3] + self.cols[3][3];
        if w.abs() < 1e-8 {
            return Vec3::new(x, y, z);
        }
        Vec3::new(x / w, y / w, z / w)
    }

    pub fn transform_direction(self, dir: Vec3) -> Vec3 {
        let x = dir.x * self.cols[0][0] + dir.y * self.cols[1][0] + dir.z * self.cols[2][0];
        let y = dir.x * self.cols[0][1] + dir.y * self.cols[1][1] + dir.z * self.cols[2][1];
        let z = dir.x * self.cols[0][2] + dir.y * self.cols[1][2] + dir.z * self.cols[2][2];
        Vec3::new(x, y, z).normalize()
    }

    /// 🧮️ Full 4x4 inverse via Gauss-Jordan elimination on an augmented `[A | I]` matrix.
    /// Indexed as `a[row][col]`; `self.cols[c][r]` is read/written as `a[r][c]` throughout.
    pub fn inverse(self) -> Self {
        let mut a = [[0.0f32; 8]; 4];
        for (row, arow) in a.iter_mut().enumerate() {
            for (col, slot) in arow.iter_mut().take(4).enumerate() {
                *slot = self.cols[col][row];
            }
            arow[4 + row] = 1.0;
        }
        for pivot in 0..4 {
            let (mut best_row, mut best_val) = (pivot, a[pivot][pivot].abs());
            for (row, arow) in a.iter().enumerate().skip(pivot + 1) {
                if arow[pivot].abs() > best_val {
                    best_row = row;
                    best_val = arow[pivot].abs();
                }
            }
            if best_val < 1e-8 {
                return Self::identity();
            }
            if best_row != pivot {
                a.swap(pivot, best_row);
            }
            let pivot_value = a[pivot][pivot];
            for slot in a[pivot].iter_mut() {
                *slot /= pivot_value;
            }
            let pivot_row = a[pivot];
            for (row, arow) in a.iter_mut().enumerate() {
                if row == pivot {
                    continue;
                }
                let factor = arow[pivot];
                if factor == 0.0 {
                    continue;
                }
                for (col, slot) in arow.iter_mut().enumerate() {
                    *slot -= factor * pivot_row[col];
                }
            }
        }
        let mut inv = [[0.0f32; 4]; 4];
        for row in 0..4 {
            for col in 0..4 {
                inv[col][row] = a[row][4 + col];
            }
        }
        Self { cols: inv }
    }

    pub fn translation(v: Vec3) -> Self {
        let mut m = Self::identity();
        m.cols[3] = [v.x, v.y, v.z, 1.0];
        m
    }

    pub fn scale_vec(v: Vec3) -> Self {
        Self { cols: [[v.x, 0.0, 0.0, 0.0], [0.0, v.y, 0.0, 0.0], [0.0, 0.0, v.z, 0.0], [0.0, 0.0, 0.0, 1.0]] }
    }

    pub fn from_quat(x: f32, y: f32, z: f32, w: f32) -> Self {
        let xx = x * x;
        let yy = y * y;
        let zz = z * z;
        let xy = x * y;
        let xz = x * z;
        let yz = y * z;
        let wx = w * x;
        let wy = w * y;
        let wz = w * z;
        Self { cols: [[1.0 - 2.0 * (yy + zz), 2.0 * (xy + wz), 2.0 * (xz - wy), 0.0], [2.0 * (xy - wz), 1.0 - 2.0 * (xx + zz), 2.0 * (yz + wx), 0.0], [2.0 * (xz + wy), 2.0 * (yz - wx), 1.0 - 2.0 * (xx + yy), 0.0], [0.0, 0.0, 0.0, 1.0]] }
    }

    pub fn to_cols_array(self) -> [f32; 16] {
        let mut out = [0.0; 16];
        for col in 0..4 {
            for row in 0..4 {
                out[col * 4 + row] = self.cols[col][row];
            }
        }
        out
    }
}
// #endregion 🔖️Mat4
// #endregion 🔖️Algebra

// #region 🔖️AlgebraTests
#[cfg(test)]
mod algebra_tests {
    use super::*;

    #[test]
    fn vec3_normalize_zero_stays_zero() {
        assert_eq!(Vec3::ZERO.normalize(), Vec3::ZERO);
    }

    #[test]
    fn vec3_cross_is_perpendicular() {
        let a = Vec3::new(1.0, 0.0, 0.0);
        let b = Vec3::new(0.0, 1.0, 0.0);
        let c = a.cross(b);
        assert!((c.dot(a)).abs() < 1e-6);
        assert!((c.dot(b)).abs() < 1e-6);
        assert!((c.z - 1.0).abs() < 1e-6);
    }

    #[test]
    fn mat4_identity_transforms_point_unchanged() {
        let p = Vec3::new(1.0, 2.0, 3.0);
        let out = Mat4::identity().transform_point(p);
        assert!((out.x - p.x).abs() < 1e-6 && (out.y - p.y).abs() < 1e-6 && (out.z - p.z).abs() < 1e-6);
    }

    #[test]
    fn mat4_inverse_round_trips_translation() {
        let m = Mat4::translation(Vec3::new(3.0, -2.0, 5.0));
        let inv = m.inverse();
        let p = Vec3::new(1.0, 1.0, 1.0);
        let round = inv.transform_point(m.transform_point(p));
        assert!((round.x - p.x).abs() < 1e-4 && (round.y - p.y).abs() < 1e-4 && (round.z - p.z).abs() < 1e-4);
    }

    #[test]
    fn vec3_array_round_trip() {
        let v = Vec3::from_array([1.0, 2.0, 3.0]);
        assert_eq!(v.to_array(), [1.0, 2.0, 3.0]);
    }

    #[test]
    fn vec3_add_sub_scale_dot_length_match_hand_computation() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(0.5, 0.5, 0.5);
        assert_eq!(a.add(b), Vec3::new(1.5, 2.5, 3.5));
        assert_eq!(a.sub(b), Vec3::new(0.5, 1.5, 2.5));
        assert_eq!(a.scale(2.0), Vec3::new(2.0, 4.0, 6.0));
        assert!((a.dot(a) - 14.0).abs() < 1e-6);
        assert!((Vec3::new(3.0, 4.0, 0.0).length() - 5.0).abs() < 1e-6);
    }

    #[test]
    fn mat4_perspective_maps_near_and_far_planes_to_depth_zero_and_one() {
        let m = Mat4::perspective(std::f32::consts::FRAC_PI_2, 1.0, 1.0, 10.0);
        let near = m.transform_point(Vec3::new(0.0, 0.0, -1.0));
        let far = m.transform_point(Vec3::new(0.0, 0.0, -10.0));
        assert!(near.z.abs() < 1e-5, "near plane depth was {}", near.z);
        assert!((far.z - 1.0).abs() < 1e-5, "far plane depth was {}", far.z);
    }

    #[test]
    fn mat4_look_at_places_target_along_negative_z() {
        let m = Mat4::look_at(Vec3::new(0.0, 0.0, 5.0), Vec3::ZERO, Vec3::new(0.0, 1.0, 0.0));
        let cam_space = m.transform_point(Vec3::ZERO);
        assert!(cam_space.x.abs() < 1e-5);
        assert!(cam_space.y.abs() < 1e-5);
        assert!((cam_space.z + 5.0).abs() < 1e-5);
    }

    #[test]
    fn mat4_mul_composes_transforms_in_matrix_order() {
        let t = Mat4::translation(Vec3::new(1.0, 0.0, 0.0));
        let s = Mat4::scale_vec(Vec3::new(2.0, 2.0, 2.0));
        let combined = t.mul(s);
        let out = combined.transform_point(Vec3::new(1.0, 1.0, 1.0));
        assert!((out.x - 3.0).abs() < 1e-6 && (out.y - 2.0).abs() < 1e-6 && (out.z - 2.0).abs() < 1e-6);
    }

    #[test]
    fn mat4_transform_direction_ignores_translation_and_normalizes() {
        let m = Mat4::translation(Vec3::new(5.0, 5.0, 5.0));
        let dir = m.transform_direction(Vec3::new(2.0, 0.0, 0.0));
        assert!((dir.x - 1.0).abs() < 1e-6 && dir.y.abs() < 1e-6 && dir.z.abs() < 1e-6);
    }

    #[test]
    fn mat4_inverse_of_singular_matrix_returns_identity() {
        let singular = Mat4 { cols: [[0.0; 4]; 4] };
        assert_eq!(singular.inverse().to_cols_array(), Mat4::identity().to_cols_array());
    }

    #[test]
    fn mat4_scale_vec_scales_each_axis() {
        let m = Mat4::scale_vec(Vec3::new(2.0, 3.0, 4.0));
        let p = m.transform_point(Vec3::new(1.0, 1.0, 1.0));
        assert!((p.x - 2.0).abs() < 1e-6 && (p.y - 3.0).abs() < 1e-6 && (p.z - 4.0).abs() < 1e-6);
    }

    #[test]
    fn mat4_from_quat_identity_is_identity() {
        let m = Mat4::from_quat(0.0, 0.0, 0.0, 1.0);
        let p = Vec3::new(1.0, 2.0, 3.0);
        let out = m.transform_point(p);
        assert!((out.x - p.x).abs() < 1e-6 && (out.y - p.y).abs() < 1e-6 && (out.z - p.z).abs() < 1e-6);
    }

    #[test]
    fn mat4_from_quat_90_degrees_about_z_rotates_x_to_y() {
        let half = std::f32::consts::FRAC_PI_4;
        let m = Mat4::from_quat(0.0, 0.0, half.sin(), half.cos());
        let out = m.transform_point(Vec3::new(1.0, 0.0, 0.0));
        assert!(out.x.abs() < 1e-5);
        assert!((out.y - 1.0).abs() < 1e-5);
        assert!(out.z.abs() < 1e-5);
    }

    #[test]
    fn mat4_to_cols_array_matches_column_major_layout() {
        let m = Mat4::translation(Vec3::new(1.0, 2.0, 3.0));
        let arr = m.to_cols_array();
        assert_eq!(arr[12], 1.0);
        assert_eq!(arr[13], 2.0);
        assert_eq!(arr[14], 3.0);
        assert_eq!(arr[15], 1.0);
    }
}
// #endregion 🔖️AlgebraTests
