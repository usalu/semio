//! 🌙️ Bulged segments: a line or circular arc given by start, end and `bulge = tan(sweep / 4)` (DXF/LWPOLYLINE convention; positive bulge is a counter-clockwise arc).
//!
//! Domain-neutral planar curve kernel behind AEC axes, wall faces and loops: centre, radius, sweep, length, point and tangent by parameter, closest point, tight bounds, exact offset, splitting, flattening with a chord tolerance, segment/segment intersection with optional infinite extension, and band footprints with corner joins.
//!
//! Parameter `t` runs over `[0, 1]` and is proportional to arc length (lines by length, arcs by angle).
//! Tolerances: coincidence is [`LENGTH_EPS`] (1 nm in metres) for lengths and [`ANGLE_EPS`] for angles; see `T/r3-geometry-api.md` for the policy per function.

use crate::vector::{angle_between, cross, perp, unit, ANGLE_EPS, LENGTH_EPS};
use crate::{Affine, Point, Rect, Vec2};
use std::f64::consts::{PI, TAU};

/// 🌙️ Sweep angle (radians, signed, counter-clockwise positive) of a bulge.
pub fn sweep_from_bulge(bulge: f64) -> f64 {
    4.0 * bulge.atan()
}

/// 🌙️ Bulge (`tan(sweep / 4)`) of a signed sweep angle.
pub fn bulge_from_sweep(sweep: f64) -> f64 {
    (sweep / 4.0).tan()
}

/// ✏️ One planar segment: a straight line (`bulge == 0`) or a circular arc from `start` to `end`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BulgeSeg {
    pub start: Point,
    pub end: Point,
    pub bulge: f64,
}

/// 🎯️ Result of projecting a point onto a segment.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Closest {
    pub t: f64,
    pub point: Point,
    pub distance: f64,
}

/// 🔭️ Whether intersections consider only the segments or their infinite carriers (full line / full circle).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Extent {
    Bounded,
    Unbounded,
}

/// ✂️ One intersection of two segments with the parameter on each; `ta`/`tb` are unclamped when the extent is [`Extent::Unbounded`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SegHit {
    pub point: Point,
    pub ta: f64,
    pub tb: f64,
}

impl BulgeSeg {
    /// 📏️ Straight segment.
    pub fn line(start: Point, end: Point) -> Self {
        Self { start, end, bulge: 0.0 }
    }

    /// 🌙️ Segment from explicit endpoints and bulge.
    pub fn new(start: Point, end: Point, bulge: f64) -> Self {
        Self { start, end, bulge }
    }

    /// ⭕️ Arc around `center` from `start_angle` through the signed `sweep`.
    pub fn from_arc(center: Point, radius: f64, start_angle: f64, sweep: f64) -> Self {
        let at = |angle: f64| Point::new(center.x + radius * angle.cos(), center.y + radius * angle.sin());
        Self { start: at(start_angle), end: at(start_angle + sweep), bulge: bulge_from_sweep(sweep) }
    }

    /// 🔵️ Arc through three points (`through` lies on the arc between `a` and `b`); `None` for collinear points.
    pub fn from_three_points(a: Point, through: Point, b: Point) -> Option<Self> {
        let orientation = cross(through - a, b - through);
        let scale = (through - a).hypot().max((b - through).hypot()).max(1.0);
        if orientation.abs() <= 1e-12 * scale * scale {
            return None;
        }
        let d = 2.0 * (a.x * (through.y - b.y) + through.x * (b.y - a.y) + b.x * (a.y - through.y));
        let (a2, m2, b2) = (a.x * a.x + a.y * a.y, through.x * through.x + through.y * through.y, b.x * b.x + b.y * b.y);
        let center = Point::new((a2 * (through.y - b.y) + m2 * (b.y - a.y) + b2 * (a.y - through.y)) / d, (a2 * (b.x - through.x) + m2 * (a.x - b.x) + b2 * (through.x - a.x)) / d);
        let direction = orientation.signum();
        let sweep = (angle_between(a - center, b - center) * direction).rem_euclid(TAU) * direction;
        Some(Self { start: a, end: b, bulge: bulge_from_sweep(sweep) })
    }

    /// 📏️ `true` when the segment is a straight line (|bulge| below 1e-12).
    pub fn is_line(&self) -> bool {
        self.bulge.abs() <= 1e-12
    }

