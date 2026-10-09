//! 🕸️ Owned double-precision triangle meshes and solid builders.
//!
//! [`TriMesh`] stores positions, normals and counter-clockwise (outward) triangles; builders emit one vertex triple per triangle (flat shading), [`TriMesh::crease_normals`] smooths curved parts.
//! Measures use the divergence theorem, so they are exact for closed, outward-wound meshes: `volume = sum(a . (b x c)) / 6`.
//! Builders: [`extrude`] / [`extrude_loops`] (polygon with holes between two height planes), [`prism_between`] (two equal-length rings), [`sweep_profile`] (vertical profile swept along bulged path segments with mitered corners), [`revolve_profile`] (profile turned about the Z axis).
//! Hand the result to a renderer via [`TriMesh::positions_f32`], [`TriMesh::normals_f32`], [`TriMesh::indices_flat`].

use crate::bulge::BulgeSeg;
use crate::loops::{self, Vertex};
use crate::placement::{Affine3, ZPlane};
use crate::triangulation::triangulate;
use crate::vector::{add3, cross, cross3, dot3, length3, normalize3, perp, scale3, sub3, Xyz};
use crate::{Point, Vec2};
use std::collections::BTreeMap;

/// 🕸️ Indexed triangle mesh in metres; triangles are counter-clockwise seen from outside.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TriMesh {
    pub positions: Vec<Xyz>,
    pub normals: Vec<Xyz>,
    pub indices: Vec<[u32; 3]>,
}

fn key(p: Xyz) -> [i64; 3] {
    [(p[0] / 1e-9).round() as i64, (p[1] / 1e-9).round() as i64, (p[2] / 1e-9).round() as i64]
}

impl TriMesh {
    /// 🆕️ Empty mesh.
    pub fn new() -> Self {
        Self::default()
    }

    /// 🔢️ Number of vertices.
    pub fn vertex_count(&self) -> usize {
        self.positions.len()
    }

    /// 🔢️ Number of triangles.
    pub fn triangle_count(&self) -> usize {
        self.indices.len()
    }

    /// 🔺️ Corner positions of triangle `i`.
    pub fn triangle(&self, i: usize) -> [Xyz; 3] {
        let t = self.indices[i];
        [self.positions[t[0] as usize], self.positions[t[1] as usize], self.positions[t[2] as usize]]
    }

    /// ➕️ Adds a triangle with its own three vertices and the flat normal; zero-area triangles are skipped.
    pub fn push_triangle(&mut self, a: Xyz, b: Xyz, c: Xyz) {
        let n = cross3(sub3(b, a), sub3(c, a));
        if length3(n) <= 1e-18 {
            return;
        }
        let n = normalize3(n);
        let base = self.positions.len() as u32;
        self.positions.extend([a, b, c]);
        self.normals.extend([n, n, n]);
        self.indices.push([base, base + 1, base + 2]);
    }

    /// ➕️ Adds the quad `a b c d` (counter-clockwise) as two triangles.
    pub fn push_quad(&mut self, a: Xyz, b: Xyz, c: Xyz, d: Xyz) {
        self.push_triangle(a, b, c);
        self.push_triangle(a, c, d);
    }

    /// 🔗️ Appends another mesh.
    pub fn append(&mut self, other: &Self) {
        let base = self.positions.len() as u32;
        self.positions.extend_from_slice(&other.positions);
        self.normals.extend_from_slice(&other.normals);
        self.indices.extend(other.indices.iter().map(|t| [t[0] + base, t[1] + base, t[2] + base]));
    }

    /// 🔁️ Image under `m`; mirrors keep outward winding.
    pub fn transformed(&self, m: &Affine3) -> Self {
        let mirrored = m.determinant() < 0.0;
        Self {
            positions: self.positions.iter().map(|&p| m.apply_point(p)).collect(),
            normals: self.normals.iter().map(|&n| m.apply_normal(n)).collect(),
            indices: self.indices.iter().map(|t| if mirrored { [t[0], t[2], t[1]] } else { *t }).collect(),
        }
    }

    /// ➡️ Translated copy.
    pub fn translated(&self, t: Xyz) -> Self {
        self.transformed(&Affine3::translation(t))
    }

    /// 📦️ Signed volume (positive for outward-wound closed meshes).
    pub fn signed_volume(&self) -> f64 {
        (0..self.indices.len())
            .map(|i| {
                let [a, b, c] = self.triangle(i);
                dot3(a, cross3(b, c)) / 6.0
            })
            .sum()
    }

