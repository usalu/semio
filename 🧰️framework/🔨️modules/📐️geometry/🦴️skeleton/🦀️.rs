//! 🦴️ Weighted straight skeleton of simple polygons with holes.
//!
//! Every input edge moves inward (into the material, to its left) at its own constant `speed` per unit of time; the wavefront
//! vertices travel along the bisecting velocity that keeps them on both adjacent edge lines, and the skeleton is the trace of those
//! vertices. With the time read as a height and `speed = 1 / tan(pitch)` the faces are the planes of a hipped roof; a speed of zero
//! is a vertical edge (a gable end), realised as the speed [`VERTICAL_SPEED`] (about a nanometre of drift per metre of height) so
//! that the surface stays continuous and every face keeps a plan polygon; its face is reported with `speed == 0`. All speeds
//! equal is the classic straight skeleton.
//!
//! The wavefront is simulated event by event (Felkel and Obdrzalek): an edge event collapses an edge to a point, a split event lets a
//! reflex vertex run into an edge and splits the wavefront ring in two (or merges an outer ring with a hole). Every event is
//! validated against the live wavefront when it is popped, simultaneous events are ordered deterministically and a ring that shrinks
//! to two vertices is closed at once, so symmetric inputs (rectangles, squares, plus and H shapes) are handled without special cases.
//! Cost is O(n^2 log n) in the number of edges; a `control` callback is polled once per event and returns `false` to cancel.
//!
//! Faces are recovered from the planar graph of arcs, input edges and (when a horizon stops the simulation early) the wavefront
//! pieces of the horizon, so a face of an edge is the exact region swept by it. Related:
//! <https://en.wikipedia.org/wiki/Straight_skeleton>, Aichholzer et al., "A novel type of skeleton for polygons" (1995).

use crate::vector::{perp, unit, LENGTH_EPS};
use crate::{Point, Vec2};
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap};

type V2 = [f64; 2];

fn sub(a: V2, b: V2) -> V2 {
    [a[0] - b[0], a[1] - b[1]]
}

fn add(a: V2, b: V2) -> V2 {
    [a[0] + b[0], a[1] + b[1]]
}

fn scale(a: V2, k: f64) -> V2 {
    [a[0] * k, a[1] * k]
}

fn dot(a: V2, b: V2) -> f64 {
    a[0] * b[0] + a[1] * b[1]
}

fn cross2(a: V2, b: V2) -> f64 {
    a[0] * b[1] - a[1] * b[0]
}

fn dist(a: V2, b: V2) -> f64 {
    (a[0] - b[0]).hypot(a[1] - b[1])
}

fn xy(p: Point) -> V2 {
    [p.x, p.y]
}

fn point(p: V2) -> Point {
    Point::new(p[0], p[1])
}

//#region 🔖️Values
/// 🔼 The speed that stands in for a vertical edge (input speed zero): the nearest realisable skeleton, one part in a hundred million.
pub const VERTICAL_SPEED: f64 = 1e-8;

/// 🧱️ One ring of the input: closed polygon vertices, the speed of every edge (edge `i` runs from vertex `i` to vertex `i + 1`) and a
/// caller tag per edge that is carried to the faces and the wavefront.
#[derive(Clone, Debug, PartialEq)]
pub struct Ring {
    pub points: Vec<Point>,
    pub speeds: Vec<f64>,
    pub tags: Vec<u32>,
}

impl Ring {
    /// 🧱️ A ring whose edges all move at `speed` and are tagged with their index in the ring.
    pub fn uniform(points: &[Point], speed: f64) -> Self {
        Self { points: points.to_vec(), speeds: vec![speed; points.len()], tags: (0..points.len() as u32).collect() }
    }
}

/// 📍️ A skeleton node: a plan position and the time (the height of a roof) at which the wavefront reaches it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SkeletonNode {
    pub point: Point,
    pub time: f64,
}

