//! 🧭️ Planar-map face splitting for the boolean imprint engine: one face's boundary rings and the imprint pieces queued on it are
//! one graph in the face's `(u, v)` chart, the faces of that graph are traced by the rotation of tangent directions at every node,
//! and the traced cycles become the loops of the replacement faces. One algorithm serves every surface kind, so a pole, a cone
//! apex, a periodic seam, a face that already carries holes and a closed imprint are all just nodes and edges of the same map.
//!
//! Chart conventions: a pole (or apex) is a boundary LINE of the chart, so one vertex stands for many nodes, one per `u` it is
//! attached at; a seam is one edge whose two sides are the same region, so the cycles on its two sides are one face.

use super::*;
use crate::brep::operations::euler::{add_face, make_loop, split_edge_with_vertex};
use crate::brep::representation::curve::curve_ops::{interpolate_curve, ParamMethod};
use crate::brep::representation::curve::Curve3;
use crate::brep::representation::surface::surface_ops::closest_uv;
use std::f64::consts::{PI, TAU};

const NO_POLE: i64 = i64::MIN;
const TANGENT_EPSILON: f64 = 1e-9;
const FIT_SAMPLE_LIMIT: usize = 1024;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct NodeKey(u32, i64);

/// 🧭️ The parameter chart of one face's surface: its periods and its degenerate (collapsed) boundary lines.
struct Chart {
    surface: Surface,
    u_period: Option<f64>,
    v_period: Option<f64>,
}

impl Chart {
    fn new(surface: Surface) -> Self {
        let (u_period, v_period) = (surface.is_u_periodic().then_some(TAU), surface.is_v_periodic().then_some(TAU));
        Self { surface, u_period, v_period }
    }

    fn canonical_u(&self, u: f64) -> f64 {
        match self.u_period {
            Some(period) => {
                let reduced = u - (u / period).floor() * period;
                if period - reduced < 1e-9 { 0.0 } else { reduced }
            }
            None => u,
        }
    }

    fn key(&self, vertex: VertexId, uv: Pnt2) -> NodeKey {
        if self.surface.is_degenerate_uv(uv.x, uv.y) {
            NodeKey(vertex.raw_index(), (self.canonical_u(uv.x) * 1e7).round() as i64)
        } else {
            NodeKey(vertex.raw_index(), NO_POLE)
        }
    }

    fn shift_towards(&self, reference: (f64, f64), uv: (f64, f64)) -> (f64, f64) {
        let along = |period: Option<f64>, target: f64, raw: f64| period.map_or(0.0, |p| ((target - raw) / p).round() * p);
        (along(self.u_period, reference.0, uv.0), along(self.v_period, reference.1, uv.1))
    }
}

/// 🧭️ One traversal of one edge inside the face: a boundary coedge, or one direction of an imprint piece.
#[derive(Clone)]
struct Half {
    edge: EdgeId,
    forward: bool,
    pcurve: Curve2Id,
    prange: (f64, f64),
    from: usize,
    to: usize,
    start: (f64, f64),
    end: (f64, f64),
    twin: Option<usize>,
    seam: bool,
}

struct Graph {
    chart: Chart,
    pcurves: HashMap<Curve2Id, Curve2>,
    nodes: HashMap<NodeKey, usize>,
    halves: Vec<Half>,
    outgoing: Vec<Vec<usize>>,
}

/// 🧭️ A closed cycle of halves with the period shifts that make its p-curves one continuous polygon.
struct Ring {
    halves: Vec<usize>,
    shifts: Vec<(f64, f64)>,
    polygon: Vec<(f64, f64)>,
    area: f64,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn normalize(angle: f64) -> f64 {
    let reduced = angle - (angle / TAU).floor() * TAU;
    if reduced >= TAU { 0.0 } else { reduced }
}

impl Graph {
    fn node(&mut self, vertex: VertexId, uv: (f64, f64)) -> usize {
        let key = self.chart.key(vertex, Pnt2::new(uv.0, uv.1));
        let next = self.nodes.len();
        let index = *self.nodes.entry(key).or_insert(next);
        if index == self.outgoing.len() {
            self.outgoing.push(Vec::new());
        }
        index
    }

