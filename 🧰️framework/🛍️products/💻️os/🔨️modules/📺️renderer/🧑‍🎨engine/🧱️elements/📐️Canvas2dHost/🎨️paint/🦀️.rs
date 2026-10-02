//! 🖊️ Rust twin of `🎨️paint/🟦️.ts`'s path geometry: what `drawSceneNode` hands the platform canvas for a `segments`
//! scene node — the SVG path semantics of its segments under the node `transform` and the host camera, its fill
//! tessellated into trapezoids under the `evenodd` / `nonzero` rule and its stroke into convex pieces (width, caps, joins
//! with the canvas miter limit, dash pattern). Pure screen-space geometry: `🎞️Scenes`' wgpu Canvas2d paint pushes every
//! piece as one triangle fan. React and this twin replay the shared corpus `🧫️fixtures/🧫️path-paint`
//! (`🧪️tests/🧪️path-paint`).

use serde::Deserialize;
use std::f64::consts::{FRAC_PI_2, TAU};

//#region 🧾️Record
/// ✏️ One `CanvasSceneNode.segments` entry in its `◻️2d` `PathSegment` wire form, read leniently: an unknown kind is
/// skipped (`pathSegmentsToSvgD` emits nothing for it) and a missing field ends the path where the SVG parser would.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ScenePathSegment {
    #[serde(default)]
    pub kind: String,
    #[serde(default)]
    pub to: Option<[f64; 2]>,
    #[serde(default)]
    pub ctrl: Option<[f64; 2]>,
    #[serde(default)]
    pub ctrl1: Option<[f64; 2]>,
    #[serde(default)]
    pub ctrl2: Option<[f64; 2]>,
    #[serde(default)]
    pub rx: Option<f64>,
    #[serde(default)]
    pub ry: Option<f64>,
    #[serde(default)]
    pub rotation: Option<f64>,
    #[serde(default)]
    pub large_arc: Option<bool>,
    #[serde(default)]
    pub sweep: Option<bool>,
}

/// 🪢️ The fill rule: `evenodd` unless the node says `nonzero` (`drawSceneNode`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum FillRule {
    EvenOdd,
    NonZero,
}

impl FillRule {
    pub(crate) fn of(rule: Option<&str>) -> Self {
        if rule == Some("nonzero") {
            Self::NonZero
        } else {
            Self::EvenOdd
        }
    }

    fn inside(self, winding: i32) -> bool {
        match self {
            Self::EvenOdd => winding % 2 != 0,
            Self::NonZero => winding != 0,
        }
    }
}

/// 🧢️ The stroke end cap; `butt` unless named.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum LineCap {
    Butt,
    Round,
    Square,
}

impl LineCap {
    pub(crate) fn of(cap: Option<&str>) -> Self {
        match cap {
            Some("round") => Self::Round,
            Some("square") => Self::Square,
            _ => Self::Butt,
        }
    }
}

/// 🦵️ The stroke join; `miter` unless named.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum LineJoin {
    Miter,
    Round,
    Bevel,
}

impl LineJoin {
    pub(crate) fn of(join: Option<&str>) -> Self {
        match join {
            Some("round") => Self::Round,
            Some("bevel") => Self::Bevel,
            _ => Self::Miter,
        }
    }
}
//#endregion 🧾️Record

//#region 📐️Affine
/// 📐️ A 2D affine map in canvas `transform` order `[a, b, c, d, e, f]`: `x' = a·x + c·y + e`, `y' = b·x + d·y + f`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Affine(pub [f64; 6]);

impl Affine {
    pub(crate) const IDENTITY: Self = Self([1.0, 0.0, 0.0, 1.0, 0.0, 0.0]);

    /// 🔁️ The node's own `transform`, read like `applySceneTransform`: only a list of at least six numbers applies.
    pub(crate) fn node(transform: Option<&[f64]>) -> Self {
        match transform {
            Some(m) if m.len() >= 6 => Self([m[0], m[1], m[2], m[3], m[4], m[5]]),
            _ => Self::IDENTITY,
        }
    }

