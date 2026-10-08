//! 🔺️ Polygon triangulation with holes: hole bridging (Eberly) followed by ear clipping.
//!
//! Deterministic, exact on the input vertices (no Steiner points): every triangle corner is an input vertex, so the triangle areas sum to the polygon area minus the hole areas.
//! Collinear boundary vertices are kept (a vertex lying on a boundary edge splits the adjacent triangle), so the boundary of the triangulation equals the input rings vertex for vertex; this keeps developed curved faces crack-free.
//! Rings may come in either orientation; the outer ring is normalised to counter-clockwise and holes to clockwise internally. Output triangles are counter-clockwise.

use crate::vector::cross;
use crate::Point;

/// 🔺️ Vertices (outer ring followed by every hole, in input order) and triangle index triples into them.
#[derive(Clone, Debug, PartialEq)]
pub struct Triangulation {
    pub vertices: Vec<Point>,
    pub triangles: Vec<[u32; 3]>,
}

impl Triangulation {
    /// 📐️ Sum of the triangle areas (all counter-clockwise, so positive).
    pub fn area(&self) -> f64 {
        self.triangles.iter().map(|t| cross2(self.vertices[t[0] as usize], self.vertices[t[1] as usize], self.vertices[t[2] as usize]) / 2.0).sum()
    }
}

fn cross2(a: Point, b: Point, c: Point) -> f64 {
    cross(b - a, c - a)
}

fn ring_area(points: &[Point], ring: &[usize]) -> f64 {
    let n = ring.len();
    (0..n).map(|i| cross(points[ring[i]] - Point::ZERO, points[ring[(i + 1) % n]] - Point::ZERO)).sum::<f64>() / 2.0
}

struct Nodes {
    index: Vec<usize>,
    next: Vec<usize>,
    prev: Vec<usize>,
}

impl Nodes {
    fn push_ring(&mut self, ring: &[usize]) -> usize {
        let first = self.index.len();
        let n = ring.len();
        for (k, &i) in ring.iter().enumerate() {
            self.index.push(i);
            self.next.push(first + (k + 1) % n);
            self.prev.push(first + (k + n - 1) % n);
        }
        first
    }

    fn duplicate(&mut self, node: usize) -> usize {
        self.index.push(self.index[node]);
        self.next.push(0);
        self.prev.push(0);
        self.index.len() - 1
    }
}

fn in_triangle(a: Point, b: Point, c: Point, p: Point) -> bool {
    cross2(a, b, p) >= 0.0 && cross2(b, c, p) >= 0.0 && cross2(c, a, p) >= 0.0
}

fn cycle_nodes(nodes: &Nodes, head: usize) -> Vec<usize> {
    let mut out = vec![head];
    let mut cursor = nodes.next[head];
    while cursor != head {
        out.push(cursor);
        cursor = nodes.next[cursor];
    }
    out
}

fn bridge_hole(points: &[Point], nodes: &mut Nodes, outer: usize, hole: usize) {
    let at = |nodes: &Nodes, node: usize| points[nodes.index[node]];
    let m = cycle_nodes(nodes, hole).into_iter().fold(hole, |best, node| if at(nodes, node).x > at(nodes, best).x { node } else { best });
    let mp = at(nodes, m);
    let ring = cycle_nodes(nodes, outer);
    let mut best_x = f64::INFINITY;
    let mut candidate = None;
    for &e in &ring {
        let (a, b) = (at(nodes, e), at(nodes, nodes.next[e]));
        if (a.y - mp.y) * (b.y - mp.y) <= 0.0 && a.y != b.y {
            let x = a.x + (mp.y - a.y) / (b.y - a.y) * (b.x - a.x);
            if x >= mp.x && x < best_x {
                best_x = x;
                candidate = Some(if a.x > b.x { e } else { nodes.next[e] });
            }
        }
    }
    let Some(mut bridge) = candidate else { return };
    let target = at(nodes, bridge);
    let intersection = Point::new(best_x, mp.y);
    if target.x != intersection.x || target.y != intersection.y {
        let (t1, t2) = if target.y < mp.y { (target, intersection) } else { (intersection, target) };
        let mut best_angle = f64::INFINITY;
        for &node in &ring {
            let q = at(nodes, node);
            if q.x < mp.x || !in_triangle(mp, t1, t2, q) || node == bridge {
                continue;
            }
            let (p, n) = (at(nodes, nodes.prev[node]), at(nodes, nodes.next[node]));
            let angle = (q.y - mp.y).abs().atan2(q.x - mp.x);
            if cross2(p, q, n) < 0.0 && angle < best_angle {
                bridge = node;
                best_angle = angle;
            }
        }
    }
    let v2 = nodes.duplicate(bridge);
    let m2 = nodes.duplicate(m);
    let (bridge_next, m_prev) = (nodes.next[bridge], nodes.prev[m]);
    nodes.next[bridge] = m;
    nodes.prev[m] = bridge;
    nodes.next[m_prev] = m2;
    nodes.prev[m2] = m_prev;
    nodes.next[m2] = v2;
    nodes.prev[v2] = m2;
    nodes.next[v2] = bridge_next;
    nodes.prev[bridge_next] = v2;
}