    fn add_half(&mut self, body: &Body, edge: EdgeId, forward: bool, pcurve: Curve2Id, prange: (f64, f64)) -> Result<usize, KernelError> {
        let data = body.edges.get(edge).ok_or_else(|| KernelError::MissingEntity(format!("edge {edge}")))?;
        let curve = self.pcurves.get(&pcurve).ok_or_else(|| KernelError::MissingEntity(format!("curve2 {pcurve}")))?;
        let (t_start, t_end) = if forward { prange } else { (prange.1, prange.0) };
        let (a, b) = (curve.eval(t_start), curve.eval(t_end));
        let (v_start, v_end) = if forward { (data.v0, data.v1) } else { (data.v1, data.v0) };
        let (start, end) = ((a.x, a.y), (b.x, b.y));
        let (from, to) = (self.node(v_start, start), self.node(v_end, end));
        let index = self.halves.len();
        self.halves.push(Half { edge, forward, pcurve, prange, from, to, start, end, twin: None, seam: false });
        self.outgoing[from].push(index);
        Ok(index)
    }

    /// 🧭️ Direction angle and signed curvature of `half` leaving its start (`at_end == false`) or leaving its end backwards.
    fn leave(&self, half: &Half, at_end: bool) -> (f64, f64) {
        let curve = &self.pcurves[&half.pcurve];
        let (t_start, t_end) = if half.forward { half.prange } else { (half.prange.1, half.prange.0) };
        let direction = if t_end >= t_start { 1.0 } else { -1.0 };
        let (t, sign) = if at_end { (t_end, -direction) } else { (t_start, direction) };
        let mut d1 = curve.d1(t);
        d1 = Vec2::new(d1.x * sign, d1.y * sign);
        if d1.x.hypot(d1.y) < 1e-14 {
            let probe = curve.eval(t + sign * 1e-4 * (t_end - t_start).abs().max(1e-9));
            let here = curve.eval(t);
            d1 = Vec2::new(probe.x - here.x, probe.y - here.y);
        }
        let d2 = curve.d2(t);
        let speed = d1.x.hypot(d1.y).max(1e-300);
        (normalize(d1.y.atan2(d1.x)), (d1.x * d2.y - d1.y * d2.x) / (speed * speed * speed))
    }

    fn next(&self, index: usize) -> Option<usize> {
        let half = &self.halves[index];
        let (reference, reference_curvature) = self.leave(half, true);
        let mut best: Option<(f64, usize)> = None;
        for &candidate in &self.outgoing[half.to] {
            let (angle, curvature) = self.leave(&self.halves[candidate], false);
            let mut delta = normalize(reference - angle);
            if Some(candidate) == half.twin {
                delta = 2.0 * TAU;
            } else if delta < TANGENT_EPSILON || delta > TAU - TANGENT_EPSILON {
                delta = if curvature < reference_curvature { 1e-10 * (1.0 + (reference_curvature - curvature).abs()) } else { TAU - 1e-10 * (1.0 + (curvature - reference_curvature).abs()) };
            }
            if best.is_none_or(|(d, _)| delta < d) {
                best = Some((delta, candidate));
            }
        }
        best.map(|(_, candidate)| candidate)
    }

    fn trace(&self) -> Result<Vec<Vec<usize>>, KernelError> {
        let mut visited = vec![false; self.halves.len()];
        let mut cycles = Vec::new();
        for start in 0..self.halves.len() {
            if visited[start] {
                continue;
            }
            let mut cycle = Vec::new();
            let mut cursor = start;
            loop {
                visited[cursor] = true;
                cycle.push(cursor);
                cursor = self.next(cursor).ok_or_else(|| KernelError::Boolean(BooleanError::ImprintFailed("imprint graph has a dead-end node".into())))?;
                if cursor == start {
                    break;
                }
                if visited[cursor] {
                    return Err(KernelError::Boolean(BooleanError::ImprintFailed("imprint graph cycle re-entered itself".into())));
                }
            }
            cycles.push(cycle);
        }
        Ok(cycles)
    }

