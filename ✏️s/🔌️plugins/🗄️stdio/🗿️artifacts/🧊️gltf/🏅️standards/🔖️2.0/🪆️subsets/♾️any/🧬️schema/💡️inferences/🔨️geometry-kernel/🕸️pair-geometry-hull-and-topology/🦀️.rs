//! 🕸️ Pair geometry sampling, convex hull metrics, thickness and roughness samples, mesh topology and distance queries.

use std::{collections::BTreeMap, collections::BTreeSet};
use crate::standards::v2_0::subsets::any::schema::modules::mesh_topology::Topology;
use crate::standards::v2_0::subsets::any::schema::inferences::geometry_core::*;

//#region 🧮️Kernel
/// 🧩️ Internal pairwise geometry evidence; leaves decide their public result records.
pub struct GltfPairGeometry {
    pub first: GltfEntityAddress,
    pub second: GltfEntityAddress,
    pub minimum_distance: f64,
    pub adjacent: bool,
    pub contact_area: Option<f64>,
    pub overlap: Option<(f64, usize)>,
    pub sample_count: usize,
}

fn sampled_points(points: &[V3], limit: usize) -> impl Iterator<Item = V3> + '_ {
    let step = points.len().div_ceil(limit.max(1));
    points.iter().step_by(step).copied()
}

pub fn pair_geometry(first: &RawPart, second: &RawPart, policy: &GltfAnalysisPolicy) -> Option<GltfPairGeometry> {
    let (first_lo, first_hi, _) = bounds(&first.points)?;
    let (second_lo, second_hi, _) = bounds(&second.points)?;
    let point_limit = ((policy.sampling_budget as f64).sqrt() as usize).max(1);
    let first_samples = sampled_points(&first.points, point_limit).collect::<Vec<_>>();
    let second_samples = sampled_points(&second.points, point_limit).collect::<Vec<_>>();
    let minimum_distance = first_samples.iter().flat_map(|a| second_samples.iter().map(move |b| norm(sub(*a, *b)))).fold(f64::INFINITY, f64::min);
    let sample_count = first_samples.len().saturating_mul(second_samples.len());
    let tolerance = (norm(sub(first_hi, first_lo)).max(norm(sub(second_hi, second_lo))) * policy.relative_tolerance).max(policy.absolute_length_tolerance);
    let overlap_dimensions =
        [(first_hi[0].min(second_hi[0]) - first_lo[0].max(second_lo[0])).max(0.0), (first_hi[1].min(second_hi[1]) - first_lo[1].max(second_lo[1])).max(0.0), (first_hi[2].min(second_hi[2]) - first_lo[2].max(second_lo[2])).max(0.0)];
    let overlap_volume = overlap_dimensions.iter().product::<f64>();
    let contact_area = if minimum_distance <= policy.contact_tolerance.max(tolerance) {
        let normal_axis = overlap_dimensions.iter().enumerate().min_by(|(_, left), (_, right)| left.total_cmp(right)).map_or(0, |(axis, _)| axis);
        overlap_dimensions.iter().enumerate().filter(|(axis, _)| *axis != normal_axis).map(|(_, extent)| *extent).product()
    } else {
        0.0
    };
    Some(GltfPairGeometry {
        first: first.address.clone(),
        second: second.address.clone(),
        minimum_distance,
        adjacent: minimum_distance <= policy.contact_tolerance.max(tolerance),
        contact_area: Some(contact_area),
        overlap: Some((overlap_volume, sample_count)),
        sample_count,
    })
}

pub fn triangle_area(a: V3, b: V3, c: V3) -> f64 {
    0.5 * norm(cross(sub(b, a), sub(c, a)))
}

