//! 🔪️ Plane sections of triangle meshes as oriented 2D linework (plan cuts, elevations, sections).
//!
//! The plane is treated as nudged infinitesimally so that vertices exactly on it count as being on its positive side (symbolic perturbation): every closed mesh then yields closed, non-degenerate chains and coplanar faces yield nothing.
//! Segments are directed so that the material lies on their left; chained outer boundaries are counter-clockwise, holes clockwise.

use crate::mesh::TriMesh;
use crate::vector::{cross, cross3, dot3, length3, normalize3, sub3, Xyz};
use crate::Point;
use std::collections::BTreeMap;

/// 〰️ A chain of section segments.
#[derive(Clone, Debug, PartialEq)]
pub struct Polyline {
    pub points: Vec<Point>,
    pub closed: bool,
}

/// 🔪️ Section of `mesh` with the plane through `origin` with `normal`; 2D coordinates are `(dot(p - origin, u), dot(p - origin, v))` with `u` the projection of `u_axis` onto the plane and `v = normal x u`. `eps` is the classification band for the signed distance.
pub fn section_plane(mesh: &TriMesh, origin: Xyz, normal: Xyz, u_axis: Xyz, eps: f64) -> Vec<[Point; 2]> {
    let n = normalize3(normal);
    let u = normalize3(sub3(u_axis, [n[0] * dot3(u_axis, n), n[1] * dot3(u_axis, n), n[2] * dot3(u_axis, n)]));
    let v = cross3(n, u);
    let mut out = Vec::new();
    for i in 0..mesh.indices.len() {
        let corners = mesh.triangle(i);
        let d = corners.map(|p| dot3(sub3(p, origin), n));
        let below = d.map(|x| x < -eps);
        if below.iter().all(|&b| b) || below.iter().all(|&b| !b) {
            continue;
        }
        let mut hits: Vec<Xyz> = Vec::with_capacity(2);
        for e in 0..3 {
            let (a, b) = (e, (e + 1) % 3);
            if below[a] != below[b] {
                let t = d[a] / (d[a] - d[b]);
                hits.push([corners[a][0] + (corners[b][0] - corners[a][0]) * t, corners[a][1] + (corners[b][1] - corners[a][1]) * t, corners[a][2] + (corners[b][2] - corners[a][2]) * t]);
            }
        }
        if hits.len() != 2 {
            continue;
        }
        let face = cross3(sub3(corners[1], corners[0]), sub3(corners[2], corners[0]));
        if length3(face) <= 1e-18 {
            continue;
        }
        let direction = cross3(n, normalize3(face));
        let (a, b) = if dot3(sub3(hits[1], hits[0]), direction) >= 0.0 { (hits[0], hits[1]) } else { (hits[1], hits[0]) };
        let project = |p: Xyz| {
            let r = sub3(p, origin);
            Point::new(dot3(r, u), dot3(r, v))
        };
        let (pa, pb) = (project(a), project(b));
        if (pb - pa).hypot() > 0.0 {
            out.push([pa, pb]);
        }
    }
    out
}

/// 🔪️ Horizontal cut at height `z` in world XY coordinates (plan view looking down).
pub fn section_z(mesh: &TriMesh, z: f64, eps: f64) -> Vec<[Point; 2]> {
    section_plane(mesh, [0.0, 0.0, z], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0], eps)
}

struct Snap {
    eps: f64,
    cells: BTreeMap<(i64, i64), Vec<usize>>,
    points: Vec<Point>,
}

impl Snap {
    fn node(&mut self, p: Point) -> usize {
        let (cx, cy) = ((p.x / self.eps).floor() as i64, (p.y / self.eps).floor() as i64);
        for dx in -1..=1 {
            for dy in -1..=1 {
                if let Some(list) = self.cells.get(&(cx + dx, cy + dy)) {
                    if let Some(&found) = list.iter().find(|&&i| (self.points[i] - p).hypot() <= self.eps) {
                        return found;
                    }
                }
            }
        }
        self.points.push(p);
        self.cells.entry((cx, cy)).or_default().push(self.points.len() - 1);
        self.points.len() - 1
    }
}

fn drop_collinear(points: Vec<Point>, closed: bool, eps: f64) -> Vec<Point> {
    let n = points.len();
    let keep = |i: usize| {
        if !closed && (i == 0 || i + 1 == n) {
            return true;
        }
        let (a, b, c) = (points[(i + n - 1) % n], points[i], points[(i + 1) % n]);
        let (ab, bc) = (b - a, c - b);
        let length = (c - a).hypot();
        length <= eps || cross(ab, bc).abs() / length > eps || ab.dot(bc) <= 0.0
    };
    (0..n).filter(|&i| keep(i)).map(|i| points[i]).collect()
}

/// 〰️ Chains directed segments end-to-start (endpoints closer than `eps` coincide) into polylines with collinear intermediate points removed; open chains start where no segment arrives.
pub fn chain(segments: &[[Point; 2]], eps: f64) -> Vec<Polyline> {
    let mut snap = Snap { eps: eps.max(1e-12), cells: BTreeMap::new(), points: Vec::new() };
    let edges: Vec<(usize, usize)> = segments.iter().map(|s| (snap.node(s[0]), snap.node(s[1]))).filter(|(a, b)| a != b).collect();
    let mut outgoing: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    let mut incoming: BTreeMap<usize, usize> = BTreeMap::new();
    for (i, &(a, b)) in edges.iter().enumerate() {
        outgoing.entry(a).or_default().push(i);
        *incoming.entry(b).or_insert(0) += 1;
    }
    let mut used = vec![false; edges.len()];
    let mut order: Vec<usize> = (0..edges.len()).filter(|&i| !incoming.contains_key(&edges[i].0)).collect();
    order.extend(0..edges.len());
    let mut out = Vec::new();
    for start in order {
        if used[start] {
            continue;
        }
        let mut points = vec![snap.points[edges[start].0]];
        let mut edge = start;
        let mut closed = false;
        loop {
            used[edge] = true;
            let to = edges[edge].1;
            if to == edges[start].0 {
                closed = true;
                break;
            }
            points.push(snap.points[to]);
            match outgoing.get(&to).and_then(|list| list.iter().copied().find(|&j| !used[j])) {
                Some(next) => edge = next,
                None => break,
            }
        }
        let points = drop_collinear(points, closed, snap.eps);
        if points.len() >= 2 {
            out.push(Polyline { points, closed });
        }
    }
    out
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