    /// 📏️ Straight distance between the endpoints.
    pub fn chord(&self) -> f64 {
        (self.end - self.start).hypot()
    }

    /// 🧭️ Signed sweep angle, counter-clockwise positive; zero for lines.
    pub fn sweep(&self) -> f64 {
        sweep_from_bulge(self.bulge)
    }

    /// 📐️ Arc radius; infinite for lines.
    pub fn radius(&self) -> f64 {
        if self.is_line() {
            return f64::INFINITY;
        }
        self.chord() / (2.0 * (self.sweep().abs() / 2.0).sin())
    }

    /// 🎯️ Arc centre; `None` for lines and for degenerate zero-chord arcs.
    pub fn center(&self) -> Option<Point> {
        let chord = self.chord();
        if self.is_line() || chord <= LENGTH_EPS {
            return None;
        }
        let direction = (self.end - self.start) / chord;
        let mid = Point::new((self.start.x + self.end.x) / 2.0, (self.start.y + self.end.y) / 2.0);
        let offset = chord / 2.0 * (1.0 - self.bulge * self.bulge) / (2.0 * self.bulge);
        Some(mid + perp(direction) * offset)
    }

    /// 📏️ Arc length (chord length for lines).
    pub fn length(&self) -> f64 {
        if self.is_line() {
            self.chord()
        } else {
            self.radius() * self.sweep().abs()
        }
    }

    /// 🧭️ Angle of the start point around the centre; `None` for lines.
    pub fn start_angle(&self) -> Option<f64> {
        let center = self.center()?;
        let v = self.start - center;
        Some(v.y.atan2(v.x))
    }

    /// 📍️ Point at parameter `t` (not clamped).
    pub fn point_at(&self, t: f64) -> Point {
        match (self.center(), self.start_angle()) {
            (Some(center), Some(angle)) => {
                let a = angle + self.sweep() * t;
                let r = self.radius();
                Point::new(center.x + r * a.cos(), center.y + r * a.sin())
            }
            _ => Point::new(self.start.x + (self.end.x - self.start.x) * t, self.start.y + (self.end.y - self.start.y) * t),
        }
    }

    /// 📍️ Point at arc length `s` from the start (not clamped).
    pub fn point_at_length(&self, s: f64) -> Point {
        let length = self.length();
        self.point_at(if length > 0.0 { s / length } else { 0.0 })
    }

    /// ➡️ Unit tangent (direction of travel) at parameter `t`; the zero vector for a zero-length line.
    pub fn tangent_at(&self, t: f64) -> Vec2 {
        match (self.center(), self.start_angle()) {
            (Some(_), Some(angle)) => {
                let a = angle + self.sweep() * t;
                Vec2::new(-a.sin(), a.cos()) * self.sweep().signum()
            }
            _ => unit(self.end - self.start, 0.0).unwrap_or(Vec2::ZERO),
        }
    }

    /// ➡️ Unit tangent at arc length `s` from the start.
    pub fn tangent_at_length(&self, s: f64) -> Vec2 {
        let length = self.length();
        self.tangent_at(if length > 0.0 { s / length } else { 0.0 })
    }

    /// 🧭️ Angular fraction (in direction of travel, relative to the start, wrapped to a tiny negative tolerance) of a point seen from the centre; for lines the projection fraction.
    pub fn param_of(&self, p: Point) -> f64 {
        match self.center() {
            Some(center) => {
                let sweep = self.sweep();
                let start = self.start - center;
                let mut a = (angle_between(start, p - center) * sweep.signum()).rem_euclid(TAU);
                if a > TAU - 1e-9 {
                    a -= TAU;
                }
                a / sweep.abs()
            }
            None => {
                let d = self.end - self.start;
                let l2 = d.dot(d);
                if l2 > 0.0 {
                    (p - self.start).dot(d) / l2
                } else {
                    0.0
                }
            }
        }
    }