    /// 📦️ Absolute volume in cubic metres.
    pub fn volume(&self) -> f64 {
        self.signed_volume().abs()
    }

    /// 📐️ Total surface area.
    pub fn surface_area(&self) -> f64 {
        (0..self.indices.len())
            .map(|i| {
                let [a, b, c] = self.triangle(i);
                length3(cross3(sub3(b, a), sub3(c, a))) / 2.0
            })
            .sum()
    }

    /// ⚖️ Centroid of the enclosed volume (vertex mean for zero-volume meshes); `None` for an empty mesh.
    pub fn volume_centroid(&self) -> Option<Xyz> {
        if self.positions.is_empty() {
            return None;
        }
        let (mut sum, mut total) = ([0.0; 3], 0.0);
        for i in 0..self.indices.len() {
            let [a, b, c] = self.triangle(i);
            let v = dot3(a, cross3(b, c)) / 6.0;
            sum = add3(sum, scale3(add3(add3(a, b), c), v / 4.0));
            total += v;
        }
        if total.abs() > 1e-18 {
            return Some(scale3(sum, 1.0 / total));
        }
        let n = self.positions.len() as f64;
        Some(scale3(self.positions.iter().fold([0.0; 3], |s, &p| add3(s, p)), 1.0 / n))
    }

    /// 🧮️ Axis-aligned bounds `(min, max)`; `None` for an empty mesh.
    pub fn bounds(&self) -> Option<(Xyz, Xyz)> {
        let first = *self.positions.first()?;
        Some(self.positions.iter().fold((first, first), |(lo, hi), p| ([lo[0].min(p[0]), lo[1].min(p[1]), lo[2].min(p[2])], [hi[0].max(p[0]), hi[1].max(p[1]), hi[2].max(p[2])])))
    }

    /// 🧵️ `true` when every directed edge (matched by position within 1 nm) has exactly one opposite edge, i.e. the surface is closed and consistently wound.
    pub fn is_watertight(&self) -> bool {
        let mut edges: BTreeMap<([i64; 3], [i64; 3]), i32> = BTreeMap::new();
        for t in &self.indices {
            let k = [key(self.positions[t[0] as usize]), key(self.positions[t[1] as usize]), key(self.positions[t[2] as usize])];
            for e in 0..3 {
                let (a, b) = (k[e], k[(e + 1) % 3]);
                if a != b {
                    *edges.entry((a, b)).or_insert(0) += 1;
                }
            }
        }
        !edges.is_empty() && edges.iter().all(|(&(a, b), &n)| n == 1 && edges.get(&(b, a)) == Some(&1))
    }

    /// 🔓️ Copy with one vertex triple per triangle (needed before per-corner normal editing).
    pub fn unwelded(&self) -> Self {
        let mut out = Self::new();
        for t in &self.indices {
            let base = out.positions.len() as u32;
            for &c in t {
                out.positions.push(self.positions[c as usize]);
                out.normals.push(self.normals.get(c as usize).copied().unwrap_or([0.0; 3]));
            }
            out.indices.push([base, base + 1, base + 2]);
        }
        out
    }

    /// 🔒️ Copy with vertices at the same position (within 1 nm) merged, keeping the first normal; the compact topology that exporters and half-edge consumers expect.
    pub fn welded(&self) -> Self {
        let mut out = Self::new();
        let mut seen: BTreeMap<[i64; 3], u32> = BTreeMap::new();
        let mut remap = Vec::with_capacity(self.positions.len());
        for (i, &p) in self.positions.iter().enumerate() {
            let next = out.positions.len() as u32;
            let id = *seen.entry(key(p)).or_insert_with(|| {
                out.positions.push(p);
                out.normals.push(self.normals.get(i).copied().unwrap_or([0.0; 3]));
                next
            });
            remap.push(id);
        }
        out.indices = self.indices.iter().map(|t| t.map(|c| remap[c as usize])).filter(|t| t[0] != t[1] && t[1] != t[2] && t[0] != t[2]).collect();
        out
    }