    /// 🧭️ Joins the cycles that touch the two sides of one seam edge: a seam bounds nothing, so both sides are one face.
    fn merge_across_seams(&self, mut cycles: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
        loop {
            let owner = |cycles: &Vec<Vec<usize>>, half: usize| cycles.iter().position(|cycle| cycle.contains(&half));
            let pair = (0..self.halves.len()).find_map(|h| {
                let half = &self.halves[h];
                let twin = half.twin.filter(|_| half.seam && h < half.twin.unwrap_or(0))?;
                let (x, y) = (owner(&cycles, h)?, owner(&cycles, twin)?);
                (x != y).then_some((h, twin, x, y))
            });
            let Some((h, twin, x, y)) = pair else { return cycles };
            let (cycle_x, cycle_y) = (cycles[x].clone(), cycles[y].clone());
            let (px, py) = (cycle_x.iter().position(|&i| i == h).unwrap_or(0), cycle_y.iter().position(|&i| i == twin).unwrap_or(0));
            let mut merged: Vec<usize> = cycle_x[..px].to_vec();
            merged.extend_from_slice(&cycle_y[py + 1..]);
            merged.extend_from_slice(&cycle_y[..py]);
            merged.extend_from_slice(&cycle_x[px + 1..]);
            let (low, high) = (x.min(y), x.max(y));
            cycles.remove(high);
            cycles[low] = merged;
        }
    }

    fn ring(&self, body: &Body, halves: Vec<usize>) -> Result<Ring, KernelError> {
        let mut shifts = Vec::with_capacity(halves.len());
        let mut polygon: Vec<(f64, f64)> = Vec::new();
        let mut cursor: Option<(f64, f64)> = None;
        for &index in &halves {
            let half = &self.halves[index];
            let shift = cursor.map_or((0.0, 0.0), |target| self.chart.shift_towards(target, half.start));
            let (t_start, t_end) = if half.forward { half.prange } else { (half.prange.1, half.prange.0) };
            let curve = &self.pcurves[&half.pcurve];
            const SAMPLES: usize = 8;
            for k in 0..SAMPLES {
                let p = curve.eval(t_start + (t_end - t_start) * k as f64 / SAMPLES as f64);
                polygon.push((p.x + shift.0, p.y + shift.1));
            }
            cursor = Some((half.end.0 + shift.0, half.end.1 + shift.1));
            shifts.push(shift);
        }
        let (first, last) = (polygon.first().copied().unwrap_or((0.0, 0.0)), cursor.unwrap_or((0.0, 0.0)));
        let scale = polygon.iter().fold(1.0f64, |m, p| m.max(p.0.abs()).max(p.1.abs()));
        if (first.0 - last.0).abs() > 1e-6 * scale || (first.1 - last.1).abs() > 1e-6 * scale {
            let _ = body;
            return Err(KernelError::Boolean(BooleanError::ImprintFailed(format!("a traced boundary does not close in the face's parameter chart (gap {:.3e}, {:.3e})", last.0 - first.0, last.1 - first.1))));
        }
        let area = 0.5 * polygon.iter().enumerate().map(|(i, p)| { let q = polygon[(i + 1) % polygon.len()]; p.0 * q.1 - q.0 * p.1 }).sum::<f64>();
        Ok(Ring { halves, shifts, polygon, area })
    }

