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
    /// 🧾️ Clockwise corner radii from top-left, the order `ctx.roundRect` itself takes.
    pub fn as_clockwise(self) -> [f64; 4] {
        [self.top_left, self.top_right, self.bottom_right, self.bottom_left]
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RoundedRect {
    pub(crate) rect: Rect,
    pub(crate) radii: RoundedRectRadii,
}

impl RoundedRect {
    /// 🧾️ The normalized body rectangle, for encoders that emit the primitive itself rather than
    /// its flattening (`canvas::draw_list`, whose `ctx.roundRect` twin needs the exact corners).
    pub fn rect(self) -> Rect {
        self.rect
    }
    /// 🧾️ The four normalized corner radii, clockwise from top-left.
    pub fn radii(self) -> RoundedRectRadii {
        self.radii
    }
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
    /// 🧾️ Center point, for encoders that emit the primitive rather than its flattening.
    pub fn center(self) -> Point {
        self.center
    }
    /// 🧾️ Radius, for encoders that emit the primitive rather than its flattening.
    pub fn radius(self) -> f64 {
        self.radius
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
    /// 🧾️ Start point, for encoders that emit the primitive rather than its flattening.
    pub fn p0(self) -> Point {
        self.p0
    }
    /// 🧾️ End point, for encoders that emit the primitive rather than its flattening.
    pub fn p1(self) -> Point {
        self.p1
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
    /// 🧾️ `(center, radii, start_angle, sweep, x_rotation)` — the exact `ctx.ellipse` arguments,
    /// for encoders that emit the primitive rather than its flattening.
    pub fn parameters(self) -> (Point, (f64, f64), f64, f64, f64) {
        (self.center, self.radii, self.start_angle, self.sweep, self.x_rotation)
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests

// #region 🔖️PathSegTests
#[cfg(test)]
#[path = "🧪️tests/🔬️path-seg/🦀️.rs"]
mod path_seg_tests;
// #endregion 🔖️PathSegTests

// #region 🔖️FirstPartyShapeTests
#[cfg(test)]
#[path = "🧪️tests/🔬️first-party-shape/🦀️.rs"]
mod first_party_shape_tests;
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
#[path = "🧪️tests/🔬️algebra/🦀️.rs"]
mod algebra_tests;
// #endregion 🔖️AlgebraTests
