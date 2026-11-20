//! 📐️ Original planar face topology from one certified closed wire.

use super::*;
use semio_framework_3d_brep_representation::arena::{CoedgeId, LoopId};
use semio_framework_3d_brep_representation::topology::{Coedge, Face, Loop, dummy_coedge};
use semio_framework_3d_brep_curve::Curve2;
use semio_framework_3d_brep_spline_trim::QuadraticPcurve;
use semio_framework_3d_brep_vector::{Pnt2, Vec2};
use semio_framework_value::{RetainedAllocationEffect, RetainedCloneGrant as Grant, RetainedCloneProgress as Progress, RetainedCloneStep as Step, RetirementDemand, ValueError, ValueRefusalKind};
use semio_framework_value::retained_clone::retained_clone_grant_admits;
use solid_publication::{remove_topology_slot, store_demand, store_insert, topology_slot_depth};
use std::mem::{size_of, take};

#[path = "🔁️mapped-boundary/🦀️.rs"]
mod mapped_boundary;
use mapped_boundary::MappedBoundary;

#[derive(Clone, Copy)]
enum Build {
    MappedBoundary,
    Normal(usize),
    Plane(usize),
    Crossing { first: usize, second: usize },
    SplineParameters,
    SplineControl(usize),
    SplineCertify,
    SplinePcurve,
    Surface,
    Face,
    Ring,
    Trim(usize),
    Use(usize),
    Lead(usize),
    CloseLead,
    FinishRing,
    FinishFace,
    ReleaseJournal,
    Ready,
}

/// 🛠️ Owns unpublished slots while validating and filling one original polygonal boundary.
pub struct PolygonFacePublication {
    owner: usize,
    revision: u64,
    wire_label: Option<PersistentLabel>,
    boundary: Option<PolygonWirePublication>,
    mapped: Option<MappedBoundary>,
    edges: usize,
    xy_only: bool,
    spline_index: Option<usize>,
    spline: Option<QuadraticPcurve>,
    pcurve: Option<Curve2>,
    origin: Option<Pnt3>,
    normal: NativeVec3,
    frame: Option<Frame3>,
    shape: PersistentLabel,
    handle: [u8; 64],
    journal: Vec<EntityRef>,
    face: Option<FaceId>,
    ring: Option<LoopId>,
    first_use: Option<CoedgeId>,
    last_use: Option<CoedgeId>,
    build: Build,
    open: bool,
}

impl PolygonFacePublication {
    pub fn handle(&self) -> &str { std::str::from_utf8(&self.handle).expect("digest uses lowercase hexadecimal") }
    pub fn topology_ready(&self) -> bool { matches!(self.build, Build::Ready) }
    pub fn terminal_is_empty(&self) -> bool { !self.open && self.boundary.is_none() }

    /// 📍️ Identifies the next borrowed point while constructing a private boundary.
    pub fn next_point_index(&self) -> Option<usize> {
        if self.mapped.is_some() { None } else { self.boundary.as_ref()?.next_point_index() }
    }

    fn check_owner(&self, kernel: &BrepRegistry) -> Result<(), ValueError> {
        if self.open && self.owner == std::ptr::from_ref(kernel) as usize { Ok(()) }
        else { Err(ValueError::invariant("polygon face requires its original unpublished session")) }
    }

    fn journal_capacity(&self) -> usize { 3 + 2 * self.edges }

    fn source<'a>(&'a self, kernel: &'a BrepRegistry, source: Option<&str>) -> Result<&'a Wire, ValueError> {
        if kernel.shape_mutation_revision != self.revision {
            return Err(ValueError::invariant("polygon face source changed during construction"));
        }
        if let Some(boundary) = &self.boundary {
            if source.is_some() && self.mapped.is_none() { return Err(ValueError::invariant("private polygon face takes only its drafted boundary")); }
            return boundary.face_boundary(kernel);
        }
        let source = source.ok_or_else(invalid)?;
        let Entity::Wire(wire, label) = kernel.original_query_entity(source)? else { return Err(invalid()); };
        if Some(*label) != self.wire_label || wire.members.len() != self.edges || !wire.closed {
            return Err(ValueError::invariant("polygon face source changed during construction"));
        }
        Ok(wire)
    }

