//! 📐️ Owned drawing geometry, affine transforms, and path evaluation.

use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint2, SemioTransform};
use crate::standards::v1::subsets::drawing::schema::snapshot::PathSegment;

pub type Affine = [f64; 6];

pub fn compose_affine(outer: &Affine, inner: &Affine) -> Affine {
    let [a, b, c, d, e, f] = *outer;
    let [a2, b2, c2, d2, e2, f2] = *inner;
    [a * a2 + c * b2, b * a2 + d * b2, a * c2 + c * d2, b * c2 + d * d2, a * e2 + c * f2 + e, b * e2 + d * f2 + f]
}

pub fn transform_point(m: &Affine, p: [f64; 2]) -> [f64; 2] {
    [m[0] * p[0] + m[2] * p[1] + m[4], m[1] * p[0] + m[3] * p[1] + m[5]]
}

/// 🧭️ The 2D affine matrix a `Group`'s transform denotes — identical to the svg leaf's `matrix(...)`.
pub fn semio_transform_affine(t: &SemioTransform) -> [f64; 6] {
    let theta = 2.0 * t.rotation.z.atan2(t.rotation.w);
    let (sin, cos) = theta.sin_cos();
    [cos * t.scale.x, sin * t.scale.x, -sin * t.scale.y, cos * t.scale.y, t.translation.x, t.translation.y]
}

//#region 🔖️Flatten
/// ✏️ One flattened subpath in local coordinates.
pub(crate) struct Subpath {
    pub(crate) points: Vec<[f64; 2]>,
    pub(crate) closed: bool,
}

fn steps_for(length_in_pixels: f64) -> usize {
    (length_in_pixels / 2.0).ceil().clamp(1.0, 512.0) as usize
}

pub(crate) fn distance(a: [f64; 2], b: [f64; 2]) -> f64 {
    ((b[0] - a[0]).powi(2) + (b[1] - a[1]).powi(2)).sqrt()
}

/// 🌙️ An SVG endpoint-parameterized elliptical arc as cubic Béziers `[c1, c2, to]`, one per quarter
/// turn at most (radial error below 0.03 % of the radius), radii scaled up when too small to reach.
pub fn arc_to_cubics(from: [f64; 2], rx: f64, ry: f64, x_rotation_degrees: f64, large_arc: bool, sweep: bool, to: [f64; 2]) -> Vec<[[f64; 2]; 3]> {
    let (mut rx, mut ry) = (rx.abs(), ry.abs());
    if distance(from, to) == 0.0 {
        return Vec::new();
    }
    if rx == 0.0 || ry == 0.0 {
        return vec![[from, to, to]];
    }
    let phi = x_rotation_degrees.to_radians();
    let (sin_phi, cos_phi) = phi.sin_cos();
    let dx = (from[0] - to[0]) / 2.0;
    let dy = (from[1] - to[1]) / 2.0;
    let x1 = cos_phi * dx + sin_phi * dy;
    let y1 = -sin_phi * dx + cos_phi * dy;
    let lambda = (x1 * x1) / (rx * rx) + (y1 * y1) / (ry * ry);
    if lambda > 1.0 {
        rx *= lambda.sqrt();
        ry *= lambda.sqrt();
    }
    let numerator = (rx * rx * ry * ry - rx * rx * y1 * y1 - ry * ry * x1 * x1).max(0.0);
    let denominator = rx * rx * y1 * y1 + ry * ry * x1 * x1;
    let root = if denominator > 0.0 { (numerator / denominator).sqrt() } else { 0.0 };
    let sign = if large_arc == sweep { -1.0 } else { 1.0 };
    let cx1 = sign * root * rx * y1 / ry;
    let cy1 = -sign * root * ry * x1 / rx;
    let cx = cos_phi * cx1 - sin_phi * cy1 + (from[0] + to[0]) / 2.0;
    let cy = sin_phi * cx1 + cos_phi * cy1 + (from[1] + to[1]) / 2.0;
    let angle = |ux: f64, uy: f64, vx: f64, vy: f64| {
        let turn = (ux * vy - uy * vx).signum();
        let cosine = ((ux * vx + uy * vy) / ((ux * ux + uy * uy).sqrt() * (vx * vx + vy * vy).sqrt())).clamp(-1.0, 1.0);
        if turn < 0.0 { -cosine.acos() } else { cosine.acos() }
    };
    let theta1 = angle(1.0, 0.0, (x1 - cx1) / rx, (y1 - cy1) / ry);
    let mut delta = angle((x1 - cx1) / rx, (y1 - cy1) / ry, (-x1 - cx1) / rx, (-y1 - cy1) / ry);
    if !sweep && delta > 0.0 {
        delta -= std::f64::consts::TAU;
    } else if sweep && delta < 0.0 {
        delta += std::f64::consts::TAU;
    }
    let pieces = (delta.abs() / std::f64::consts::FRAC_PI_2).ceil().max(1.0) as usize;
    let step = delta / pieces as f64;
    let handle = 4.0 / 3.0 * (step / 4.0).tan();
    let on_ellipse = |theta: f64| {
        let (sin, cos) = theta.sin_cos();
        ([cos_phi * rx * cos - sin_phi * ry * sin + cx, sin_phi * rx * cos + cos_phi * ry * sin + cy], [-cos_phi * rx * sin - sin_phi * ry * cos, -sin_phi * rx * sin + cos_phi * ry * cos])
    };
    (0..pieces)
        .map(|piece| {
            let (a, b) = (theta1 + step * piece as f64, theta1 + step * (piece + 1) as f64);
            let ((p0, d0), (p1, d1)) = (on_ellipse(a), on_ellipse(b));
            let end = if piece + 1 == pieces { to } else { p1 };
            [[p0[0] + handle * d0[0], p0[1] + handle * d0[1]], [p1[0] - handle * d1[0], p1[1] - handle * d1[1]], end]
        })
        .collect()
}