    /// 🌗️ Smooths normals across edges whose faces differ by at most `max_angle` radians (area-weighted), leaving hard edges sharp.
    pub fn crease_normals(&self, max_angle: f64) -> Self {
        let mut out = self.unwelded();
        let faces: Vec<Xyz> = (0..out.indices.len())
            .map(|i| {
                let [a, b, c] = out.triangle(i);
                cross3(sub3(b, a), sub3(c, a))
            })
            .collect();
        let mut groups: BTreeMap<[i64; 3], Vec<usize>> = BTreeMap::new();
        for (i, t) in out.indices.iter().enumerate() {
            for &c in t {
                groups.entry(key(out.positions[c as usize])).or_default().push(i);
            }
        }
        let threshold = max_angle.cos();
        for (i, t) in out.indices.clone().iter().enumerate() {
            let own = normalize3(faces[i]);
            for &c in t {
                let mut sum = [0.0; 3];
                for &j in &groups[&key(out.positions[c as usize])] {
                    if dot3(own, normalize3(faces[j])) >= threshold {
                        sum = add3(sum, faces[j]);
                    }
                }
                out.normals[c as usize] = normalize3(sum);
            }
        }
        out
    }

    /// 📤️ Flat `f32` positions for rendering.
    pub fn positions_f32(&self) -> Vec<f32> {
        self.positions.iter().flat_map(|p| p.map(|v| v as f32)).collect()
    }

    /// 📤️ Flat `f32` normals for rendering.
    pub fn normals_f32(&self) -> Vec<f32> {
        self.normals.iter().flat_map(|p| p.map(|v| v as f32)).collect()
    }

    /// 📤️ Flat triangle index list for rendering.
    pub fn indices_flat(&self) -> Vec<u32> {
        self.indices.iter().flatten().copied().collect()
    }
}

fn oriented(points: &[Point], want_ccw: bool) -> Vec<Point> {
    let n = points.len();
    let area: f64 = (0..n).map(|i| cross(points[i] - Point::ZERO, points[(i + 1) % n] - Point::ZERO)).sum();
    let mut ring = points.to_vec();
    if (area > 0.0) != want_ccw {
        ring.reverse();
    }
    ring
}

fn dedup_ring(points: &[Point]) -> Vec<Point> {
    let mut out: Vec<Point> = Vec::with_capacity(points.len());
    for &p in points {
        if out.last().map_or(true, |q| (p - *q).hypot() > 1e-12) {
            out.push(p);
        }
    }
    while out.len() > 1 && (out[0] - out[out.len() - 1]).hypot() <= 1e-12 {
        out.pop();
    }
    out
}

/// 🧱️ Prism over a polygon with holes between two height planes: bottom cap, top cap and vertical side walls; rings may have either orientation.
/// The top plane should lie above the bottom plane everywhere (otherwise the volume is signed).
pub fn extrude(outer: &[Point], holes: &[Vec<Point>], bottom: ZPlane, top: ZPlane) -> TriMesh {
    let outer = dedup_ring(&oriented(&dedup_ring(outer), true));
    let holes: Vec<Vec<Point>> = holes.iter().map(|h| dedup_ring(&oriented(&dedup_ring(h), false))).filter(|h| h.len() >= 3).collect();
    let mut mesh = TriMesh::new();
    if outer.len() < 3 {
        return mesh;
    }
    let at = |plane: &ZPlane, p: Point| [p.x, p.y, plane.at(p)];
    let triangulation = triangulate(&outer, &holes);
    for t in &triangulation.triangles {
        let [a, b, c] = t.map(|i| triangulation.vertices[i as usize]);
        mesh.push_triangle(at(&bottom, a), at(&bottom, c), at(&bottom, b));
        mesh.push_triangle(at(&top, a), at(&top, b), at(&top, c));
    }
    for ring in std::iter::once(&outer).chain(holes.iter()) {
        for i in 0..ring.len() {
            let (a, b) = (ring[i], ring[(i + 1) % ring.len()]);
            mesh.push_quad(at(&bottom, a), at(&bottom, b), at(&top, b), at(&top, a));
        }
    }
    mesh
}

/// 🧱️ [`extrude`] over bulged loops: arcs are flattened within `tolerance` and the curved walls get smooth normals.
pub fn extrude_loops(outer: &[Vertex], holes: &[Vec<Vertex>], tolerance: f64, bottom: ZPlane, top: ZPlane) -> TriMesh {
    let curved = outer.iter().chain(holes.iter().flatten()).any(|v| v.bulge != 0.0);
    let mesh = extrude(&loops::flatten(outer, tolerance), &holes.iter().map(|h| loops::flatten(h, tolerance)).collect::<Vec<_>>(), bottom, top);
    if curved {
        mesh.crease_normals(35f64.to_radians())
    } else {
        mesh
    }
}

