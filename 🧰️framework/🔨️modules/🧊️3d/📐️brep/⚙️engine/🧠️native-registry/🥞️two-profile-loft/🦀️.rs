//! 🥞️ Exact original lofts between two correspondingly translated planar sections.

use super::*;
use semio_framework_3d_brep_representation::arena::{CoedgeId, LoopId};
use semio_framework_value::{RetainedAllocationEffect, RetainedCloneProgress as Progress, RetirementDemand};
use semio_framework_value::retained_clone::retained_clone_grant_admits;
use std::mem::size_of;

fn unsupported() -> ValueError {
    ValueError::literal(ValueRefusalKind::UnsupportedOwner, "original Loft requires two matching translated line or coaxial circular profiles")
}

fn invalid() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, "Loft profiles must enclose a finite nondegenerate solid")
}

fn matching_line_offset(previous: Option<NativeVec3>, first: [Pnt3; 2], last: [Pnt3; 2]) -> Option<NativeVec3> {
    let [start, end] = std::array::from_fn(|index| last[index] - first[index]);
    let displacement = previous.unwrap_or(start);
    [start, end].into_iter().all(|offset| {
        let difference = (offset - displacement).norm();
        difference.is_finite() && difference <= Tol::DEFAULT.value()
    }).then_some(displacement)
}

#[derive(Clone, Copy)]
struct Profile {
    id: FaceId,
    ring: LoopId,
    first: CoedgeId,
    current: CoedgeId,
    frame: Frame3,
}

/// 🧮️ Verifies one pair of directed line trims per original turn.
pub struct LoftDraft {
    owner: usize,
    revision: u64,
    profiles: [Profile; 2],
    displacement: Option<NativeVec3>,
    pairs: usize,
    complete: bool,
}

/// 🧱️ The exact solid or the live line correspondence check selected for a two-profile loft.
pub enum LoftPlan {
    Cylinder(SolidPublication),
    Lines(LoftDraft),
}

impl BrepRegistry {
    fn loft_profile(&self, handle: &str) -> Result<Profile, ValueError> {
        let Entity::Face(id) = self.original_query_entity(handle)? else { return Err(unsupported()); };
        let face = self.body.faces.get(*id).ok_or_else(unsupported)?;
        if !face.inners.is_empty() { return Err(unsupported()); }
        let Some(Surface::Plane { frame }) = self.body.surfaces.get(face.surface) else { return Err(unsupported()); };
        let ring = face.outer.ok_or_else(unsupported)?;
        let loop_ = self.body.loops.get(ring).ok_or_else(unsupported)?;
        if loop_.face != *id { return Err(unsupported()); }
        Ok(Profile { id: *id, ring, first: loop_.first, current: loop_.first, frame: *frame })
    }

    /// 🎟️ Quotes the paired original face and boundary paths for initial Loft planning.
    pub fn loft_depth(&self, handles: [&str; 2]) -> Result<usize, ValueError> {
        Ok(self.planar_face_boundary_depth(handles[0])?.max(self.planar_face_boundary_depth(handles[1])?))
    }

    /// 🥞️ Stages a congruent coaxial cylinder or starts line correspondence acquisition.
    pub fn plan_exact_loft(&mut self, handles: [&str; 2]) -> Result<LoftPlan, ValueError> {
        let profiles = [self.loft_profile(handles[0])?, self.loft_profile(handles[1])?];
        let [a, b] = profiles;
        let displacement = b.frame.origin - a.frame.origin;
        if a.frame.z.cross(b.frame.z).norm() > 1e-9 { return Err(unsupported()); }
        let height = displacement.dot(a.frame.z);
        if !height.is_finite() || height.abs() <= Tol::DEFAULT.value() { return Err(invalid()); }
        match [self.conic_face_boundary(a.id), self.conic_face_boundary(b.id)] {
            [Some((Curve3::Circle { radius: first, .. }, _, _)), Some((Curve3::Circle { radius: last, .. }, _, _))]
                if first == last && a.frame.x == b.frame.x && a.frame.y == b.frame.y && a.frame.z == b.frame.z
                    && displacement.cross(a.frame.z).norm() <= Tol::DEFAULT.value() =>
            {
                self.plan_circular_extrusion(handles[0], [displacement.x, displacement.y, displacement.z])?
                    .map(LoftPlan::Cylinder).ok_or_else(unsupported)
            }
            [None, None] => Ok(LoftPlan::Lines(LoftDraft {
                owner: std::ptr::from_ref(&*self) as usize, revision: self.shape_mutation_revision,
                profiles, displacement: None, pairs: 0, complete: false,
            })),
            _ => Err(unsupported()),
        }
    }