pub fn convex_hull_metrics(points: &[V3], tolerance: f64) -> Option<ConvexHullMetrics> {
    if points.len() < 4 {
        return None;
    }
    let mut planes = BTreeMap::<(i64, i64, i64, i64), (V3, f64)>::new();
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            for k in j + 1..points.len() {
                let mut n = cross(sub(points[j], points[i]), sub(points[k], points[i]));
                if norm(n) <= tolerance {
                    continue;
                }
                n = normalize(n);
                let mut d = dot(n, points[i]);
                let (mut positive, mut negative) = (false, false);
                for p in points {
                    let s = dot(n, *p) - d;
                    positive |= s > tolerance;
                    negative |= s < -tolerance;
                }
                if positive && negative {
                    continue;
                }
                if positive {
                    n = mul(n, -1.0);
                    d = -d;
                }
                let q = 1e-8;
                planes.entry(((n[0] / q).round() as i64, (n[1] / q).round() as i64, (n[2] / q).round() as i64, (d / q).round() as i64)).or_insert((n, d));
            }
        }
    }
    let supporting_planes = planes.values().copied().collect::<Vec<_>>();
    let mut area = 0.0;
    let mut volume = 0.0;
    for (n, d) in &supporting_planes {
        let ids = points.iter().enumerate().filter(|(_, p)| (dot(*n, **p) - *d).abs() <= tolerance * 4.0).map(|(i, _)| i).collect::<Vec<_>>();
        if ids.len() < 3 {
            continue;
        }
        let seed = if n[0].abs() < 0.8 { [1.0, 0.0, 0.0] } else { [0.0, 1.0, 0.0] };
        let u = normalize(cross(seed, *n));
        let v = cross(*n, u);
        let mut q = ids.iter().map(|i| (dot(points[*i], u), dot(points[*i], v), *i)).collect::<Vec<_>>();
        q.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.total_cmp(&b.1)));
        let turn = |a: (f64, f64, usize), b: (f64, f64, usize), c: (f64, f64, usize)| (b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0);
        let mut h = Vec::new();
        for p in q.iter().copied() {
            while h.len() >= 2 && turn(h[h.len() - 2], h[h.len() - 1], p) <= tolerance {
                h.pop();
            }
            h.push(p);
        }
        let lower = h.len();
        for p in q.iter().rev().skip(1).copied() {
            while h.len() > lower && turn(h[h.len() - 2], h[h.len() - 1], p) <= tolerance {
                h.pop();
            }
            h.push(p);
        }
        if h.len() > 1 {
            h.pop();
        }
        if h.len() < 3 {
            continue;
        }
        let a = points[h[0].2];
        for i in 1..h.len() - 1 {
            let (b, c) = (points[h[i].2], points[h[i + 1].2]);
            area += triangle_area(a, b, c);
            volume += dot(a, cross(b, c)) / 6.0;
        }
    }
    Some((area, volume.abs(), supporting_planes))
}

pub fn hull_sample(points: &[V3], budget: usize) -> Vec<V3> {
    let limit = budget.clamp(4, 32);
    if points.len() <= limit {
        return points.to_vec();
    }
    let mut selected = BTreeSet::new();
    for axis in 0..3 {
        selected.insert((0..points.len()).min_by(|a, b| points[*a][axis].total_cmp(&points[*b][axis])).unwrap());
        selected.insert((0..points.len()).max_by(|a, b| points[*a][axis].total_cmp(&points[*b][axis])).unwrap());
    }
    for slot in 0..limit {
        selected.insert(slot * (points.len() - 1) / (limit - 1));
        if selected.len() == limit {
            break;
        }
    }
    for index in 0..points.len() {
        if selected.len() == limit {
            break;
        }
        selected.insert(index);
    }
    selected.into_iter().take(limit).map(|index| points[index]).collect()
}

fn ray_triangle(origin: V3, direction: V3, a: V3, b: V3, c: V3, tolerance: f64) -> Option<f64> {
    let e1 = sub(b, a);
    let e2 = sub(c, a);
    let h = cross(direction, e2);
    let det = dot(e1, h);
    if det.abs() <= tolerance {
        return None;
    }
    let inv = 1.0 / det;
    let s = sub(origin, a);
    let u = inv * dot(s, h);
    if u < -tolerance || u > 1.0 + tolerance {
        return None;
    }
    let q = cross(s, e1);
    let v = inv * dot(direction, q);
    if v < -tolerance || u + v > 1.0 + tolerance {
        return None;
    }
    let t = inv * dot(e2, q);
    if t > tolerance {
        Some(t)
    } else {
        None
    }
}

pub fn thickness_samples(points: &[V3], faces: &[[usize; 3]], budget: usize, tolerance: f64) -> Vec<f64> {
    let mut normals = vec![[0.0; 3]; points.len()];
    for f in faces {
        let n = cross(sub(points[f[1]], points[f[0]]), sub(points[f[2]], points[f[0]]));
        for i in f {
            normals[*i] = add(normals[*i], n);
        }
    }
    let budget = budget.max(1);
    let face_target = (budget as f64).sqrt().floor().max(1.0) as usize;
    let face_step = faces.len().div_ceil(face_target).max(1);
    let sampled_faces = faces.len().div_ceil(face_step).max(1);
    let point_target = (budget / sampled_faces).max(1);
    let point_step = points.len().div_ceil(point_target).max(1);
    let mut out = Vec::new();
    for (i, p) in points.iter().enumerate().step_by(point_step) {
        let n = normalize(normals[i]);
        let mut best = f64::INFINITY;
        for direction in [n, mul(n, -1.0)] {
            for f in faces.iter().step_by(face_step) {
                if f.contains(&i) {
                    continue;
                }
                if let Some(t) = ray_triangle(*p, direction, points[f[0]], points[f[1]], points[f[2]], tolerance) {
                    best = best.min(t);
                }
            }
        }
        if best.is_finite() {
            out.push(best);
        }
    }
    out
}