    fn member(&self, kernel: &BrepRegistry, source: Option<&str>, index: usize) -> Result<(EdgeId, bool, Pnt3, Pnt3, (f64, f64)), ValueError> {
        let wire = self.source(kernel, source)?;
        let (edge, forward) = wire.members[index];
        let record = kernel.body.edges.get(edge).ok_or_else(invalid)?;
        let a = kernel.body.vertices.get(record.v0).ok_or_else(invalid)?.position;
        let b = kernel.body.vertices.get(record.v1).ok_or_else(invalid)?.position;
        Ok((edge, forward, a, b, record.range))
    }

    fn curve<'a>(&self, kernel: &'a BrepRegistry, edge: EdgeId) -> Result<&'a Curve3, ValueError> {
        let edge = kernel.body.edges.get(edge).ok_or_else(invalid)?;
        kernel.body.curves3.get(edge.curve).ok_or_else(invalid)
    }

    fn member_depth(&self, kernel: &BrepRegistry, source: Option<&str>, index: usize) -> Result<usize, ValueError> {
        let (id, _) = self.source(kernel, source)?.members[index];
        let edge = kernel.body.edges.get(id).ok_or_else(invalid)?;
        Ok(kernel.body.edges.next_remove_depth_demand(id)?
            .max(kernel.body.curves3.next_remove_depth_demand(edge.curve)?)
            .max(kernel.body.vertices.next_remove_depth_demand(edge.v0)?)
            .max(kernel.body.vertices.next_remove_depth_demand(edge.v1)?))
    }

    fn trim_phase(&self, index: usize) -> Build {
        if self.spline_index == Some(index) { Build::SplinePcurve } else { Build::Trim(index) }
    }

    fn frame(&self) -> Frame3 { self.frame.expect("original polygon face has a validated plane") }

    fn tolerance(&self) -> Tol { self.mapped.and_then(|boundary| boundary.plane).map_or(Tol::DEFAULT, |plane| plane.tol) }

    fn record(&mut self, entry: Option<EntityRef>, mut progress: Progress, next: Build) -> Progress {
        if let Some(entry) = entry {
            self.journal.push(entry);
            self.build = next;
            progress.copied_bytes = size_of::<EntityRef>();
        }
        progress
    }

    fn demand(&self, kernel: &BrepRegistry, source: Option<&str>) -> Result<RetirementDemand, ValueError> {
        let source_depth = source.map(|key| kernel.tessellation_source_depth_demand(key)).transpose()?.unwrap_or(1).max(1);
        if let Some(boundary) = &self.mapped {
            if self.revision != kernel.shape_mutation_revision {
                return Err(ValueError::invariant("mapped polygon face source changed during construction"));
            }
            boundary.check_source(kernel, source)?;
            if matches!(self.build, Build::MappedBoundary) {
                let mut demand = boundary.scan_demand(&kernel.body)?;
                demand.depth = demand.depth.max(source_depth);
                return Ok(demand);
            }
        }
        if let Some(boundary) = &self.boundary {
            if !boundary.topology_ready() {
                let mut demand = kernel.polygon_wire_demand(boundary, 0)?;
                if let Some(mapped) = self.mapped {
                    demand.depth = demand.depth.max(source_depth);
                    if demand.capacity_bytes == 0 && boundary.next_point_index().is_some() {
                        let point = mapped.point_demand(&kernel.body)?;
                        demand.depth = demand.depth.max(point.depth);
                        demand.copy_bytes += point.copy_bytes;
                    }
                }
                return Ok(demand);
            }
        }
        if self.journal.capacity() == 0 {
            return Ok(RetirementDemand { capacity_bytes: self.journal_capacity() * size_of::<EntityRef>(), depth: source_depth, ..Default::default() });
        }
        let body = &kernel.body;
        let mut store = match self.build {
            Build::Normal(index) => {
                let depth = source_depth.max(self.member_depth(kernel, source, index)?);
                return Ok(RetirementDemand { depth, copy_bytes: 6 * size_of::<f64>(), ..Default::default() });
            }
            Build::Plane(index) => {
                let depth = source_depth.max(self.member_depth(kernel, source, index)?)
                    .max(self.member_depth(kernel, source, (index + 1) % self.edges)?);
                return Ok(RetirementDemand { depth, copy_bytes: 12 * size_of::<f64>(), ..Default::default() });
            }
            Build::SplineParameters | Build::SplineControl(_) | Build::SplineCertify => {
                let index = self.spline_index.ok_or_else(invalid)?;
                let mut depth = source_depth.max(self.member_depth(kernel, source, index)?);
                let scalars = match self.build {
                    Build::SplineParameters => 8,
                    Build::SplineControl(_) => 4,
                    _ => {
                        depth = depth.max(self.member_depth(kernel, source, (index + 2) % 3)?);
                        9
                    }
                };
                return Ok(RetirementDemand { depth, copy_bytes: scalars * size_of::<f64>(), ..Default::default() });
            }
            Build::SplinePcurve => return Ok(RetirementDemand {
                depth: source_depth, capacity_bytes: QuadraticPcurve::STORAGE, copy_bytes: QuadraticPcurve::STORAGE, ..Default::default()
            }),
            Build::Crossing { first, second } => {
                let depth = source_depth.max(self.member_depth(kernel, source, first)?).max(self.member_depth(kernel, source, second)?);
                return Ok(RetirementDemand { depth, copy_bytes: 12 * size_of::<f64>(), ..Default::default() });
            }
            Build::Lead(_) => {
                let depth = match self.last_use {
                    Some(id) => source_depth.max(body.coedges.next_remove_depth_demand(id)?),
                    None => source_depth,
                };
                return Ok(RetirementDemand { depth, ..Default::default() });
            }
            Build::CloseLead => {
                let depth = source_depth.max(body.coedges.next_remove_depth_demand(self.first_use.unwrap())?);
                return Ok(RetirementDemand { depth, ..Default::default() });
            }
            Build::Surface => store_demand(&body.surfaces)?,
            Build::Face => store_demand(&body.faces)?,
            Build::Ring => store_demand(&body.loops)?,
            Build::Trim(_) => store_demand(&body.curves2)?,
            Build::Use(_) => store_demand(&body.coedges)?,
            Build::FinishRing => return Ok(RetirementDemand { depth: body.loops.next_remove_depth_demand(self.ring.unwrap())?, ..Default::default() }),
            Build::FinishFace => return Ok(RetirementDemand { depth: body.faces.next_remove_depth_demand(self.face.unwrap())?, ..Default::default() }),
            Build::MappedBoundary | Build::ReleaseJournal | Build::Ready => return Err(ValueError::invariant("polygon face does not have a topology step")),
        };
        store.depth = store.depth.max(source_depth);
        if store.capacity_bytes == 0 {
            store.copy_bytes = size_of::<EntityRef>();
            if let Build::Trim(index) | Build::Use(index) = self.build {
                store.depth = store.depth.max(self.member_depth(kernel, source, index)?);
                store.copy_bytes += 8 * size_of::<f64>();
            }
        }
        Ok(store)
    }
}