    /// 🎥️ The host camera: world → screen for a canvas whose top-left corner is `(left, top)`, logical size
    /// `width × height`, centred on `(x, y)` at `zoom` (`worldToScreenLogical`).
    pub(crate) fn camera(x: f64, y: f64, zoom: f64, left: f64, top: f64, width: f64, height: f64) -> Self {
        Self([zoom, 0.0, 0.0, zoom, left + width * 0.5 - x * zoom, top + height * 0.5 - y * zoom])
    }

    /// ➡️ `self` first, then `next`.
    pub(crate) fn then(self, next: Self) -> Self {
        let [a, b, c, d, e, f] = self.0;
        let [na, nb, nc, nd, ne, nf] = next.0;
        Self([na * a + nc * b, nb * a + nd * b, na * c + nc * d, nb * c + nd * d, na * e + nc * f + ne, nb * e + nd * f + nf])
    }

    pub(crate) fn apply(&self, [x, y]: [f64; 2]) -> [f64; 2] {
        let [a, b, c, d, e, f] = self.0;
        [a * x + c * y + e, b * x + d * y + f]
    }

    /// ↩️ The inverse map, `None` when the map collapses the plane.
    pub(crate) fn inverse(&self) -> Option<Self> {
        let [a, b, c, d, e, f] = self.0;
        let det = a * d - b * c;
        (det != 0.0 && det.is_finite()).then(|| Self([d / det, -b / det, -c / det, a / det, (c * f - d * e) / det, (b * e - a * f) / det]))
    }

    /// 📏️ The factor a line width scales by: the geometric mean of the axes, `√|det|`.
    pub(crate) fn line_scale(&self) -> f64 {
        let [a, b, c, d, ..] = self.0;
        (a * d - b * c).abs().sqrt()
    }

    /// 🔭️ The largest stretch the map applies to any direction (its spectral norm).
    fn max_stretch(&self) -> f64 {
        let [a, b, c, d, ..] = self.0;
        let sum = a * a + b * b + c * c + d * d;
        let det = a * d - b * c;
        ((sum + (sum * sum - 4.0 * det * det).max(0.0).sqrt()) * 0.5).sqrt()
    }
}
//#endregion 📐️Affine

//#region ✏️Flatten
/// 🧵️ One flattened subpath in screen space: its points (no two consecutive equal) and whether a `close` ended it.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct Contour {
    pub points: Vec<[f64; 2]>,
    pub closed: bool,
}

/// 🎯️ The screen distance a flattened curve may stray from the true one.
pub(crate) const FLATNESS: f64 = 0.2;
const MAX_CURVE_STEPS: usize = 256;
const MAX_ARC_STEPS: usize = 512;

fn push_point(contour: &mut Contour, point: [f64; 2]) {
    if contour.points.last() != Some(&point) {
        contour.points.push(point);
    }
}

fn second_difference(a: [f64; 2], b: [f64; 2], c: [f64; 2]) -> f64 {
    (a[0] - 2.0 * b[0] + c[0]).hypot(a[1] - 2.0 * b[1] + c[1])
}

/// 🪜️ Wang's step count for a Bézier whose largest control second difference is `difference` (`factor` = d(d−1)/8).
fn curve_steps(difference: f64, factor: f64) -> usize {
    let steps = (factor * difference / FLATNESS).sqrt().ceil();
    if steps.is_finite() {
        (steps as usize).clamp(1, MAX_CURVE_STEPS)
    } else {
        1
    }
}

fn quad(contour: &mut Contour, [p0, p1, p2]: [[f64; 2]; 3]) {
    let steps = curve_steps(second_difference(p0, p1, p2), 0.25);
    for step in 1..=steps {
        let t = step as f64 / steps as f64;
        let u = 1.0 - t;
        push_point(contour, [u * u * p0[0] + 2.0 * u * t * p1[0] + t * t * p2[0], u * u * p0[1] + 2.0 * u * t * p1[1] + t * t * p2[1]]);
    }
}

