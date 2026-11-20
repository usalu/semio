//! 🎟️ Retained nearest-point traversal bound to its original live face.

use super::*;
use semio_framework_value::{RetainedAllocationEffect, RetainedCloneProgress as Progress, RetirementDemand};
use semio_framework_value::retained_clone::retained_clone_grant_admits;

/// 🎯️ Holds a face projection cursor and its exact engine and topology revision.
pub struct PlanarClosestJob {
    owner: usize,
    revision: u64,
    cursor: analysis::PlanarClosest,
}

impl Brep {
    /// 🪪️ Admits the original face boundary cursor when fixed analytic projection does not apply.
    pub fn plan_planar_closest(&self, handle: &str, target: EVec3) -> Result<Option<PlanarClosestJob>, ValueError> {
        let Entity::Face(id) = self.native.original_query_entity(handle)? else { return Ok(None); };
        if self.conic_face_preview_source(*id).is_some() { return Ok(None); }
        Ok(Some(PlanarClosestJob {
            owner: std::ptr::from_ref(self) as usize, revision: self.native.shape_mutation_revision,
            cursor: analysis::PlanarClosest::new(*id, pnt(target)),
        }))
    }

    /// 📏️ Quotes the next bounded read from the same live face and shape revision.
    pub fn planar_closest_demand(&self, handle: &str, job: &PlanarClosestJob) -> Result<RetirementDemand, ValueError> {
        if job.owner != std::ptr::from_ref(self) as usize || job.revision != self.native.shape_mutation_revision
            || !matches!(self.native.live.get(handle), Some(Entity::Face(id)) if *id == job.cursor.source())
        {
            return Err(ValueError::invariant("original closest point face source changed"));
        }
        let mut demand = job.cursor.next_demand(&self.native.body)?;
        demand.depth = demand.depth.max(self.native.tessellation_source_depth_demand(handle)?);
        Ok(demand)
    }

    /// ⏱️ Advances one original planar cursor step using the caller's grant.
    pub fn step_planar_closest(&self, handle: &str, job: &mut PlanarClosestJob, effect: &mut RetainedAllocationEffect<'_>) -> Result<(Option<EVec3>, Progress), ValueError> {
        let demand = self.planar_closest_demand(handle, job)?;
        if effect.grant().maximum_items == 0 || !retained_clone_grant_admits(effect.grant(), demand) || !effect.performed(1)? {
            return Ok((None, Progress::default()));
        }
        let progress = Progress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() };
        let point = job.cursor.step(&self.native.body).map_err(|error| error.with_retained_progress(progress))?;
        Ok((point.map(evec), progress))
    }
}