fn newell(ring: &[Xyz]) -> Xyz {
    let n = ring.len();
    (0..n).fold([0.0; 3], |s, i| add3(s, cross3(ring[i], ring[(i + 1) % n])))
}

fn mean(ring: &[Xyz]) -> Xyz {
    scale3(ring.iter().fold([0.0; 3], |s, &p| add3(s, p)), 1.0 / ring.len() as f64)
}

/// 🧱️ Closed prism between two rings of equal length (`lower[i]` pairs with `upper[i]`); rings may have either orientation, caps are triangulated in the plane of the lower ring.
pub fn prism_between(lower: &[Xyz], upper: &[Xyz]) -> TriMesh {
    let mut mesh = TriMesh::new();
    if lower.len() < 3 || lower.len() != upper.len() {
        return mesh;
    }
    let n = lower.len();
    let normal = newell(lower);
    let mut order: Vec<usize> = (0..n).collect();
    if dot3(normal, sub3(mean(upper), mean(lower))) < 0.0 {
        order.reverse();
    }
    let (lo, up): (Vec<Xyz>, Vec<Xyz>) = (order.iter().map(|&i| lower[i]).collect(), order.iter().map(|&i| upper[i]).collect());
    let axis_n = normalize3(newell(&lo));
    let seed = if axis_n[0].abs() < 0.9 { [1.0, 0.0, 0.0] } else { [0.0, 1.0, 0.0] };
    let u = normalize3(cross3(axis_n, seed));
    let v = cross3(axis_n, u);
    let flat: Vec<Point> = lo.iter().map(|&p| Point::new(dot3(p, u), dot3(p, v))).collect();
    let triangulation = triangulate(&flat, &[]);
    for t in &triangulation.triangles {
        let [a, b, c] = t.map(|i| i as usize);
        mesh.push_triangle(lo[a], lo[c], lo[b]);
        mesh.push_triangle(up[a], up[b], up[c]);
    }
    for i in 0..n {
        let j = (i + 1) % n;
        mesh.push_quad(lo[i], lo[j], up[j], up[i]);
    }
    mesh
}

/// 🧱️ One developed face of a wall-like solid: an outline with holes in `(s, z)` coordinates, `s` along the axis and `z` up from the base.
#[derive(Clone, Debug, PartialEq)]
pub struct ElevationFace {
    pub outer: Vec<Point>,
    pub holes: Vec<Vec<Point>>,
}

#[derive(Clone, Copy)]
struct Pair {
    l: Point,
    r: Point,
}

fn mix(a: Pair, b: Pair, f: f64) -> Pair {
    let at = |p: Point, q: Point| Point::new(p.x + (q.x - p.x) * f, p.y + (q.y - p.y) * f);
    Pair { l: at(a.l, b.l), r: at(a.r, b.r) }
}

fn cross_at(a: Pair, b: Pair, s: f64) -> Pair {
    let (a, b) = if (a.l.x, a.l.y) <= (b.l.x, b.l.y) { (a, b) } else { (b, a) };
    mix(a, b, (s - a.l.x) / (b.l.x - a.l.x))
}

fn cut_ring(ring: &[Pair], delta: f64) -> Vec<Pair> {
    let mut out = Vec::new();
    for i in 0..ring.len() {
        let (a, b) = (ring[i], ring[(i + 1) % ring.len()]);
        out.push(a);
        if delta.is_finite() && a.l.x != b.l.x {
            let (lo, hi) = (a.l.x.min(b.l.x), a.l.x.max(b.l.x));
            let mut cuts: Vec<Pair> = Vec::new();
            let mut j = (lo / delta).floor() as i64 + 1;
            while (j as f64) * delta < hi - 1e-12 * delta {
                if (j as f64) * delta > lo + 1e-12 * delta {
                    cuts.push(cross_at(a, b, j as f64 * delta));
                }
                j += 1;
            }
            if a.l.x > b.l.x {
                cuts.reverse();
            }
            out.extend(cuts);
        }
    }
    out
}