    fn contains(&self, ring: &Ring, point: (f64, f64)) -> bool {
        let shifts = |period: Option<f64>| -> Vec<f64> { period.map_or(vec![0.0], |p| vec![-p, 0.0, p]) };
        for du in shifts(self.chart.u_period) {
            for dv in shifts(self.chart.v_period) {
                let (x, y) = (point.0 + du, point.1 + dv);
                let mut inside = false;
                let n = ring.polygon.len();
                for i in 0..n {
                    let (a, b) = (ring.polygon[i], ring.polygon[(i + 1) % n]);
                    if (a.1 > y) != (b.1 > y) && x < (b.0 - a.0) * (y - a.1) / (b.1 - a.1) + a.0 {
                        inside = !inside;
                    }
                }
                if inside {
                    return true;
                }
            }
        }
        false
    }
}

/// 🧭️ The p-curve of boundary coedge `edge` when the coedge carries none: a uniform-parameter fit on `surface`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn fitted_pcurve(surface: &Surface, curve: &Curve3, (t0, t1): (f64, f64), tol: f64) -> (Curve2, (f64, f64)) {
    if let (Surface::Plane { frame }, Curve3::Line { origin, dir }) = (surface, curve) {
        let (o, d) = (frame.to_local(*origin), frame.to_local_vector(*dir));
        return (Curve2::Line { origin: Pnt2::new(o.x, o.y), dir: Vec2::new(d.x, d.y) }, (t0, t1));
    }
    let domain = surface.domain();
    let mut samples = 8usize;
    loop {
        let params: Vec<f64> = (0..=samples).map(|i| i as f64 / samples as f64).collect();
        let mut uv: Vec<(f64, f64, bool)> = params
            .iter()
            .map(|&s| {
                let found = closest_uv(surface, domain, curve.eval(t0 + (t1 - t0) * s), tol.min(1e-12));
                (found.u, found.v, surface.is_degenerate_uv(found.u, found.v))
            })
            .collect();
        if let Some(anchor) = uv.iter().position(|p| !p.2) {
            for i in (0..anchor).rev() {
                uv[i].0 = uv[i + 1].0;
            }
            for i in anchor + 1..uv.len() {
                if uv[i].2 {
                    uv[i].0 = uv[i - 1].0;
                }
            }
        }
        for i in 1..uv.len() {
            if surface.is_u_periodic() {
                uv[i].0 = unwrap_to(uv[i - 1].0, uv[i].0);
            }
            if surface.is_v_periodic() {
                uv[i].1 = unwrap_to(uv[i - 1].1, uv[i].1);
            }
        }
        let points: Vec<Pnt3> = uv.iter().map(|p| Pnt3::new(p.0, p.1, 0.0)).collect();
        if let Some(fit) = interpolate_curve(&points, 3, ParamMethod::Uniform, None, false) {
            let pcurve = Curve2::Nurbs { knots: fit.knots.clone(), controls: fit.controls.iter().map(|p| Pnt2::new(p.x, p.y)).collect(), weights: fit.weights.clone() };
            let (lo, hi) = pcurve.domain();
            let mut worst = 0.0f64;
            for i in 0..samples {
                for k in 1..=4 {
                    let s = (i as f64 + k as f64 / 5.0) / samples as f64;
                    let p = pcurve.eval(lo + (hi - lo) * s);
                    worst = worst.max(surface.eval(p.x, p.y).distance(curve.eval(t0 + (t1 - t0) * s)));
                }
            }
            if worst <= tol || samples >= FIT_SAMPLE_LIMIT {
                return (pcurve, (lo, hi));
            }
        }
        samples *= 2;
    }
}