    /// 🎯️ Closest point on the segment (endpoints included).
    pub fn closest(&self, p: Point) -> Closest {
        let make = |t: f64, point: Point| Closest { t, point, distance: (p - point).hypot() };
        match self.center() {
            Some(center) => {
                let v = p - center;
                let t = self.param_of(p);
                if v.hypot() > LENGTH_EPS && t <= 1.0 {
                    let r = self.radius();
                    return make(t.max(0.0), center + v / v.hypot() * r);
                }
                let (ds, de) = ((p - self.start).hypot(), (p - self.end).hypot());
                if ds <= de {
                    make(0.0, self.start)
                } else {
                    make(1.0, self.end)
                }
            }
            None => {
                let t = self.param_of(p).clamp(0.0, 1.0);
                make(t, self.point_at(t))
            }
        }
    }

    /// 🧮️ Tight axis-aligned bounds including arc extremes.
    pub fn bounds(&self) -> Rect {
        let mut x0 = self.start.x.min(self.end.x);
        let mut x1 = self.start.x.max(self.end.x);
        let mut y0 = self.start.y.min(self.end.y);
        let mut y1 = self.start.y.max(self.end.y);
        if let (Some(center), Some(a0)) = (self.center(), self.start_angle()) {
            let r = self.radius();
            let sweep = self.sweep();
            for k in 0..4 {
                let target = k as f64 * PI / 2.0;
                let mut a = ((target - a0) * sweep.signum()).rem_euclid(TAU);
                if a > TAU - ANGLE_EPS {
                    a = 0.0;
                }
                if a <= sweep.abs() {
                    let p = Point::new(center.x + r * target.cos(), center.y + r * target.sin());
                    x0 = x0.min(p.x);
                    x1 = x1.max(p.x);
                    y0 = y0.min(p.y);
                    y1 = y1.max(p.y);
                }
            }
        }
        Rect::new(x0, y0, x1, y1)
    }

    /// 🔄️ Same curve traversed backwards.
    pub fn reversed(&self) -> Self {
        Self { start: self.end, end: self.start, bulge: -self.bulge }
    }

    /// ✂️ Splits at parameter `t` into two segments on the same carrier.
    pub fn split_at(&self, t: f64) -> (Self, Self) {
        let t = t.clamp(0.0, 1.0);
        let mid = self.point_at(t);
        let sweep = self.sweep();
        (Self { start: self.start, end: mid, bulge: bulge_from_sweep(sweep * t) }, Self { start: mid, end: self.end, bulge: bulge_from_sweep(sweep * (1.0 - t)) })
    }

    /// ✂️ Sub-segment between parameters `t0 <= t1`.
    pub fn subsegment(&self, t0: f64, t1: f64) -> Self {
        let (t0, t1) = (t0.clamp(0.0, 1.0), t1.clamp(0.0, 1.0));
        let (a, b) = (self.point_at(t0), self.point_at(t1));
        Self { start: a, end: b, bulge: bulge_from_sweep(self.sweep() * (t1 - t0)) }
    }

    /// ↔️ Parallel curve at signed `distance` to the left of the direction of travel: a shifted line, or a concentric arc with radius `r - distance * sign(sweep)`. `None` when the arc collapses (radius below 1 nm) or the line has no length.
    pub fn offset(&self, distance: f64) -> Option<Self> {
        match self.center() {
            Some(center) => {
                let r = self.radius();
                let r2 = r - distance * self.sweep().signum();
                if r2 <= LENGTH_EPS {
                    return None;
                }
                let k = r2 / r;
                Some(Self { start: center + (self.start - center) * k, end: center + (self.end - center) * k, bulge: self.bulge })
            }
            None => {
                let direction = unit(self.end - self.start, LENGTH_EPS)?;
                let shift = perp(direction) * distance;
                Some(Self { start: self.start + shift, end: self.end + shift, bulge: self.bulge })
            }
        }
    }

    /// 🧷️ Same carrier (line, or circle and direction) with new endpoints; the sweep is re-measured in the direction of travel in `(0, 2pi]`, so use it for small trims and extensions.
    pub fn retarget(&self, start: Point, end: Point) -> Self {
        match self.center() {
            Some(center) => {
                let direction = self.sweep().signum();
                let mut a = (angle_between(start - center, end - center) * direction).rem_euclid(TAU);
                if a <= ANGLE_EPS && (end - start).hypot() > LENGTH_EPS {
                    a = TAU;
                }
                Self { start, end, bulge: bulge_from_sweep(a * direction) }
            }
            None => Self::line(start, end),
        }
    }