fn clip(polygon: &[Pair], bound: f64, keep_above: bool) -> Vec<Pair> {
    let inside = |p: &Pair| if keep_above { p.l.x >= bound } else { p.l.x <= bound };
    let mut out = Vec::new();
    for i in 0..polygon.len() {
        let (a, b) = (polygon[i], polygon[(i + 1) % polygon.len()]);
        match (inside(&a), inside(&b)) {
            (true, true) => out.push(b),
            (true, false) => out.push(cross_at(a, b, bound)),
            (false, true) => {
                out.push(cross_at(a, b, bound));
                out.push(b);
            }
            (false, false) => {}
        }
    }
    out
}

fn slice_triangle(t: [Pair; 3], delta: f64, out: &mut Vec<[Pair; 3]>) {
    let (lo, hi) = t.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), p| (lo.min(p.l.x), hi.max(p.l.x)));
    let (j0, j1) = ((lo / delta).floor() as i64, (hi / delta).ceil() as i64);
    if j1 - j0 <= 1 {
        out.push(t);
        return;
    }
    for j in j0..j1 {
        let piece = clip(&clip(&t, j as f64 * delta, true), (j + 1) as f64 * delta, false);
        let mut piece: Vec<Pair> = piece.into_iter().fold(Vec::new(), |mut v: Vec<Pair>, p| {
            if v.last().map_or(true, |q| (q.l - p.l).hypot() > 1e-12) {
                v.push(p);
            }
            v
        });
        while piece.len() > 1 && (piece[0].l - piece[piece.len() - 1].l).hypot() <= 1e-12 {
            piece.pop();
        }
        for k in 1..piece.len().saturating_sub(1) {
            let tri = [piece[0], piece[k], piece[k + 1]];
            if cross(tri[1].l - tri[0].l, tri[2].l - tri[0].l) > 1e-18 {
                out.push(tri);
            }
        }
    }
}

/// 🧱️ Wall-like solid between two developed faces: `left` is mapped onto `left_curve`, `right` onto `right_curve` (point `(s, z)` goes to `curve.point_at(s / axis_length)` at height `base_z + z`).
/// Both faces must have the same ring structure (same vertex counts, vertex `i` of one pairs with vertex `i` of the other); they may differ in the `s` of their end vertices, which yields slanted (mitered) ends.
/// Holes become openings with reveals (the ring edges are joined by side walls), outline notches become door-like cut-outs.
/// On curved carriers the faces are sliced at `s` grid lines so that no edge spans more than the chord `tolerance` allows; the result is crack-free and gets smooth normals on the curved parts.
/// Rings must be counter-clockwise (outer) and clockwise (holes) when `s` points right and `z` up, i.e. as seen from the right-hand side of the direction of travel; the left face then faces left and the right face faces right.
pub fn extrude_between_faces(left: &ElevationFace, right: &ElevationFace, left_curve: &BulgeSeg, right_curve: &BulgeSeg, axis_length: f64, base_z: f64, tolerance: f64) -> TriMesh {
    let mut mesh = TriMesh::new();
    let same = left.outer.len() == right.outer.len() && left.holes.len() == right.holes.len() && left.holes.iter().zip(&right.holes).all(|(a, b)| a.len() == b.len());
    if !same || left.outer.len() < 3 || axis_length <= 0.0 {
        return mesh;
    }
    let widest = [left_curve, right_curve].iter().filter(|c| !c.is_line()).map(|c| c.radius()).fold(0.0f64, f64::max);
    let sweep = left_curve.sweep().abs().max(right_curve.sweep().abs());
    let delta = if widest > 0.0 && sweep > 0.0 {
        let step = (2.0 * (1.0 - (tolerance / widest).clamp(1e-12, 1.0)).acos()).max(1e-6);
        let cells = ((axis_length * sweep / step).ceil()).clamp(1.0, (1 << 14) as f64);
        axis_length / cells
    } else {
        f64::INFINITY
    };
    let lift = |curve: &BulgeSeg, p: Point| {
        let q = curve.point_at(p.x / axis_length);
        [q.x, q.y, base_z + p.y]
    };
    let mut raw: Vec<Vec<Pair>> = Vec::new();
    for (k, (a, b)) in std::iter::once((&left.outer, &right.outer)).chain(left.holes.iter().zip(&right.holes)).enumerate() {
        let mut pairs: Vec<Pair> = a.iter().zip(b).map(|(&l, &r)| Pair { l, r }).collect();
        if (loops::signed_area(&loops::from_polygon(a)) > 0.0) != (k == 0) {
            pairs.reverse();
        }
        raw.push(pairs);
    }
    let rings: Vec<Vec<Pair>> = raw.iter().map(|r| cut_ring(r, delta)).collect();
    let to_points = |ring: &Vec<Pair>| ring.iter().map(|p| p.l).collect::<Vec<Point>>();
    let triangulation = triangulate(&to_points(&raw[0]), &raw[1..].iter().map(to_points).collect::<Vec<_>>());
    let vertices: Vec<Pair> = raw.iter().flatten().copied().collect();
    let mut faces: Vec<[Pair; 3]> = Vec::new();
    for t in &triangulation.triangles {
        let tri = t.map(|i| vertices[i as usize]);
        if delta.is_finite() {
            slice_triangle(tri, delta, &mut faces);
        } else {
            faces.push(tri);
        }
    }
    for [a, b, c] in &faces {
        mesh.push_triangle(lift(left_curve, a.l), lift(left_curve, c.l), lift(left_curve, b.l));
        mesh.push_triangle(lift(right_curve, a.r), lift(right_curve, b.r), lift(right_curve, c.r));
    }
    for ring in &rings {
        let n = ring.len();
        for i in 0..n {
            let (p, q) = (ring[i], ring[(i + 1) % n]);
            mesh.push_quad(lift(left_curve, p.l), lift(left_curve, q.l), lift(right_curve, q.r), lift(right_curve, p.r));
        }
    }
    if delta.is_finite() {
        mesh.crease_normals(35f64.to_radians())
    } else {
        mesh
    }
}