pub fn roughness_samples(points: &[V3], faces: &[[usize; 3]]) -> Vec<f64> {
    let mut neighbors = vec![BTreeSet::new(); points.len()];
    for f in faces {
        for &(a, b) in &[(f[0], f[1]), (f[1], f[2]), (f[2], f[0])] {
            neighbors[a].insert(b);
            neighbors[b].insert(a);
        }
    }
    points
        .iter()
        .enumerate()
        .filter_map(|(i, p)| {
            if neighbors[i].is_empty() {
                return None;
            }
            let avg = mul(neighbors[i].iter().fold([0.0; 3], |s, j| add(s, points[*j])), 1.0 / neighbors[i].len() as f64);
            Some(norm(sub(*p, avg)))
        })
        .collect()
}

#[derive(Clone)]
struct Dsu {
    pub(crate) p: Vec<usize>,
    r: Vec<u8>,
}
impl Dsu {
    pub(crate) fn new(n: usize) -> Self {
        Self { p: (0..n).collect(), r: vec![0; n] }
    }
    pub(crate) fn find(&mut self, x: usize) -> usize {
        if self.p[x] != x {
            self.p[x] = self.find(self.p[x]);
        }
        self.p[x]
    }
    fn union(&mut self, a: usize, b: usize) {
        let (mut a, mut b) = (self.find(a), self.find(b));
        if a == b {
            return;
        }
        if self.r[a] < self.r[b] {
            std::mem::swap(&mut a, &mut b);
        }
        self.p[b] = a;
        if self.r[a] == self.r[b] {
            self.r[a] += 1;
        }
    }
}

pub(crate) fn topology(points: &[V3], faces: &[[usize; 3]]) -> MeshTopology {
    let diagonal = bounds(points).map_or(0.0, |x| norm(x.2));
    let tol = (diagonal * 1e-9).max(1e-9);
    let mut map = BTreeMap::<(i64, i64, i64), usize>::new();
    let mut welded = Vec::new();
    let mut remap = Vec::with_capacity(points.len());
    for p in points {
        let key = ((p[0] / tol).round() as i64, (p[1] / tol).round() as i64, (p[2] / tol).round() as i64);
        let id = *map.entry(key).or_insert_with(|| {
            let i = welded.len();
            welded.push(*p);
            i
        });
        remap.push(id);
    }
    let mut clean = Vec::new();
    for f in faces {
        if f.iter().all(|i| *i < remap.len()) {
            let w = [remap[f[0]], remap[f[1]], remap[f[2]]];
            if w[0] != w[1] && w[1] != w[2] && w[2] != w[0] && triangle_area(welded[w[0]], welded[w[1]], welded[w[2]]) > tol * tol {
                clean.push(w);
            }
        }
    }
    let mut edges = BTreeMap::<(usize, usize), Vec<(usize, bool)>>::new();
    let mut used = BTreeSet::new();
    let mut dsu = Dsu::new(welded.len());
    for (fi, f) in clean.iter().enumerate() {
        for &(a, b) in &[(f[0], f[1]), (f[1], f[2]), (f[2], f[0])] {
            used.insert(a);
            used.insert(b);
            dsu.union(a, b);
            let key = (a.min(b), a.max(b));
            edges.entry(key).or_default().push((fi, a < b));
        }
    }
    let manifold = edges.values().all(|e| e.len() <= 2);
    let watertight = !clean.is_empty() && edges.values().all(|e| e.len() == 2);
    let oriented = edges.values().filter(|e| e.len() == 2).all(|e| e[0].1 != e[1].1);
    let components = used.iter().map(|i| dsu.find(*i)).collect::<BTreeSet<_>>().len() as u64;
    let mut bd = Dsu::new(welded.len());
    let mut boundary_vertices = BTreeSet::new();
    for (&(a, b), e) in &edges {
        if e.len() == 1 {
            bd.union(a, b);
            boundary_vertices.insert(a);
            boundary_vertices.insert(b);
        }
    }
    let boundary_loops = boundary_vertices.iter().map(|i| bd.find(*i)).collect::<BTreeSet<_>>().len() as u64;
    let chi = used.len() as i64 - edges.len() as i64 + clean.len() as i64;
    let g2 = 2 * components as i64 - boundary_loops as i64 - chi;
    let genus = if manifold && g2 >= 0 && g2 % 2 == 0 { Some((g2 / 2) as u64) } else { None };
    (Topology { components, boundary_loops, chi, genus, manifold, watertight, oriented }, welded, clean, edges)
}

pub fn topology_summary(points: &[V3], faces: &[[usize; 3]]) -> Topology {
    topology(points, faces).0
}