    fn loft_line(&self, profile: Profile) -> Result<(CoedgeId, bool, &Curve3, (f64, f64)), ValueError> {
        let use_ = self.body.coedges.get(profile.current).ok_or_else(unsupported)?;
        if use_.loop_id != profile.ring { return Err(unsupported()); }
        let edge = self.body.edges.get(use_.edge).ok_or_else(unsupported)?;
        let curve = self.body.curves3.get(edge.curve).ok_or_else(unsupported)?;
        if !matches!(curve, Curve3::Line { .. }) { return Err(unsupported()); }
        Ok((use_.next, use_.forward, curve, edge.range))
    }

    /// 📏️ Quotes the live profile roots and one corresponding trim on each side.
    pub fn loft_draft_demand(&self, handles: [&str; 2], draft: &LoftDraft) -> Result<RetirementDemand, ValueError> {
        if draft.owner != std::ptr::from_ref(self) as usize || draft.revision != self.shape_mutation_revision {
            return Err(ValueError::invariant("original Loft profile changed during acquisition"));
        }
        let mut depth = 1;
        for (handle, profile) in handles.into_iter().zip(draft.profiles) {
            depth = depth.max(self.tessellation_source_depth_demand(handle)?);
            if !matches!(self.original_query_entity(handle)?, Entity::Face(id) if *id == profile.id) {
                return Err(ValueError::invariant("original Loft input no longer matches its face"));
            }
            if !draft.complete {
                let use_ = self.body.coedges.get(profile.current).ok_or_else(unsupported)?;
                let edge = self.body.edges.get(use_.edge).ok_or_else(unsupported)?;
                depth = depth.max(self.body.coedges.next_remove_depth_demand(profile.current)?)
                    .max(self.body.edges.next_remove_depth_demand(use_.edge)?)
                    .max(self.body.curves3.next_remove_depth_demand(edge.curve)?);
            }
        }
        if draft.complete { depth = depth.max(self.planar_face_boundary_depth(handles[0])?); }
        let copy_bytes = if draft.complete { 0 } else { 4 * size_of::<Pnt3>() };
        Ok(RetirementDemand { depth, copy_bytes, ..Default::default() })
    }

    /// 🛠️ Advances one exact paired edge check or transfers the first section into the prism publisher.
    pub fn step_loft_draft(&self, handles: [&str; 2], draft: &mut LoftDraft, effect: &mut RetainedAllocationEffect<'_>)
        -> Result<(Option<PrismDraft>, Progress), ValueError> {
        let demand = self.loft_draft_demand(handles, draft)?;
        if !retained_clone_grant_admits(effect.grant(), demand) || !effect.performed(1)? {
            return Ok((None, Progress::default()));
        }
        let receipt = Progress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() };
        if draft.complete {
            let offset = draft.displacement.ok_or_else(invalid)?;
            let prism = self.plan_prism(handles[0], [offset.x, offset.y, offset.z])?.ok_or_else(unsupported)?;
            return Ok((Some(prism), receipt));
        }
        let [a, b] = draft.profiles;
        let (a_next, a_forward, a_curve, a_range) = self.loft_line(a)?;
        let (b_next, b_forward, b_curve, b_range) = self.loft_line(b)?;
        if a_forward != b_forward { return Err(unsupported()); }
        let first = [a_curve.eval(a_range.0), a_curve.eval(a_range.1)];
        let last = [b_curve.eval(b_range.0), b_curve.eval(b_range.1)];
        draft.displacement = Some(matching_line_offset(draft.displacement, first, last).ok_or_else(unsupported)?);
        let closed = a_next == a.first;
        if closed != (b_next == b.first) { return Err(unsupported()); }
        draft.pairs += 1;
        if (closed && draft.pairs < 3) || (!closed && draft.pairs == semio_framework_3d_brep_construction::prism_boundary::MAX_PRISM_CORNERS) {
            return Err(unsupported());
        }
        draft.profiles[0].current = a_next;
        draft.profiles[1].current = b_next;
        draft.complete = closed;
        Ok((None, receipt))
    }
}
