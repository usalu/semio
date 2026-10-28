//! 🕳️ Planar polygon rings with bounded validation, bridging and original-index triangulation.
use std::collections::{BTreeMap, BTreeSet};

type Point = [f64; 3];
type Uv = [f64; 2];
type Triangle = [u32; 3];
type RingResult<T> = Result<T, String>;
const EPS: f64 = 128. * f64::EPSILON;

fn cross(a: Uv, b: Uv, c: Uv) -> f64 { (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0]) }
fn on(a: Uv, b: Uv, p: Uv) -> bool {
    cross(a, b, p).abs() <= EPS && p[0] >= a[0].min(b[0]) - EPS && p[0] <= a[0].max(b[0]) + EPS && p[1] >= a[1].min(b[1]) - EPS && p[1] <= a[1].max(b[1]) + EPS
}
fn intersects(a: Uv, b: Uv, c: Uv, d: Uv) -> bool {
    let (x, y, z, w) = (cross(a, b, c), cross(a, b, d), cross(c, d, a), cross(c, d, b));
    ((x > EPS && y < -EPS || x < -EPS && y > EPS) && (z > EPS && w < -EPS || z < -EPS && w > EPS)) || on(a, b, c) || on(a, b, d) || on(c, d, a) || on(c, d, b)
}
fn edge_key(a: u32, b: u32) -> (u32, u32) { if a < b { (a, b) } else { (b, a) } }

struct Triangulator<'a> {
    uv: BTreeMap<u32, Uv>,
    tick: &'a mut dyn FnMut() -> RingResult<()>,
}

