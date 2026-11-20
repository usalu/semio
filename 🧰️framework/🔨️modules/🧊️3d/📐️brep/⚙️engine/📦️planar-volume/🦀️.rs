//! 📦️ Engine ownership and retained grants for planar area and volume integration.

use super::{Brep, Entity};
use semio_framework_3d_brep_analysis::{PlanarMeasure, analytic_curve_length};
use semio_framework_value::{RetainedAllocationEffect, RetainedCloneProgress as Progress, RetirementDemand, ValueError, ValueRefusalKind};
use semio_framework_value::retained_clone::retained_clone_grant_admits;

/// 🔬️ Keeps the planar integration bound to its original live engine and revision.
pub struct PlanarMeasureJob {
    owner: usize,
    revision: u64,
    integration: PlanarMeasure,
}

impl Brep {
    /// 🎟️ Opens a read-only planar integration against one live solid and topology revision.
    pub fn plan_planar_measure(&self, handle: &str, measure_area: bool) -> Result<PlanarMeasureJob, ValueError> {
        let Some(Entity::Solid(source, _)) = self.native.live.get(handle) else { return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "original measurement requires a certified primitive or planar polygonal solid")); };
        Ok(PlanarMeasureJob {
            owner: std::ptr::from_ref(self) as usize,
            revision: self.native.shape_mutation_revision,
            integration: PlanarMeasure::new(*source, measure_area),
        })
    }

    /// 🪙️ Quotes the same bound source and next original topology projection.
    pub fn planar_measure_demand(&self, handle: &str, job: &PlanarMeasureJob) -> Result<RetirementDemand, ValueError> {
        if job.owner != std::ptr::from_ref(self) as usize || job.revision != self.native.shape_mutation_revision
            || !matches!(self.native.live.get(handle), Some(Entity::Solid(id, _)) if *id == job.integration.source())
        {
            return Err(ValueError::invariant("original planar measurement source changed"));
        }
        let live_depth = self.native.live.next_contains_depth_demand(handle)?;
        let mut demand = job.integration.next_demand(&self.native.body)?;
        demand.depth = demand.depth.max(live_depth);
        Ok(demand)
    }

    /// ⏱️ Integrates one polygon boundary step using the original caller effect.
    pub fn step_planar_measure(&self, handle: &str, job: &mut PlanarMeasureJob, effect: &mut RetainedAllocationEffect<'_>)
        -> Result<(Option<f64>, Progress), ValueError> {
        let demand = self.planar_measure_demand(handle, job)?;
        if !retained_clone_grant_admits(effect.grant(), demand) || !effect.performed(1)? { return Ok((None, Progress::default())); }
        let progress = Progress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() };
        let volume = job.integration.step(&self.native.body).map_err(|error| error.with_retained_progress(progress))?;
        Ok((volume, progress))
    }
}


/// 📏️ Measures distinct wire edges from the original live source without a copied edge set.
pub struct WireLengthJob {
    owner: usize,
    revision: u64,
    label: super::PersistentLabel,
    total: f64,
    index: usize,
    prior: usize,
    ascending: bool,
}

impl Brep {
    /// 🧮️ Opens an original read cursor only for a wire.
    pub fn plan_wire_length(&self, handle: &str) -> Result<Option<WireLengthJob>, ValueError> {
        let Entity::Wire(_, label) = self.native.original_query_entity(handle)? else { return Ok(None); };
        Ok(Some(WireLengthJob {
            owner: std::ptr::from_ref(self) as usize, revision: self.native.shape_mutation_revision,
            label: *label, total: 0., index: 0, prior: 0, ascending: true,
        }))
    }

    fn wire_length_source<'a>(&'a self, handle: &str, job: &WireLengthJob) -> Result<&'a super::Wire, ValueError> {
        if job.owner == std::ptr::from_ref(self) as usize && job.revision == self.native.shape_mutation_revision {
            if let Entity::Wire(wire, label) = self.native.original_query_entity(handle)? {
                if *label == job.label { return Ok(wire); }
            }
        }
        Err(ValueError::invariant("original length wire source changed"))
    }

    /// 🥾️ Quotes one distinct-edge check or one fixed analytic measurement.
    pub fn wire_length_demand(&self, handle: &str, job: &WireLengthJob) -> Result<RetirementDemand, ValueError> {
        let wire = self.wire_length_source(handle, job)?;
        let mut depth = self.tessellation_source_depth_demand(handle)?.max(1);
        let mut copy_bytes = 0;
        if let Some(&(id, _)) = wire.members.get(job.index) {
            depth = depth.max(self.native.body.edges.next_remove_depth_demand(id)?);
            let edge = self.native.body.edges.get(id).ok_or_else(|| ValueError::invariant("live length edge is missing"))?;
            depth = depth.max(self.native.body.curves3.next_remove_depth_demand(edge.curve)?);
            copy_bytes = if job.prior == job.index || job.ascending { 2 * std::mem::size_of::<f64>() } else { std::mem::size_of::<super::EdgeId>() };
        }
        Ok(RetirementDemand { depth, copy_bytes, ..Default::default() })
    }

    /// ⏱️ Measures one wire edge or skips one previously counted orientation.
    pub fn step_wire_length(&self, handle: &str, job: &mut WireLengthJob, effect: &mut RetainedAllocationEffect<'_>) -> Result<(Option<f64>, Progress), ValueError> {
        let demand = self.wire_length_demand(handle, job)?;
        if effect.grant().maximum_items == 0 || !retained_clone_grant_admits(effect.grant(), demand) || !effect.performed(1)? {
            return Ok((None, Default::default()));
        }
        let wire = self.wire_length_source(handle, job)?;
        let receipt = Progress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() };
        let Some(&(id, _)) = wire.members.get(job.index) else { return Ok((Some(job.total), receipt)); };
        let edge = self.native.body.edges.get(id).ok_or_else(|| ValueError::invariant("live length edge is missing"))?;
        job.ascending &= job.label.0.checked_sub((wire.members.len() - job.index) as u64) == Some(edge.label.0);
        if !job.ascending && job.prior < job.index {
            if wire.members[job.prior].0 == id {
                job.index += 1;
                job.prior = 0;
            } else {
                job.prior += 1;
            }
            return Ok((None, receipt));
        }
        let curve = self.native.body.curves3.get(edge.curve).ok_or_else(|| ValueError::invariant("live length curve is missing"))?;
        job.total += semio_framework_3d_brep_analysis::analytic_curve_length(curve, edge.range)?;
        if !job.total.is_finite() {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "wire length must be finite"));
        }
        job.index += 1;
        job.prior = 0;
        Ok((None, receipt))
    }
}