fn cubic(contour: &mut Contour, [p0, p1, p2, p3]: [[f64; 2]; 4]) {
    let steps = curve_steps(second_difference(p0, p1, p2).max(second_difference(p1, p2, p3)), 0.75);
    for step in 1..=steps {
        let t = step as f64 / steps as f64;
        let u = 1.0 - t;
        let (w0, w1, w2, w3) = (u * u * u, 3.0 * u * u * t, 3.0 * u * t * t, t * t * t);
        push_point(contour, [w0 * p0[0] + w1 * p1[0] + w2 * p2[0] + w3 * p3[0], w0 * p0[1] + w1 * p1[1] + w2 * p2[1] + w3 * p3[1]]);
    }
}

/// 🌙️ One SVG elliptical arc from `from` to `to` (local units, `rotation` in degrees): the endpoint → centre conversion
/// with the radius correction of SVG 1.1 F.6.5/F.6.6, sampled in local units and mapped, so a transformed arc stays exact.
#[allow(clippy::too_many_arguments, reason = "one argument per SVG arc parameter")]
fn arc(contour: &mut Contour, map: &Affine, from: [f64; 2], to: [f64; 2], rx: f64, ry: f64, rotation: f64, large_arc: bool, sweep: bool) {
    if from == to {
        return;
    }
    let (mut rx, mut ry) = (rx.abs(), ry.abs());
    if rx == 0.0 || ry == 0.0 {
        push_point(contour, map.apply(to));
        return;
    }
    let (sin, cos) = rotation.to_radians().sin_cos();
    let (hx, hy) = ((from[0] - to[0]) * 0.5, (from[1] - to[1]) * 0.5);
    let (px, py) = (cos * hx + sin * hy, -sin * hx + cos * hy);
    let lambda = px * px / (rx * rx) + py * py / (ry * ry);
    if lambda > 1.0 {
        rx *= lambda.sqrt();
        ry *= lambda.sqrt();
    }
    let denominator = rx * rx * py * py + ry * ry * px * px;
    let mut root = if denominator > 0.0 { ((rx * rx * ry * ry - denominator) / denominator).max(0.0).sqrt() } else { 0.0 };
    if large_arc == sweep {
        root = -root;
    }
    let (cpx, cpy) = (root * rx * py / ry, -root * ry * px / rx);
    let (cx, cy) = (cos * cpx - sin * cpy + (from[0] + to[0]) * 0.5, sin * cpx + cos * cpy + (from[1] + to[1]) * 0.5);
    let theta = ((py - cpy) / ry).atan2((px - cpx) / rx);
    let mut delta = ((-py - cpy) / ry).atan2((-px - cpx) / rx) - theta;
    if sweep && delta < 0.0 {
        delta += TAU;
    }
    if !sweep && delta > 0.0 {
        delta -= TAU;
    }
    let radius = rx.max(ry) * map.max_stretch();
    let step = if radius > FLATNESS { 2.0 * (1.0 - FLATNESS / radius).acos() } else { FRAC_PI_2 };
    let steps = (delta.abs() / step).ceil();
    let steps = if steps.is_finite() { (steps as usize).clamp(1, MAX_ARC_STEPS) } else { 1 };
    for index in 1..steps {
        let (sin_t, cos_t) = (theta + delta * index as f64 / steps as f64).sin_cos();
        push_point(contour, map.apply([cx + rx * cos * cos_t - ry * sin * sin_t, cy + rx * sin * cos_t + ry * cos * sin_t]));
    }
    push_point(contour, map.apply(to));
}