impl Triangulator<'_> {
    fn p(&self, id: u32) -> Uv { self.uv[&id] }
    fn area(&mut self, ring: &[u32]) -> RingResult<f64> {
        let mut sum = 0.;
        for i in 0..ring.len() {
            (self.tick)()?;
            let (a, b) = (self.p(ring[i]), self.p(ring[(i + 1) % ring.len()]));
            sum += a[0] * b[1] - b[0] * a[1];
        }
        Ok(sum / 2.)
    }
    fn inside(&mut self, q: Uv, ring: &[u32]) -> RingResult<i8> {
        let mut value = false;
        for i in 0..ring.len() {
            (self.tick)()?;
            let (a, b) = (self.p(ring[(i + ring.len() - 1) % ring.len()]), self.p(ring[i]));
            if on(a, b, q) { return Ok(0); }
            if (a[1] > q[1]) != (b[1] > q[1]) && q[0] < (b[0] - a[0]) * (q[1] - a[1]) / (b[1] - a[1]) + a[0] { value = !value; }
        }
        Ok(if value { 1 } else { -1 })
    }
    fn filled(&mut self, q: Uv, rings: &[Vec<u32>]) -> RingResult<bool> {
        if self.inside(q, &rings[0])? != 1 { return Ok(false); }
        for ring in &rings[1..] { if self.inside(q, ring)? != -1 { return Ok(false); } }
        Ok(true)
    }
    fn blocked(&mut self, a: u32, b: u32, ring: &[u32]) -> RingResult<bool> {
        for i in 0..ring.len() {
            (self.tick)()?;
            let (c, d) = (ring[i], ring[(i + 1) % ring.len()]);
            if !intersects(self.p(a), self.p(b), self.p(c), self.p(d)) { continue; }
            if a == c && b == d || a == d && b == c { return Ok(true); }
            let shared = [a, b].into_iter().find(|&value| value == c || value == d);
            let Some(shared) = shared else { return Ok(true); };
            let other = if c == shared { d } else { c };
            if on(self.p(a), self.p(b), self.p(other)) { return Ok(true); }
        }
        Ok(false)
    }
    fn locally_inside(&self, ring: &[u32], at: usize, target: u32) -> bool {
        let (a, b, c, q) = (self.p(ring[(at + ring.len() - 1) % ring.len()]), self.p(ring[at]), self.p(ring[(at + 1) % ring.len()]), self.p(target));
        if cross(a, b, c) > EPS { cross(b, q, c) <= EPS && cross(b, a, q) <= EPS } else { cross(b, q, a) > EPS || cross(b, c, q) > EPS }
    }
    fn run(&mut self, input: &[Vec<u32>]) -> RingResult<Vec<Triangle>> {
        let mut rings = Vec::new();
        for source in input {
            let mut ring = source.clone();
            if ring.len() > 1 && self.p(ring[0]) == self.p(*ring.last().unwrap()) { ring.pop(); }
            rings.push(ring);
        }
        for ring in &rings {
            if ring.len() < 3 || self.area(ring)?.abs() <= EPS { return Err("degenerate-polygon".into()); }
            for i in 0..ring.len() {
                (self.tick)()?;
                let (a, b) = (self.p(ring[i]), self.p(ring[(i + 1) % ring.len()]));
                if a == b { return Err("zero-length-polygon-edge".into()); }
                for j in i + 1..ring.len() {
                    (self.tick)()?;
                    if j == i + 1 || i == 0 && j == ring.len() - 1 { continue; }
                    if intersects(a, b, self.p(ring[j]), self.p(ring[(j + 1) % ring.len()])) { return Err("self-intersecting-polygon".into()); }
                }
            }
        }
        for i in 0..rings.len() { for j in i + 1..rings.len() {
            for a in 0..rings[i].len() { for b in 0..rings[j].len() {
                (self.tick)()?;
                if intersects(self.p(rings[i][a]), self.p(rings[i][(a + 1) % rings[i].len()]), self.p(rings[j][b]), self.p(rings[j][(b + 1) % rings[j].len()])) { return Err("intersecting-polygon-rings".into()); }
            } }
            if i == 0 && self.inside(self.p(rings[j][0]), &rings[0])? != 1 { return Err("hole-outside-polygon".into()); }
            if i > 0 && (self.inside(self.p(rings[i][0]), &rings[j])? != -1 || self.inside(self.p(rings[j][0]), &rings[i])? != -1) { return Err("nested-polygon-holes".into()); }
        } }
        let winding = self.area(&rings[0])?.signum();
        for (i, ring) in rings.iter_mut().enumerate() { if self.area(ring)?.signum() != if i == 0 { 1. } else { -1. } { ring.reverse(); } }
        let mut merged = rings[0].clone();
        let mut holes = Vec::new();
        for ring in &rings[1..] {
            let mut at = 0;
            for i in 1..ring.len() {
                (self.tick)()?;
                let (a, b) = (self.p(ring[i]), self.p(ring[at]));
                if a[0] < b[0] || a[0] == b[0] && a[1] < b[1] { at = i; }
            }
            holes.push((ring, at));
        }
        holes.sort_by(|(a, ai), (b, bi)| {
            let (a, b) = (self.p(a[*ai]), self.p(b[*bi]));
            a[0].total_cmp(&b[0]).then_with(|| a[1].total_cmp(&b[1]))
        });
        for (ring, at) in holes {
            let h = ring[at];
            let mut best = None;
            let mut distance = f64::INFINITY;
            for i in 0..merged.len() {
                (self.tick)()?;
                let o = merged[i];
                let (a, b) = (self.p(h), self.p(o));
                let d = (a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2);
                if d >= distance || !self.locally_inside(&merged, i, h) || !self.locally_inside(ring, at, o) || !self.filled([(a[0] + b[0]) / 2., (a[1] + b[1]) / 2.], &rings)? { continue; }
                let mut blocked = false;
                for boundary in &rings { if self.blocked(h, o, boundary)? { blocked = true; break; } }
                if blocked || self.blocked(h, o, &merged)? { continue; }
                best = Some(i);
                distance = d;
            }
            let best = best.ok_or("polygon-hole-bridge-failed")?;
            let mut next = Vec::new();
            next.extend_from_slice(&merged[..=best]);
            next.extend_from_slice(&ring[at..]);
            next.extend_from_slice(&ring[..at]);
            next.extend_from_slice(&[h, merged[best]]);
            next.extend_from_slice(&merged[best + 1..]);
            merged = next;
        }
        let mut triangles = Vec::new();
        while merged.len() > 3 {
            let mut ear = None;
            for i in 0..merged.len() {
                (self.tick)()?;
                let triangle = [merged[(i + merged.len() - 1) % merged.len()], merged[i], merged[(i + 1) % merged.len()]];
                let [a, b, c] = triangle.map(|id| self.p(id));
                if cross(a, b, c) <= EPS { continue; }
                let mut contains = false;
                for &id in &merged {
                    (self.tick)()?;
                    let q = self.p(id);
                    if q == a || q == b || q == c { continue; }
                    if cross(a, b, q) >= -EPS && cross(b, c, q) >= -EPS && cross(c, a, q) >= -EPS { contains = true; break; }
                }
                if !contains { ear = Some(i); triangles.push(triangle); break; }
            }
            merged.remove(ear.ok_or("polygon-triangulation-failed")?);
        }
        if merged.len() != 3 || cross(self.p(merged[0]), self.p(merged[1]), self.p(merged[2])) <= EPS { return Err("degenerate-polygon-triangle".into()); }
        triangles.push([merged[0], merged[1], merged[2]]);
        let mut expected = 0.;
        for ring in &rings { expected += self.area(ring)?; }
        let mut edges = BTreeMap::new();
        let mut covered = 0.;
        for &triangle in &triangles {
            (self.tick)()?;
            let [a, b, c] = triangle.map(|id| self.p(id));
            covered += cross(a, b, c) / 2.;
            if !self.filled([(a[0] + b[0] + c[0]) / 3., (a[1] + b[1] + c[1]) / 3.], &rings)? { return Err("polygon-triangle-outside-domain".into()); }
            for i in 0..3 { *edges.entry(edge_key(triangle[i], triangle[(i + 1) % 3])).or_insert(0) += 1; }
        }
        if (covered - expected).abs() > EPS.max(expected.abs() * 1e-9) { return Err("polygon-coverage-mismatch".into()); }
        let mut boundary = BTreeSet::new();
        for ring in &rings { for i in 0..ring.len() { boundary.insert(edge_key(ring[i], ring[(i + 1) % ring.len()])); } }
        for (key, count) in &edges {
            (self.tick)()?;
            if *count != if boundary.contains(key) { 1 } else { 2 } { return Err("polygon-edge-incidence".into()); }
        }
        for key in &boundary { (self.tick)()?; if edges.get(key) != Some(&1) { return Err("polygon-boundary-missing".into()); } }
        for i in 0..triangles.len() { for j in i + 1..triangles.len() { for a in 0..3 { for b in 0..3 {
            (self.tick)()?;
            let (u, v, x, y) = (triangles[i][a], triangles[i][(a + 1) % 3], triangles[j][b], triangles[j][(b + 1) % 3]);
            if u == x || u == y || v == x || v == y { continue; }
            if intersects(self.p(u), self.p(v), self.p(x), self.p(y)) { return Err("polygon-triangles-overlap".into()); }
        } } } }
        if winding < 0. { for triangle in &mut triangles { triangle.swap(1, 2); } }
        Ok(triangles)
    }
}