fn is_ear(points: &[Point], nodes: &Nodes, ear: usize) -> bool {
    let (p, n) = (nodes.prev[ear], nodes.next[ear]);
    let (a, b, c) = (points[nodes.index[p]], points[nodes.index[ear]], points[nodes.index[n]]);
    if cross2(a, b, c) <= 0.0 {
        return false;
    }
    let mut cursor = nodes.next[n];
    while cursor != p {
        let q = points[nodes.index[cursor]];
        let corner = |x: Point| x.x == q.x && x.y == q.y;
        if !corner(a) && !corner(b) && !corner(c) && in_triangle(a, b, c, q) {
            let (qp, qn) = (points[nodes.index[nodes.prev[cursor]]], points[nodes.index[nodes.next[cursor]]]);
            if cross2(qp, q, qn) <= 0.0 {
                return false;
            }
        }
        cursor = nodes.next[cursor];
    }
    true
}

fn unlink(nodes: &mut Nodes, node: usize) -> usize {
    let (p, n) = (nodes.prev[node], nodes.next[node]);
    nodes.next[p] = n;
    nodes.prev[n] = p;
    n
}

/// 🔺️ Triangulates `outer` with `holes`; degenerate rings (fewer than three vertices) are ignored.
pub fn triangulate(outer: &[Point], holes: &[Vec<Point>]) -> Triangulation {
    let mut vertices: Vec<Point> = outer.to_vec();
    let mut offsets = vec![0usize];
    for hole in holes {
        offsets.push(vertices.len());
        vertices.extend_from_slice(hole);
    }
    let mut result = Triangulation { vertices, triangles: Vec::new() };
    if outer.len() < 3 {
        return result;
    }
    let points = result.vertices.clone();
    let orient = |range: std::ops::Range<usize>, want_ccw: bool| {
        let mut ring: Vec<usize> = range.collect();
        if (ring_area(&points, &ring) > 0.0) != want_ccw {
            ring.reverse();
        }
        ring
    };
    let mut nodes = Nodes { index: Vec::new(), next: Vec::new(), prev: Vec::new() };
    let outer_head = nodes.push_ring(&orient(0..outer.len(), true));
    let mut hole_heads: Vec<(f64, usize)> = Vec::new();
    for (k, hole) in holes.iter().enumerate() {
        if hole.len() >= 3 {
            let ring = orient(offsets[k + 1]..offsets[k + 1] + hole.len(), false);
            let max_x = ring.iter().map(|&i| points[i].x).fold(f64::NEG_INFINITY, f64::max);
            hole_heads.push((max_x, nodes.push_ring(&ring)));
        }
    }
    hole_heads.sort_by(|a, b| b.0.total_cmp(&a.0));
    for (_, head) in hole_heads {
        bridge_hole(&points, &mut nodes, outer_head, head);
    }
    let mut size = cycle_nodes(&nodes, outer_head).len();
    let mut cursor = outer_head;
    let mut idle = 0usize;
    let mut removed: Vec<[usize; 3]> = Vec::new();
    let triangle = |nodes: &Nodes, node: usize| [nodes.index[nodes.prev[node]] as u32, nodes.index[node] as u32, nodes.index[nodes.next[node]] as u32];
    while size > 3 {
        let (a, b, c) = (points[nodes.index[nodes.prev[cursor]]], points[nodes.index[cursor]], points[nodes.index[nodes.next[cursor]]]);
        if cross2(a, b, c) == 0.0 {
            removed.push([nodes.index[nodes.prev[cursor]], nodes.index[cursor], nodes.index[nodes.next[cursor]]]);
            cursor = unlink(&mut nodes, cursor);
            size -= 1;
            idle = 0;
        } else if is_ear(&points, &nodes, cursor) {
            result.triangles.push(triangle(&nodes, cursor));
            cursor = unlink(&mut nodes, cursor);
            size -= 1;
            idle = 0;
        } else {
            cursor = nodes.next[cursor];
            idle += 1;
            if idle > size {
                let convex = cycle_nodes(&nodes, cursor).into_iter().find(|&c| cross2(points[nodes.index[nodes.prev[c]]], points[nodes.index[c]], points[nodes.index[nodes.next[c]]]) > 0.0);
                let Some(c) = convex else { break };
                result.triangles.push(triangle(&nodes, c));
                cursor = unlink(&mut nodes, c);
                size -= 1;
                idle = 0;
            }
        }
    }
    if size == 3 {
        let (a, b, c) = (points[nodes.index[nodes.prev[cursor]]], points[nodes.index[cursor]], points[nodes.index[nodes.next[cursor]]]);
        if cross2(a, b, c) > 0.0 {
            result.triangles.push(triangle(&nodes, cursor));
        }
    }
    for [a, v, b] in removed.into_iter().rev() {
        let found = result.triangles.iter().position(|t| (0..3).any(|k| t[k] as usize == a && t[(k + 1) % 3] as usize == b));
        if let Some(i) = found {
            let t = result.triangles[i];
            let k = (0..3).find(|&k| t[k] as usize == a && t[(k + 1) % 3] as usize == b).unwrap_or(0);
            let c = t[(k + 2) % 3];
            result.triangles[i] = [a as u32, v as u32, c];
            result.triangles.push([v as u32, b as u32, c]);
        }
    }
    result
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