/// ✏️ Flattens `segments` through `map` (node transform then camera) into screen-space subpaths with the SVG path
/// semantics `Path2D` parses them with: a path starts with a move (anything else before it ends the path), a segment after
/// a close starts a new subpath at the closed subpath's start, an unknown kind is skipped and a missing field ends the path.
pub(crate) fn flatten(segments: &[ScenePathSegment], map: &Affine) -> Vec<Contour> {
    let mut contours = Vec::new();
    let mut current: Option<Contour> = None;
    let mut cursor: Option<[f64; 2]> = None;
    let mut start = [0.0, 0.0];
    for segment in segments {
        match segment.kind.as_str() {
            "move" => {
                let Some(to) = segment.to else { break };
                contours.extend(current.take());
                current = Some(Contour { points: vec![map.apply(to)], closed: false });
                cursor = Some(to);
                start = to;
            }
            "close" => {
                if cursor.is_none() {
                    break;
                }
                if let Some(mut contour) = current.take() {
                    contour.closed = true;
                    contours.push(contour);
                }
                cursor = Some(start);
            }
            "line" | "quad" | "cubic" | "arc" => {
                let (Some(from), Some(to)) = (cursor, segment.to) else { break };
                let contour = current.get_or_insert_with(|| Contour { points: vec![map.apply(from)], closed: false });
                match (segment.kind.as_str(), segment.ctrl, segment.ctrl1, segment.ctrl2, segment.rx, segment.ry, segment.rotation) {
                    ("line", ..) => push_point(contour, map.apply(to)),
                    ("quad", Some(ctrl), ..) => quad(contour, [map.apply(from), map.apply(ctrl), map.apply(to)]),
                    ("cubic", _, Some(ctrl1), Some(ctrl2), ..) => cubic(contour, [map.apply(from), map.apply(ctrl1), map.apply(ctrl2), map.apply(to)]),
                    ("arc", _, _, _, Some(rx), Some(ry), Some(rotation)) => arc(contour, map, from, to, rx, ry, rotation, segment.large_arc.unwrap_or(false), segment.sweep.unwrap_or(false)),
                    _ => break,
                }
                cursor = Some(to);
            }
            _ => {}
        }
    }
    contours.extend(current);
    contours
}
//#endregion ✏️Flatten

//#region 🪣️Fill
/// 🟫️ One fill trapezoid in screen space: a horizontal top and bottom and the left and right x at each.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Trapezoid {
    pub top: f64,
    pub bottom: f64,
    pub top_left: f64,
    pub top_right: f64,
    pub bottom_left: f64,
    pub bottom_right: f64,
}

impl Trapezoid {
    /// 🔷️ Its corners clockwise from the top left — a convex polygon (two corners may coincide).
    pub(crate) fn corners(&self) -> [[f64; 2]; 4] {
        [[self.top_left, self.top], [self.top_right, self.top], [self.bottom_right, self.bottom], [self.bottom_left, self.bottom]]
    }

    /// ⚖️ The mean of its corners, where a gradient piece samples its colour.
    pub(crate) fn center(&self) -> [f64; 2] {
        [(self.top_left + self.top_right + self.bottom_left + self.bottom_right) * 0.25, (self.top + self.bottom) * 0.5]
    }

    /// ✂️ Cuts it into pieces at most `step` pixels tall and wide: horizontal cuts, then ruled cuts joining matching points
    /// of the top and the bottom, so every piece is a trapezoid again.
    pub(crate) fn pieces(&self, step: f64) -> Vec<Trapezoid> {
        let rows = ((self.bottom - self.top) / step).ceil().max(1.0) as usize;
        let mut pieces = Vec::new();
        for row in 0..rows {
            let (t0, t1) = (row as f64 / rows as f64, (row + 1) as f64 / rows as f64);
            let lerp = |a: f64, b: f64, t: f64| a + (b - a) * t;
            let (top, bottom) = (lerp(self.top, self.bottom, t0), lerp(self.top, self.bottom, t1));
            let (top_left, top_right) = (lerp(self.top_left, self.bottom_left, t0), lerp(self.top_right, self.bottom_right, t0));
            let (bottom_left, bottom_right) = (lerp(self.top_left, self.bottom_left, t1), lerp(self.top_right, self.bottom_right, t1));
            let columns = ((top_right - top_left).max(bottom_right - bottom_left) / step).ceil().max(1.0) as usize;
            for column in 0..columns {
                let (s0, s1) = (column as f64 / columns as f64, (column + 1) as f64 / columns as f64);
                pieces.push(Trapezoid { top, bottom, top_left: lerp(top_left, top_right, s0), top_right: lerp(top_left, top_right, s1), bottom_left: lerp(bottom_left, bottom_right, s0), bottom_right: lerp(bottom_left, bottom_right, s1) });
            }
        }
        pieces
    }
}

