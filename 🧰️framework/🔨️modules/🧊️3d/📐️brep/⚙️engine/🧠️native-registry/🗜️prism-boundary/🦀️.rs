//! 🗜️ A retained extruded boundary for simple planar polygon faces.

use super::*;
use semio_framework_3d_brep_construction::prism_boundary::MAX_PRISM_CORNERS as MAX_CORNERS;
pub(super) use semio_framework_3d_brep_construction::prism_boundary::PrismBoundary;
use semio_framework_3d_brep_representation::arena::{CoedgeId, LoopId};
use semio_framework_value::{RetainedAllocationEffect, RetainedCloneProgress as Progress, RetirementDemand, ValueError, ValueRefusalKind};
use semio_framework_value::retained_clone::retained_clone_grant_admits;
use std::mem::size_of;

const ZERO: Pnt3 = Pnt3 { x: 0., y: 0., z: 0. };

#[derive(Clone, Copy)]
enum Source {
    Face { id: FaceId, ring: LoopId, first: CoedgeId, current: CoedgeId },
    Wire { label: PersistentLabel, edges: usize },
}

#[derive(Clone, Copy)]
enum DraftPhase {
    Gather,
    Coplanar(usize),
    Crossing { first: usize, second: usize },
    Ready,
}

/// 🧮️ Collects one original boundary corner per admitted topology turn.
pub struct PrismDraft {
    owner: usize,
    revision: u64,
    source: Source,
    expected_vertex: Option<VertexId>,
    first_vertex: Option<VertexId>,
    corners: [Pnt3; MAX_CORNERS],
    count: usize,
    normal: NativeVec3,
    vector: NativeVec3,
    plane: Option<Frame3>,
    placement: Option<Frame3>,
    phase: DraftPhase,
}

fn invalid() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, "extrusion requires a simple planar line face and a finite transverse vector")
}

fn unsupported() -> ValueError {
    ValueError::literal(ValueRefusalKind::UnsupportedOwner, "original prism accepts up to 32 line edges and no holes")
}

impl BrepRegistry {
    /// 🌱️ Starts a bound, allocation-free extrusion recipe from one planar face.
    pub fn plan_prism(&self, source: &str, vector: EVec3) -> Result<Option<PrismDraft>, ValueError> {
        self.plan_prism_with_placement(source, vector, None)
    }

    pub(super) fn plan_prism_with_placement(&self, source: &str, vector: EVec3, placement: Option<Frame3>) -> Result<Option<PrismDraft>, ValueError> {
        let (source, plane) = match self.original_query_entity(source)? {
            Entity::Face(id) => {
                let face = self.body.faces.get(*id).ok_or_else(invalid)?;
                if !face.inners.is_empty() { return Err(unsupported()); }
                let Surface::Plane { frame } = self.body.surfaces.get(face.surface).ok_or_else(invalid)? else { return Err(unsupported()); };
                let ring = face.outer.ok_or_else(invalid)?;
                let first = self.body.loops.get(ring).ok_or_else(invalid)?.first;
                (Source::Face { id: *id, ring, first, current: first }, Some(*frame))
            }
            Entity::Wire(wire, label) if placement.is_none() && wire.closed && (3..=MAX_CORNERS).contains(&wire.members.len()) => {
                (Source::Wire { label: *label, edges: wire.members.len() }, None)
            }
            _ => return Ok(None),
        };
        let vector = vec3(vector);
        if !vector.x.is_finite() || !vector.y.is_finite() || !vector.z.is_finite()
            || vector.norm() <= Tol::DEFAULT.value()
            || placement.or(plane).is_some_and(|frame| frame.z.dot(vector).abs() <= Tol::DEFAULT.value()) {
            return Err(invalid());
        }
        Ok(Some(PrismDraft {
            owner: std::ptr::from_ref(self) as usize, revision: self.shape_mutation_revision, source,
            expected_vertex: None, first_vertex: None, corners: [ZERO; MAX_CORNERS], count: 0, normal: NativeVec3::ZERO,
            vector, plane, placement, phase: DraftPhase::Gather,
        }))
    }

    fn prism_member(&self, handle: &str, draft: &PrismDraft) -> Result<(EdgeId, bool, Option<CoedgeId>), ValueError> {
        match (draft.source, self.original_query_entity(handle)?) {
            (Source::Face { id, ring, current, .. }, Entity::Face(live)) if id == *live => {
                let use_ = self.body.coedges.get(current).ok_or_else(invalid)?;
                if use_.loop_id != ring { return Err(invalid()); }
                Ok((use_.edge, use_.forward, Some(use_.next)))
            }
            (Source::Wire { label, edges }, Entity::Wire(wire, live)) if label == *live && edges == wire.members.len() => {
                let (edge, forward) = wire.members.get(draft.count).copied().ok_or_else(invalid)?;
                Ok((edge, forward, None))
            }
            _ => Err(ValueError::invariant("original prism source handle changed")),
        }
    }

