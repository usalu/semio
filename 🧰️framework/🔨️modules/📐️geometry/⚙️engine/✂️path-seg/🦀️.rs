//! ✂️ Path segment of the geometry primitives: the line, quadratic and cubic path segment with its evaluation, subdivision, arc length and tight bounds. See [`super`].

use super::*;

// #region 🔖️Shapes
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
// #endregion 🔖️Shapes