#[derive(Clone, Copy, Debug)]
struct Edge {
    top: [f64; 2],
    bottom: [f64; 2],
    winding: i32,
}

impl Edge {
    fn x_at(&self, y: f64) -> f64 {
        self.top[0] + (self.bottom[0] - self.top[0]) * (y - self.top[1]) / (self.bottom[1] - self.top[1])
    }
}

/// 🪣️ Tessellates the fill of `contours` (every subpath implicitly closed) under `rule` into trapezoids between the screen
/// rows `clip.0` and `clip.1`: a sweep over every vertex row, bands split where edges cross, spans walked by winding.
pub(crate) fn fill_trapezoids(contours: &[Contour], rule: FillRule, clip: (f64, f64)) -> Vec<Trapezoid> {
    let mut edges = Vec::new();
    for contour in contours.iter().filter(|contour| contour.points.len() >= 3) {
        let count = contour.points.len();
        for index in 0..count {
            let (a, b) = (contour.points[index], contour.points[(index + 1) % count]);
            if a[1] == b[1] || !(a[0].is_finite() && a[1].is_finite() && b[0].is_finite() && b[1].is_finite()) {
                continue;
            }
            edges.push(if a[1] < b[1] { Edge { top: a, bottom: b, winding: 1 } } else { Edge { top: b, bottom: a, winding: -1 } });
        }
    }
    if edges.is_empty() || clip.1 <= clip.0 {
        return Vec::new();
    }
    edges.sort_by(|a, b| a.top[1].total_cmp(&b.top[1]));
    let mut rows: Vec<f64> = edges.iter().flat_map(|edge| [edge.top[1], edge.bottom[1]]).filter(|row| *row > clip.0 && *row < clip.1).chain([clip.0, clip.1]).collect();
    rows.sort_by(f64::total_cmp);
    rows.dedup();
    let (mut next, mut active, mut out) = (0, Vec::new(), Vec::new());
    for band in rows.windows(2) {
        while next < edges.len() && edges[next].top[1] <= band[0] {
            active.push(edges[next]);
            next += 1;
        }
        active.retain(|edge| edge.bottom[1] > band[0]);
        if active.len() >= 2 {
            fill_band(&active, band[0], band[1], rule, &mut out);
        }
    }
    out
}

fn fill_band(active: &[Edge], top: f64, bottom: f64, rule: FillRule, out: &mut Vec<Trapezoid>) {
    let mut order: Vec<(f64, f64)> = active.iter().map(|edge| (edge.x_at(top), edge.x_at(bottom))).collect();
    order.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.total_cmp(&b.1)));
    let mut cuts = vec![top, bottom];
    if order.windows(2).any(|pair| pair[1].1 < pair[0].1) {
        for (index, left) in order.iter().enumerate() {
            for right in &order[index + 1..] {
                if right.1 < left.1 {
                    let (gap_top, gap_bottom) = (right.0 - left.0, right.1 - left.1);
                    cuts.push(top + (bottom - top) * gap_top / (gap_top - gap_bottom));
                }
            }
        }
        cuts.sort_by(f64::total_cmp);
        cuts.dedup();
    }
    for slab in cuts.windows(2).filter(|slab| slab[1] > slab[0]) {
        let middle = (slab[0] + slab[1]) * 0.5;
        let mut crossings: Vec<(f64, f64, f64, i32)> = active.iter().map(|edge| (edge.x_at(middle), edge.x_at(slab[0]), edge.x_at(slab[1]), edge.winding)).collect();
        crossings.sort_by(|a, b| a.0.total_cmp(&b.0));
        let (mut winding, mut entry) = (0, None);
        for (_, x_top, x_bottom, turn) in crossings {
            let was = rule.inside(winding);
            winding += turn;
            match (was, rule.inside(winding), entry) {
                (false, true, _) => entry = Some((x_top, x_bottom)),
                (true, false, Some((left_top, left_bottom))) => {
                    out.push(Trapezoid { top: slab[0], bottom: slab[1], top_left: left_top, top_right: x_top, bottom_left: left_bottom, bottom_right: x_bottom });
                    entry = None;
                }
                _ => {}
            }
        }
    }
}
//#endregion 🪣️Fill