fn invalid() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, "face requires a simple closed coplanar boundary of supported curves")
}

fn planar_line_pcurve(curve: &Curve3, frame: Frame3) -> Result<Curve2, ValueError> {
    let Curve3::Line { origin, dir } = *curve else { return Err(invalid()); };
    let normal = frame.z.normalized().ok_or_else(invalid)?;
    let u = frame.y.cross(normal);
    let v = normal.cross(frame.x);
    let u = u * frame.x.dot(u).recip();
    let v = v * frame.y.dot(v).recip();
    let shift = origin - frame.origin;
    let uv = [shift.dot(u), shift.dot(v), dir.dot(u), dir.dot(v)];
    if !uv.into_iter().all(f64::is_finite) { return Err(invalid()); }
    Ok(Curve2::Line { origin: Pnt2::new(uv[0], uv[1]), dir: Vec2::new(uv[2], uv[3]) })
}

pub(super) fn point2(frame: Frame3, point: Pnt3) -> Pnt2 {
    let local = frame.to_local(point);
    Pnt2::new(local.x, local.y)
}

pub(super) fn intersects(a: Pnt2, b: Pnt2, c: Pnt2, d: Pnt2) -> bool {
    use semio_framework_3d_brep_vector::predicates::{Orient, orient2d};
    let overlap = |a: f64, b: f64, c: f64, d: f64| a.min(b).max(c.min(d)) <= a.max(b).min(c.max(d));
    if !overlap(a.x, b.x, c.x, d.x) || !overlap(a.y, b.y, c.y, d.y) { return false; }
    let separates = |first, second| first == second && first != Orient::Zero;
    !separates(orient2d(a, b, c), orient2d(a, b, d)) && !separates(orient2d(c, d, a), orient2d(c, d, b))
}

