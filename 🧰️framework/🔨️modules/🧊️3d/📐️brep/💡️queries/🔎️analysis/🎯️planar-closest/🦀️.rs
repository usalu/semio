//! 🎯️ Exact closest points on authored planar regions and full-longitude spherical charts.

use super::linear_distance::{original_linear_distance, plane_probe};
use super::planar_volume::SphericalBand;
use semio_framework_3d_brep_curve::Curve3;
use semio_framework_3d_brep_representation::arena::{CoedgeId, FaceId, LoopId, VertexId};
use semio_framework_3d_brep_representation::topology::{Body, Face};
use semio_framework_3d_brep_surface::Surface;
use semio_framework_3d_brep_vector::{Pnt2, Pnt3, Vec3, matrix::Frame3, predicates::ray_event};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
use std::mem::size_of;

#[path = "🪢️arc/🦀️.rs"]
mod arc;
pub(crate) use arc::area_correction;

struct Plane {
    frame: Frame3,
    u: Vec3,
    v: Vec3,
    sx: f64,
    sy: f64,
    sine: f64,
    normal: Vec3,
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
        Ok(Self { frame, u: y.cross(normal), v: normal.cross(x), sx, sy, sine, normal })
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
    has_arc: bool,
    inside: bool,
}

struct Foot {
    plane: Plane,
    uv: Pnt2,
    point: Pnt3,
    linear_distance: Option<f64>,
}

enum Target {
    Point(Pnt3),
    VertexDistance(Pnt3),
    Linear(Curve3, Option<(f64, f64)>),
}

/// 🎁️ The original point or scalar separation produced by the face traversal.
pub enum PlanarClosestOutput {
    Point(Pnt3),
    Distance(f64),
}

#[path = "🧺️solid/🦀️.rs"]
mod solid;
pub use solid::SolidClosest;

/// 🔬️ Scans original planar or spherical face rings in authored order.
pub struct PlanarClosest {
    source: FaceId,
    target: Target,
    foot: Option<Foot>,
    sphere: Option<SphericalBand>,
    ring: Option<Ring>,
    boundary: usize,
    inside: bool,
    on_boundary: bool,
    best: Option<(Pnt3, f64)>,
    boundary_distance: f64,
}

impl PlanarClosest {
    /// 🧭️ Opens a cursor over one original face.
    pub fn new(source: FaceId, target: Pnt3) -> Self { Self::open(source, Target::Point(target)) }

    /// 📍️ Opens a scalar distance measurement from an original vertex.
    pub fn from_vertex(source: FaceId, target: Pnt3) -> Self { Self::open(source, Target::VertexDistance(target)) }

    /// 📏️ Binds an original line and its retained edge interval before walking the face.
    pub fn from_line(source: FaceId, (curve, trim): (&Curve3, Option<(f64, f64)>)) -> Result<Self, ValueError> {
        let Curve3::Line { origin, dir } = curve else { return Err(unsupported_linear()); };
        Ok(Self::open(source, Target::Linear(Curve3::Line { origin: *origin, dir: *dir }, trim)))
    }

    fn open(source: FaceId, target: Target) -> Self {
        Self { source, target, foot: None, sphere: None, ring: None, boundary: 0, inside: false, on_boundary: false, best: None, boundary_distance: f64::INFINITY }
    }

    /// 🪪️ The face that owns the original loops.
    pub fn source(&self) -> FaceId {
        self.source
    }

    /// 📏️ Quotes one support, loop or coedge read.
    pub fn next_demand(&self, body: &Body) -> Result<RetirementDemand, ValueError> {
        let face = body.faces.get(self.source).ok_or_else(invalid)?;
        let mut depth = body.faces.next_remove_depth_demand(self.source)?;
        let mut copy_bytes = 0;
        if self.sphere.is_some() {
            depth = depth.max(body.surfaces.next_remove_depth_demand(face.surface)?);
        }
        if self.foot.is_none() && self.sphere.is_none() {
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
            if let Some(trim) = coedge.pcurve.filter(|_| self.sphere.is_some() || !matches!(body.curves3.get(edge.curve), Some(Curve3::Line { .. }))) {
                depth = depth.max(body.curves2.next_remove_depth_demand(trim)?);
            }
            copy_bytes = 4 * size_of::<Pnt3>();
        } else if let Some(id) = boundary(face, self.boundary) {
            depth = depth.max(body.loops.next_remove_depth_demand(id)?);
        } else {
            copy_bytes = size_of::<Pnt3>();
        }
        Ok(RetirementDemand { depth, copy_bytes, ..Default::default() })
    }