//#region 🖊️Stroke
/// 🖊️ A stroke in screen pixels: width, cap, join and the dash pattern (empty = solid).
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct StrokeGeometry {
    pub width: f64,
    pub cap: LineCap,
    pub join: LineJoin,
    pub dash: Vec<f64>,
}

/// 📐️ The canvas default `miterLimit`: a miter longer than this many half widths becomes a bevel.
pub(crate) const MITER_LIMIT: f64 = 10.0;

fn sub(a: [f64; 2], b: [f64; 2]) -> [f64; 2] {
    [a[0] - b[0], a[1] - b[1]]
}

fn add(a: [f64; 2], b: [f64; 2]) -> [f64; 2] {
    [a[0] + b[0], a[1] + b[1]]
}

fn scale(a: [f64; 2], factor: f64) -> [f64; 2] {
    [a[0] * factor, a[1] * factor]
}

fn unit(a: [f64; 2]) -> [f64; 2] {
    scale(a, 1.0 / a[0].hypot(a[1]))
}

fn left_normal(direction: [f64; 2]) -> [f64; 2] {
    [-direction[1], direction[0]]
}

/// 🌗️ The points of the arc about `center` from `from` to `to` (offsets of length `radius`), turning the short way.
fn arc_fan(center: [f64; 2], from: [f64; 2], to: [f64; 2], radius: f64) -> Vec<[f64; 2]> {
    let start = from[1].atan2(from[0]);
    let mut sweep = to[1].atan2(to[0]) - start;
    if sweep > std::f64::consts::PI {
        sweep -= TAU;
    }
    if sweep < -std::f64::consts::PI {
        sweep += TAU;
    }
    let step = if radius > FLATNESS { 2.0 * (1.0 - FLATNESS / radius).acos() } else { FRAC_PI_2 };
    let steps = ((sweep.abs() / step).ceil() as usize).clamp(1, MAX_ARC_STEPS);
    (0..=steps).map(|index| start + sweep * index as f64 / steps as f64).map(|angle| add(center, [radius * angle.cos(), radius * angle.sin()])).collect()
}

/// ✂️ The dash pattern canvas `setLineDash` keeps: none when any value is negative or not finite or all are zero; an odd
/// list repeats once.
fn dash_pattern(dash: &[f64]) -> Option<Vec<f64>> {
    if dash.is_empty() || dash.iter().any(|value| !value.is_finite() || *value < 0.0) || dash.iter().sum::<f64>() <= 0.0 {
        return None;
    }
    Some(if dash.len() % 2 == 1 { dash.repeat(2) } else { dash.to_vec() })
}

/// 🪓️ The "on" runs of `contour` under `pattern`, restarting the pattern at the subpath's start.
fn dash_runs(contour: &Contour, pattern: &[f64]) -> Vec<Contour> {
    let mut points = contour.points.clone();
    if contour.closed && points.len() > 1 {
        points.push(points[0]);
    }
    let Some(&first) = points.first() else { return Vec::new() };
    let (mut runs, mut run, mut index, mut left, mut on) = (Vec::new(), vec![first], 0, pattern[0], true);
    for edge in points.windows(2) {
        let length = (edge[1][0] - edge[0][0]).hypot(edge[1][1] - edge[0][1]);
        let mut travelled = 0.0;
        while length - travelled > left {
            travelled += left;
            let point = add(edge[0], scale(sub(edge[1], edge[0]), travelled / length));
            if on {
                run.push(point);
                runs.push(Contour { points: std::mem::take(&mut run), closed: false });
            } else {
                run = vec![point];
            }
            on = !on;
            index = (index + 1) % pattern.len();
            left = pattern[index];
        }
        left -= length - travelled;
        if on {
            run.push(edge[1]);
        }
    }
    if on && run.len() >= 2 {
        runs.push(Contour { points: run, closed: false });
    }
    runs
}