    /// 🔁️ Image under a similarity transform (uniform scale, rotation, translation, optional mirror); a mirror flips the bulge sign.
    pub fn transformed(&self, affine: Affine) -> Self {
        let c = affine.as_coeffs();
        let det = c[0] * c[3] - c[1] * c[2];
        Self { start: affine * self.start, end: affine * self.end, bulge: if det < 0.0 { -self.bulge } else { self.bulge } }
    }

    /// 🔷️ Appends the polyline approximation after `start` (interior chord points then `end`) so that the sagitta stays within `tolerance`.
    pub fn flatten_into(&self, tolerance: f64, out: &mut Vec<Point>) {
        if let Some(center) = self.center() {
            let r = self.radius();
            let sweep = self.sweep().abs();
            let step = 2.0 * (1.0 - (tolerance / r).clamp(1e-12, 1.0)).acos();
            let n = ((sweep / step.max(1e-6)).ceil() as usize).clamp(1, 1 << 16);
            let a0 = self.start_angle().unwrap_or(0.0);
            let direction = self.sweep().signum();
            for i in 1..n {
                let a = a0 + direction * sweep * i as f64 / n as f64;
                out.push(Point::new(center.x + r * a.cos(), center.y + r * a.sin()));
            }
        }
        out.push(self.end);
    }

    /// 🔷️ Polyline approximation including the start point.
    pub fn flatten(&self, tolerance: f64) -> Vec<Point> {
        let mut out = vec![self.start];
        self.flatten_into(tolerance, &mut out);
        out
    }

    /// 📐️ Signed area between chord and arc (positive for counter-clockwise arcs): `r^2 / 2 * (sweep - sin sweep)`; zero for lines.
    pub fn segment_area(&self) -> f64 {
        if self.is_line() {
            return 0.0;
        }
        let r = self.radius();
        let sweep = self.sweep();
        r * r / 2.0 * (sweep - sweep.sin())
    }

    /// ⚖️ Signed first moment of the chord-to-arc region about the origin: `segment_area * centroid`.
    pub fn segment_first_moment(&self) -> Vec2 {
        let (Some(center), false) = (self.center(), self.is_line()) else { return Vec2::ZERO };
        let r = self.radius();
        let sweep = self.sweep().abs();
        let area = r * r / 2.0 * (sweep - sweep.sin());
        if area.abs() <= f64::MIN_POSITIVE {
            return Vec2::ZERO;
        }
        let mid = self.point_at(0.5);
        let axis = unit(mid - center, 0.0).unwrap_or(Vec2::ZERO);
        let distance = 4.0 * r * (sweep / 2.0).sin().powi(3) / (3.0 * (sweep - sweep.sin()));
        let centroid = center + axis * distance;
        Vec2::new(centroid.x, centroid.y) * (area * self.sweep().signum())
    }
}

enum Carrier {
    Line { origin: Point, direction: Vec2 },
    Circle { center: Point, radius: f64 },
}

fn carrier(seg: &BulgeSeg) -> Carrier {
    match seg.center() {
        Some(center) => Carrier::Circle { center, radius: seg.radius() },
        None => Carrier::Line { origin: seg.start, direction: seg.end - seg.start },
    }
}

fn candidate_points(a: &Carrier, b: &Carrier) -> Vec<Point> {
    match (a, b) {
        (Carrier::Line { origin: oa, direction: da }, Carrier::Line { origin: ob, direction: db }) => {
            let denominator = cross(*da, *db);
            if denominator.abs() <= 1e-12 * da.hypot() * db.hypot() {
                return Vec::new();
            }
            let t = cross(*ob - *oa, *db) / denominator;
            vec![*oa + *da * t]
        }
        (Carrier::Line { origin, direction }, Carrier::Circle { center, radius }) | (Carrier::Circle { center, radius }, Carrier::Line { origin, direction }) => line_circle(*origin, *direction, *center, *radius),
        (Carrier::Circle { center: ca, radius: ra }, Carrier::Circle { center: cb, radius: rb }) => circle_circle(*ca, *ra, *cb, *rb),
    }
}