struct Station {
    point: Point,
    lateral: Vec2,
    scale: f64,
    fraction: f64,
}

fn stations(path: &[BulgeSeg], tolerance: f64) -> Vec<Station> {
    let total: f64 = path.iter().map(BulgeSeg::length).sum();
    let fraction = |before: f64, seg: &BulgeSeg, t: f64| if total > 0.0 { (before + seg.length() * t) / total } else { 0.0 };
    let mut out: Vec<Station> = Vec::new();
    let mut before = 0.0;
    for (k, seg) in path.iter().enumerate() {
        if k == 0 {
            out.push(Station { point: seg.start, lateral: perp(seg.tangent_at(0.0)), scale: 1.0, fraction: 0.0 });
        } else {
            let (n_in, n_out) = (perp(path[k - 1].tangent_at(1.0)), perp(seg.tangent_at(0.0)));
            let sum = n_in + n_out;
            let m = if sum.hypot() > 1e-9 { sum / sum.hypot() } else { n_in };
            let last = out.len() - 1;
            out[last] = Station { point: seg.start, lateral: m, scale: 1.0 / m.dot(n_in).max(0.2), fraction: fraction(before, seg, 0.0) };
        }
        let mut points = Vec::new();
        seg.flatten_into(tolerance, &mut points);
        let count = points.len();
        for (i, p) in points.into_iter().enumerate() {
            let t = if i + 1 == count { 1.0 } else { seg.param_of(p) };
            out.push(Station { point: p, lateral: perp(seg.tangent_at(t)), scale: 1.0, fraction: fraction(before, seg, t) });
        }
        before += seg.length();
    }
    out
}

/// 🧱️ Solid swept from a vertical cross-section along a path of bulged segments (consecutive segments must touch).
/// Profile coordinates are `(u, v)`: `u` horizontal to the left of the direction of travel, `v` up from `base_z`; rings may have either orientation.
/// Corners between segments are mitered, arcs are flattened within `tolerance`, both ends are capped. Curved parts get smooth normals.
pub fn sweep_profile(outer: &[Point], holes: &[Vec<Point>], path: &[BulgeSeg], base_z: f64, tolerance: f64) -> TriMesh {
    sweep_profile_ramped(outer, holes, path, base_z, 0.0, tolerance)
}