impl BrepRegistry {
    /// 🎟️ Admits one closed wire with a stable source address and label.
    pub fn plan_polygon_face(&mut self, source: &str, xy_only: bool) -> Result<Option<PolygonFacePublication>, ValueError> {
        let Entity::Wire(wire, label) = self.original_query_entity(source)? else { return Ok(None); };
        if wire.members.len() <= 1 { return Ok(None); }
        if !wire.closed || wire.members.len() < 3 || wire.members.len() > 65_536 {
            return Err(invalid());
        }
        let edges = wire.members.len();
        let label = *label;
        Ok(Some(self.start_polygon_face(edges, xy_only, Some(label), None)))
    }

    /// 🔒️ Admits a planar point-list face with a private closed polygon boundary.
    pub fn plan_polygon_face_points(&mut self, vertices: usize) -> Result<PolygonFacePublication, ValueError> {
        let wire = self.plan_polygon_wire(PolygonWireRecipe::Polyline { vertices, closed: true })?;
        Ok(self.start_polygon_face(vertices, false, None, Some(wire)))
    }

    fn start_polygon_face(&mut self, edges: usize, xy_only: bool, wire_label: Option<PersistentLabel>, boundary: Option<PolygonWirePublication>) -> PolygonFacePublication {
        let revision = self.shape_mutation_revision;
        let shape = self.body.new_label();
        self.unpublished_topology += 1;
        PolygonFacePublication {
            owner: std::ptr::from_ref(&*self) as usize, revision, wire_label, boundary, mapped: None, edges, xy_only,
            spline_index: None, spline: None, pcurve: None,
            origin: None, normal: NativeVec3::ZERO, frame: None, shape,
            handle: analytic_publication::primitive_handle(b"Face:", shape.0),
            journal: Vec::new(), face: None, ring: None, first_use: None, last_use: None,
            build: Build::Normal(0), open: true,
        }
    }

    /// 📏️ Quotes the original source and one topology operation.
    pub fn polygon_face_topology_demand(&self, face: &PolygonFacePublication, source: Option<&str>) -> Result<RetirementDemand, ValueError> {
        face.check_owner(self)?;
        face.demand(self, source)
    }

