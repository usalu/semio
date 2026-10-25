//! 🧱️ Curve shapes of the geometry primitives: the circle, the line, the elliptical arc and the cubic bezier. See [`super`].

use super::*;

// #region 🔖️Shapes
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
    /// 📤️ Radius, for encoders that emit the primitive rather than its flattening.
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
    /// 📍️ Start point, for encoders that emit the primitive rather than its flattening.
    pub fn p0(self) -> Point {
        self.p0
    }
    /// ⏹️ End point, for encoders that emit the primitive rather than its flattening.
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
    /// 🧹️ `(center, radii, start_angle, sweep, x_rotation)` — the exact `ctx.ellipse` arguments,
    /// for encoders that emit the primitive rather than its flattening.
    pub fn parameters(self) -> (Point, (f64, f64), f64, f64, f64) {
        (self.center, self.radii, self.start_angle, self.sweep, self.x_rotation)
    }
    /// 📂️ [`elliptical_arc_segments`] over this arc's own angle range — open (no `ClosePath`),
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
// #endregion 🔖️Shapes