fn point_triangle_distance_squared(point: V3, a: V3, b: V3, c: V3) -> f64 {
    let ab = sub(b, a);
    let ac = sub(c, a);
    let ap = sub(point, a);
    let d1 = dot(ab, ap);
    let d2 = dot(ac, ap);
    if d1 <= 0.0 && d2 <= 0.0 {
        return dot(ap, ap);
    }
    let bp = sub(point, b);
    let d3 = dot(ab, bp);
    let d4 = dot(ac, bp);
    if d3 >= 0.0 && d4 <= d3 {
        return dot(bp, bp);
    }
    let vc = d1 * d4 - d3 * d2;
    if vc <= 0.0 && d1 >= 0.0 && d3 <= 0.0 {
        let projection = add(a, mul(ab, d1 / (d1 - d3)));
        return dot(sub(point, projection), sub(point, projection));
    }
    let cp = sub(point, c);
    let d5 = dot(ab, cp);
    let d6 = dot(ac, cp);
    if d6 >= 0.0 && d5 <= d6 {
        return dot(cp, cp);
    }
    let vb = d5 * d2 - d1 * d6;
    if vb <= 0.0 && d2 >= 0.0 && d6 <= 0.0 {
        let projection = add(a, mul(ac, d2 / (d2 - d6)));
        return dot(sub(point, projection), sub(point, projection));
    }
    let va = d3 * d6 - d5 * d4;
    if va <= 0.0 && d4 - d3 >= 0.0 && d5 - d6 >= 0.0 {
        let projection = add(b, mul(sub(c, b), (d4 - d3) / ((d4 - d3) + (d5 - d6))));
        return dot(sub(point, projection), sub(point, projection));
    }
    let denominator = 1.0 / (va + vb + vc);
    let projection = add(a, add(mul(ab, vb * denominator), mul(ac, vc * denominator)));
    dot(sub(point, projection), sub(point, projection))
}

fn segment_distance_squared(first_start: V3, first_end: V3, second_start: V3, second_end: V3) -> f64 {
    let first_direction = sub(first_end, first_start);
    let second_direction = sub(second_end, second_start);
    let between = sub(first_start, second_start);
    let aa = dot(first_direction, first_direction);
    let bb = dot(first_direction, second_direction);
    let cc = dot(second_direction, second_direction);
    let dd = dot(first_direction, between);
    let ee = dot(second_direction, between);
    let denominator = aa * cc - bb * bb;
    let mut first = if denominator <= f64::EPSILON { 0.0 } else { (bb * ee - cc * dd) / denominator };
    first = first.clamp(0.0, 1.0);
    let second = if cc > 0.0 { ((bb * first + ee) / cc).clamp(0.0, 1.0) } else { 0.0 };
    first = if aa > 0.0 { ((bb * second - dd) / aa).clamp(0.0, 1.0) } else { 0.0 };
    let delta = sub(add(first_start, mul(first_direction, first)), add(second_start, mul(second_direction, second)));
    dot(delta, delta)
}

pub(crate) fn triangle_distance(first: [V3; 3], second: [V3; 3]) -> f64 {
    let mut squared =
        first.iter().map(|point| point_triangle_distance_squared(*point, second[0], second[1], second[2])).chain(second.iter().map(|point| point_triangle_distance_squared(*point, first[0], first[1], first[2]))).fold(f64::INFINITY, f64::min);
    for first_edge in [(first[0], first[1]), (first[1], first[2]), (first[2], first[0])] {
        for second_edge in [(second[0], second[1]), (second[1], second[2]), (second[2], second[0])] {
            squared = squared.min(segment_distance_squared(first_edge.0, first_edge.1, second_edge.0, second_edge.1));
        }
    }
    squared.sqrt()
}

pub(crate) fn point_in_closed_mesh(point: V3, points: &[V3], faces: &[[usize; 3]], tolerance: f64) -> Option<bool> {
    let directions = [normalize([1.0, 0.371, 0.529]), normalize([0.233, 1.0, 0.419]), normalize([0.317, 0.271, 1.0])];
    let mut votes = Vec::new();
    for direction in directions {
        let mut hits = faces.iter().filter_map(|face| ray_triangle(point, direction, points[face[0]], points[face[1]], points[face[2]], tolerance)).collect::<Vec<_>>();
        hits.sort_by(f64::total_cmp);
        hits.dedup_by(|a, b| (*a - *b).abs() <= tolerance * (1.0 + a.abs().max(b.abs())));
        votes.push(hits.len() % 2 == 1);
    }
    let inside = votes.iter().filter(|vote| **vote).count();
    if inside >= 2 {
        Some(true)
    } else if inside <= 1 {
        Some(false)
    } else {
        None
    }
}
//#endregion 🧮️Kernel