/// 📏️ The trace of one wavefront vertex between two nodes; `reflex` marks the trace of a reflex vertex (a valley of a roof).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SkeletonArc {
    pub from: u32,
    pub to: u32,
    pub reflex: bool,
}

/// ▭️ The region swept by one input edge: its nodes in counter-clockwise order. A face of a zero-speed edge is vertical (its nodes
/// lie on the edge line and their times are heights above it).
#[derive(Clone, Debug, PartialEq)]
pub struct SkeletonFace {
    pub ring: usize,
    pub edge: usize,
    pub tag: u32,
    pub speed: f64,
    pub nodes: Vec<u32>,
}

/// 🌊️ One ring of the wavefront when the simulation stopped at a horizon: positions at that time, with the speed and tag of the
/// edge that follows each vertex. Its orientation keeps the material on the left, so it can be fed back as an input [`Ring`].
#[derive(Clone, Debug, PartialEq)]
pub struct WavefrontRing {
    pub points: Vec<Point>,
    pub speeds: Vec<f64>,
    pub tags: Vec<u32>,
}

/// 🦴️ The skeleton: nodes (the first ones are the input vertices at time zero), arcs, one face per input edge and, for a run stopped
/// at a horizon, the wavefront left at that time.
#[derive(Clone, Debug, PartialEq)]
pub struct Skeleton {
    pub nodes: Vec<SkeletonNode>,
    pub arcs: Vec<SkeletonArc>,
    pub faces: Vec<SkeletonFace>,
    pub wavefront: Vec<WavefrontRing>,
    pub input_nodes: Vec<Vec<u32>>,
    pub horizon: Option<f64>,
}

impl Skeleton {
    /// 📐️ Plan area of the polygon of a face, zero for a vertical face.
    pub fn face_area(&self, face: &SkeletonFace) -> f64 {
        let points: Vec<V2> = face.nodes.iter().map(|&n| xy(self.nodes[n as usize].point)).collect();
        (0..points.len()).map(|i| cross2(points[i], points[(i + 1) % points.len()])).sum::<f64>() / 2.0
    }

    /// 🏔️ The latest time of any node: the height of the highest ridge.
    pub fn peak(&self) -> f64 {
        self.nodes.iter().map(|node| node.time).fold(0.0, f64::max)
    }
}

/// ⚠️ Why a skeleton cannot be built.
#[derive(Clone, Debug, PartialEq)]
pub enum SkeletonError {
    NoRings,
    TooFewVertices { ring: usize },
    NonFinite,
    InvalidSpeed { ring: usize, edge: usize },
    DegenerateEdge { ring: usize, edge: usize },
    Spike { ring: usize, vertex: usize },
    SelfIntersecting,
    SpeedMismatch { ring: usize, vertex: usize },
    Cancelled,
    NoConvergence,
}

impl std::fmt::Display for SkeletonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for SkeletonError {}
//#endregion 🔖️Values

//#region 🔖️Simulation
struct Line {
    d: V2,
    n: V2,
    c: f64,
    speed: f64,
    tag: u32,
}

struct Vertex {
    e_in: usize,
    e_out: usize,
    prev: usize,
    next: usize,
    p0: V2,
    t0: f64,
    v: V2,
    node: u32,
    alive: bool,
    reflex: bool,
    frozen: bool,
}

