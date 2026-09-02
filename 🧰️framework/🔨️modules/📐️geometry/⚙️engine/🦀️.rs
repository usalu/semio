//! 📐️ 2D geometry: single-source first-party Point/Vec2/Affine/shape primitives, selection math, and curve/polygon algorithms. `kurbo` is a test-only differential oracle.

// #region 🔖️Shapes
//! @emoji 📦️ First-party 2D geometry primitives — the one interface boundary the rest of the codebase depends on. Every guest-reachable type here is a plain value on every target, including `wasm32-wasip2`.

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
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    pub fn distance(self, other: Self) -> f64 {
        ((other.x - self.x).powi(2) + (other.y - self.y).powi(2)).sqrt()
    }
    // 🚫️async: E1 pure field accessor consumed by sync closures (sort_by/dedup_by/map) — see R9
    pub fn x(&self) -> f64 {
        self.x
    }
    // 🚫️async: E1 pure field accessor consumed by sync closures (sort_by/dedup_by/map) — see R9
    pub fn y(&self) -> f64 {
        self.y
    }
}

impl std::ops::Add<Vec2> for Point {
    type Output = Self;
    fn add(self, rhs: Vec2) -> Self {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl std::ops::Sub<Vec2> for Point {
    type Output = Self;
    fn sub(self, rhs: Vec2) -> Self {
        Self { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl std::ops::Sub for Point {
    type Output = Vec2;
    fn sub(self, rhs: Self) -> Vec2 {
        Vec2 { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    pub fn hypot(self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y
    }
    // 🚫️async: E1 pure field accessor consumed by sync closures (sort_by/dedup_by/map) — see R9
    pub fn x(&self) -> f64 {
        self.x
    }
    // 🚫️async: E1 pure field accessor consumed by sync closures (sort_by/dedup_by/map) — see R9
    pub fn y(&self) -> f64 {
        self.y
    }
}

impl From<(f64, f64)> for Vec2 {
    // 🚫️async: E1 `From::from` — pure field construction, inlined sync rather than routed through the async `Vec2::new` — see R9
    fn from((x, y): (f64, f64)) -> Self {
        Self { x, y }
    }
}

impl std::ops::Add for Vec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl std::ops::AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl std::ops::SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl std::ops::Div<f64> for Vec2 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Self { x: self.x / rhs, y: self.y / rhs }
    }
}

impl std::ops::Mul<f64> for Vec2 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self { x: self.x * rhs, y: self.y * rhs }
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
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl std::ops::Neg for Vec2 {
    type Output = Self;
    fn neg(self) -> Self {
        Self { x: -self.x, y: -self.y }
    }
}

/// 🧮️ Six-coefficient 2D affine `[a, b, c, d, e, f]` — same layout as `kurbo::Affine`:
/// `x' = a*x + c*y + e`, `y' = b*x + d*y + f`. `Mul` composes "self after other" (matrix product
/// `self * other`), matching `kurbo::Affine`'s own composition order — see `affine_tests`'s
/// differential proof against the real `kurbo::Affine`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Affine {
    pub(crate) coeffs: [f64; 6],
}

impl Affine {
    pub const IDENTITY: Self = Self { coeffs: [1.0, 0.0, 0.0, 1.0, 0.0, 0.0] };
    pub fn new(coeffs: [f64; 6]) -> Self {
        Self { coeffs }
    }
    fn translation(offset: Vec2) -> Self {
        Self { coeffs: [1.0, 0.0, 0.0, 1.0, offset.x, offset.y] }
    }
    fn scaling(s: f64) -> Self {
        Self { coeffs: [s, 0.0, 0.0, s, 0.0, 0.0] }
    }
    fn rotation(angle: f64) -> Self {
        let (sin_a, cos_a) = angle.sin_cos();
        Self { coeffs: [cos_a, sin_a, -sin_a, cos_a, 0.0, 0.0] }
    }
    pub fn translate(self, offset: impl Into<Vec2>) -> Self {
        self * Self::translation(offset.into())
    }
    pub fn scale(self, s: f64) -> Self {
        self * Self::scaling(s)
    }
    pub fn rotate(self, angle: f64) -> Self {
        self * Self::rotation(angle)
    }
    /// 🔢️ Raw `[a, b, c, d, e, f]` matrix coefficients, for callers that need direct numeric access (e.g. north/south orientation checks) without importing `kurbo` themselves.
    pub fn as_coeffs(&self) -> [f64; 6] {
        self.coeffs
    }
}

impl std::ops::Mul for Affine {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let [a1, b1, c1, d1, e1, f1] = self.coeffs;
        let [a2, b2, c2, d2, e2, f2] = rhs.coeffs;
        Self {
            coeffs: [
                a1 * a2 + c1 * b2,
                b1 * a2 + d1 * b2,
                a1 * c2 + c1 * d2,
                b1 * c2 + d1 * d2,
                a1 * e2 + c1 * f2 + e1,
                b1 * e2 + d1 * f2 + f1,
            ],
        }
    }
}

impl std::ops::Mul<Point> for Affine {
    type Output = Point;
    fn mul(self, rhs: Point) -> Point {
        let [a, b, c, d, e, f] = self.coeffs;
        Point::new(a * rhs.x + c * rhs.y + e, b * rhs.x + d * rhs.y + f)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rect {
    pub(crate) x0: f64,
    pub(crate) y0: f64,
    pub(crate) x1: f64,
    pub(crate) y1: f64,
}

impl Rect {
    pub fn new(x0: f64, y0: f64, x1: f64, y1: f64) -> Self {
        Self { x0, y0, x1, y1 }
    }
    pub fn from_points(p0: Point, p1: Point) -> Self {
        Self { x0: p0.x.min(p1.x), y0: p0.y.min(p1.y), x1: p0.x.max(p1.x), y1: p0.y.max(p1.y) }
    }
    pub fn inflate(self, dx: f64, dy: f64) -> Self {
        Self { x0: self.x0 - dx, y0: self.y0 - dy, x1: self.x1 + dx, y1: self.y1 + dy }
    }
    pub fn x0(self) -> f64 {
        self.x0
    }
    pub fn y0(self) -> f64 {
        self.y0
    }
    pub fn x1(self) -> f64 {
        self.x1
    }
    pub fn y1(self) -> f64 {
        self.y1
    }
    pub fn width(self) -> f64 {
        self.x1 - self.x0
    }
    pub fn height(self) -> f64 {
        self.y1 - self.y0
    }
    /// 📦️ Exact — a rect's own outline needs no curve flattening, so `tolerance` is unused.
    pub fn path_elements(&self, _tolerance: f64) -> Vec<PathEl> {
        vec![
            PathEl::MoveTo(Point::new(self.x0, self.y0)),
            PathEl::LineTo(Point::new(self.x1, self.y0)),
            PathEl::LineTo(Point::new(self.x1, self.y1)),
            PathEl::LineTo(Point::new(self.x0, self.y1)),
            PathEl::ClosePath,
        ]
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RoundedRectRadii {
    pub(crate) top_left: f64,
    pub(crate) top_right: f64,
    pub(crate) bottom_right: f64,
    pub(crate) bottom_left: f64,
}

impl RoundedRectRadii {
    pub fn new(top_left: f64, top_right: f64, bottom_right: f64, bottom_left: f64) -> Self {
        Self { top_left, top_right, bottom_right, bottom_left }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RoundedRect {
    pub(crate) rect: Rect,
    pub(crate) radii: RoundedRectRadii,
}

impl RoundedRect {
    pub fn new(rect: Rect, radii: RoundedRectRadii) -> Self {
        let rect = Rect::new(rect.x0.min(rect.x1), rect.y0.min(rect.y1), rect.x0.max(rect.x1), rect.y0.max(rect.y1));
        let max_radius = rect.width().min(rect.height()) / 2.0;
        let normalize = |radius: f64| radius.abs().min(max_radius);
        let radii = RoundedRectRadii::new(normalize(radii.top_left), normalize(radii.top_right), normalize(radii.bottom_right), normalize(radii.bottom_left));
        Self { rect, radii }
    }
    /// 🍕️ Four straight edges plus four quarter-elliptical corners (circular, one radius per
    /// corner), preserving the `kurbo` oracle's element order and zero-radius cubic elements.
    pub fn path_elements(&self, tolerance: f64) -> Vec<PathEl> {
        let Rect { x0, y0, x1, y1 } = self.rect;
        let tl = self.radii.top_left;
        let tr = self.radii.top_right;
        let br = self.radii.bottom_right;
        let bl = self.radii.bottom_left;
        let quarter = std::f64::consts::FRAC_PI_2;
        let mut out = vec![PathEl::MoveTo(Point::new(x0, y0 + tl))];
        push_corner_arc(&mut out, Point::new(x0 + tl, y0 + tl), tl, 2.0 * quarter, quarter, tolerance);
        out.push(PathEl::LineTo(Point::new(x1 - tr, y0)));
        push_corner_arc(&mut out, Point::new(x1 - tr, y0 + tr), tr, 3.0 * quarter, quarter, tolerance);
        out.push(PathEl::LineTo(Point::new(x1, y1 - br)));
        push_corner_arc(&mut out, Point::new(x1 - br, y1 - br), br, 0.0, quarter, tolerance);
        out.push(PathEl::LineTo(Point::new(x0 + bl, y1)));
        push_corner_arc(&mut out, Point::new(x0 + bl, y1 - bl), bl, quarter, quarter, tolerance);
        out.push(PathEl::ClosePath);
        out
    }
}

fn push_corner_arc(out: &mut Vec<PathEl>, center: Point, radius: f64, start_angle: f64, sweep: f64, tolerance: f64) {
    for seg in elliptical_arc_segments(center, (radius, radius), 0.0, start_angle, sweep, tolerance) {
        out.push(seg.as_path_el());
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Circle {
    pub(crate) center: Point,
    pub(crate) radius: f64,
}

impl Circle {
    pub fn new(center: Point, radius: f64) -> Self {
        Self { center, radius }
    }
    /// ⭕️ A full-circle elliptical-arc flattening ([`elliptical_arc_segments`], radii `(r, r)`),
    /// explicitly closed.
    pub fn path_elements(&self, tolerance: f64) -> Vec<PathEl> {
        circle_path_elements(self.center, self.radius, tolerance)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Line {
    pub(crate) p0: Point,
    pub(crate) p1: Point,
}

impl Line {
    pub fn new(p0: Point, p1: Point) -> Self {
        Self { p0, p1 }
    }
    /// 📏️ Exact — a single line segment needs no flattening, so `tolerance` is unused.
    pub fn path_elements(&self, _tolerance: f64) -> Vec<PathEl> {
        vec![PathEl::MoveTo(self.p0), PathEl::LineTo(self.p1)]
    }
}

/// 🌓️ A general elliptical arc: `radii` in local (pre-rotation) space, angles in radians where
/// `0` is `+x` and increasing angle sweeps toward `+y` — same convention as `kurbo::Arc`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Arc {
    pub(crate) center: Point,
    pub(crate) radii: (f64, f64),
    pub(crate) start_angle: f64,
    pub(crate) sweep: f64,
    pub(crate) x_rotation: f64,
}

impl Arc {
    pub fn new(center: Point, radii: (f64, f64), start_angle: f64, sweep: f64, x_rotation: f64) -> Self {
        Self { center, radii, start_angle, sweep, x_rotation }
    }
    pub fn eval(self, t: f64) -> Point {
        elliptical_point(self.center, self.radii, self.x_rotation, self.start_angle + t * self.sweep)
    }
    /// 🌓️ [`elliptical_arc_segments`] over this arc's own angle range — open (no `ClosePath`),
    /// since an arc is a curve, not necessarily a closed region.
    pub fn path_elements(&self, tolerance: f64) -> Vec<PathEl> {
        let mut elements = vec![PathEl::MoveTo(self.eval(0.0))];
        elements.extend(elliptical_arc_segments(self.center, self.radii, self.x_rotation, self.start_angle, self.sweep, tolerance).into_iter().map(|segment| segment.as_path_el()));
        elements
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CubicBez {
    pub p0: Point,
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,
}

impl CubicBez {
    pub fn new(p0: Point, p1: Point, p2: Point, p3: Point) -> Self {
        Self { p0, p1, p2, p3 }
    }
    pub fn eval(self, t: f64) -> Point {
        let mt = 1.0 - t;
        Point::new(
            self.p0.x * (mt * mt * mt) + (self.p1.x * (mt * mt * 3.0) + (self.p2.x * (mt * 3.0) + self.p3.x * t) * t) * t,
            self.p0.y * (mt * mt * mt) + (self.p1.y * (mt * mt * 3.0) + (self.p2.y * (mt * 3.0) + self.p3.y * t) * t) * t,
        )
    }
    pub fn p0(self) -> Point {
        self.p0
    }
    pub fn p1(self) -> Point {
        self.p1
    }
    pub fn p2(self) -> Point {
        self.p2
    }
    pub fn p3(self) -> Point {
        self.p3
    }
    /// 📐️ Exact — [`PathEl::CurveTo`] represents a cubic natively, so `tolerance` is unused.
    pub fn path_elements(&self, _tolerance: f64) -> Vec<PathEl> {
        vec![PathEl::MoveTo(self.p0), PathEl::CurveTo(self.p1, self.p2, self.p3)]
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

fn bounds_of_points(points: impl Iterator<Item = Point>) -> (f64, f64, f64, f64) {
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

fn solve_quadratic(c0: f64, c1: f64, c2: f64) -> Vec<f64> {
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

/// ✏️ One line/quadratic/cubic path segment with a known start point — the first-party
/// replacement for `kurbo::PathSeg`, produced by [`BezPath::path_segments`]. Every method here
/// (`eval`/`subsegment`/`arclen`/`tight_bounds`) is a from-scratch implementation (De Casteljau
/// subdivision, adaptive control-polygon-length arc length, analytic per-axis extrema) rather
/// than a thin call into `kurbo` — see [`PathSeg::arclen`]'s own docstring for why, and this
/// module's `path_seg_tests`/`bezpath_tests` for the differential proofs against `kurbo`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PathSeg {
    Line(Point, Point),
    Quad(Point, Point, Point),
    Cubic(Point, Point, Point, Point),
}

impl PathSeg {
    pub fn start(&self) -> Point {
        match *self {
            Self::Line(p0, _) => p0,
            Self::Quad(p0, _, _) => p0,
            Self::Cubic(p0, _, _, _) => p0,
        }
    }

    pub fn end(&self) -> Point {
        match *self {
            Self::Line(_, p1) => p1,
            Self::Quad(_, _, p2) => p2,
            Self::Cubic(_, _, _, p3) => p3,
        }
    }

    /// 📐️ De Casteljau evaluation at `t` (clamped to `[0,1]`) — plain Bernstein-basis lerp
    /// nesting, the same construction [`cubic_split`] already uses for `CubicBez`.
    pub fn eval(&self, t: f64) -> Point {
        let t = clamp_f64(t, 0.0, 1.0);
        match *self {
            Self::Line(p0, p1) => lerp_point(p0, p1, t),
            Self::Quad(p0, p1, p2) => {
                let a = lerp_point(p0, p1, t);
                let b = lerp_point(p1, p2, t);
                lerp_point(a, b, t)
            }
            Self::Cubic(p0, p1, p2, p3) => {
                let a = lerp_point(p0, p1, t);
                let b = lerp_point(p1, p2, t);
                let c = lerp_point(p2, p3, t);
                let d = lerp_point(a, b, t);
                let e = lerp_point(b, c, t);
                lerp_point(d, e, t)
            }
        }
    }

    fn control_polygon_length(&self) -> f64 {
        match *self {
            Self::Line(p0, p1) => distance_between(p0, p1),
            Self::Quad(p0, p1, p2) => distance_between(p0, p1) + distance_between(p1, p2),
            Self::Cubic(p0, p1, p2, p3) => distance_between(p0, p1) + distance_between(p1, p2) + distance_between(p2, p3),
        }
    }

    /// ✂️ Splits at `t` (De Casteljau), returning `(self[0..t], self[t..1])`.
    pub fn subdivide_at(&self, t: f64) -> (Self, Self) {
        let t = clamp_f64(t, 0.0, 1.0);
        match *self {
            Self::Line(p0, p1) => {
                let m = lerp_point(p0, p1, t);
                (Self::Line(p0, m), Self::Line(m, p1))
            }
            Self::Quad(p0, p1, p2) => {
                let a = lerp_point(p0, p1, t);
                let b = lerp_point(p1, p2, t);
                let m = lerp_point(a, b, t);
                (Self::Quad(p0, a, m), Self::Quad(m, b, p2))
            }
            Self::Cubic(p0, p1, p2, p3) => {
                let a = lerp_point(p0, p1, t);
                let b = lerp_point(p1, p2, t);
                let c = lerp_point(p2, p3, t);
                let d = lerp_point(a, b, t);
                let e = lerp_point(b, c, t);
                let m = lerp_point(d, e, t);
                (Self::Cubic(p0, a, d, m), Self::Cubic(m, e, c, p3))
            }
        }
    }

    /// ✂️ The portion of this segment between parameters `t0` and `t1` (each clamped to
    /// `[0,1]`), via two De Casteljau splits — matches `kurbo::PathSeg::subsegment`'s semantics
    /// for a `Range<f64>`.
    pub fn subsegment(&self, t0: f64, t1: f64) -> Self {
        let t0 = clamp_f64(t0, 0.0, 1.0);
        let t1 = clamp_f64(t1, 0.0, 1.0);
        if t0 <= 0.0 && t1 >= 1.0 {
            return *self;
        }
        let (_, from_t0) = self.subdivide_at(t0);
        if (1.0 - t0).abs() < 1e-12 {
            return from_t0;
        }
        let local_t1 = clamp_f64((t1 - t0) / (1.0 - t0), 0.0, 1.0);
        let (left, _) = from_t0.subdivide_at(local_t1);
        left
    }

    /// 📎️ The [`PathEl`] that appends this segment onto a path whose current point is already
    /// [`Self::start`] — matches `kurbo::PathSeg::as_path_el`.
    pub fn as_path_el(&self) -> PathEl {
        match *self {
            Self::Line(_, p1) => PathEl::LineTo(p1),
            Self::Quad(_, ctrl, p2) => PathEl::QuadTo(ctrl, p2),
            Self::Cubic(_, c1, c2, p3) => PathEl::CurveTo(c1, c2, p3),
        }
    }

    /// 📏️ Arc length accurate to within roughly `accuracy` (same parameter meaning as
    /// `kurbo::ParamCurveArclen::arclen`'s `accuracy`): adaptive recursive subdivision by
    /// control-polygon length, not a call into `kurbo`. A line's length is exact (`Self::Line`
    /// short-circuits below); a quad/cubic recurses via [`Self::subdivide_at`] at the segment's
    /// midpoint until the gap between the chord length and the control-polygon length (an upper
    /// bound on the true arc length — the polygon can only get longer than the curve it bounds by
    /// cutting corners) is within `accuracy` at that level, returning the average of the two as
    /// the estimate for that piece; `accuracy` is halved per recursion level so the sum of every
    /// leaf's error stays bounded by roughly the original `accuracy` (a geometric series, not a
    /// per-leaf budget that grows with subdivision depth). See `path_seg_tests` for the
    /// differential proof against `kurbo::ParamCurveArclen` on the same control points.
    pub fn arclen(&self, accuracy: f64) -> f64 {
        if let Self::Line(p0, p1) = *self {
            return distance_between(p0, p1);
        }
        Self::arclen_adaptive(*self, accuracy.max(1e-12), 0)
    }

    fn arclen_adaptive(seg: Self, tolerance: f64, depth: u32) -> f64 {
        let chord = distance_between(seg.start(), seg.end());
        let control_net = seg.control_polygon_length();
        if control_net - chord <= tolerance || depth >= 32 {
            return (chord + control_net) / 2.0;
        }
        let (left, right) = seg.subdivide_at(0.5);
        Self::arclen_adaptive(left, tolerance / 2.0, depth + 1) + Self::arclen_adaptive(right, tolerance / 2.0, depth + 1)
    }

    /// 🧮️ Tight per-axis bounding box `(min_x, min_y, max_x, max_y)`: for a line, just the two
    /// endpoints; for a quad/cubic, the endpoints plus every analytic derivative-root ("extrema")
    /// candidate `t` in `(0,1)` for each axis independently (a quad's derivative is linear per
    /// axis — one root; a cubic's is quadratic per axis — up to two roots), each evaluated via
    /// [`Self::eval`]. This is the standard exact-bezier-bbox algorithm (not the looser
    /// control-point box) — see `bezpath_tests`'s differential proof against
    /// `kurbo::Shape::bounding_box`.
    pub fn tight_bounds(&self) -> (f64, f64, f64, f64) {
        let mut candidates = vec![0.0, 1.0];
        match *self {
            Self::Line(..) => {}
            Self::Quad(p0, p1, p2) => {
                for (v0, v1, v2) in [(p0.x, p1.x, p2.x), (p0.y, p1.y, p2.y)] {
                    let denom = v0 - 2.0 * v1 + v2;
                    if denom != 0.0 {
                        let t = (v0 - v1) / denom;
                        if t > 0.0 && t < 1.0 {
                            candidates.push(t);
                        }
                    }
                }
            }
            Self::Cubic(p0, p1, p2, p3) => {
                for (v0, v1, v2, v3) in [(p0.x, p1.x, p2.x, p3.x), (p0.y, p1.y, p2.y, p3.y)] {
                    let d0 = v1 - v0;
                    let d1 = v2 - v1;
                    let d2 = v3 - v2;
                    let a = d0 - 2.0 * d1 + d2;
                    let b = 2.0 * (d1 - d0);
                    let c = d0;
                    for t in solve_quadratic(c, b, a) {
                        if t > 0.0 && t < 1.0 {
                            candidates.push(t);
                        }
                    }
                }
            }
        }
        bounds_of_points(candidates.into_iter().map(|t| self.eval(t)))
    }
}

/// 🌓️ Adaptive cubic approximation used by [`Arc`] and [`RoundedRect`]. The sixth-root segment
/// count and tangent-arm construction preserve the former `kurbo::Arc::append_iter` behavior for
/// every positive tolerance while remaining first-party at runtime.
fn elliptical_arc_segments(center: Point, radii: (f64, f64), x_rotation: f64, start_angle: f64, sweep: f64, tolerance: f64) -> Vec<PathSeg> {
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
fn circle_path_elements(center: Point, radius: f64, tolerance: f64) -> Vec<PathEl> {
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
fn elliptical_point(center: Point, radii: (f64, f64), x_rotation: f64, angle: f64) -> Point {
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

/// @emoji 📐️ Appends flattened path elements from a shape into a path buffer.
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
        let edges = world_box_edges(box_);
        for (a, b) in edges {
            if segments_intersect(start, end, a, b) {
                return true;
            }
        }
        false
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
        let corners = world_box_corners(box_);
        for p in corners {
            if !point_in_polygon(p, polygon) {
                return false;
            }
        }
        true
    }

    pub fn polygon_intersects_world_box(polygon: &[Point], box_: WorldBox) -> bool {
        let corners = world_box_corners(box_);
        for p in corners {
            if point_in_polygon(p, polygon) {
                return true;
            }
        }
        for &p in polygon {
            if world_box_contains_point(box_, p) {
                return true;
            }
        }
        let segments = polygon_segments(polygon);
        for (s, e) in segments {
            if segment_intersects_world_box(s, e, box_) {
                return true;
            }
        }
        false
    }

    pub fn segment_intersects_polygon(start: Point, end: Point, polygon: &[Point]) -> bool {
        if point_in_polygon(start, polygon) || point_in_polygon(end, polygon) {
            return true;
        }
        let segments = polygon_segments(polygon);
        for (a, b) in segments {
            if segments_intersect(start, end, a, b) {
                return true;
            }
        }
        false
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

// #region 🔖️PathSegTests
#[cfg(test)]
mod path_seg_tests {
    use super::*;

    /// 🎲️ Constant-seeded LCG (never the `rand` crate) — deterministic pseudo-random control
    /// points for the differential oracle test below, same idiom the `ticket`'s dev-dependency
    /// rule asks for.
    struct Lcg(u64);

    impl Lcg {
        fn next_f64(&mut self, lo: f64, hi: f64) -> f64 {
            self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            let bits = (self.0 >> 11) as f64 / (1u64 << 53) as f64;
            lo + bits * (hi - lo)
        }
    }

    fn oracle_point(point: Point) -> kurbo::Point {
        kurbo::Point::new(point.x, point.y)
    }

    /// 🧪️ Language-agnostic fixture table: (segment, t, expected point) for [`PathSeg::eval`],
    /// hand-computed from the Bernstein-basis formulas so any language's own De Casteljau
    /// implementation can be checked against the same table.
    #[test]
    fn eval_matches_hand_computed_fixtures() {
        let fixtures: &[(PathSeg, f64, Point)] = &[
            (PathSeg::Line(Point::new(0.0, 0.0), Point::new(10.0, 0.0)), 0.5, Point::new(5.0, 0.0)),
            (PathSeg::Line(Point::new(0.0, 0.0), Point::new(10.0, 20.0)), 0.25, Point::new(2.5, 5.0)),
            (PathSeg::Quad(Point::new(0.0, 0.0), Point::new(5.0, 10.0), Point::new(10.0, 0.0)), 0.5, Point::new(5.0, 5.0)),
            (PathSeg::Cubic(Point::new(0.0, 0.0), Point::new(0.0, 10.0), Point::new(10.0, 10.0), Point::new(10.0, 0.0)), 0.5, Point::new(5.0, 7.5)),
        ];
        for (seg, t, expected) in fixtures {
            let actual = seg.eval(*t);
            assert!(distance_between(actual, *expected) < 1e-9, "eval({t}) on {seg:?} expected {expected:?}, got {actual:?}");
        }
    }

    #[test]
    fn start_and_end_match_endpoints_for_every_variant() {
        let line = PathSeg::Line(Point::new(0.0, 0.0), Point::new(1.0, 1.0));
        let quad = PathSeg::Quad(Point::new(0.0, 0.0), Point::new(1.0, 2.0), Point::new(2.0, 0.0));
        let cubic = PathSeg::Cubic(Point::new(0.0, 0.0), Point::new(1.0, 1.0), Point::new(2.0, 1.0), Point::new(3.0, 0.0));
        for seg in [line, quad, cubic] {
            assert_eq!(seg.eval(0.0), seg.start());
            assert!(distance_between(seg.eval(1.0), seg.end()) < 1e-9);
        }
    }

    #[test]
    fn subdivide_at_endpoints_join_at_the_split_point_and_reproduce_original_endpoints() {
        let cubic = PathSeg::Cubic(Point::new(0.0, 0.0), Point::new(0.0, 10.0), Point::new(10.0, 10.0), Point::new(10.0, 0.0));
        for t in [0.1, 0.3, 0.5, 0.7, 0.9] {
            let (left, right) = cubic.subdivide_at(t);
            assert!(distance_between(left.start(), cubic.start()) < 1e-9);
            assert!(distance_between(right.end(), cubic.end()) < 1e-9);
            assert!(distance_between(left.end(), right.start()) < 1e-9, "split halves must join exactly at t={t}");
            assert!(distance_between(left.end(), cubic.eval(t)) < 1e-9, "split point must equal eval(t) at t={t}");
        }
    }

    #[test]
    fn subsegment_full_range_is_identity_and_half_ranges_sum_to_whole_arclen() {
        let cubic = PathSeg::Cubic(Point::new(0.0, 0.0), Point::new(0.0, 10.0), Point::new(10.0, 10.0), Point::new(10.0, 0.0));
        let whole = cubic.subsegment(0.0, 1.0);
        assert!(distance_between(whole.start(), cubic.start()) < 1e-9);
        assert!(distance_between(whole.end(), cubic.end()) < 1e-9);
        let first_half = cubic.subsegment(0.0, 0.5);
        let second_half = cubic.subsegment(0.5, 1.0);
        let whole_len = cubic.arclen(1e-6);
        let split_len = first_half.arclen(1e-6) + second_half.arclen(1e-6);
        assert!((whole_len - split_len).abs() < 1e-4, "arclen of the two halves ({split_len}) must sum to the whole ({whole_len})");
    }

    #[test]
    fn line_arclen_is_exact() {
        let line = PathSeg::Line(Point::new(0.0, 0.0), Point::new(3.0, 4.0));
        assert!((line.arclen(1e-9) - 5.0).abs() < 1e-12);
    }

    /// 🧪️ A cubic Bezier built from the standard `kappa ≈ 0.5522847498` construction approximates
    /// a quarter circle of radius `r` to within a few parts in 10,000 — a well-known analytic
    /// cross-check independent of `kurbo`.
    #[test]
    fn cubic_quarter_circle_approximation_matches_analytic_arc_length() {
        let r = 10.0;
        let k = 0.5522847498307936 * r;
        let seg = PathSeg::Cubic(Point::new(r, 0.0), Point::new(r, k), Point::new(k, r), Point::new(0.0, r));
        let analytic = std::f64::consts::FRAC_PI_2 * r;
        let estimated = seg.arclen(1e-6);
        assert!((estimated - analytic).abs() < analytic * 0.001, "quarter-circle cubic approximation arclen {estimated} should be within 0.1% of the analytic {analytic}");
    }

    /// 🔬️ DIFFERENTIAL ORACLE: our from-scratch [`PathSeg::arclen`] (recursive control-polygon
    /// subdivision) vs `kurbo::ParamCurveArclen::arclen` (the crate's own adaptive Gauss-Legendre
    /// quadrature) on 32 deterministic pseudo-random cubic/quad curves. Both estimators converge
    /// to the true arc length as `accuracy` shrinks but via unrelated numerical methods, so
    /// agreement is real evidence of correctness, not shared-bug coincidence. Tolerance is
    /// relative (`0.5%` of the oracle's own length) rather than absolute, since curve sizes here
    /// range from ~1 to ~200 units — an absolute epsilon would be either too loose on small
    /// curves or too tight on large ones.
    #[test]
    fn arclen_agrees_with_kurbo_param_curve_arclen_across_random_curves() {
        let mut rng = Lcg(0x9E3779B97F4A7C15);
        let accuracy = 1e-4;
        for i in 0..32 {
            let p0 = Point::new(rng.next_f64(-50.0, 50.0), rng.next_f64(-50.0, 50.0));
            let p1 = Point::new(rng.next_f64(-50.0, 50.0), rng.next_f64(-50.0, 50.0));
            let p2 = Point::new(rng.next_f64(-50.0, 50.0), rng.next_f64(-50.0, 50.0));
            if i % 2 == 0 {
                let seg = PathSeg::Quad(p0, p1, p2);
                let oracle = kurbo::ParamCurveArclen::arclen(&kurbo::QuadBez::new(oracle_point(p0), oracle_point(p1), oracle_point(p2)), accuracy);
                let ours = seg.arclen(accuracy);
                assert!((ours - oracle).abs() <= oracle.max(1.0) * 0.005, "quad #{i}: ours={ours} oracle={oracle}");
            } else {
                let p3 = Point::new(rng.next_f64(-50.0, 50.0), rng.next_f64(-50.0, 50.0));
                let seg = PathSeg::Cubic(p0, p1, p2, p3);
                let oracle = kurbo::ParamCurveArclen::arclen(&kurbo::CubicBez::new(oracle_point(p0), oracle_point(p1), oracle_point(p2), oracle_point(p3)), accuracy);
                let ours = seg.arclen(accuracy);
                assert!((ours - oracle).abs() <= oracle.max(1.0) * 0.005, "cubic #{i}: ours={ours} oracle={oracle}");
            }
        }
    }

    #[test]
    fn path_segments_walks_a_multi_subpath_document_and_closes_each_subpath() {
        let mut path = BezPath::new();
        path.move_to((0.0, 0.0));
        path.line_to((10.0, 0.0));
        path.line_to((10.0, 10.0));
        path.close_path();
        path.move_to((20.0, 20.0));
        path.curve_to(Point::new(20.0, 30.0), Point::new(30.0, 30.0), Point::new(30.0, 20.0));
        let segments = path.path_segments();
        assert_eq!(segments.len(), 4, "2 lines + 1 implicit close-line + 1 cubic");
        assert!(matches!(segments[0], PathSeg::Line(..)));
        assert!(matches!(segments[1], PathSeg::Line(..)));
        assert!(matches!(segments[2], PathSeg::Line(..)), "ClosePath must become an implicit closing Line");
        assert!(matches!(segments[3], PathSeg::Cubic(..)));
        assert!(distance_between(segments[2].end(), Point::new(0.0, 0.0)) < 1e-9, "the implicit close must land back on the subpath's MoveTo");
    }

    #[test]
    fn path_segments_skips_a_redundant_close_when_already_at_the_start_point() {
        let mut path = BezPath::new();
        path.move_to((0.0, 0.0));
        path.line_to((10.0, 0.0));
        path.line_to((0.0, 0.0));
        path.close_path();
        let segments = path.path_segments();
        assert_eq!(segments.len(), 2, "a ClosePath that is already back at the start must not add a zero-length segment");
    }

    #[test]
    fn apply_affine_translates_every_point_including_control_points() {
        let mut path = BezPath::new();
        path.move_to((0.0, 0.0));
        path.curve_to(Point::new(0.0, 10.0), Point::new(10.0, 10.0), Point::new(10.0, 0.0));
        let moved = path.apply_affine(Affine::IDENTITY.translate(Vec2::new(5.0, 5.0)));
        let original_segments = path.path_segments();
        let moved_segments = moved.path_segments();
        let PathSeg::Cubic(_, oc1, oc2, oend) = original_segments[0] else { panic!("expected a cubic segment") };
        let PathSeg::Cubic(_, mc1, mc2, mend) = moved_segments[0] else { panic!("expected a cubic segment") };
        assert!(distance_between(mc1, oc1 + Vec2::new(5.0, 5.0)) < 1e-9, "control point 1 must be translated");
        assert!(distance_between(mc2, oc2 + Vec2::new(5.0, 5.0)) < 1e-9, "control point 2 must be translated");
        assert!(distance_between(mend, oend + Vec2::new(5.0, 5.0)) < 1e-9, "endpoint must be translated");
    }

    #[test]
    fn as_path_el_round_trips_through_a_fresh_bezpath() {
        let mut path = BezPath::new();
        path.move_to((0.0, 0.0));
        path.line_to((10.0, 0.0));
        path.curve_to(Point::new(10.0, 10.0), Point::new(20.0, 10.0), Point::new(20.0, 0.0));
        let segments = path.path_segments();
        let mut rebuilt = BezPath::new();
        rebuilt.move_to(segments[0].start());
        for seg in &segments {
            rebuilt.push(seg.as_path_el());
        }
        assert_eq!(rebuilt.elements(), path.elements());
    }
}
// #endregion 🔖️PathSegTests

// #region 🔖️FirstPartyShapeTests
#[cfg(test)]
mod first_party_shape_tests {
    use super::*;

    /// 🎲️ Constant-seeded LCG (never the `rand` crate) — same idiom `path_seg_tests` uses.
    struct Lcg(u64);

    impl Lcg {
        fn next_f64(&mut self, lo: f64, hi: f64) -> f64 {
            self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            let bits = (self.0 >> 11) as f64 / (1u64 << 53) as f64;
            lo + bits * (hi - lo)
        }
    }

    fn oracle_point(point: Point) -> kurbo::Point {
        kurbo::Point::new(point.x, point.y)
    }

    fn assert_points_close(ours: Point, oracle: kurbo::Point, epsilon: f64, context: &str) {
        let d = ((ours.x - oracle.x).powi(2) + (ours.y - oracle.y).powi(2)).sqrt();
        assert!(d < epsilon, "{context}: ours={ours:?} oracle={oracle:?} dist={d}");
    }

    fn assert_path_elements_close(ours: &[PathEl], oracle: &[kurbo::PathEl], epsilon: f64, context: &str) {
        assert_eq!(ours.len(), oracle.len(), "{context}: ours={ours:?} oracle={oracle:?}");
        for (index, (ours, oracle)) in ours.iter().zip(oracle).enumerate() {
            let element_context = format!("{context} element {index}");
            match (ours, oracle) {
                (PathEl::MoveTo(ours), kurbo::PathEl::MoveTo(oracle)) | (PathEl::LineTo(ours), kurbo::PathEl::LineTo(oracle)) => {
                    assert_points_close(*ours, *oracle, epsilon, &element_context);
                }
                (PathEl::QuadTo(ours_control, ours_point), kurbo::PathEl::QuadTo(oracle_control, oracle_point)) => {
                    assert_points_close(*ours_control, *oracle_control, epsilon, &element_context);
                    assert_points_close(*ours_point, *oracle_point, epsilon, &element_context);
                }
                (PathEl::CurveTo(ours_control1, ours_control2, ours_point), kurbo::PathEl::CurveTo(oracle_control1, oracle_control2, oracle_point)) => {
                    assert_points_close(*ours_control1, *oracle_control1, epsilon, &element_context);
                    assert_points_close(*ours_control2, *oracle_control2, epsilon, &element_context);
                    assert_points_close(*ours_point, *oracle_point, epsilon, &element_context);
                }
                (PathEl::ClosePath, kurbo::PathEl::ClosePath) => {}
                _ => panic!("{element_context}: ours={ours:?} oracle={oracle:?}"),
            }
        }
    }

    #[test]
    fn point_vec2_arithmetic_matches_hand_computation() {
        let p = Point::new(1.0, 2.0) + Vec2::new(3.0, 4.0);
        assert_eq!(p, Point::new(4.0, 6.0));
        let d = Point::new(4.0, 6.0) - Point::new(1.0, 2.0);
        assert_eq!(d, Vec2::new(3.0, 4.0));
        assert!((Vec2::new(3.0, 4.0).hypot() - 5.0).abs() < 1e-12);
    }

    #[test]
    fn point_vec2_and_rect_arithmetic_agree_with_kurbo() {
        let mut rng = Lcg(0xA0761D6478BD642F);
        for case in 0..64 {
            let p0 = Point::new(rng.next_f64(-100.0, 100.0), rng.next_f64(-100.0, 100.0));
            let p1 = Point::new(rng.next_f64(-100.0, 100.0), rng.next_f64(-100.0, 100.0));
            let vector = Vec2::new(rng.next_f64(-20.0, 20.0), rng.next_f64(-20.0, 20.0));
            let oracle_p0 = oracle_point(p0);
            let oracle_p1 = oracle_point(p1);
            let oracle_vector = kurbo::Vec2::new(vector.x, vector.y);
            assert!((p0.distance(p1) - oracle_p0.distance(oracle_p1)).abs() < 1e-12, "case {case} point distance");
            assert!((vector.hypot() - oracle_vector.hypot()).abs() < 1e-12, "case {case} vector hypot");
            assert!((vector.dot(p1 - p0) - oracle_vector.dot(oracle_p1 - oracle_p0)).abs() < 1e-9, "case {case} vector dot");
            assert_points_close(p0 + vector, oracle_p0 + oracle_vector, 1e-12, "point plus vector");
            let ours = Rect::from_points(p0, p1).inflate(2.5, 1.25);
            let oracle = kurbo::Rect::from_points(oracle_p0, oracle_p1).inflate(2.5, 1.25);
            assert!((ours.x0() - oracle.x0).abs() < 1e-12 && (ours.y0() - oracle.y0).abs() < 1e-12 && (ours.x1() - oracle.x1).abs() < 1e-12 && (ours.y1() - oracle.y1).abs() < 1e-12, "case {case} rect: ours={ours:?} oracle={oracle:?}");
        }
    }

    /// 🔬️ DIFFERENTIAL ORACLE: `Affine::translate`/`rotate`/`scale`/composition/apply-to-point vs
    /// the real `kurbo::Affine`, built independently from the same deterministic pseudo-random
    /// parameters (not by converting one representation into the other), across 64 cases.
    #[test]
    fn affine_translate_rotate_scale_composition_and_apply_agree_with_kurbo() {
        let mut rng = Lcg(0xD1B54A32D192ED03);
        for i in 0..64 {
            let tx = rng.next_f64(-50.0, 50.0);
            let ty = rng.next_f64(-50.0, 50.0);
            let angle = rng.next_f64(-std::f64::consts::PI, std::f64::consts::PI);
            let scale = rng.next_f64(0.1, 5.0);
            let ours = Affine::IDENTITY.translate(Vec2::new(tx, ty)).rotate(angle).scale(scale);
            let oracle = kurbo::Affine::IDENTITY * kurbo::Affine::translate(kurbo::Vec2::new(tx, ty)) * kurbo::Affine::rotate(angle) * kurbo::Affine::scale(scale);
            let our_coeffs = ours.as_coeffs();
            let oracle_coeffs = oracle.as_coeffs();
            for k in 0..6 {
                assert!((our_coeffs[k] - oracle_coeffs[k]).abs() < 1e-9, "case {i} coeff {k}: ours={our_coeffs:?} oracle={oracle_coeffs:?}");
            }
            let p = Point::new(rng.next_f64(-100.0, 100.0), rng.next_f64(-100.0, 100.0));
            assert_points_close(ours * p, oracle * oracle_point(p), 1e-6, "affine apply-to-point");
        }
    }

    /// 🔬️ DIFFERENTIAL ORACLE: `BezPath::bounding_box`'s tight per-segment extrema box vs
    /// `kurbo::Shape::bounding_box` on the same mixed line/quad/cubic path, across 32 randomly
    /// generated multi-segment paths.
    #[test]
    fn bezpath_bounding_box_agrees_with_kurbo_shape_bounding_box_on_curved_paths() {
        let mut rng = Lcg(0x2545F4914F6CDD1D);
        for case in 0..32 {
            let mut ours = BezPath::new();
            let mut oracle = kurbo::BezPath::new();
            let start = (rng.next_f64(-50.0, 50.0), rng.next_f64(-50.0, 50.0));
            ours.move_to(start);
            oracle.move_to(start);
            let steps = 3 + (case % 4);
            for _ in 0..steps {
                match (rng.next_f64(0.0, 3.0) as u32).min(2) {
                    0 => {
                        let p = (rng.next_f64(-50.0, 50.0), rng.next_f64(-50.0, 50.0));
                        ours.line_to(p);
                        oracle.line_to(p);
                    }
                    1 => {
                        let c = Point::new(rng.next_f64(-80.0, 80.0), rng.next_f64(-80.0, 80.0));
                        let p = Point::new(rng.next_f64(-80.0, 80.0), rng.next_f64(-80.0, 80.0));
                        ours.quad_to(c, p);
                        oracle.quad_to(oracle_point(c), oracle_point(p));
                    }
                    _ => {
                        let c1 = Point::new(rng.next_f64(-80.0, 80.0), rng.next_f64(-80.0, 80.0));
                        let c2 = Point::new(rng.next_f64(-80.0, 80.0), rng.next_f64(-80.0, 80.0));
                        let p = Point::new(rng.next_f64(-80.0, 80.0), rng.next_f64(-80.0, 80.0));
                        ours.curve_to(c1, c2, p);
                        oracle.curve_to(oracle_point(c1), oracle_point(c2), oracle_point(p));
                    }
                }
            }
            let ours_bb = ours.bounding_box();
            let oracle_bb = kurbo::Shape::bounding_box(&oracle);
            assert!((ours_bb.x0() - oracle_bb.x0).abs() < 1e-6, "case {case} x0: ours={ours_bb:?} oracle={oracle_bb:?}");
            assert!((ours_bb.y0() - oracle_bb.y0).abs() < 1e-6, "case {case} y0: ours={ours_bb:?} oracle={oracle_bb:?}");
            assert!((ours_bb.x1() - oracle_bb.x1).abs() < 1e-6, "case {case} x1: ours={ours_bb:?} oracle={oracle_bb:?}");
            assert!((ours_bb.y1() - oracle_bb.y1).abs() < 1e-6, "case {case} y1: ours={ours_bb:?} oracle={oracle_bb:?}");
        }
    }

    #[test]
    fn rect_line_cubic_path_elements_are_exact() {
        let rect = Rect::new(-5.0, -5.0, 15.0, 25.0);
        let els = rect.path_elements(0.1);
        assert_eq!(els.len(), 5);
        assert!(matches!(els[0], PathEl::MoveTo(_)));
        assert!(matches!(els[4], PathEl::ClosePath));

        let line = Line::new(Point::new(1.0, 2.0), Point::new(3.0, 4.0));
        assert_eq!(line.path_elements(0.1), vec![PathEl::MoveTo(Point::new(1.0, 2.0)), PathEl::LineTo(Point::new(3.0, 4.0))]);

        let cubic = CubicBez::new(Point::new(0.0, 0.0), Point::new(0.0, 10.0), Point::new(10.0, 10.0), Point::new(10.0, 0.0));
        assert_eq!(cubic.path_elements(0.1), vec![PathEl::MoveTo(Point::new(0.0, 0.0)), PathEl::CurveTo(Point::new(0.0, 10.0), Point::new(10.0, 10.0), Point::new(10.0, 0.0))]);
    }

    #[test]
    fn cubic_eval_agrees_with_kurbo_inside_and_outside_unit_interval() {
        let ours = CubicBez::new(Point::new(-3.0, 2.0), Point::new(4.0, 11.0), Point::new(9.0, -7.0), Point::new(15.0, 5.0));
        let oracle = kurbo::CubicBez::new(oracle_point(ours.p0), oracle_point(ours.p1), oracle_point(ours.p2), oracle_point(ours.p3));
        for t in [-1.0, -0.25, 0.0, 0.2, 0.5, 1.0, 1.5, 3.0] {
            assert_points_close(ours.eval(t), kurbo::ParamCurve::eval(&oracle, t), 1e-10, "cubic eval");
        }
    }

    #[test]
    fn circle_arc_and_rounded_rect_path_elements_agree_with_kurbo() {
        for (center, radius, tolerance) in [
            (Point::new(0.0, 0.0), 1.0, 0.1),
            (Point::new(12.5, -9.0), 75.0, 0.01),
            (Point::new(-4.0, 3.0), -12.0, 0.0001),
        ] {
            let ours = Circle::new(center, radius).path_elements(tolerance);
            let oracle = kurbo::Shape::path_elements(&kurbo::Circle::new(oracle_point(center), radius), tolerance).collect::<Vec<_>>();
            assert_path_elements_close(&ours, &oracle, 1e-10, "circle");
        }

        for (center, radii, start, sweep, rotation, tolerance) in [
            (Point::new(0.0, 0.0), (20.0, 10.0), 0.0, std::f64::consts::FRAC_PI_2, 0.0, 0.1),
            (Point::new(3.0, -7.0), (40.0, 5.0), -1.25, 5.5, 0.4, 0.01),
            (Point::new(-8.0, 2.0), (7.0, 19.0), 2.0, -4.75, -0.7, 0.0005),
            (Point::new(5.0, 6.0), (8.0, 3.0), 1.0, 0.0, 0.2, 0.1),
        ] {
            let ours = Arc::new(center, radii, start, sweep, rotation).path_elements(tolerance);
            let oracle = kurbo::Shape::path_elements(&kurbo::Arc::new(oracle_point(center), radii, start, sweep, rotation), tolerance).collect::<Vec<_>>();
            assert_path_elements_close(&ours, &oracle, 1e-10, "arc");
        }

        for (rect, radii, tolerance) in [
            (Rect::new(0.0, 0.0, 200.0, 100.0), RoundedRectRadii::new(10.0, 20.0, 30.0, 40.0), 0.1),
            (Rect::new(30.0, 20.0, -10.0, -40.0), RoundedRectRadii::new(-5.0, 100.0, 0.0, 11.0), 0.01),
        ] {
            let ours = RoundedRect::new(rect, radii).path_elements(tolerance);
            let oracle_radii = kurbo::RoundedRectRadii::new(radii.top_left, radii.top_right, radii.bottom_right, radii.bottom_left);
            let oracle = kurbo::Shape::path_elements(&kurbo::RoundedRect::new(rect.x0, rect.y0, rect.x1, rect.y1, oracle_radii), tolerance).collect::<Vec<_>>();
            assert_path_elements_close(&ours, &oracle, 1e-10, "rounded rect");
        }
    }

    #[test]
    fn empty_or_move_only_bezpath_bounding_box_agrees_with_kurbo() {
        let empty = BezPath::new();
        assert_eq!(empty.bounding_box(), Rect::new(0.0, 0.0, 0.0, 0.0));
        let mut move_only = BezPath::new();
        move_only.move_to((25.0, -15.0));
        let ours = move_only.bounding_box();
        let mut oracle = kurbo::BezPath::new();
        oracle.move_to((25.0, -15.0));
        let oracle = kurbo::Shape::bounding_box(&oracle);
        assert!((ours.x0() - oracle.x0).abs() < 1e-12 && (ours.y0() - oracle.y0).abs() < 1e-12 && (ours.x1() - oracle.x1).abs() < 1e-12 && (ours.y1() - oracle.y1).abs() < 1e-12);
    }

    /// 🔬️ DIFFERENTIAL ORACLE: `Arc::eval` vs `kurbo::ParamCurve::eval(&kurbo::Arc, t)`, built
    /// independently from the same deterministic pseudo-random center/radii/angles/rotation.
    #[test]
    fn arc_eval_agrees_with_kurbo_arc_eval() {
        let mut rng = Lcg(0x853C49E6748FEA9B);
        for _ in 0..32 {
            let center = Point::new(rng.next_f64(-50.0, 50.0), rng.next_f64(-50.0, 50.0));
            let radii = (rng.next_f64(1.0, 80.0), rng.next_f64(1.0, 80.0));
            let start_angle = rng.next_f64(-6.0, 6.0);
            let sweep = rng.next_f64(-6.0, 6.0);
            let x_rotation = rng.next_f64(-3.2, 3.2);
            let arc = Arc::new(center, radii, start_angle, sweep, x_rotation);
            let oracle_arc = kurbo::Arc::new(oracle_point(center), radii, start_angle, sweep, x_rotation);
            for k in 0..=8 {
                let t = k as f64 / 8.0;
                assert_points_close(arc.eval(t), kurbo::ParamCurve::eval(&oracle_arc, t), 1e-6, "arc eval");
            }
        }
    }

    /// 🔬️ SELF-CONSISTENCY: every point sampled along a flattened circle's cubic segments stays
    /// within a generous multiple of `tolerance` of the analytic circle (`|distance_to_center -
    /// radius|`) — proves [`elliptical_arc_segments`]' tolerance-driven segment count actually
    /// honors the requested accuracy, independent of `kurbo`.
    #[test]
    fn circle_flattening_stays_within_tolerance_of_the_analytic_circle() {
        let mut rng = Lcg(0x9E3779B97F4A7C15);
        for _ in 0..16 {
            let center = Point::new(rng.next_f64(-50.0, 50.0), rng.next_f64(-50.0, 50.0));
            let radius = rng.next_f64(1.0, 100.0);
            let tolerance = rng.next_f64(0.001, 1.0);
            let circle = Circle::new(center, radius);
            let mut path = BezPath::new();
            append_shape_to_path(&mut path, &circle, tolerance);
            for seg in path.path_segments() {
                for k in 0..=8 {
                    let t = k as f64 / 8.0;
                    let p = seg.eval(t);
                    let deviation = (distance_between(p, center) - radius).abs();
                    assert!(deviation <= tolerance * 3.0 + radius * 1e-6, "deviation {deviation} exceeds slack for tolerance {tolerance} radius {radius}");
                }
            }
        }
    }

    /// 🔬️ DIFFERENTIAL ORACLE: a flattened circle's `BezPath::bounding_box` vs the analytic
    /// `kurbo::Circle`'s own exact `bounding_box` (`center ± radius`), within `2 * tolerance`.
    #[test]
    fn circle_flattening_bounding_box_matches_kurbo_circle_bounding_box() {
        let center = Point::new(10.0, -5.0);
        let radius = 25.0;
        let tolerance = 0.01;
        let mut path = BezPath::new();
        append_shape_to_path(&mut path, &Circle::new(center, radius), tolerance);
        let bb = path.bounding_box();
        let oracle_bb = kurbo::Shape::bounding_box(&kurbo::Circle::new(oracle_point(center), radius));
        assert!((bb.x0() - oracle_bb.x0).abs() < tolerance * 2.0, "x0: ours={bb:?} oracle={oracle_bb:?}");
        assert!((bb.y0() - oracle_bb.y0).abs() < tolerance * 2.0, "y0: ours={bb:?} oracle={oracle_bb:?}");
        assert!((bb.x1() - oracle_bb.x1).abs() < tolerance * 2.0, "x1: ours={bb:?} oracle={oracle_bb:?}");
        assert!((bb.y1() - oracle_bb.y1).abs() < tolerance * 2.0, "y1: ours={bb:?} oracle={oracle_bb:?}");
    }

    /// 🔬️ SELF-CONSISTENCY + shape smoke test: a rounded rect's flattened outline is closed and
    /// its bounding box matches the outer rect within a small slack.
    #[test]
    fn rounded_rect_flattening_is_closed_and_bounded_by_the_outer_rect() {
        let rect = Rect::new(0.0, 0.0, 200.0, 100.0);
        let radii = RoundedRectRadii::new(10.0, 20.0, 30.0, 0.0);
        let tolerance = 0.05;
        let mut path = BezPath::new();
        append_shape_to_path(&mut path, &RoundedRect::new(rect, radii), tolerance);
        assert!(matches!(path.elements().last(), Some(PathEl::ClosePath)));
        let bb = path.bounding_box();
        assert!(bb.x0() >= -1.0 && bb.x0() <= 1.0, "x0={}", bb.x0());
        assert!(bb.y0() >= -1.0 && bb.y0() <= 1.0, "y0={}", bb.y0());
        assert!(bb.x1() >= 199.0 && bb.x1() <= 201.0, "x1={}", bb.x1());
        assert!(bb.y1() >= 99.0 && bb.y1() <= 101.0, "y1={}", bb.y1());
    }
}
// #endregion 🔖️FirstPartyShapeTests

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