    /// 🦶️ Advances a point-only projection using the original face.
    pub fn step(&mut self, body: &Body) -> Result<Option<Pnt3>, ValueError> {
        match self.advance(body)? {
            Some(PlanarClosestOutput::Point(point)) => Ok(Some(point)),
            Some(PlanarClosestOutput::Distance(_)) => Err(ValueError::invariant("point projection received a scalar")),
            None => Ok(None),
        }
    }

    /// ⏱️ Reads one support or boundary row, or returns the closest point.
    pub fn advance(&mut self, body: &Body) -> Result<Option<PlanarClosestOutput>, ValueError> {
        let face = body.faces.get(self.source).ok_or_else(invalid)?;
        if self.foot.is_none() && self.sphere.is_none() {
            match body.surfaces.get(face.surface).ok_or_else(invalid)? {
                Surface::Plane { frame } => self.foot = Some(self.project_foot(Plane::new(*frame)?)?),
                Surface::Sphere { .. } if matches!(self.target, Target::Linear(..)) => return Err(unsupported_linear()),
                Surface::Sphere { .. } if face.inners.is_empty() => self.sphere = Some(SphericalBand::default()),
                _ => return Err(unsupported()),
            }
            if face.outer.is_none() { return Err(invalid()); }
            return Ok(None);
        }
        let Some(ring) = &mut self.ring else {
            let Some(id) = boundary(face, self.boundary) else {
                return if let Some(sphere) = &self.sphere {
                    let surface = body.surfaces.get(face.surface).ok_or_else(invalid)?;
                    let (Target::Point(target) | Target::VertexDistance(target)) = self.target else { return Err(unsupported_linear()); };
                    self.output_point(sphere.project(surface, target)?).map(Some)
                } else {
                    self.output(self.foot.as_ref().ok_or_else(invalid)?).map(Some)
                };
            };
            let first = body.loops.get(id).filter(|ring| ring.face == self.source).ok_or_else(invalid)?.first;
            self.ring = Some(Ring { id, first, current: first, endpoints: None, count: 0, inside: false, has_arc: false });
            return Ok(None);
        };
        let coedge = body.coedges.get(ring.current).filter(|coedge| coedge.loop_id == ring.id).ok_or_else(invalid)?;
        let edge = body.edges.get(coedge.edge).ok_or_else(invalid)?;
        let curve = body.curves3.get(edge.curve).ok_or_else(invalid)?;
        let (start, end) = if coedge.forward { (edge.v0, edge.v1) } else { (edge.v1, edge.v0) };
        let first_vertex = match ring.endpoints {
            Some((first, previous)) if previous == start => first,
            None => start,
            _ => return Err(invalid()),
        };
        if let Some(sphere) = &mut self.sphere {
            let surface = body.surfaces.get(face.surface).ok_or_else(invalid)?;
            let closed = coedge.next == ring.first;
            if (ring.count == 3) != closed || (closed && end != first_vertex) { return Err(invalid()); }
            sphere.row(ring.count, body, surface, coedge, edge, curve)?;
            if closed {
                self.ring = None;
                self.boundary = 1;
            } else {
                ring.count += 1;
                ring.current = coedge.next;
                ring.endpoints = Some((first_vertex, end));
            }
            return Ok(None);
        }
        let foot = self.foot.as_ref().ok_or_else(invalid)?;
        let start_point = body.vertices.get(start).ok_or_else(invalid)?.position;
        let end_point = body.vertices.get(end).ok_or_else(invalid)?.position;
        let (a, b) = (foot.plane.coordinates(start_point), foot.plane.coordinates(end_point));
        if !valid_uv(a) || !valid_uv(b) || a == b { return Err(invalid()); }
        let (crosses, touches) = if let Target::Linear(line, range) = &self.target {
            let vertices = if coedge.forward { [start_point, end_point] } else { [end_point, start_point] };
            validate_linear_edge(curve, edge.range, vertices, &foot.plane)?;
            self.boundary_distance = self.boundary_distance.min(original_linear_distance((line, *range), (curve, Some(edge.range)))?);
            ray_event(a, b, foot.uv)
        } else {
            let (crosses, touches, candidate) = if matches!(curve, Curve3::Line { .. }) {
                let (crosses, touches) = ray_event(a, b, foot.uv);
                let candidate = nearest_segment(curve.eval(edge.range.0), curve.eval(edge.range.1), foot.point)?;
                (crosses, touches, candidate)
            } else {
                if coedge.prange != edge.range { return Err(unsupported()); }
                let trim = body.curves2.get(coedge.pcurve.ok_or_else(unsupported)?).ok_or_else(invalid)?;
                let vertices = if coedge.forward { [start_point, end_point] } else { [end_point, start_point] };
                let event = arc::project(curve, trim, edge.range, vertices, foot)?;
                ring.has_arc = true;
                event
            };
            let distance = length(candidate - foot.point);
            if !distance.is_finite() { return Err(invalid()); }
            if self.best.is_none_or(|(_, best)| distance < best) { self.best = Some((candidate, distance)); }
            (crosses, touches)
        };
        self.on_boundary |= touches;
        ring.inside ^= crosses;
        ring.count += 1;
        if coedge.next == ring.first {
            if ring.count < 2 || (ring.count == 2 && !ring.has_arc) || end != first_vertex { return Err(invalid()); }
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

    fn project_foot(&self, plane: Plane) -> Result<Foot, ValueError> {
        let frame = plane.frame;
        let (probe, mut linear_distance) = match &self.target {
            Target::Point(point) | Target::VertexDistance(point) => (*point, None),
            Target::Linear(line, range) => match plane_probe((line, *range), frame.origin, plane.normal)? {
                Some((probe, distance)) => (probe, Some(distance)),
                None => (frame.origin, None),
            },
        };
        let mut uv = plane.coordinates(probe);
        if linear_distance.is_some() && [uv.x, uv.y].into_iter().any(|value| !value.is_finite() || value.abs() > 1e140) {
            linear_distance = None;
            uv = Pnt2::new(0.0, 0.0);
        }
        let point = frame.origin + frame.x * uv.x + frame.y * uv.y;
        if !valid_uv(uv) || !finite(point) { return Err(invalid()); }
        Ok(Foot { plane, uv, point, linear_distance })
    }

    fn output(&self, foot: &Foot) -> Result<PlanarClosestOutput, ValueError> {
        let contains = self.on_boundary || self.inside;
        if matches!(&self.target, Target::Linear(..)) {
            let interior = if contains { foot.linear_distance.unwrap_or(f64::INFINITY) } else { f64::INFINITY };
            let distance = self.boundary_distance.min(interior);
            return if distance.is_finite() { Ok(PlanarClosestOutput::Distance(distance)) } else { Err(invalid()) };
        }
        let point = if contains { foot.point } else { self.best.ok_or_else(invalid)?.0 };
        self.output_point(point)
    }

    fn output_point(&self, point: Pnt3) -> Result<PlanarClosestOutput, ValueError> {
        if let Target::VertexDistance(target) = &self.target {
            let distance = length(point - *target);
            return if distance.is_finite() { Ok(PlanarClosestOutput::Distance(distance)) } else { Err(invalid()) };
        }
        Ok(PlanarClosestOutput::Point(point))
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
    ValueError::literal(ValueRefusalKind::UnsupportedOwner, "closest point requires supported original planar trims or a spherical pole-to-rim chart")
}


fn validate_linear_edge(curve: &Curve3, (lo, hi): (f64, f64), vertices: [Pnt3; 2], plane: &Plane) -> Result<(), ValueError> {
    let Curve3::Line { origin, dir } = curve else { return Err(unsupported_linear()); };
    for (vertex, parameter) in vertices.into_iter().zip([lo, hi]) {
        let origin = origin.to_array();
        let direction = dir.to_array();
        let point = Pnt3::from_array(std::array::from_fn(|i| direction[i].mul_add(parameter, origin[i])));
        let scale = point.to_array().into_iter().chain(vertex.to_array()).chain(plane.frame.origin.to_array())
            .map(f64::abs).fold(0.0, f64::max);
        let allowance = 128.0 * f64::EPSILON * scale;
        if !finite(point) || length(point - vertex) > allowance
            || !((vertex - plane.frame.origin).dot(plane.normal).abs() <= allowance)
        { return Err(invalid()); }
    }
    Ok(())
}

fn unsupported_linear() -> ValueError {
    ValueError::literal(ValueRefusalKind::UnsupportedOwner, "linear face distance requires original linear operands and line-bounded planar trims")
}