fn push_cubic(current: &mut Vec<[f64; 2]>, p0: [f64; 2], p1: [f64; 2], p2: [f64; 2], p3: [f64; 2], pixel_scale: f64) {
    let steps = steps_for((distance(p0, p1) + distance(p1, p2) + distance(p2, p3)) * pixel_scale);
    for step in 1..=steps {
        let t = step as f64 / steps as f64;
        let u = 1.0 - t;
        let (w0, w1, w2, w3) = (u * u * u, 3.0 * u * u * t, 3.0 * u * t * t, t * t * t);
        current.push([w0 * p0[0] + w1 * p1[0] + w2 * p2[0] + w3 * p3[0], w0 * p0[1] + w1 * p1[1] + w2 * p2[1] + w3 * p3[1]]);
    }
}

/// 📐️ `segments` mapped through `matrix`: points move, arcs become cubics (an affine image of an
/// ellipse arc is not an SVG arc in general); the identity leaves the path untouched.
pub fn transformed_segments(segments: &[PathSegment], matrix: &[f64; 6]) -> Vec<PathSegment> {
    if *matrix == [1.0, 0.0, 0.0, 1.0, 0.0, 0.0] {
        return segments.to_vec();
    }
    let map = |p: [f64; 2]| {
        let [x, y] = transform_point(matrix, p);
        SemioPoint2 { x, y }
    };
    let mut pen = [0.0, 0.0];
    let mut start = [0.0, 0.0];
    let mut out = Vec::with_capacity(segments.len());
    for segment in segments {
        match segment {
            PathSegment::MoveTo { to } => {
                pen = [to.x, to.y];
                start = pen;
                out.push(PathSegment::MoveTo { to: map(pen) });
            }
            PathSegment::LineTo { to } => {
                pen = [to.x, to.y];
                out.push(PathSegment::LineTo { to: map(pen) });
            }
            PathSegment::QuadTo { c, to } => {
                pen = [to.x, to.y];
                out.push(PathSegment::QuadTo { c: map([c.x, c.y]), to: map(pen) });
            }
            PathSegment::CubicTo { c1, c2, to } => {
                pen = [to.x, to.y];
                out.push(PathSegment::CubicTo { c1: map([c1.x, c1.y]), c2: map([c2.x, c2.y]), to: map(pen) });
            }
            PathSegment::ArcTo { rx, ry, x_rotation, large_arc, sweep, to } => {
                out.extend(arc_to_cubics(pen, *rx, *ry, *x_rotation, *large_arc, *sweep, [to.x, to.y]).into_iter().map(|[c1, c2, end]| PathSegment::CubicTo { c1: map(c1), c2: map(c2), to: map(end) }));
                pen = [to.x, to.y];
            }
            PathSegment::Close => {
                pen = start;
                out.push(PathSegment::Close);
            }
        }
    }
    out
}