/// 🧩 Triangulate a planar outer boundary with disjoint holes, preserving input indices and winding.
pub fn triangulate_rings(vertices: &[[f64; 3]], input: &[Vec<u32>], tick: &mut dyn FnMut() -> Result<(), String>) -> Result<Vec<[u32; 3]>, String> {
    tick()?;
    if input.is_empty() || input[0].len() < 3 { return Err("polygon-needs-three-points".into()); }
    let origin = *vertices.get(input[0][0] as usize).ok_or("invalid-polygon-index")?;
    let mut extent: f64 = 0.;
    let mut points = BTreeMap::new();
    for ring in input { for &id in ring {
        tick()?;
        let point = vertices.get(id as usize).ok_or("invalid-polygon-index")?;
        if !point.iter().all(|value| value.is_finite()) { return Err("invalid-polygon-index".into()); }
        let p = [0, 1, 2].map(|i| point[i] - origin[i]);
        if !p.iter().all(|value| value.is_finite()) { return Err("polygon-coordinate-range".into()); }
        for value in p { extent = extent.max(value.abs()); }
        points.insert(id, p);
    } }
    if extent <= 0. { return Err("degenerate-polygon".into()); }
    for p in points.values_mut() { for value in p { *value /= extent; } }
    let mut normal = [0.; 3];
    let outer = &input[0];
    for i in 0..outer.len() {
        tick()?;
        let (a, b) = (points[&outer[i]], points[&outer[(i + 1) % outer.len()]]);
        normal[0] += a[1] * b[2] - a[2] * b[1];
        normal[1] += a[2] * b[0] - a[0] * b[2];
        normal[2] += a[0] * b[1] - a[1] * b[0];
    }
    let mut axis = 0;
    for i in 1..3 { if normal[i].abs() > normal[axis].abs() { axis = i; } }
    let length = normal[0].hypot(normal[1]).hypot(normal[2]);
    if length <= EPS { return Err("degenerate-polygon".into()); }
    let mut uv = BTreeMap::new();
    for (id, p) in points {
        tick()?;
        if (0..3).map(|i| p[i] * normal[i] / length).sum::<f64>().abs() > 1e-9 { return Err("nonplanar-polygon".into()); }
        uv.insert(id, [p[(axis + 1) % 3], p[(axis + 2) % 3]]);
    }
    Triangulator { uv, tick }.run(input)
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