    /// 🧱️ Advances one source check or one exact planar topology record.
    pub fn step_polygon_face_topology(&mut self, plan: &mut PolygonFacePublication, source: Option<&str>, point: Option<EVec3>, effect: &mut RetainedAllocationEffect<'_>) -> Result<Progress, ValueError> {
        plan.check_owner(self)?;
        let grant = effect.grant();
        let demand = self.polygon_face_topology_demand(plan, source)?;
        if matches!(plan.build, Build::MappedBoundary) { return self.step_mapped_polygon_boundary(plan, demand, effect); }
        if let Some(boundary) = &mut plan.boundary {
            if !boundary.topology_ready() {
                if grant.maximum_items == 0 || !retained_clone_grant_admits(grant, demand) { return Ok(Default::default()); }
                let mut point = point;
                let mut next = None;
                if let Some(mapped) = plan.mapped {
                    if demand.capacity_bytes == 0 && boundary.next_point_index().is_some() {
                        let (translated, successor) = mapped.point(&self.body)?;
                        point = Some(translated);
                        next = Some(successor);
                    }
                }
                let mut progress = self.step_polygon_wire_topology(boundary, point, effect)?;
                if progress.copied_items > 0 {
                    if let (Some(mapped), Some(next)) = (&mut plan.mapped, next) {
                        mapped.advance(next);
                        progress.copied_bytes = demand.copy_bytes;
                    }
                }
                return Ok(progress);
            }
        }
        if grant.maximum_items == 0 || !retained_clone_grant_admits(grant, demand) || !effect.performed(1)? {
            return Ok(Default::default());
        }
        plan.source(self, source)?;
        let mut item = Progress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() };
        if plan.journal.capacity() == 0 {
            let count = plan.journal_capacity();
            plan.journal.try_reserve_exact(count)
                .map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "polygon face journal allocation failed"))?;
            item.retained_capacity_bytes = plan.journal.capacity() * size_of::<EntityRef>();
            if !item.fits(grant) {
                return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "polygon face journal exceeded grant").with_retained_progress(item));
            }
            return Ok(item);
        }
        match plan.build {
            Build::Normal(index) => {
                let (edge, forward, a, b, _) = plan.member(self, source, index)?;
                match plan.curve(self, edge)? {
                    Curve3::Line { .. } => {}
                    Curve3::Nurbs { .. } if plan.edges == 3 && plan.mapped.is_none() && plan.spline_index.is_none() => plan.spline_index = Some(index),
                    _ => return Err(invalid()),
                }
                let (a, b) = if forward { (a, b) } else { (b, a) };
                let origin = *plan.origin.get_or_insert(a);
                plan.normal = plan.normal + (a - origin).cross(b - origin);
                plan.build = if index + 1 == plan.edges {
                    let frame = Frame3::from_normal(origin, plan.normal).ok_or_else(invalid)?;
                    if plan.xy_only && frame.z.z.abs() < 1.0 - 1e-9 {
                        return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "planar face boundary must be parallel to the XY plane"));
                    }
                    plan.frame = Some(plan.mapped.and_then(|boundary| boundary.plane).map_or(frame, |plane| plane.frame));
                    Build::Plane(0)
                } else { Build::Normal(index + 1) };
            }
            Build::Plane(index) => {
                let (_, forward, a, b, _) = plan.member(self, source, index)?;
                let (_, next_forward, next_a, next_b, _) = plan.member(self, source, (index + 1) % plan.edges)?;
                let end = if forward { b } else { a };
                let next = if next_forward { next_a } else { next_b };
                let frame = plan.frame();
                let normal = frame.z.normalized().ok_or_else(invalid)?;
                let coplanarity = (a - frame.origin).dot(normal).abs().max((b - frame.origin).dot(normal).abs());
                let separation = (end - next).norm();
                let length = (a - b).norm();
                let tolerance = plan.tolerance().value();
                if !coplanarity.is_finite() || coplanarity > tolerance
                    || !separation.is_finite() || separation > tolerance
                    || !length.is_finite() || length <= tolerance {
                    return Err(invalid());
                }
                plan.build = if index + 1 < plan.edges { Build::Plane(index + 1) }
                    else if plan.spline_index.is_some() { Build::SplineParameters }
                    else if plan.edges > 3 { Build::Crossing { first: 0, second: 2 } } else { Build::Surface };
            }
            Build::Crossing { first, second } => {
                let (_, _, a, b, _) = plan.member(self, source, first)?;
                let (_, _, c, d, _) = plan.member(self, source, second)?;
                let frame = plan.frame();
                if intersects(point2(frame, a), point2(frame, b), point2(frame, c), point2(frame, d)) {
                    return Err(invalid());
                }
                if second + 1 < plan.edges - usize::from(first == 0) {
                    plan.build = Build::Crossing { first, second: second + 1 };
                } else if first + 3 < plan.edges {
                    plan.build = Build::Crossing { first: first + 1, second: first + 3 };
                } else {
                    plan.build = Build::Surface;
                }
            }
            Build::SplineParameters => {
                let (edge, _, _, _, range) = plan.member(self, source, plan.spline_index.ok_or_else(invalid)?)?;
                plan.spline = Some(QuadraticPcurve::new(plan.curve(self, edge)?, range).ok_or_else(invalid)?);
                plan.build = Build::SplineControl(0);
            }
            Build::SplineControl(index) => {
                let (edge, _) = plan.source(self, source)?.members[plan.spline_index.ok_or_else(invalid)?];
                let frame = plan.frame();
                let tolerance = plan.tolerance().value();
                let curve = plan.curve(self, edge)?;
                plan.spline.as_mut().ok_or_else(invalid)?.project(curve, index, frame, tolerance).ok_or_else(invalid)?;
                plan.build = if index + 1 < 3 { Build::SplineControl(index + 1) } else { Build::SplineCertify };
            }
            Build::SplineCertify => {
                let index = plan.spline_index.ok_or_else(invalid)?;
                let (_, _, a, b, _) = plan.member(self, source, index)?;
                let (_, forward, c, d, _) = plan.member(self, source, (index + 2) % 3)?;
                let third = if forward { c } else { d };
                if !plan.spline.as_ref().ok_or_else(invalid)?.verifies(plan.frame(), [a, b], third, plan.tolerance().value()) { return Err(invalid()); }
                plan.build = Build::Surface;
            }
            Build::SplinePcurve => {
                let curve = plan.spline.take().ok_or_else(invalid)?.materialize()?;
                item.retained_capacity_bytes = QuadraticPcurve::capacity(&curve);
                plan.pcurve = Some(curve);
                if !item.fits(grant) {
                    return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "planar spline pcurve exceeded grant").with_retained_progress(item));
                }
                plan.build = Build::Trim(plan.spline_index.ok_or_else(invalid)?);
            }
            Build::Surface => {
                let (id, receipt) = store_insert(&mut self.body.surfaces, grant, || Surface::Plane { frame: plan.frame() })?;
                return Ok(plan.record(id.map(EntityRef::Surface), receipt, Build::Face));
            }
            Build::Face => {
                let Some(EntityRef::Surface(surface)) = plan.journal.first().copied() else { return Err(ValueError::invariant("surface has not been admitted")); };
                let (id, receipt) = store_insert(&mut self.body.faces, grant, || Face {
                    surface, outer: None, inners: Vec::new(), flipped: plan.mapped.and_then(|boundary| boundary.plane).is_some_and(|plane| plane.flipped),
                    tol: plan.tolerance(), label: plan.shape,
                })?;
                plan.face = id;
                return Ok(plan.record(id.map(EntityRef::Face), receipt, Build::Ring));
            }
            Build::Ring => {
                let (id, receipt) = store_insert(&mut self.body.loops, grant, || Loop { first: dummy_coedge(), face: plan.face.unwrap() })?;
                plan.ring = id;
                return Ok(plan.record(id.map(EntityRef::Loop), receipt, plan.trim_phase(0)));
            }
            Build::Trim(index) => {
                if plan.spline_index == Some(index) && plan.pcurve.is_none() {
                    return Err(ValueError::invariant("spline trim was not funded before insertion"));
                }
                if self.body.curves2.next_insert_capacity_byte_demand(0)? != 0 {
                    return self.body.curves2.reserve_insert_step(grant).map_err(|(error, progress)| error.with_retained_progress(progress));
                }
                let (edge, _, a, b, range) = plan.member(self, source, index)?;
                if !range.0.is_finite() || !range.1.is_finite() || (range.1 - range.0).abs() < f64::EPSILON { return Err(invalid()); }
                let spline_trim = plan.spline_index == Some(index);
                let trim = if spline_trim { plan.pcurve.take().ok_or_else(invalid)? } else {
                    let curve = plan.curve(self, edge)?;
                    let tolerance = plan.tolerance().value();
                    if plan.spline_index.is_some() && !((curve.eval(range.0) - a).norm() <= tolerance
                        && (curve.eval(range.1) - b).norm() <= tolerance)
                    { return Err(invalid()); }
                    planar_line_pcurve(curve, plan.frame())?
                };
                let (id, receipt) = self.body.curves2.insert_reserved(trim, grant).map_err(|(error, trim)| {
                    if spline_trim { plan.pcurve = Some(trim); }
                    error
                })?;
                let mut progress = plan.record(Some(EntityRef::Curve2(id)), receipt, Build::Use(index));
                progress.copied_bytes = demand.copy_bytes;
                return Ok(progress);
            }
            Build::Use(index) => {
                if self.body.coedges.next_insert_capacity_byte_demand(0)? != 0 {
                    return self.body.coedges.reserve_insert_step(grant).map_err(|(error, progress)| error.with_retained_progress(progress));
                }
                let (edge, forward, _, _, range) = plan.member(self, source, index)?;
                let Some(EntityRef::Curve2(pcurve)) = plan.journal.last().copied() else { return Err(ValueError::invariant("pcurve has not been admitted")); };
                let next = if index + 1 == plan.edges { plan.first_use.unwrap() } else { dummy_coedge() };
                let prev = plan.last_use.unwrap_or_else(dummy_coedge);
                let (id, receipt) = self.body.coedges.insert_reserved(Coedge {
                    edge, forward, pcurve: Some(pcurve), prange: range,
                    loop_id: plan.ring.unwrap(), next, prev,
                }, grant).map_err(|(error, _)| error)?;
                if index == 0 { plan.first_use = Some(id); }
                let mut progress = plan.record(Some(EntityRef::Coedge(id)), receipt, Build::Lead(index));
                progress.copied_bytes = demand.copy_bytes;
                return Ok(progress);
            }
            Build::Lead(index) => {
                let Some(EntityRef::Coedge(current)) = plan.journal.last().copied() else { return Err(ValueError::invariant("coedge has not been admitted")); };
                if let Some(previous) = plan.last_use {
                    self.body.coedges.get_mut(previous).ok_or_else(invalid)?.next = current;
                }
                plan.last_use = Some(current);
                plan.build = if index + 1 < plan.edges { plan.trim_phase(index + 1) } else { Build::CloseLead };
            }
            Build::CloseLead => {
                self.body.coedges.get_mut(plan.first_use.unwrap()).ok_or_else(invalid)?.prev = plan.last_use.ok_or_else(invalid)?;
                plan.build = Build::FinishRing;
            }
            Build::FinishRing => {
                self.body.loops.get_mut(plan.ring.unwrap()).ok_or_else(invalid)?.first = plan.first_use.unwrap();
                plan.build = Build::FinishFace;
            }
            Build::FinishFace => {
                self.body.faces.get_mut(plan.face.unwrap()).ok_or_else(invalid)?.outer = plan.ring;
                plan.build = Build::Ready;
            }
            Build::MappedBoundary | Build::ReleaseJournal | Build::Ready => return Err(ValueError::invariant("polygon face does not have a topology step")),
        }
        Ok(item)
    }

    /// 📦️ Recognizes one axis-aligned horizontal rectangular wire or face without copying topology.
    pub fn plan_rectangular_extrusion(&mut self, source: &str, vector: EVec3) -> Result<Option<SolidPublication>, ValueError> {
        use semio_framework_3d_brep_solid_transaction::{RectangleProfile,rectangular_extrusion};
        let profile = match self.original_query_entity(source)? {
            Entity::Wire(wire,_) => RectangleProfile::Wire(wire),
            Entity::Face(face) => RectangleProfile::Face(*face),
            _ => return Ok(None),
        };
        rectangular_extrusion(&self.body,profile,vector)?.map(|primitive|self.plan_solid_primitive(primitive)).transpose()
    }

    /// 🗄️ Quotes the live index reservation for a completed polygon face.
    pub fn polygon_face_demand(&self, face: &PolygonFacePublication, copy: usize) -> Result<RetirementDemand, ValueError> {
        face.check_owner(self)?;
        let key = face.handle();
        Ok(RetirementDemand { capacity_bytes: self.live.next_insert_capacity_byte_demand(key, copy)?, depth: self.live.next_insert_depth_demand(key)?, ..Default::default() })
    }

    /// 💾️ Reserves the live index for an admitted planar face.
    pub fn reserve_polygon_face(&mut self, face: &PolygonFacePublication, grant: Grant) -> Result<Progress, ValueError> {
        self.polygon_face_demand(face, grant.maximum_copy_bytes)?;
        self.live.reserve_insert_step(face.handle(), grant).map_err(|(error, progress)| error.with_retained_progress(progress))
    }

    /// 📤️ Transfers the completed face and keeps its referenced boundary live.
    pub fn publish_polygon_face(&mut self, face: &mut PolygonFacePublication, key: String, grant: Grant) -> Result<Progress, (ValueError, String)> {
        if !face.topology_ready() || key != face.handle() || self.live.contains_key(&key) { return Err((ValueError::invariant("invalid polygon face publication"), key)); }
        if let Err(error) = face.check_owner(self) { return Err((error, key)); }
        if face.revision != self.shape_mutation_revision { return Err((ValueError::invariant("polygon face source changed before publication"), key)); }
        let (previous, receipt) = self.live.insert_reserved(key, Entity::Face(face.face.unwrap()), grant).map_err(|(error, key, _)| (error, key))?;
        assert!(previous.is_none());
        if let Some(boundary) = &mut face.boundary { self.transfer_polygon_wire_boundary(boundary); }
        self.unpublished_topology -= 1;
        self.advance_live_revision();
        face.build = Build::ReleaseJournal;
        Ok(receipt)
    }

    /// 🧹️ Quotes one unpublished native slot or the published cursor's journal release.
    pub fn polygon_face_close_demand(&self, face: &PolygonFacePublication) -> Result<RetirementDemand, ValueError> {
        if !face.open {
            let boundary = face.boundary.as_ref().ok_or_else(|| ValueError::invariant("polygon face is already retired"))?;
            return self.polygon_wire_close_demand(boundary);
        }
        face.check_owner(self)?;
        if !matches!(face.build, Build::ReleaseJournal) {
            if let Some(curve) = &face.pcurve {
                return Ok(RetirementDemand { depth: 1, release_bytes: QuadraticPcurve::capacity(curve), ..Default::default() });
            }
            if let Some(&entity) = face.journal.last() {
                let release_bytes = match entity {
                    EntityRef::Curve2(id) => self.body.curves2.get(id).map_or(0, QuadraticPcurve::capacity),
                    _ => 0,
                };
                return Ok(RetirementDemand { depth: topology_slot_depth(&self.body, entity)?, release_bytes, ..Default::default() });
            }
        }
        Ok(RetirementDemand { depth: 1, release_bytes: face.journal.capacity() * size_of::<EntityRef>(), ..Default::default() })
    }

    /// ♻️ Removes unpublished topology or only releases the published rollback journal.
    pub fn close_polygon_face(&mut self, face: &mut PolygonFacePublication, effect: &mut RetainedAllocationEffect<'_>) -> Result<Step, ValueError> {
        if !face.open {
            let boundary = face.boundary.as_mut().ok_or_else(|| ValueError::invariant("polygon face is already retired"))?;
            let step = self.close_polygon_wire(boundary, effect)?;
            if boundary.terminal_is_empty() { face.boundary = None; }
            return Ok(step);
        }
        let demand = self.polygon_face_close_demand(face)?;
        if effect.grant().maximum_items == 0 || !retained_clone_grant_admits(effect.grant(), demand) || !effect.performed(1)? {
            return Ok(Step::Progress(Progress::default()));
        }
        let progress = Progress { copied_items: 1, released_bytes: demand.release_bytes, ..Default::default() };
        if !matches!(face.build, Build::ReleaseJournal) {
            if face.pcurve.take().is_some() { return Ok(Step::Progress(progress)); }
            if let Some(entity) = face.journal.pop() {
                remove_topology_slot(&mut self.body, entity);
                return Ok(Step::Progress(progress));
            }
            self.unpublished_topology -= 1;
        }
        drop(take(&mut face.journal));
        face.open = false;
        Ok(if face.boundary.is_some() { Step::Progress(progress) } else { Step::Complete(progress) })
    }
}