/// ⭕️ `(centre, radius)` of a path in the two-arc circle normal form
/// `[MoveTo(p), ArcTo(r, r → q), ArcTo(r, r → p), Close]`, the shape every circle-bearing bridge
/// (svg, dxf `CIRCLE`, dwg `Circle`) reads and writes.
pub fn circle_normal_form(segments: &[PathSegment]) -> Option<([f64; 2], f64)> {
    match segments {
        [PathSegment::MoveTo { to: p }, PathSegment::ArcTo { rx: r1, ry: q1, to: q, .. }, PathSegment::ArcTo { rx: r2, ry: q2, to: back, .. }, PathSegment::Close]
            if (r1 - q1).abs() < 1e-9 && (r2 - q2).abs() < 1e-9 && (r1 - r2).abs() < 1e-9 && (p.x - back.x).abs() < 1e-9 && (p.y - back.y).abs() < 1e-9 && (distance([p.x, p.y], [q.x, q.y]) - 2.0 * r1).abs() < 1e-9 * r1.max(1.0) =>
        {
            Some(([(p.x + q.x) / 2.0, (p.y + q.y) / 2.0], *r1))
        }
        _ => None,
    }
}

/// 🧭️ `Some(scale)` when `matrix` is a similarity (rotation, uniform scale, translation, no
/// mirror), under which a circle stays a circle of radius `scale · r`.
pub fn similarity_scale(matrix: &[f64; 6]) -> Option<f64> {
    let [a, b, c, d, _, _] = *matrix;
    ((a - d).abs() < 1e-12 && (b + c).abs() < 1e-12 && a * d - b * c > 0.0).then(|| (a * d - b * c).sqrt())
}

/// 〰️ A path's subpaths as polylines `(points, closed)` — curves and arcs sampled at about two
/// units of `pixel_scale`-scaled length per point — for consumers that need vertices, not paint.
pub fn flatten_segments(segments: &[PathSegment], pixel_scale: f64) -> Vec<(Vec<[f64; 2]>, bool)> {
    flatten(segments, pixel_scale).into_iter().map(|subpath| (subpath.points, subpath.closed)).collect()
}

pub(crate) fn flatten(segments: &[PathSegment], pixel_scale: f64) -> Vec<Subpath> {
    let mut subpaths: Vec<Subpath> = Vec::new();
    let mut current: Vec<[f64; 2]> = Vec::new();
    let mut start = [0.0, 0.0];
    let mut pen = [0.0, 0.0];
    let point = |p: &SemioPoint2| [p.x, p.y];
    let flush = |current: &mut Vec<[f64; 2]>, subpaths: &mut Vec<Subpath>, closed: bool| {
        if current.len() > 1 {
            subpaths.push(Subpath { points: std::mem::take(current), closed });
        } else {
            current.clear();
        }
    };
    for segment in segments {
        match segment {
            PathSegment::MoveTo { to } => {
                flush(&mut current, &mut subpaths, false);
                start = point(to);
                pen = start;
                current.push(pen);
            }
            PathSegment::LineTo { to } => {
                if current.is_empty() {
                    current.push(pen);
                }
                pen = point(to);
                current.push(pen);
            }
            PathSegment::QuadTo { c, to } => {
                if current.is_empty() {
                    current.push(pen);
                }
                let (p0, p1, p2) = (pen, point(c), point(to));
                let steps = steps_for((distance(p0, p1) + distance(p1, p2)) * pixel_scale);
                for step in 1..=steps {
                    let t = step as f64 / steps as f64;
                    let u = 1.0 - t;
                    current.push([u * u * p0[0] + 2.0 * u * t * p1[0] + t * t * p2[0], u * u * p0[1] + 2.0 * u * t * p1[1] + t * t * p2[1]]);
                }
                pen = p2;
            }
            PathSegment::CubicTo { c1, c2, to } => {
                if current.is_empty() {
                    current.push(pen);
                }
                push_cubic(&mut current, pen, point(c1), point(c2), point(to), pixel_scale);
                pen = point(to);
            }
            PathSegment::ArcTo { rx, ry, x_rotation, large_arc, sweep, to } => {
                if current.is_empty() {
                    current.push(pen);
                }
                let mut from = pen;
                for [c1, c2, end] in arc_to_cubics(pen, *rx, *ry, *x_rotation, *large_arc, *sweep, point(to)) {
                    push_cubic(&mut current, from, c1, c2, end, pixel_scale);
                    from = end;
                }
                pen = point(to);
            }
            PathSegment::Close => {
                flush(&mut current, &mut subpaths, true);
                pen = start;
            }
        }
    }
    flush(&mut current, &mut subpaths, false);
    subpaths
}
//#endregion 🔖️Flatten