fn join(previous: [f64; 2], point: [f64; 2], next: [f64; 2], half: f64, kind: LineJoin, out: &mut Vec<Vec<[f64; 2]>>) {
    let (incoming, outgoing) = (unit(sub(point, previous)), unit(sub(next, point)));
    let turn = incoming[0] * outgoing[1] - incoming[1] * outgoing[0];
    if turn.abs() < 1e-12 && incoming[0] * outgoing[0] + incoming[1] * outgoing[1] > 0.0 {
        return;
    }
    let side = if turn > 0.0 { -half } else { half };
    let (outer_in, outer_out) = (scale(left_normal(incoming), side), scale(left_normal(outgoing), side));
    match kind {
        LineJoin::Round => {
            let mut fan = vec![point];
            fan.extend(arc_fan(point, outer_in, outer_out, half));
            out.push(fan);
        }
        LineJoin::Miter | LineJoin::Bevel => {
            let bisector = add(outer_in, outer_out);
            let length = bisector[0] * bisector[0] + bisector[1] * bisector[1];
            if kind == LineJoin::Miter && length > 0.0 && 2.0 * half / length.sqrt() <= MITER_LIMIT {
                out.push(vec![point, add(point, outer_in), add(point, scale(bisector, 2.0 * half * half / length)), add(point, outer_out)]);
            } else {
                out.push(vec![point, add(point, outer_in), add(point, outer_out)]);
            }
        }
    }
}

fn cap(inner: [f64; 2], end: [f64; 2], half: f64, kind: LineCap, out: &mut Vec<Vec<[f64; 2]>>) {
    let direction = unit(sub(end, inner));
    let normal = scale(left_normal(direction), half);
    match kind {
        LineCap::Butt => {}
        LineCap::Square => {
            let reach = scale(direction, half);
            out.push(vec![add(end, normal), add(add(end, normal), reach), add(sub(end, normal), reach), sub(end, normal)]);
        }
        LineCap::Round => {
            let mut fan = arc_fan(end, normal, scale(direction, half), half);
            fan.extend(arc_fan(end, scale(direction, half), scale(normal, -1.0), half).into_iter().skip(1));
            out.push(fan);
        }
    }
}

fn stroke_run(contour: &Contour, half: f64, style: &StrokeGeometry, out: &mut Vec<Vec<[f64; 2]>>) {
    let mut path: Vec<[f64; 2]> = Vec::with_capacity(contour.points.len());
    for point in &contour.points {
        if path.last() != Some(point) {
            path.push(*point);
        }
    }
    let closed = contour.closed && path.len() > 2;
    if closed && path.first() == path.last() {
        path.pop();
    }
    let count = path.len();
    if count < 2 {
        return;
    }
    let edges = if closed { count } else { count - 1 };
    for index in 0..edges {
        let (a, b) = (path[index], path[(index + 1) % count]);
        let normal = scale(left_normal(unit(sub(b, a))), half);
        out.push(vec![add(a, normal), add(b, normal), sub(b, normal), sub(a, normal)]);
    }
    let joins = if closed { 0..count } else { 1..count - 1 };
    for index in joins {
        join(path[(index + count - 1) % count], path[index], path[(index + 1) % count], half, style.join, out);
    }
    if !closed {
        cap(path[1], path[0], half, style.cap, out);
        cap(path[count - 2], path[count - 1], half, style.cap, out);
    }
}

/// 🖊️ The stroke of `contours` as convex screen-space polygons: one quad per edge, one piece per join, the caps of open
/// runs, the dash pattern walked along every subpath.
pub(crate) fn stroke_polygons(contours: &[Contour], style: &StrokeGeometry) -> Vec<Vec<[f64; 2]>> {
    let half = style.width * 0.5;
    let mut out = Vec::new();
    if !(half > 0.0 && half.is_finite()) {
        return out;
    }
    let pattern = dash_pattern(&style.dash);
    for contour in contours {
        match &pattern {
            Some(pattern) => dash_runs(contour, pattern).iter().for_each(|run| stroke_run(run, half, style, &mut out)),
            None => stroke_run(contour, half, style, &mut out),
        }
    }
    out
}
//#endregion 🖊️Stroke

#[cfg(test)]
#[path = "../🧪️tests/🧪️path-paint/🦀️.rs"]
mod tests;