/// 🧭️ Makes `vertex` (the end of a queued piece, at chart point `uv`) a vertex of the face's boundary when it lies on that boundary.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn attach_to_boundary(body: &mut Body, face: FaceId, chart: &Chart, vertex: VertexId, uv: Pnt2, tol: f64, rec: &mut OpRecorder) -> Result<(), KernelError> {
    let linear = tol.max(1e-9);
    let key = chart.key(vertex, uv);
    let position = body.vertices.get(vertex).ok_or_else(|| KernelError::MissingEntity(format!("vertex {vertex}")))?.position;
    let coedges: Vec<CoedgeId> = body.face_coedges(face);
    for &cid in &coedges {
        let Some((a, b)) = body.coedge_endpoints(cid) else { continue };
        let Some(coedge) = body.coedges.get(cid) else { continue };
        let Some(pcurve) = coedge.pcurve.and_then(|id| body.curves2.get(id)) else { continue };
        let (t_start, t_end) = if coedge.forward { coedge.prange } else { (coedge.prange.1, coedge.prange.0) };
        let (pa, pb) = (pcurve.eval(t_start), pcurve.eval(t_end));
        if chart.key(a, pa) == key || chart.key(b, pb) == key {
            return Ok(());
        }
    }
    if chart.surface.is_degenerate_uv(uv.x, uv.y) {
        for &cid in &coedges {
            let Some(coedge) = body.coedges.get(cid).cloned() else { continue };
            let Some(edge) = body.edges.get(coedge.edge).cloned() else { continue };
            let Some(pcurve) = coedge.pcurve.and_then(|id| body.curves2.get(id)).cloned() else { continue };
            if edge.v0 != vertex || edge.v1 != vertex || !body.vertices.get(edge.v0).is_some_and(|v| v.position.distance(position) <= linear) {
                continue;
            }
            let (u_a, u_b) = (pcurve.eval(coedge.prange.0).x, pcurve.eval(coedge.prange.1).x);
            let (low, high) = (u_a.min(u_b), u_a.max(u_b));
            let Some(period) = chart.u_period else { continue };
            for k in -2i32..=2 {
                let u = uv.x + f64::from(k) * period;
                if u > low + 1e-9 && u < high - 1e-9 {
                    let s = (u - u_a) / (u_b - u_a);
                    let t = edge.range.0 + (edge.range.1 - edge.range.0) * s;
                    split_edge_with_vertex(body, coedge.edge, t, vertex, rec);
                    return Ok(());
                }
            }
        }
        return Ok(());
    }
    let mut best: Option<(EdgeId, f64, f64)> = None;
    for &cid in &coedges {
        let Some(coedge) = body.coedges.get(cid) else { continue };
        let Some(edge) = body.edges.get(coedge.edge) else { continue };
        let Some(curve) = body.curves3.get(edge.curve) else { continue };
        if edge.v0 == edge.v1 && curve.eval(edge.range.0).distance(curve.eval(0.5 * (edge.range.0 + edge.range.1))) <= linear {
            continue;
        }
        let hit = closest_parameter(curve, edge.range, position, linear);
        let span = edge.range.1 - edge.range.0;
        let inside = hit.t > edge.range.0 + 1e-12 * span.abs().max(1.0) && hit.t < edge.range.1 - 1e-12 * span.abs().max(1.0);
        if inside && hit.distance <= linear * 10.0 && best.is_none_or(|(_, _, d)| hit.distance < d) {
            best = Some((coedge.edge, hit.t, hit.distance));
        }
    }
    if let Some((edge, t, _)) = best {
        split_edge_with_vertex(body, edge, t, vertex, rec);
    }
    Ok(())
}

