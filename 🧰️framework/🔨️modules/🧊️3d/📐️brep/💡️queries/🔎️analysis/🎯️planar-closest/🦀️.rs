//! 🎯️ Exact closest points on original planar regions bounded by simple line rings.

use semio_framework_3d_brep_curve::Curve3;
use semio_framework_3d_brep_representation::arena::{CoedgeId, FaceId, LoopId, VertexId};
use semio_framework_3d_brep_representation::topology::{Body, Face};
use semio_framework_3d_brep_surface::Surface;
use semio_framework_3d_brep_vector::{Pnt2, Pnt3, Vec3, matrix::Frame3, predicates::{Orient, orient2d}};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
use std::mem::size_of;

struct Plane {
    frame: Frame3,
    u: Vec3,
    v: Vec3,
    sx: f64,
    sy: f64,
    sine: f64,
}

fn length(vector: Vec3) -> f64 {
    vector.x.hypot(vector.y).hypot(vector.z)
}

fn unit(vector: Vec3, norm: f64) -> Result<Vec3, ValueError> {
    if norm == 0.0 || !norm.is_finite() { return Err(invalid()); }
    Ok(Vec3::new(vector.x / norm, vector.y / norm, vector.z / norm))
}

impl Plane {
    fn new(frame: Frame3) -> Result<Self, ValueError> {
        let sx = length(frame.x);
        let sy = length(frame.y);
        let x = unit(frame.x, sx)?;
        let y = unit(frame.y, sy)?;
        let cross = x.cross(y);
        let sine = length(cross);
        let normal = unit(cross, sine)?;
        Ok(Self { frame, u: y.cross(normal), v: normal.cross(x), sx, sy, sine })
    }

    fn coordinates(&self, point: Pnt3) -> Pnt2 {
        let delta = point - self.frame.origin;
        Pnt2::new((delta.dot(self.u) / self.sine) / self.sx, (delta.dot(self.v) / self.sine) / self.sy)
    }
}

/// 🧮️ Returns the orthogonal plane foot's coordinates in a possibly skew basis.
pub fn planar_dual_coordinates(frame: Frame3, point: Pnt3) -> Result<[f64; 2], ValueError> {
    let Pnt2 { x, y } = Plane::new(frame)?.coordinates(point);
    if !x.is_finite() || !y.is_finite() { return Err(invalid()); }
    Ok([x, y])
}

/// 📏️ Quotes the bounded face rows consulted when choosing an existing analytic or planar cursor.
pub fn closest_face_depth(body: &Body, id: FaceId) -> Result<usize, ValueError> {
    let face = body.faces.get(id).ok_or_else(|| ValueError::invariant("original face is missing"))?;
    let mut depth = body.faces.next_remove_depth_demand(id)?;
    if !face.inners.is_empty() { return Ok(depth); }
    depth = depth.max(body.surfaces.next_remove_depth_demand(face.surface)?);
    let Some(id) = face.outer else { return Ok(depth); };
    depth = depth.max(body.loops.next_remove_depth_demand(id)?);
    let Some(ring) = body.loops.get(id) else { return Ok(depth); };
    depth = depth.max(body.coedges.next_remove_depth_demand(ring.first)?);
    let Some(coedge) = body.coedges.get(ring.first) else { return Ok(depth); };
    if let Some(trim) = coedge.pcurve {
        depth = depth.max(body.curves2.next_remove_depth_demand(trim)?)
            .max(body.edges.next_remove_depth_demand(coedge.edge)?);
        if let Some(edge) = body.edges.get(coedge.edge) { depth = depth.max(body.curves3.next_remove_depth_demand(edge.curve)?); }
    }
    Ok(depth)
}

struct Ring {
    id: LoopId,
    first: CoedgeId,
    current: CoedgeId,
    endpoints: Option<(VertexId, VertexId)>,
    count: usize,
    inside: bool,
}

struct Foot {
    plane: Plane,
    uv: Pnt2,
    point: Pnt3,
}

/// 🔬️ Scans validated rings in authored order, retaining a plane foot and one boundary candidate.
pub struct PlanarClosest {
    source: FaceId,
    target: Pnt3,
    foot: Option<Foot>,
    ring: Option<Ring>,
    boundary: usize,
    inside: bool,
    on_boundary: bool,
    best: Option<(Pnt3, f64)>,
}

impl PlanarClosest {
    /// 🧭️ Opens a cursor over one original planar face.
    pub fn new(source: FaceId, target: Pnt3) -> Self {
        Self { source, target, foot: None, ring: None, boundary: 0, inside: false, on_boundary: false, best: None }
    }

    /// 🪪️ The face that owns the original loops.
    pub fn source(&self) -> FaceId {
        self.source
    }

    /// 📏️ Quotes one plane, loop or coedge read.
    pub fn next_demand(&self, body: &Body) -> Result<RetirementDemand, ValueError> {
        let face = body.faces.get(self.source).ok_or_else(invalid)?;
        let mut depth = body.faces.next_remove_depth_demand(self.source)?;
        let mut copy_bytes = 0;
        if self.foot.is_none() {
            depth = depth.max(body.surfaces.next_remove_depth_demand(face.surface)?);
            copy_bytes = size_of::<Frame3>();
        } else if let Some(ring) = &self.ring {
            let coedge = body.coedges.get(ring.current).ok_or_else(invalid)?;
            let edge = body.edges.get(coedge.edge).ok_or_else(invalid)?;
            depth = depth.max(body.coedges.next_remove_depth_demand(ring.current)?)
                .max(body.edges.next_remove_depth_demand(coedge.edge)?)
                .max(body.curves3.next_remove_depth_demand(edge.curve)?)
                .max(body.vertices.next_remove_depth_demand(edge.v0)?)
                .max(body.vertices.next_remove_depth_demand(edge.v1)?);
            copy_bytes = 4 * size_of::<Pnt3>();
        } else if let Some(id) = boundary(face, self.boundary) {
            depth = depth.max(body.loops.next_remove_depth_demand(id)?);
        } else {
            copy_bytes = size_of::<Pnt3>();
        }
        Ok(RetirementDemand { depth, copy_bytes, ..Default::default() })
    }