/// 📐️ [`sweep_profile`] along a path that climbs linearly with its arc length by `rise` metres (negative descends): the reference line of the profile starts at `base_z` and ends at `base_z + rise`, and the cross-section
/// is turned about the horizontal lateral axis so that it stays perpendicular to the inclined path (the profile `v` runs along the up direction of the slope, not along the vertical). Corners and caps behave like [`sweep_profile`]; the horizontal
/// sweep is unchanged, so with `rise = 0` the result is identical.
pub fn sweep_profile_ramped(outer: &[Point], holes: &[Vec<Point>], path: &[BulgeSeg], base_z: f64, rise: f64, tolerance: f64) -> TriMesh {
    let outer = dedup_ring(&oriented(&dedup_ring(outer), true));
    let holes: Vec<Vec<Point>> = holes.iter().map(|h| dedup_ring(&oriented(&dedup_ring(h), false))).filter(|h| h.len() >= 3).collect();
    let mut mesh = TriMesh::new();
    if outer.len() < 3 || path.is_empty() {
        return mesh;
    }
    let st = stations(path, tolerance);
    let run: f64 = path.iter().map(BulgeSeg::length).sum();
    let slope = if run > 0.0 { rise.atan2(run) } else { 0.0 };
    let (sine, cosine) = slope.sin_cos();
    let place = |s: &Station, p: Point| {
        let ahead = Vec2::new(s.lateral.y, -s.lateral.x);
        let q = s.point + s.lateral * (p.x * s.scale) - ahead * (p.y * sine);
        [q.x, q.y, base_z + rise * s.fraction + p.y * cosine]
    };
    let rings: Vec<&Vec<Point>> = std::iter::once(&outer).chain(holes.iter()).collect();
    let triangulation = triangulate(&outer, &holes);
    let (first, last) = (&st[0], &st[st.len() - 1]);
    for t in &triangulation.triangles {
        let [a, b, c] = t.map(|i| triangulation.vertices[i as usize]);
        mesh.push_triangle(place(first, a), place(first, c), place(first, b));
        mesh.push_triangle(place(last, a), place(last, b), place(last, c));
    }
    for w in st.windows(2) {
        for ring in &rings {
            for i in 0..ring.len() {
                let (a, b) = (ring[i], ring[(i + 1) % ring.len()]);
                mesh.push_quad(place(&w[0], a), place(&w[0], b), place(&w[1], b), place(&w[1], a));
            }
        }
    }
    mesh.crease_normals(35f64.to_radians())
}

/// 🌀 Solid of revolution: a profile in the half plane `(r, z)` (`x` is the distance from the axis, `y` the height along it, rings of either orientation, no vertex left of the axis) turned about the Z axis through
/// `sweep` radians (clamped to a full turn). A partial sweep is capped at both ends; arcs are flattened within `tolerance`. Curved parts get smooth normals. An invalid profile or a non-positive sweep yields an empty mesh.
pub fn revolve_profile(profile: &[Point], sweep: f64, tolerance: f64) -> TriMesh {
    let ring = dedup_ring(&oriented(&dedup_ring(profile), true));
    let mut mesh = TriMesh::new();
    if ring.len() < 3 || !(sweep > 0.0) || !sweep.is_finite() || ring.iter().any(|p| !(p.x >= -1e-9) || !p.y.is_finite()) {
        return mesh;
    }
    let sweep = sweep.min(std::f64::consts::TAU);
    let full = sweep >= std::f64::consts::TAU - 1e-12;
    let reach = ring.iter().map(|p| p.x).fold(0.0, f64::max);
    let step = if reach > tolerance { 2.0 * (1.0 - tolerance / reach).acos() } else { sweep };
    let steps = ((sweep / step.max(1e-6)).ceil() as usize).clamp(if full { 8 } else { 1 }, 4096);
    let place = |p: Point, turn: f64| [p.x.max(0.0) * turn.cos(), p.x.max(0.0) * turn.sin(), p.y];
    let angle = |i: usize| sweep * i as f64 / steps as f64;
    for i in 0..steps {
        let (from, to) = (angle(i), angle(i + 1));
        for j in 0..ring.len() {
            let (a, b) = (ring[j], ring[(j + 1) % ring.len()]);
            mesh.push_quad(place(a, from), place(a, to), place(b, to), place(b, from));
        }
    }
    if !full {
        let triangulation = triangulate(&ring, &[]);
        for t in &triangulation.triangles {
            let [a, b, c] = t.map(|i| triangulation.vertices[i as usize]);
            mesh.push_triangle(place(a, 0.0), place(b, 0.0), place(c, 0.0));
            mesh.push_triangle(place(a, sweep), place(c, sweep), place(b, sweep));
        }
    }
    mesh.crease_normals(35f64.to_radians())
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
