//! 📏️ Generic geometry of the geometry primitives: the clamp, distance, normalise, ray and polyline and cubic distance helpers shared by every consumer. See [`super`].

use super::*;

// #region 🔖️GenericGeometry
// 📏️ Distance/normalize/clamp primitives shared by every geometry consumer.

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