/// 🧭️ Replaces `face` by the faces its boundary rings and the imprint `pieces` queued on it partition it into; the first is `face`
/// itself. A face without pieces is returned untouched.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn split_face(body: &mut Body, face: FaceId, pieces: &[Pending], tol: f64, rec: &mut OpRecorder) -> Result<Vec<FaceId>, KernelError> {
    if pieces.is_empty() {
        return Ok(vec![face]);
    }
    let face_data = body.faces.get(face).ok_or_else(|| KernelError::MissingEntity(format!("face {face}")))?.clone();
    let surface = body.surfaces.get(face_data.surface).ok_or_else(|| KernelError::MissingEntity(format!("surface {}", face_data.surface)))?.clone();
    let chart = Chart::new(surface.clone());
    ensure_boundary_pcurves(body, face, &surface, tol)?;
    for piece in pieces {
        let edge = body.edges.get(piece.edge_id).ok_or_else(|| KernelError::MissingEntity(format!("edge {}", piece.edge_id)))?.clone();
        let pcurve = body.curves2.get(piece.pcurve_id).ok_or_else(|| KernelError::MissingEntity(format!("curve2 {}", piece.pcurve_id)))?.clone();
        attach_to_boundary(body, face, &chart, edge.v0, pcurve.eval(piece.prange.0), tol, rec)?;
        attach_to_boundary(body, face, &chart, edge.v1, pcurve.eval(piece.prange.1), tol, rec)?;
    }
    let mut graph = Graph { chart, pcurves: HashMap::new(), nodes: HashMap::new(), halves: Vec::new(), outgoing: Vec::new() };
    let mut boundary: Vec<usize> = Vec::new();
    for loop_id in face_data.outer.into_iter().chain(face_data.inners.iter().copied()) {
        for cid in body.loop_coedges(loop_id) {
            let coedge = body.coedges.get(cid).ok_or_else(|| KernelError::MissingEntity(format!("coedge {cid}")))?.clone();
            let pcurve = coedge.pcurve.ok_or_else(|| KernelError::Operation(format!("coedge {cid} has no p-curve")))?;
            graph.pcurves.insert(pcurve, body.curves2.get(pcurve).ok_or_else(|| KernelError::MissingEntity(format!("curve2 {pcurve}")))?.clone());
            boundary.push(graph.add_half(body, coedge.edge, coedge.forward, pcurve, coedge.prange)?);
        }
    }
    for (a, &x) in boundary.iter().enumerate() {
        for &y in &boundary[a + 1..] {
            if graph.halves[x].edge == graph.halves[y].edge && graph.halves[x].forward != graph.halves[y].forward && graph.halves[x].twin.is_none() && graph.halves[y].twin.is_none() {
                graph.halves[x].twin = Some(y);
                graph.halves[y].twin = Some(x);
                graph.halves[x].seam = true;
                graph.halves[y].seam = true;
            }
        }
    }
    for piece in pieces {
        graph.pcurves.insert(piece.pcurve_id, body.curves2.get(piece.pcurve_id).ok_or_else(|| KernelError::MissingEntity(format!("curve2 {}", piece.pcurve_id)))?.clone());
        let forward = graph.add_half(body, piece.edge_id, true, piece.pcurve_id, piece.prange)?;
        let backward = graph.add_half(body, piece.edge_id, false, piece.pcurve_id, piece.prange)?;
        graph.halves[forward].twin = Some(backward);
        graph.halves[backward].twin = Some(forward);
    }
    let cycles = graph.merge_across_seams(graph.trace()?);
    let mut outers: Vec<Ring> = Vec::new();
    let mut holes: Vec<Ring> = Vec::new();
    for cycle in cycles {
        let ring = graph.ring(body, cycle)?;
        if ring.area > 0.0 { outers.push(ring) } else { holes.push(ring) }
    }
    if outers.is_empty() {
        return Err(KernelError::Boolean(BooleanError::ImprintFailed(format!("face {face} has no bounded region after imprinting"))));
    }
    let mut owned: Vec<Vec<Ring>> = outers.iter().map(|_| Vec::new()).collect();
    for hole in holes {
        let probe = hole.polygon[0];
        let owner = (0..outers.len()).filter(|&i| !outers[i].halves.iter().any(|&h| graph.halves[h].twin.is_some_and(|t| hole.halves.contains(&t))) && graph.contains(&outers[i], probe)).min_by(|&a, &b| outers[a].area.partial_cmp(&outers[b].area).unwrap_or(std::cmp::Ordering::Equal));
        match owner {
            Some(i) => owned[i].push(hole),
            None => return Err(KernelError::Boolean(BooleanError::ImprintFailed("an inner boundary lies in no region of its face".into()))),
        }
    }
    let old: Vec<LoopId> = face_data.outer.into_iter().chain(face_data.inners.iter().copied()).collect();
    let mut shifted: HashMap<(Curve2Id, i64, i64), Curve2Id> = HashMap::new();
    let mut build_loop = |body: &mut Body, graph: &Graph, ring: &Ring, owner: FaceId| -> LoopId {
        let members: Vec<(EdgeId, bool)> = ring.halves.iter().map(|&h| (graph.halves[h].edge, graph.halves[h].forward)).collect();
        let loop_id = make_loop(body, owner, &members);
        for ((cid, &h), &shift) in body.loop_coedges(loop_id).into_iter().zip(&ring.halves).zip(&ring.shifts) {
            let half = &graph.halves[h];
            let pcurve = if shift == (0.0, 0.0) {
                half.pcurve
            } else {
                let key = (half.pcurve, (shift.0 * 1e9).round() as i64, (shift.1 * 1e9).round() as i64);
                *shifted.entry(key).or_insert_with(|| body.curves2.insert(graph.pcurves[&half.pcurve].translated(Vec2::new(shift.0, shift.1))))
            };
            if let Some(coedge) = body.coedges.get_mut(cid) {
                coedge.pcurve = Some(pcurve);
                coedge.prange = half.prange;
            }
        }
        loop_id
    };
    let mut specs: Vec<(Ring, Vec<Ring>)> = outers.into_iter().zip(owned).collect();
    specs.sort_by(|a, b| b.0.area.partial_cmp(&a.0.area).unwrap_or(std::cmp::Ordering::Equal));
    let mut loops: Vec<(LoopId, Vec<LoopId>)> = Vec::new();
    for (outer, inners) in &specs {
        let outer_loop = build_loop(body, &graph, outer, face);
        let inner_loops = inners.iter().map(|ring| build_loop(body, &graph, ring, face)).collect();
        loops.push((outer_loop, inner_loops));
    }
    for loop_id in old {
        for cid in body.loop_coedges(loop_id) {
            body.coedges.remove(cid);
        }
        body.loops.remove(loop_id);
    }
    let mut faces = Vec::new();
    for (index, (outer_loop, inner_loops)) in loops.into_iter().enumerate() {
        if index == 0 {
            let first = body.faces.get_mut(face).ok_or_else(|| KernelError::MissingEntity(format!("face {face}")))?;
            first.outer = Some(outer_loop);
            first.inners = inner_loops;
            faces.push(face);
            continue;
        }
        let created = add_face(body, face_data.surface, Some(outer_loop), inner_loops.clone(), face_data.flipped, face_data.tol, rec);
        for loop_id in std::iter::once(outer_loop).chain(inner_loops) {
            if let Some(entry) = body.loops.get_mut(loop_id) {
                entry.face = created;
            }
        }
        for (_, shell) in body.shells.iter_mut() {
            if shell.faces.contains(&face) && !shell.faces.contains(&created) {
                shell.faces.push(created);
            }
        }
        faces.push(created);
    }
    let _ = PI;
    Ok(faces)
}

/// 🧭️ Gives every boundary coedge of `face` a p-curve, fitting the ones a hand-built ring left without.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn ensure_boundary_pcurves(body: &mut Body, face: FaceId, surface: &Surface, tol: f64) -> Result<(), KernelError> {
    for cid in body.face_coedges(face) {
        let Some(coedge) = body.coedges.get(cid).cloned() else { continue };
        if coedge.pcurve.is_some() {
            continue;
        }
        let edge = body.edges.get(coedge.edge).ok_or_else(|| KernelError::MissingEntity(format!("edge {}", coedge.edge)))?.clone();
        let curve = body.curves3.get(edge.curve).ok_or_else(|| KernelError::MissingEntity(format!("curve {}", edge.curve)))?.clone();
        let (pcurve, prange) = fitted_pcurve(surface, &curve, edge.range, edge.tol.value().min(tol) * 0.5);
        let id = body.curves2.insert(pcurve);
        if let Some(entry) = body.coedges.get_mut(cid) {
            entry.pcurve = Some(id);
            entry.prange = prange;
        }
    }
    Ok(())
}