    /// 📏️ Quotes one source lookup and bounded curve/vertex read.
    pub fn prism_draft_demand(&self, source: &str, draft: &PrismDraft) -> Result<RetirementDemand, ValueError> {
        if draft.owner != std::ptr::from_ref(self) as usize || draft.revision != self.shape_mutation_revision {
            return Err(ValueError::invariant("original prism face source changed"));
        }
        let mut depth = self.tessellation_source_depth_demand(source)?.max(1);
        if !matches!(draft.phase, DraftPhase::Gather) {
            let copy_bytes = if matches!(draft.phase, DraftPhase::Crossing { .. }) { 4 * size_of::<Pnt3>() } else { 0 };
            return Ok(RetirementDemand { depth, copy_bytes, ..Default::default() });
        }
        if let Source::Face { current, .. } = draft.source {
            depth = depth.max(self.body.coedges.next_remove_depth_demand(current)?);
        }
        let (edge, _, _) = self.prism_member(source, draft)?;
        depth = depth.max(self.body.edges.next_remove_depth_demand(edge)?);
        let edge = self.body.edges.get(edge).ok_or_else(invalid)?;
        depth = depth.max(self.body.curves3.next_remove_depth_demand(edge.curve)?)
            .max(self.body.vertices.next_remove_depth_demand(edge.v0)?)
            .max(self.body.vertices.next_remove_depth_demand(edge.v1)?);
        Ok(RetirementDemand { depth, copy_bytes: 2 * size_of::<Pnt3>(), ..Default::default() })
    }

    /// 🛠️ Advances one face boundary member or transfers the resulting exact solid recipe.
    pub fn step_prism_draft(&mut self, source: &str, draft: &mut PrismDraft, effect: &mut RetainedAllocationEffect<'_>) -> Result<(Option<SolidPublication>, Progress), ValueError> {
        let demand = self.prism_draft_demand(source, draft)?;
        let grant = effect.grant();
        if grant.maximum_items == 0 || !retained_clone_grant_admits(grant, demand) || !effect.performed(1)? {
            return Ok((None, Default::default()));
        }
        let receipt = Progress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() };
        match draft.phase {
            DraftPhase::Ready => {
                let boundary = PrismBoundary::from_planar_ring(draft.corners, draft.count, draft.vector, draft.normal).ok_or_else(invalid)?;
                draft.corners = [ZERO; MAX_CORNERS];
                return self.plan_exchange_boundary(SolidExchangeBoundary::Prism(boundary)).map(|solid| (Some(solid), receipt));
            }
            DraftPhase::Coplanar(index) => {
                let frame = draft.plane.ok_or_else(invalid)?;
                let distance = frame.to_local(draft.corners[index]).z.abs();
                if !distance.is_finite() || distance > Tol::DEFAULT.value() { return Err(invalid()); }
                draft.phase = if index + 1 < draft.count { DraftPhase::Coplanar(index + 1) }
                    else if draft.count > 3 { DraftPhase::Crossing { first: 0, second: 2 } }
                    else { DraftPhase::Ready };
                return Ok((None, receipt));
            }
            DraftPhase::Crossing { first, second } => {
                use semio_framework_3d_brep_solid_topology::{point2, segments_intersect};
                let frame = draft.plane.ok_or_else(invalid)?;
                let corner = |index| point2(frame, draft.corners[index % draft.count]);
                if segments_intersect(corner(first), corner(first + 1), corner(second), corner(second + 1)) { return Err(invalid()); }
                draft.phase = if second + 1 < draft.count - usize::from(first == 0) {
                    DraftPhase::Crossing { first, second: second + 1 }
                } else if first + 3 < draft.count {
                    DraftPhase::Crossing { first: first + 1, second: first + 3 }
                } else {
                    DraftPhase::Ready
                };
                return Ok((None, receipt));
            }
            DraftPhase::Gather => {}
        }
        if draft.count == MAX_CORNERS { return Err(unsupported()); }
        let (edge, forward, next) = self.prism_member(source, draft)?;
        let edge = self.body.edges.get(edge).ok_or_else(invalid)?;
        if !matches!(self.body.curves3.get(edge.curve), Some(Curve3::Line { .. })) { return Err(unsupported()); }
        let (start, end) = if forward { (edge.v0, edge.v1) } else { (edge.v1, edge.v0) };
        if draft.expected_vertex.is_some_and(|expected| start != expected) { return Err(invalid()); }
        let point = |id| self.body.vertices.get(id).map(|vertex| vertex.position).ok_or_else(invalid);
        let (mut a, mut b) = (point(start)?, point(end)?);
        if let Some((from, to)) = draft.plane.zip(draft.placement) {
            a = to.to_world(from.to_local(a));
            b = to.to_world(from.to_local(b));
        }
        let tol = Tol::DEFAULT.value();
        if (a-b).norm() <= tol || (b-a).cross(draft.vector).normalized().is_none()
            || [a, b, a+draft.vector, b+draft.vector].into_iter().any(|point| [point.x,point.y,point.z].into_iter().any(|value| !value.is_finite())) {
            return Err(invalid());
        }
        if draft.count == 0 { draft.first_vertex = Some(start); }
        draft.corners[draft.count] = a;
        draft.count += 1;
        let origin = draft.corners[0];
        draft.normal = draft.normal + (a - origin).cross(b - origin);
        draft.expected_vertex = Some(end);
        let complete = match &mut draft.source {
            Source::Face { current, first, .. } => {
                *current = next.ok_or_else(invalid)?;
                *current == *first
            }
            Source::Wire { edges, .. } => draft.count == *edges,
        };
        if complete {
            if draft.count < 3 || Some(end) != draft.first_vertex { return Err(invalid()); }
            let frame = Frame3::from_normal(draft.corners[0], draft.normal).ok_or_else(invalid)?;
            let transverse = frame.z.dot(draft.vector).abs();
            if !transverse.is_finite() || transverse <= Tol::DEFAULT.value() { return Err(invalid()); }
            draft.plane = Some(frame);
            draft.phase = DraftPhase::Coplanar(0);
        }
        Ok((None, receipt))
    }
}
