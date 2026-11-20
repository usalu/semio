//! 📐️ PDF user-space transforms and cropped, rotated page viewports.

const PDF_IDENTITY_MATRIX: [f64; 6] = [1.0, 0.0, 0.0, 1.0, 0.0, 0.0];

#[derive(Clone, Copy)]
pub struct PageViewport {
    bounds: [f64; 4],
    rotate: i32,
    pub unit: f64,
    pub top: f64,
}

impl PageViewport {
    pub fn new(media: [f64; 4], crop: Option<[f64; 4]>, rotation: i32, user_unit: Option<f64>, top: f64) -> Self {
        let unit = user_unit.filter(|unit| unit.is_finite() && *unit > 0.0).unwrap_or(1.0);
        Self { bounds: visible_page_box(media, crop), rotate: rotation.rem_euclid(360), unit, top }
    }

    pub fn clip_rect(self) -> [f64; 4] {
        let [left, bottom, right, top] = self.bounds;
        let width = (right - left) * self.unit;
        let height = (top - bottom) * self.unit;
        match self.rotate {
            90 | 270 => [0.0, self.top, height, width],
            _ => [0.0, self.top, width, height],
        }
    }

    pub fn contains(self, x: f64, y: f64) -> bool {
        let [left, top, width, height] = self.clip_rect();
        width > 0.0 && height > 0.0 && x >= left && x <= left + width && y >= top && y <= top + height
    }

    pub fn point(self, x: f64, y: f64) -> (f64, f64) {
        let [left, bottom, right, top] = self.bounds;
        let (x, y) = match self.rotate {
            90 => (y - bottom, x - left),
            180 => (right - x, y - bottom),
            270 => (top - y, right - x),
            _ => (x - left, top - y),
        };
        (x * self.unit, self.top + y * self.unit)
    }

    pub fn rect(self, x: f64, y: f64, width: f64, height: f64) -> (f64, f64, f64, f64) {
        let (x0, y0) = self.point(x, y);
        let (x1, y1) = self.point(x + width, y + height);
        (x0.min(x1), y0.min(y1), (x1 - x0).abs(), (y1 - y0).abs())
    }
}

pub fn visible_page_box(media: [f64; 4], crop: Option<[f64; 4]>) -> [f64; 4] {
    fn ordered([x0, y0, x1, y1]: [f64; 4]) -> [f64; 4] {
        [x0.min(x1), y0.min(y1), x0.max(x1), y0.max(y1)]
    }
    let [left, bottom, right, top] = ordered(media);
    let Some(crop) = crop else { return [left, bottom, right, top] };
    let [crop_left, crop_bottom, crop_right, crop_top] = ordered(crop);
    let visible_left = left.max(crop_left);
    let visible_bottom = bottom.max(crop_bottom);
    let visible_right = right.min(crop_right);
    let visible_top = top.min(crop_top);
    if visible_right > visible_left && visible_top > visible_bottom {
        [visible_left, visible_bottom, visible_right, visible_top]
    } else {
        [left, bottom, right, top]
    }
}

pub fn cubic(p0: (f64, f64), p1: (f64, f64), p2: (f64, f64), p3: (f64, f64), t: f64) -> (f64, f64) {
    let u = 1.0 - t;
    let a = u * u * u;
    let b = 3.0 * u * u * t;
    let c = 3.0 * u * t * t;
    let d = t * t * t;
    (a * p0.0 + b * p1.0 + c * p2.0 + d * p3.0, a * p0.1 + b * p1.1 + c * p2.1 + d * p3.1)
}

pub fn unit_square(ctm: [f64; 6]) -> [(f64, f64); 4] {
    box_corners(ctm, [0.0, 0.0, 1.0, 1.0])
}

pub fn box_corners(ctm: [f64; 6], [left, bottom, right, top]: [f64; 4]) -> [(f64, f64); 4] {
    [apply(ctm, left, bottom), apply(ctm, right, bottom), apply(ctm, right, top), apply(ctm, left, top)]
}

pub fn bounds(points: &[(f64, f64)]) -> (f64, f64, f64, f64) {
    if points.is_empty() {
        return (0.0, 0.0, 0.0, 0.0);
    }
    let min_x = points.iter().map(|point| point.0).fold(f64::INFINITY, f64::min);
    let min_y = points.iter().map(|point| point.1).fold(f64::INFINITY, f64::min);
    let max_x = points.iter().map(|point| point.0).fold(f64::NEG_INFINITY, f64::max);
    let max_y = points.iter().map(|point| point.1).fold(f64::NEG_INFINITY, f64::max);
    (min_x, min_y, (max_x - min_x).max(0.0), (max_y - min_y).max(0.0))
}