fn line_circle(origin: Point, direction: Vec2, center: Point, radius: f64) -> Vec<Point> {
    let l2 = direction.dot(direction);
    if l2 <= 0.0 {
        return Vec::new();
    }
    let t0 = (center - origin).dot(direction) / l2;
    let foot = origin + direction * t0;
    let distance = (foot - center).hypot();
    if distance > radius + LENGTH_EPS {
        return Vec::new();
    }
    if distance >= radius - LENGTH_EPS {
        return vec![foot];
    }
    let half = (radius * radius - distance * distance).sqrt() / l2.sqrt();
    vec![foot - direction * half, foot + direction * half]
}

fn circle_circle(ca: Point, ra: f64, cb: Point, rb: f64) -> Vec<Point> {
    let d = (cb - ca).hypot();
    if d <= LENGTH_EPS || d > ra + rb + LENGTH_EPS || d < (ra - rb).abs() - LENGTH_EPS {
        return Vec::new();
    }
    let along = (ra * ra - rb * rb + d * d) / (2.0 * d);
    let h2 = ra * ra - along * along;
    let axis = (cb - ca) / d;
    let base = ca + axis * along;
    if h2.sqrt() <= LENGTH_EPS {
        return vec![base];
    }
    let h = h2.sqrt();
    vec![base + perp(axis) * h, base - perp(axis) * h]
}

fn within(t: f64, seg: &BulgeSeg) -> bool {
    let slack = LENGTH_EPS / seg.length().max(LENGTH_EPS);
    t >= -slack && t <= 1.0 + slack
}

/// ✂️ Intersections of two segments sorted by `ta`; parallel, coincident and concentric carriers yield none, tangential contact yields one point.
pub fn intersect(a: &BulgeSeg, b: &BulgeSeg, extent: Extent) -> Vec<SegHit> {
    let mut hits: Vec<SegHit> = candidate_points(&carrier(a), &carrier(b)).into_iter().map(|point| SegHit { point, ta: a.param_of(point), tb: b.param_of(point) }).filter(|hit| extent == Extent::Unbounded || (within(hit.ta, a) && within(hit.tb, b))).collect();
    hits.sort_by(|x, y| x.ta.total_cmp(&y.ta));
    hits
}

/// 🎯️ Intersection of the infinite carriers (or the bounded segments) closest to `near`; the miter point of a corner when `near` is the shared vertex.
pub fn nearest_intersection(a: &BulgeSeg, b: &BulgeSeg, extent: Extent, near: Point) -> Option<Point> {
    intersect(a, b, extent).into_iter().map(|hit| hit.point).min_by(|p, q| (*p - near).hypot().total_cmp(&(*q - near).hypot()))
}

/// 🧱️ Offset end points of one band side at a corner (`None` where the sides are parallel or no intersection exists).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Corner {
    pub left: Option<Point>,
    pub right: Option<Point>,
}

/// 🧱️ Miter join of two bands meeting at `prev.end == next.start`: each band has its own `(left, right)` half widths; returns the points where the left faces and the right faces meet.
pub fn corner_join(prev: &BulgeSeg, prev_widths: (f64, f64), next: &BulgeSeg, next_widths: (f64, f64)) -> Corner {
    let side = |a: Option<BulgeSeg>, b: Option<BulgeSeg>| match (a, b) {
        (Some(a), Some(b)) => nearest_intersection(&a, &b, Extent::Unbounded, prev.end),
        _ => None,
    };
    Corner { left: side(prev.offset(prev_widths.0), next.offset(next_widths.0)), right: side(prev.offset(-prev_widths.1), next.offset(-next_widths.1)) }
}

/// 🧱️ Footprint of a band around `axis` as a counter-clockwise loop of four bulged vertices `[right.start, right.end, left.end, left.start]`.
/// `start_trim`/`end_trim` replace the `(left, right)` end points (for example from [`corner_join`]); `None` keeps the square cut. `None` result when a side collapses.
pub fn band_loop(axis: &BulgeSeg, left: f64, right: f64, start_trim: Option<(Point, Point)>, end_trim: Option<(Point, Point)>) -> Option<[(Point, f64); 4]> {
    let l = axis.offset(left)?;
    let r = axis.offset(-right)?;
    let (ls, rs) = start_trim.unwrap_or((l.start, r.start));
    let (le, re) = end_trim.unwrap_or((l.end, r.end));
    let l = l.retarget(ls, le);
    let r = r.retarget(rs, re);
    Some([(r.start, r.bulge), (r.end, 0.0), (l.end, -l.bulge), (l.start, 0.0)])
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