    /// ⏱️ Projects to the plane, opens one ring, reads one coedge, or returns the closest point.
    pub fn step(&mut self, body: &Body) -> Result<Option<Pnt3>, ValueError> {
        let face = body.faces.get(self.source).ok_or_else(invalid)?;
        let Some(foot) = &self.foot else {
            let Surface::Plane { frame } = body.surfaces.get(face.surface).ok_or_else(invalid)? else {
                return Err(unsupported());
            };
            let plane = Plane::new(*frame)?;
            let uv = plane.coordinates(self.target);
            let point = frame.origin + frame.x * uv.x + frame.y * uv.y;
            if !valid_uv(uv) || !finite(point) || face.outer.is_none() { return Err(invalid()); }
            self.foot = Some(Foot { plane, uv, point });
            return Ok(None);
        };
        let Some(ring) = &mut self.ring else {
            let Some(id) = boundary(face, self.boundary) else {
                return if self.on_boundary || self.inside { Ok(Some(foot.point)) }
                    else { self.best.map(|(point, _)| Some(point)).ok_or_else(invalid) };
            };
            let first = body.loops.get(id).filter(|ring| ring.face == self.source).ok_or_else(invalid)?.first;
            self.ring = Some(Ring { id, first, current: first, endpoints: None, count: 0, inside: false });
            return Ok(None);
        };
        let coedge = body.coedges.get(ring.current).filter(|coedge| coedge.loop_id == ring.id).ok_or_else(invalid)?;
        let edge = body.edges.get(coedge.edge).ok_or_else(invalid)?;
        let curve = body.curves3.get(edge.curve).ok_or_else(invalid)?;
        if !matches!(curve, Curve3::Line { .. }) { return Err(unsupported()); }
        let (start, end) = if coedge.forward { (edge.v0, edge.v1) } else { (edge.v1, edge.v0) };
        let first_vertex = match ring.endpoints {
            Some((first, previous)) if previous == start => first,
            None => start,
            _ => return Err(invalid()),
        };
        let a = foot.plane.coordinates(body.vertices.get(start).ok_or_else(invalid)?.position);
        let b = foot.plane.coordinates(body.vertices.get(end).ok_or_else(invalid)?.position);
        if !valid_uv(a) || !valid_uv(b) || a == b { return Err(invalid()); }
        let (crosses, touches) = ray_event(a, b, foot.uv);
        let candidate = nearest_segment(curve.eval(edge.range.0), curve.eval(edge.range.1), foot.point)?;
        let distance = length(candidate - foot.point);
        if !distance.is_finite() { return Err(invalid()); }
        if self.best.is_none_or(|(_, best)| distance < best) { self.best = Some((candidate, distance)); }
        self.on_boundary |= touches;
        ring.inside ^= crosses;
        ring.count += 1;
        if coedge.next == ring.first {
            if ring.count < 3 || end != first_vertex { return Err(invalid()); }
            self.inside = if self.boundary == 0 { ring.inside } else { self.inside && !ring.inside };
            self.boundary += 1;
            self.ring = None;
        } else {
            if ring.count >= body.coedges.slot_count() { return Err(invalid()); }
            ring.current = coedge.next;
            ring.endpoints = Some((first_vertex, end));
        }
        Ok(None)
    }
}

fn boundary(face: &Face, index: usize) -> Option<LoopId> {
    if index == 0 { face.outer } else { face.inners.get(index - 1).copied() }
}

fn finite(point: Pnt3) -> bool {
    [point.x, point.y, point.z].into_iter().all(f64::is_finite)
}

/// 🎚️ Bounds coordinates to keep orient2d expansion products within normal f64 range.
fn valid_uv(point: Pnt2) -> bool {
    [point.x, point.y].into_iter().all(|value| value == 0.0 || (1e-120..=1e140).contains(&value.abs()))
}

fn ray_event(a: Pnt2, b: Pnt2, target: Pnt2) -> (bool, bool) {
    let by_y = (a.y > target.y) != (b.y > target.y);
    let between_x = (a.x.min(b.x)..=a.x.max(b.x)).contains(&target.x);
    let between_y = (a.y.min(b.y)..=a.y.max(b.y)).contains(&target.y);
    if !between_x { return (by_y && target.x < a.x.min(b.x), false); }
    if !between_y { return (false, false); }
    match orient2d(a, b, target) {
        Orient::Positive => (by_y && a.y < b.y, false),
        Orient::Negative => (by_y && a.y > b.y, false),
        Orient::Zero => (false, true),
    }
}

fn nearest_segment(a: Pnt3, b: Pnt3, target: Pnt3) -> Result<Pnt3, ValueError> {
    let direction = b - a;
    let norm = length(direction);
    let along = (target - a).dot(unit(direction, norm)?);
    let point = if along <= 0.0 { a } else if along >= norm { b } else { a + direction * (along / norm) };
    if !along.is_finite() || !finite(point) { return Err(invalid()); }
    Ok(point)
}

fn invalid() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, "planar closest point requires finite bounded coordinates and closed nondegenerate rings")
}

fn unsupported() -> ValueError {
    ValueError::literal(ValueRefusalKind::UnsupportedOwner, "closest point requires a planar face with original line trims")
}