pub fn user_delta(ctm: [f64; 6], dx: f64, dy: f64) -> (f64, f64) {
    let inverse = invert(ctm).unwrap_or(PDF_IDENTITY_MATRIX);
    (inverse[0] * dx + inverse[2] * dy, inverse[1] * dx + inverse[3] * dy)
}

pub fn unapply(matrix: [f64; 6], x: f64, y: f64) -> (f64, f64) {
    apply(invert(matrix).unwrap_or(PDF_IDENTITY_MATRIX), x, y)
}

pub fn apply(matrix: [f64; 6], x: f64, y: f64) -> (f64, f64) {
    (matrix[0] * x + matrix[2] * y + matrix[4], matrix[1] * x + matrix[3] * y + matrix[5])
}

pub fn multiply(left: [f64; 6], right: [f64; 6]) -> [f64; 6] {
    [
        left[0] * right[0] + left[2] * right[1],
        left[1] * right[0] + left[3] * right[1],
        left[0] * right[2] + left[2] * right[3],
        left[1] * right[2] + left[3] * right[3],
        left[0] * right[4] + left[2] * right[5] + left[4],
        left[1] * right[4] + left[3] * right[5] + left[5],
    ]
}

pub fn invert(matrix: [f64; 6]) -> Option<[f64; 6]> {
    let det = matrix[0] * matrix[3] - matrix[1] * matrix[2];
    if det.abs() < 1.0e-12 {
        return None;
    }
    let inv = 1.0 / det;
    Some([matrix[3] * inv, -matrix[1] * inv, -matrix[2] * inv, matrix[0] * inv, (matrix[2] * matrix[5] - matrix[3] * matrix[4]) * inv, (matrix[1] * matrix[4] - matrix[0] * matrix[5]) * inv])
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VectorSegment {
    Move((f64, f64)),
    Line((f64, f64)),
    Cubic { first: (f64, f64), second: (f64, f64), end: (f64, f64) },
    Close,
}

pub fn quad_contains(quad: [[f64; 2]; 4], x: f64, y: f64) -> bool {
    let [a, b, _, d] = quad;
    let area = (b[0] - a[0]) * (d[1] - a[1]) - (b[1] - a[1]) * (d[0] - a[0]);
    area.is_finite() && area != 0.0 && quad.iter().zip(quad.iter().cycle().skip(1)).all(|(a, b)| {
        let cross = (b[0] - a[0]) * (y - a[1]) - (b[1] - a[1]) * (x - a[0]);
        cross * area.signum() >= 0.0
    })
}

pub fn vector_bound_points(path: &[VectorSegment]) -> Vec<(f64, f64)> {
    let mut points = Vec::new();
    let mut current = (0.0, 0.0);
    let mut start = current;
    for &segment in path {
        match segment {
            VectorSegment::Move(to) => {
                start = to;
                current = to;
            }
            VectorSegment::Line(to) => current = to,
            VectorSegment::Cubic { first, second, end } => {
                let horizontal = cubic_extrema(current.0, first.0, second.0, end.0);
                let vertical = cubic_extrema(current.1, first.1, second.1, end.1);
                for t in horizontal.into_iter().chain(vertical) {
                    if t > 0.0 && t < 1.0 { points.push(cubic(current, first, second, end, t)); }
                }
                current = end;
            }
            VectorSegment::Close => current = start,
        }
        points.push(current);
    }
    points
}


fn cubic_extrema(p0: f64, p1: f64, p2: f64, p3: f64) -> [f64; 2] {
    let a = (p3 - p0) + 3.0 * (p1 - p2);
    let b = 2.0 * ((p0 - p1) + (p2 - p1));
    let c = p1 - p0;
    if a == 0.0 { return [-c / b, f64::NAN]; }
    let disc = b.mul_add(b, -4.0 * a * c);
    if disc < 0.0 { return [f64::NAN; 2]; }
    if disc == 0.0 { return [-0.5 * b / a, f64::NAN]; }
    let q = -0.5 * (b + disc.sqrt().copysign(b));
    [q / a, c / q]
}

#[cfg(test)]
mod curve_tests {
    use super::*;

    #[test]
    fn bounds_include_extrema_between_regular_samples() {
        let path = [VectorSegment::Move((0.0, 0.0)), VectorSegment::Cubic { first: (0.0, 9.0), second: (6.0, 0.0), end: (9.0, 0.0) }];
        let (x, y, width, height) = bounds(&vector_bound_points(&path));
        assert_eq!((x, y, width), (0.0, 0.0, 9.0));
        assert!((height - 4.0).abs() < 1e-12);
    }
}

type Point = (f64, f64);

fn segment_distance_sq(point: Point, a: Point, b: Point) -> f64 {
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let length = dx * dx + dy * dy;
    let t = if length == 0.0 { 0.0 } else { (((point.0 - a.0) * dx + (point.1 - a.1) * dy) / length).clamp(0.0, 1.0) };
    (point.0 - a.0 - t * dx).powi(2) + (point.1 - a.1 - t * dy).powi(2)
}

struct PathProbe {
    point: Point,
    radius: Option<f64>,
    winding: i32,
    stroke: bool,
}

impl PathProbe {
    fn edge(&mut self, a: Point, b: Point, stroke: bool) {
        if stroke && self.radius.is_some_and(|radius| segment_distance_sq(self.point, a, b) <= radius * radius) {
            self.stroke = true;
        }
        let (x, y) = self.point;
        if (a.1 > y) != (b.1 > y) && x < a.0 + (y - a.1) * (b.0 - a.0) / (b.1 - a.1) {
            self.winding += if b.1 > a.1 { 1 } else { -1 };
        }
    }

    fn curve(&mut self, [a, b, c, d]: [Point; 4], depth: u8) {
        let radius = self.radius.unwrap_or(0.0);
        let (x, y) = self.point;
        if a.1.max(b.1).max(c.1).max(d.1) < y - radius || a.1.min(b.1).min(c.1).min(d.1) > y + radius || a.0.max(b.0).max(c.0).max(d.0) < x - radius {
            return;
        }
        if depth == 12 || segment_distance_sq(b, a, d).max(segment_distance_sq(c, a, d)) <= 0.015625 {
            self.edge(a, d, true);
            return;
        }
        let half = |a: Point, b: Point| ((a.0 + b.0) * 0.5, (a.1 + b.1) * 0.5);
        let (ab, bc, cd) = (half(a, b), half(b, c), half(c, d));
        let (abc, bcd) = (half(ab, bc), half(bc, cd));
        let middle = half(abc, bcd);
        self.curve([a, ab, abc, middle], depth + 1);
        self.curve([middle, bcd, cd, d], depth + 1);
    }
}

/// 🖱️ Hit-test the page-space path in canvas pixels, including implicit fill closures and holes.
pub fn vector_contains(path: &[VectorSegment], viewport: PageViewport, point: (f64, f64), fill: bool, even_odd: bool, stroke_radius: Option<f64>) -> bool {
    let display = |p: Point| viewport.point(p.0, p.1);
    let mut probe = PathProbe { point, radius: stroke_radius, winding: 0, stroke: false };
    let mut current = display((0.0, 0.0));
    let mut first = current;
    for &segment in path {
        match segment {
            VectorSegment::Move(p) => {
                probe.edge(current, first, false);
                current = display(p);
                first = current;
            }
            VectorSegment::Line(p) => {
                let end = display(p);
                probe.edge(current, end, true);
                current = end;
            }
            VectorSegment::Cubic { first: b, second: c, end } => {
                let end = display(end);
                probe.curve([current, display(b), display(c), end], 0);
                current = end;
            }
            VectorSegment::Close => {
                probe.edge(current, first, true);
                current = first;
            }
        }
        if probe.stroke { return true; }
    }
    probe.edge(current, first, false);
    fill && if even_odd { probe.winding % 2 != 0 } else { probe.winding != 0 }
}

pub type ClipQuad = [(f64, f64); 4];

/// ✂️ Returns one convex four-sided path suitable for the canvas clip contract.
pub fn clip_quad(path: &[VectorSegment]) -> Option<ClipQuad> {
    let [VectorSegment::Move(a), VectorSegment::Line(b), VectorSegment::Line(c), VectorSegment::Line(d), tail @ ..] = path else { return None };
    if tail.iter().any(|segment| !matches!(segment, VectorSegment::Close)) { return None; }
    let quad = [*a, *b, *c, *d];
    let signs = std::array::from_fn::<_,4,_>(|index| {
        let [p, q, r] = [quad[index], quad[(index + 1) % 4], quad[(index + 2) % 4]];
        (q.0 - p.0) * (r.1 - q.1) - (q.1 - p.1) * (r.0 - q.0)
    });
    (signs.iter().any(|&cross| cross != 0.0) &&
        (signs.iter().all(|&cross| cross >= 0.0) || signs.iter().all(|&cross| cross <= 0.0))).then_some(quad)
}

/// 🪡️ Authored vector stroking state, measured in the paint operator's user space.
#[derive(Clone, Debug, PartialEq)]
pub struct VectorStroke {
    pub cap: &'static str,
    pub join: &'static str,
    pub dash: std::sync::Arc<[f64]>,
    pub phase: f64,
    pub miter_limit: f64,
}

impl Default for VectorStroke {
    fn default() -> Self {
        Self { cap: "butt", join: "miter", dash: Default::default(), phase: 0.0, miter_limit: 10.0 }
    }
}

impl VectorStroke {
    pub fn set_dash(&mut self, dash: &[f64], phase: f64) {
        let total: f64 = dash.iter().sum();
        if phase.is_finite() && (dash.is_empty() || (total.is_finite() && total > 0.0 && dash.iter().all(|&value| value >= 0.0))) {
            self.dash = dash.into();
            self.phase = phase;
        }
    }

    pub fn set_miter_limit(&mut self, limit: f64) {
        if limit.is_finite() && limit >= 1.0 { self.miter_limit = limit; }
    }
}

fn stroke_sub(a: Point, b: Point) -> Point { (a.0 - b.0, a.1 - b.1) }
fn stroke_add(a: Point, b: Point) -> Point { (a.0 + b.0, a.1 + b.1) }
fn stroke_scale(p: Point, scale: f64) -> Point { (p.0 * scale, p.1 * scale) }
fn stroke_cross(a: Point, b: Point) -> f64 { a.0 * b.1 - a.1 * b.0 }
fn stroke_dot(a: Point, b: Point) -> f64 { a.0 * b.0 + a.1 * b.1 }

fn stroke_unit(vector: Point) -> Point { stroke_scale(vector, 1.0 / vector.0.hypot(vector.1)) }

fn stroke_join_contains(point: Point, previous: Point, joint: Point, next: Point, radius: f64, style: &VectorStroke) -> bool {
    let delta = stroke_sub(point, joint);
    if style.join == "round" { return stroke_dot(delta, delta) <= radius * radius; }
    let incoming = stroke_unit(stroke_sub(joint, previous));
    let outgoing = stroke_unit(stroke_sub(next, joint));
    let turn = stroke_cross(incoming, outgoing);
    if turn.abs() < 1e-12 { return false; }
    let side = -radius.copysign(turn);
    let normal = |(x, y): Point| (-side * y, side * x);
    let (a, b) = (normal(incoming), normal(outgoing));
    let bisector = stroke_add(a, b);
    let bisector_length = stroke_dot(bisector, bisector);
    let miter = if style.join == "miter" && bisector_length > 0.0 && 2.0 * radius <= style.miter_limit * bisector_length.sqrt() {
        stroke_scale(bisector, 2.0 * radius * radius / bisector_length)
    } else {
        stroke_scale(bisector, 0.5)
    };
    quad_contains([(0.0, 0.0), a, miter, b].map(|(x, y)| [x, y]), delta.0, delta.1)
}

fn stroke_run_contains(points: &[Point], closed: bool, point: Point, radius: f64, style: &VectorStroke) -> bool {
    let count = points.len();
    if count < 2 { return false; }
    let edges = if closed { count } else { count - 1 };
    for index in 0..edges {
        let (a, b) = (points[index], points[(index + 1) % count]);
        let vector = stroke_sub(b, a);
        let length = vector.0.hypot(vector.1);
        let unit = stroke_scale(vector, 1.0 / length);
        let delta = stroke_sub(point, a);
        let along = stroke_dot(delta, unit);
        let lower = if !closed && index == 0 && style.cap == "square" { -radius } else { 0.0 };
        let upper = if !closed && index == edges - 1 && style.cap == "square" { length + radius } else { length };
        if along >= lower && along <= upper && stroke_cross(delta, unit).abs() <= radius { return true; }
    }
    if !closed && style.cap == "round" && [points[0], points[count - 1]].iter().any(|&end| {
        let vector = stroke_sub(point, end);
        stroke_dot(vector, vector) <= radius * radius
    }) { return true; }
    let joins = if closed { 0..count } else { 1..count - 1 };
    joins.into_iter().any(|i| stroke_join_contains(point, points[(i + count - 1) % count], points[i], points[(i + 1) % count], radius, style))
}

fn stroke_contour_contains(mut points: Vec<Point>, closed: bool, point: Point, radius: f64, style: &VectorStroke) -> bool {
    points.dedup();
    if closed && points.first() == points.last() { points.pop(); }
    if points.len() < 2 { return false; }
    if style.dash.is_empty() { return stroke_run_contains(&points, closed, point, radius, style); }
    if closed { points.push(points[0]); }
    let pattern = if style.dash.len() % 2 == 1 { style.dash.repeat(2) } else { style.dash.to_vec() };
    let period: f64 = pattern.iter().sum();
    if !period.is_finite() || period <= 0.0 { return stroke_run_contains(&points, false, point, radius, style); }
    let mut phase = style.phase.rem_euclid(period);
    let mut index = 0;
    while phase >= pattern[index] {
        phase -= pattern[index];
        index = (index + 1) % pattern.len();
    }
    let mut remaining = pattern[index] - phase;
    let mut run = if index % 2 == 0 { vec![points[0]] } else { Vec::new() };
    for edge in points.windows(2) {
        let delta = stroke_sub(edge[1], edge[0]);
        let length = delta.0.hypot(delta.1);
        let mut travelled = 0.0;
        while length - travelled > remaining {
            let advanced = travelled + remaining;
            if remaining > 0.0 && advanced == travelled { return false; }
            travelled = advanced;
            let next = stroke_add(edge[0], stroke_scale(delta, travelled / length));
            if index % 2 == 0 {
                if run.last() != Some(&next) { run.push(next); }
                if stroke_run_contains(&run, false, point, radius, style) { return true; }
                run.clear();
            } else { run.push(next); }
            index = (index + 1) % pattern.len();
            remaining = pattern[index];
        }
        remaining -= length - travelled;
        if index % 2 == 0 && run.last() != Some(&edge[1]) { run.push(edge[1]); }
    }
    stroke_run_contains(&run, false, point, radius, style)
}

fn flatten_stroke_cubic(path: &mut Vec<Point>, [a, b, c, d]: [Point; 4], tolerance_sq: f64, depth: usize) {
    if depth == 10 || segment_distance_sq(b, a, d).max(segment_distance_sq(c, a, d)) <= tolerance_sq {
        path.push(d);
        return;
    }
    let half = |a, b| stroke_scale(stroke_add(a, b), 0.5);
    let (ab, bc, cd) = (half(a, b), half(b, c), half(c, d));
    let (abc, bcd) = (half(ab, bc), half(bc, cd));
    let middle = half(abc, bcd);
    flatten_stroke_cubic(path, [a, ab, abc, middle], tolerance_sq, depth + 1);
    flatten_stroke_cubic(path, [middle, bcd, cd, d], tolerance_sq, depth + 1);
}

/// 🎯️ Tests the authored stroke outline after converting the displayed point back to paint-time user space.
pub fn stroke_contains(path: &[VectorSegment], user_space: [f64; 6], canvas: [f64; 6], point: Point, width: f64, style: &VectorStroke) -> bool {
    let Some(inverse) = invert(canvas) else { return false };
    let point = apply(inverse, point.0, point.1);
    let tolerance = 0.125 / (canvas[0].hypot(canvas[1]) + canvas[2].hypot(canvas[3])).max(1.0);
    let local = |(x, y)| apply(user_space, x, y);
    let mut points = Vec::new();
    for &command in path {
        match command {
            VectorSegment::Move(to) => {
                if stroke_contour_contains(std::mem::take(&mut points), false, point, width / 2.0, style) { return true; }
                points.push(local(to));
            }
            VectorSegment::Line(to) => points.push(local(to)),
            VectorSegment::Cubic { first, second, end } => {
                if let Some(&from) = points.last() {
                    flatten_stroke_cubic(&mut points, [from, local(first), local(second), local(end)], tolerance * tolerance, 0);
                }
            }
            VectorSegment::Close => {
                let first = points.first().copied();
                if stroke_contour_contains(std::mem::take(&mut points), true, point, width / 2.0, style) { return true; }
                points.extend(first);
            }
        }
    }
    stroke_contour_contains(points, false, point, width / 2.0, style)
}