impl Vertex {
    fn at(&self, t: f64) -> V2 {
        add(self.p0, scale(self.v, t - self.t0))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Kind {
    Edge,
    Retract,
    Split,
}

#[derive(Clone, Copy, PartialEq)]
struct Event {
    time: f64,
    kind: Kind,
    a: usize,
    b: usize,
}

impl Eq for Event {}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        other.time.total_cmp(&self.time).then(other.kind.cmp(&self.kind)).then(other.a.cmp(&self.a)).then(other.b.cmp(&self.b))
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn velocity(n1: V2, s1: f64, n2: V2, s2: f64) -> Option<(V2, bool)> {
    let det = cross2(n1, n2);
    if det.abs() > 1e-12 {
        return Some(([(s1 * n2[1] - s2 * n1[1]) / det, (n1[0] * s2 - n2[0] * s1) / det], false));
    }
    if dot(n1, n2) > 0.0 {
        return ((s1 - s2).abs() <= 1e-9 * (1.0 + s1.abs())).then_some((scale(n1, s1), false));
    }
    Some(([0.0, 0.0], true))
}

struct Sim {
    lines: Vec<Line>,
    verts: Vec<Vertex>,
    nodes: Vec<SkeletonNode>,
    arcs: Vec<SkeletonArc>,
    queue: BinaryHeap<Event>,
    eps: f64,
}

impl Sim {
    fn node_at(&mut self, p: V2, t: f64) -> u32 {
        if let Some(found) = self.nodes.iter().position(|node| dist(xy(node.point), p) <= self.eps && (node.time - t).abs() <= self.eps) {
            return found as u32;
        }
        self.nodes.push(SkeletonNode { point: point(p), time: t });
        (self.nodes.len() - 1) as u32
    }

    fn arc(&mut self, from: u32, to: u32, reflex: bool) {
        if from != to {
            self.arcs.push(SkeletonArc { from, to, reflex });
        }
    }

    fn kill(&mut self, v: usize, node: u32) {
        self.verts[v].alive = false;
        let (from, reflex) = (self.verts[v].node, self.verts[v].reflex);
        self.arc(from, node, reflex);
    }

    fn cycle(&self, start: usize) -> Vec<usize> {
        let mut out = vec![start];
        let mut at = self.verts[start].next;
        while at != start && out.len() <= self.verts.len() {
            out.push(at);
            at = self.verts[at].next;
        }
        out
    }

    fn spawn(&mut self, e_in: usize, e_out: usize, prev: usize, next: usize, p: V2, t: f64, node: u32) -> Result<usize, SkeletonError> {
        let (a, b) = (&self.lines[e_in], &self.lines[e_out]);
        let (v, frozen) = velocity(a.n, a.speed, b.n, b.speed).ok_or(SkeletonError::SpeedMismatch { ring: 0, vertex: 0 })?;
        let reflex = cross2(a.d, b.d) < -1e-12;
        self.verts.push(Vertex { e_in, e_out, prev, next, p0: p, t0: t, v, node, alive: true, reflex, frozen });
        Ok(self.verts.len() - 1)
    }

    fn edge_time(&self, a: usize, b: usize) -> Option<f64> {
        let (va, vb) = (&self.verts[a], &self.verts[b]);
        let line = &self.lines[va.e_out];
        let tr = va.t0.max(vb.t0);
        let gap = dot(line.d, sub(vb.at(tr), va.at(tr)));
        let rate = dot(line.d, sub(vb.v, va.v));
        if gap <= self.eps {
            return (rate <= 1e-12).then_some(tr);
        }
        (rate < -1e-12).then(|| tr + gap / -rate)
    }

    fn schedule_edge(&mut self, a: usize, b: usize) {
        if a != b {
            if let Some(time) = self.edge_time(a, b) {
                self.queue.push(Event { time, kind: Kind::Edge, a, b });
            }
        }
    }

    fn schedule_splits(&mut self, v: usize) {
        if !self.verts[v].reflex {
            return;
        }
        let vertex = &self.verts[v];
        let candidates: Vec<(f64, usize)> = (0..self.lines.len())
            .filter(|&l| l != vertex.e_in && l != vertex.e_out)
            .filter_map(|l| {
                let line = &self.lines[l];
                let delta = dot(line.n, vertex.p0) - line.c - line.speed * vertex.t0;
                let rate = dot(line.n, vertex.v) - line.speed;
                (delta >= -self.eps && rate < -1e-12).then(|| (vertex.t0 + delta.max(0.0) / -rate, l))
            })
            .collect();
        for (time, l) in candidates {
            self.queue.push(Event { time, kind: Kind::Split, a: v, b: l });
        }
    }

    fn arrive(&mut self, w: usize) {
        if self.verts[w].frozen {
            let time = self.verts[w].t0;
            self.queue.push(Event { time, kind: Kind::Retract, a: w, b: w });
        }
        let (prev, next) = (self.verts[w].prev, self.verts[w].next);
        self.schedule_edge(prev, w);
        self.schedule_edge(w, next);
        self.schedule_splits(w);
    }

    fn finalize(&mut self, start: usize, t: f64) {
        let ids = self.cycle(start);
        let nodes: Vec<u32> = ids.iter().map(|&v| {
            let p = self.verts[v].at(t);
            self.node_at(p, t)
        }).collect();
        for (&v, &node) in ids.iter().zip(&nodes) {
            self.kill(v, node);
        }
        if ids.len() == 2 {
            self.arc(nodes[0], nodes[1], false);
        }
    }

    fn edge_event(&mut self, a: usize, b: usize, t: f64) -> Result<(), SkeletonError> {
        let at = scale(add(self.verts[a].at(t), self.verts[b].at(t)), 0.5);
        let node = self.node_at(at, t);
        let (p, q) = (self.verts[a].prev, self.verts[b].next);
        let (e_in, e_out) = (self.verts[a].e_in, self.verts[b].e_out);
        self.kill(a, node);
        self.kill(b, node);
        if p == b {
            return Ok(());
        }
        let w = self.spawn(e_in, e_out, p, q, at, t, node)?;
        self.verts[p].next = w;
        self.verts[q].prev = w;
        if self.cycle(w).len() <= 2 {
            self.finalize(w, t);
        } else {
            self.arrive(w);
        }
        Ok(())
    }

    fn retract_event(&mut self, w: usize, t: f64) -> Result<(), SkeletonError> {
        let (p, n) = (self.verts[w].prev, self.verts[w].next);
        let here = self.verts[w].at(t);
        let (to_prev, to_next) = (dist(here, self.verts[p].at(t)), dist(here, self.verts[n].at(t)));
        let toward_next = to_next <= to_prev;
        let other = if toward_next { n } else { p };
        let x = self.verts[other].at(t);
        let node = self.node_at(x, t);
        let (e_in, e_out) = if toward_next { (self.verts[w].e_in, self.verts[other].e_out) } else { (self.verts[other].e_in, self.verts[w].e_out) };
        let (before, after) = if toward_next { (p, self.verts[other].next) } else { (self.verts[other].prev, n) };
        self.kill(w, node);
        self.kill(other, node);
        let merged = self.spawn(e_in, e_out, before, after, x, t, node)?;
        self.verts[before].next = merged;
        self.verts[after].prev = merged;
        if self.cycle(merged).len() <= 2 {
            self.finalize(merged, t);
        } else {
            self.arrive(merged);
        }
        Ok(())
    }

    fn piece_at(&self, line: usize, x: V2, t: f64, skip: usize) -> Option<(usize, usize)> {
        let d = self.lines[line].d;
        let sx = dot(d, x);
        self.verts.iter().enumerate().filter(|(i, u)| u.alive && u.e_out == line && *i != skip && u.next != skip).find_map(|(i, u)| {
            let end = &self.verts[u.next];
            let (s0, s1) = (dot(d, u.at(t)), dot(d, end.at(t)));
            (sx >= s0 - self.eps && sx <= s1 + self.eps && s0 <= s1 + self.eps).then_some((i, u.next))
        })
    }

    fn split_event(&mut self, v: usize, line: usize, t: f64) -> Result<(), SkeletonError> {
        let x = self.verts[v].at(t);
        let Some((u, u_next)) = self.piece_at(line, x, t, v) else {
            return Ok(());
        };
        let node = self.node_at(x, t);
        let (p, n) = (self.verts[v].prev, self.verts[v].next);
        let (e_in, e_out) = (self.verts[v].e_in, self.verts[v].e_out);
        self.kill(v, node);
        let first = self.spawn(e_in, line, p, u_next, x, t, node)?;
        let second = self.spawn(line, e_out, u, n, x, t, node)?;
        self.verts[p].next = first;
        self.verts[u_next].prev = first;
        self.verts[u].next = second;
        self.verts[n].prev = second;
        let merged = self.cycle(first).contains(&second);
        let starts: &[usize] = if merged { &[first] } else { &[first, second] };
        for &start in starts {
            let members = self.cycle(start);
            if members.len() <= 2 {
                self.finalize(start, t);
            } else {
                for w in [first, second].into_iter().filter(|w| members.contains(w)) {
                    self.arrive(w);
                }
            }
        }
        Ok(())
    }
}
//#endregion 🔖️Simulation

//#region 🔖️Input
struct Prepared {
    rings: Vec<Vec<V2>>,
    speeds: Vec<Vec<f64>>,
    tags: Vec<Vec<u32>>,
    origin: Vec<Vec<usize>>,
    vertex_of: Vec<Vec<usize>>,
}

fn signed_area(points: &[V2]) -> f64 {
    (0..points.len()).map(|i| cross2(points[i], points[(i + 1) % points.len()])).sum::<f64>() / 2.0
}

fn segments_cross(a: (V2, V2), b: (V2, V2), eps: f64) -> bool {
    let side = |p: V2, q: V2, r: V2| cross2(sub(q, p), sub(r, p));
    let (d1, d2, d3, d4) = (side(b.0, b.1, a.0), side(b.0, b.1, a.1), side(a.0, a.1, b.0), side(a.0, a.1, b.1));
    let near = |p: V2, q: V2, r: V2| side(p, q, r).abs() <= eps * dist(p, q).max(1.0) && dot(sub(r, p), sub(r, q)) <= eps * eps;
    (d1 * d2 < 0.0 && d3 * d4 < 0.0) || near(b.0, b.1, a.0) || near(b.0, b.1, a.1) || near(a.0, a.1, b.0) || near(a.0, a.1, b.1)
}

fn prepare(rings: &[Ring], eps: f64) -> Result<Prepared, SkeletonError> {
    if rings.is_empty() {
        return Err(SkeletonError::NoRings);
    }
    let mut out = Prepared { rings: Vec::new(), speeds: Vec::new(), tags: Vec::new(), origin: Vec::new(), vertex_of: Vec::new() };
    for (r, ring) in rings.iter().enumerate() {
        let n = ring.points.len();
        if n < 3 || ring.speeds.len() != n || ring.tags.len() != n {
            return Err(SkeletonError::TooFewVertices { ring: r });
        }
        if !ring.points.iter().all(|p| p.x.is_finite() && p.y.is_finite()) {
            return Err(SkeletonError::NonFinite);
        }
        if let Some(edge) = ring.speeds.iter().position(|s| !(s.is_finite() && *s >= 0.0)) {
            return Err(SkeletonError::InvalidSpeed { ring: r, edge });
        }
        let points: Vec<V2> = ring.points.iter().map(|p| xy(*p)).collect();
        if let Some(edge) = (0..n).position(|i| dist(points[i], points[(i + 1) % n]) <= eps) {
            return Err(SkeletonError::DegenerateEdge { ring: r, edge });
        }
        let area = signed_area(&points);
        let wanted_ccw = r == 0;
        if (area > 0.0) == wanted_ccw {
            out.rings.push(points);
            out.speeds.push(ring.speeds.clone());
            out.tags.push(ring.tags.clone());
            out.origin.push((0..n).collect());
            out.vertex_of.push((0..n).collect());
        } else {
            out.rings.push(points.iter().rev().copied().collect());
            out.speeds.push((0..n).map(|j| ring.speeds[(2 * n - 2 - j) % n]).collect());
            out.tags.push((0..n).map(|j| ring.tags[(2 * n - 2 - j) % n]).collect());
            out.origin.push((0..n).map(|j| (2 * n - 2 - j) % n).collect());
            out.vertex_of.push((0..n).map(|i| n - 1 - i).collect());
        }
    }
    let edges: Vec<(usize, V2, V2)> = out.rings.iter().enumerate().flat_map(|(r, ring)| (0..ring.len()).map(move |i| (r, ring[i], ring[(i + 1) % ring.len()]))).collect();
    let mut offsets = vec![0];
    for ring in &out.rings {
        offsets.push(offsets.last().unwrap() + ring.len());
    }
    for (i, a) in edges.iter().enumerate() {
        for (j, b) in edges.iter().enumerate().skip(i + 1) {
            let adjacent = a.0 == b.0 && {
                let n = out.rings[a.0].len();
                let (ia, ib) = (i - offsets[a.0], j - offsets[b.0]);
                (ia + 1) % n == ib || (ib + 1) % n == ia
            };
            if !adjacent && segments_cross((a.1, a.2), (b.1, b.2), eps) {
                return Err(SkeletonError::SelfIntersecting);
            }
        }
    }
    Ok(out)
}
//#endregion 🔖️Input

//#region 🔖️Run
/// 🦴️ The straight skeleton of `rings` (the first is the outer boundary, the rest are holes; either orientation) run to the end.
pub fn straight_skeleton(rings: &[Ring]) -> Result<Skeleton, SkeletonError> {
    straight_skeleton_until(rings, None, &mut || true)
}

/// ⏱️ The skeleton up to `horizon` (the whole skeleton when `None`); the wavefront at the horizon is returned beside the faces swept
/// until then. `control` is polled once per event; returning `false` cancels with [`SkeletonError::Cancelled`].
pub fn straight_skeleton_until(rings: &[Ring], horizon: Option<f64>, control: &mut dyn FnMut() -> bool) -> Result<Skeleton, SkeletonError> {
    let extent = rings.iter().flat_map(|ring| ring.points.iter()).fold(1.0_f64, |acc, p| acc.max(p.x.abs()).max(p.y.abs()));
    let eps = LENGTH_EPS * extent;
    let prepared = prepare(rings, eps)?;
    let mut sim = Sim { lines: Vec::new(), verts: Vec::new(), nodes: Vec::new(), arcs: Vec::new(), queue: BinaryHeap::new(), eps };
    let mut input_nodes: Vec<Vec<u32>> = Vec::new();
    let mut base_nodes: Vec<u32> = Vec::new();
    let mut ring_edges: Vec<Vec<usize>> = Vec::new();
    for (r, points) in prepared.rings.iter().enumerate() {
        let n = points.len();
        let base = sim.lines.len();
        for i in 0..n {
            let d = unit(Vec2::new(points[(i + 1) % n][0] - points[i][0], points[(i + 1) % n][1] - points[i][1]), eps).expect("edge length checked");
            let normal = perp(d);
            let (d, normal) = ([d.x, d.y], [normal.x, normal.y]);
            sim.lines.push(Line { d, n: normal, c: dot(normal, points[i]), speed: prepared.speeds[r][i].max(VERTICAL_SPEED), tag: prepared.tags[r][i] });
        }
        ring_edges.push((base..base + n).collect());
        let first = sim.nodes.len() as u32;
        base_nodes.push(first);
        sim.nodes.extend(points.iter().map(|p| SkeletonNode { point: point(*p), time: 0.0 }));
        input_nodes.push((0..n).map(|i| first + prepared.vertex_of[r][i] as u32).collect());
    }
    let mut started = 0;
    for (r, points) in prepared.rings.iter().enumerate() {
        let n = points.len();
        let first = sim.verts.len();
        for i in 0..n {
            let (e_in, e_out) = (ring_edges[r][(i + n - 1) % n], ring_edges[r][i]);
            let (a, b) = (&sim.lines[e_in], &sim.lines[e_out]);
            let (v, spike) = velocity(a.n, a.speed, b.n, b.speed).ok_or(SkeletonError::SpeedMismatch { ring: r, vertex: i })?;
            if spike {
                return Err(SkeletonError::Spike { ring: r, vertex: prepared.origin[r][i] });
            }
            let reflex = cross2(a.d, b.d) < -1e-12;
            sim.verts.push(Vertex { e_in, e_out, prev: first + (i + n - 1) % n, next: first + (i + 1) % n, p0: points[i], t0: 0.0, v, node: (started + i) as u32, alive: true, reflex, frozen: false });
        }
        started += n;
    }
    for w in 0..sim.verts.len() {
        let next = sim.verts[w].next;
        sim.schedule_edge(w, next);
        sim.schedule_splits(w);
    }
    let cap = 64 * sim.lines.len() * sim.lines.len() + 1024;
    let (mut steps, mut now) = (0, 0.0_f64);
    while let Some(event) = sim.queue.pop() {
        if horizon.is_some_and(|h| event.time > h) {
            break;
        }
        if !control() {
            return Err(SkeletonError::Cancelled);
        }
        steps += 1;
        if steps > cap {
            return Err(SkeletonError::NoConvergence);
        }
        now = now.max(event.time);
        match event.kind {
            Kind::Split if sim.verts[event.a].alive => sim.split_event(event.a, event.b, now)?,
            Kind::Retract if sim.verts[event.a].alive && sim.cycle(event.a).len() > 2 => sim.retract_event(event.a, now)?,
            Kind::Edge if sim.verts[event.a].alive && sim.verts[event.b].alive && sim.verts[event.a].next == event.b => sim.edge_event(event.a, event.b, now)?,
            _ => {}
        }
    }
    let end = horizon.unwrap_or(now);
    let mut wavefront = Vec::new();
    let mut wave_nodes: Vec<Vec<u32>> = Vec::new();
    let mut seen = BTreeSet::new();
    for start in 0..sim.verts.len() {
        if !sim.verts[start].alive || seen.contains(&start) {
            continue;
        }
        let ids = sim.cycle(start);
        seen.extend(ids.iter().copied());
        let nodes: Vec<u32> = ids.iter().map(|&v| {
            let p = sim.verts[v].at(end);
            sim.node_at(p, end)
        }).collect();
        for (&v, &node) in ids.iter().zip(&nodes) {
            sim.kill(v, node);
        }
        if horizon.is_some() {
            wavefront.push(WavefrontRing {
                points: nodes.iter().map(|&n| sim.nodes[n as usize].point).collect(),
                speeds: ids.iter().map(|&v| sim.lines[sim.verts[v].e_out].speed).map(|speed| if speed <= VERTICAL_SPEED { 0.0 } else { speed }).collect(),
                tags: ids.iter().map(|&v| sim.lines[sim.verts[v].e_out].tag).collect(),
            });
            wave_nodes.push(nodes);
        } else if ids.len() == 2 {
            sim.arc(nodes[0], nodes[1], false);
        }
    }
    let arcs = node_arcs(&sim.nodes, &sim.arcs, eps);
    let faces = faces_of(&sim, &arcs, &prepared, &base_nodes, &wave_nodes)?;
    Ok(Skeleton { nodes: sim.nodes, arcs, faces, wavefront, input_nodes, horizon })
}
//#endregion 🔖️Run

//#region 🔖️Faces
fn node_arcs(nodes: &[SkeletonNode], arcs: &[SkeletonArc], eps: f64) -> Vec<SkeletonArc> {
    let mut out: Vec<SkeletonArc> = Vec::new();
    for arc in arcs {
        let (a, b) = (xy(nodes[arc.from as usize].point), xy(nodes[arc.to as usize].point));
        let (t0, t1) = (nodes[arc.from as usize].time, nodes[arc.to as usize].time);
        let length = dist(a, b);
        let direction = if length > 0.0 { scale(sub(b, a), 1.0 / length) } else { [0.0, 0.0] };
        let mut cuts: Vec<(f64, u32)> = nodes
            .iter()
            .enumerate()
            .filter(|(i, _)| *i as u32 != arc.from && *i as u32 != arc.to)
            .filter_map(|(i, node)| {
                let rel = sub(xy(node.point), a);
                let s = dot(rel, direction);
                let on_line = cross2(direction, rel).abs() <= 10.0 * eps && s > 10.0 * eps && s < length - 10.0 * eps;
                (on_line && (node.time - (t0 + (t1 - t0) * s / length)).abs() <= 100.0 * eps).then_some((s, i as u32))
            })
            .collect();
        cuts.sort_by(|x, y| x.0.total_cmp(&y.0));
        let mut from = arc.from;
        for (_, cut) in cuts.into_iter().chain(std::iter::once((length, arc.to))) {
            if !out.iter().any(|other| (other.from, other.to) == (from, cut) || (other.from, other.to) == (cut, from)) {
                out.push(SkeletonArc { from, to: cut, reflex: arc.reflex });
            }
            from = cut;
        }
    }
    out
}

fn faces_of(sim: &Sim, arcs: &[SkeletonArc], prepared: &Prepared, base: &[u32], wave_nodes: &[Vec<u32>]) -> Result<Vec<SkeletonFace>, SkeletonError> {
    let nodes = &sim.nodes;
    let mut graph: BTreeMap<u32, BTreeSet<u32>> = BTreeMap::new();
    let mut link = |a: u32, b: u32| {
        if a != b && dist(xy(nodes[a as usize].point), xy(nodes[b as usize].point)) > sim.eps {
            graph.entry(a).or_default().insert(b);
            graph.entry(b).or_default().insert(a);
        }
    };
    for arc in arcs {
        link(arc.from, arc.to);
    }
    let mut starts: Vec<(usize, usize, u32, u32)> = Vec::new();
    for (r, ring) in prepared.rings.iter().enumerate() {
        let n = ring.len();
        for i in 0..n {
            let (a, b) = (base[r] + i as u32, base[r] + ((i + 1) % n) as u32);
            link(a, b);
            starts.push((r, prepared.origin[r][i], a, b));
        }
    }
    for ring in wave_nodes {
        for i in 0..ring.len() {
            link(ring[i], ring[(i + 1) % ring.len()]);
        }
    }
    let angle = |from: u32, to: u32| {
        let (a, b) = (xy(nodes[from as usize].point), xy(nodes[to as usize].point));
        (b[1] - a[1]).atan2(b[0] - a[0])
    };
    let sorted: BTreeMap<u32, Vec<u32>> = graph.iter().map(|(&node, neighbours)| {
        let mut list: Vec<u32> = neighbours.iter().copied().collect();
        list.sort_by(|&x, &y| angle(node, x).total_cmp(&angle(node, y)));
        (node, list)
    }).collect();
    let mut faces = Vec::new();
    for (ring, edge, a, b) in starts {
        let mut polygon = vec![a];
        let mut current = (a, b);
        let mut guard = 0;
        loop {
            let (u, v) = current;
            let list = sorted.get(&v).ok_or(SkeletonError::NoConvergence)?;
            let at = list.iter().position(|&w| w == u).ok_or(SkeletonError::NoConvergence)?;
            let next = list[(at + list.len() - 1) % list.len()];
            current = (v, next);
            if current == (a, b) {
                break;
            }
            polygon.push(v);
            guard += 1;
            if guard > 4 * nodes.len() + 8 {
                return Err(SkeletonError::NoConvergence);
            }
        }
        let normalized = prepared.origin[ring].iter().position(|&o| o == edge).expect("edge permutation");
        faces.push(SkeletonFace { ring, edge, tag: prepared.tags[ring][normalized], speed: prepared.speeds[ring][normalized], nodes: polygon });
    }
    faces.sort_by_key(|face| (face.ring, face.edge));
    Ok(faces)
}

//#endregion 🔖️Faces

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
