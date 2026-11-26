//! 📮️ BREP registry admission and atomic publication of an imported mesh root.

use super::*;
use semio_framework_3d_brep_mesh_io::MeshImportDraft;
use semio_framework_3d_brep_native_registry::primitive_handle;
use semio_framework_value::{RetainedAllocationEffect, RetainedCloneGrant as Grant, RetainedCloneProgress as Progress, RetainedCloneStep as Step, RetirementDemand, ValueError};

/// 🔐️ Binds a private mesh draft to the registry authorized to publish it.
pub struct MeshPublication {
    draft: MeshImportDraft,
    owner: usize,
    revision: u64,
}

impl MeshPublication {
    pub fn handle(&self) -> Option<&str> { self.draft.handle() }
    pub fn kind(&self) -> &'static str { self.draft.kind() }
    pub fn terminal_is_empty(&self) -> bool { self.draft.terminal_is_empty() }

    fn check(&self, kernel: &Brep, active: bool) -> Result<(), ValueError> {
        if self.owner != std::ptr::from_ref(&kernel.native) as usize || active && self.revision != kernel.native.shape_mutation_revision {
            return Err(ValueError::invariant("original mesh import owner changed"));
        }
        Ok(())
    }
}

impl Brep {
    pub fn plan_mesh_publication(&mut self, decoder: &mut MeshDecode) -> Result<MeshPublication, ValueError> {
        let cursor = decoder.into_cursor()?;
        self.native.unpublished_topology += 1;
        Ok(MeshPublication { draft: MeshImportDraft::new(cursor), owner: std::ptr::from_ref(&self.native) as usize, revision: self.native.shape_mutation_revision })
    }

    pub fn mesh_preparation_demand(&self, plan: &MeshPublication) -> Result<RetirementDemand, ValueError> {
        plan.check(self, true)?;
        plan.draft.preparation_demand(&self.native.body)
    }

    pub fn step_mesh_preparation(&mut self, plan: &mut MeshPublication, effect: &mut RetainedAllocationEffect<'_>) -> Result<Progress, ValueError> {
        plan.check(self, true)?;
        plan.draft.step_preparation(&mut self.native.body, primitive_handle, effect)
    }

    pub fn mesh_publication_demand(&self, plan: &MeshPublication, copy: usize) -> Result<RetirementDemand, ValueError> {
        plan.check(self, true)?;
        let key = plan.handle().filter(|key| !plan.draft.published() && !self.native.live.contains_key(*key))
            .ok_or_else(|| ValueError::invariant("original mesh root is already live or incomplete"))?;
        Ok(RetirementDemand { capacity_bytes: self.native.live.next_insert_capacity_byte_demand(key, copy)?,
            depth: self.native.live.next_insert_depth_demand(key)?.max(self.mesh_preparation_demand(plan)?.depth), ..Default::default() })
    }

    pub fn reserve_mesh_publication(&mut self, plan: &MeshPublication, grant: Grant) -> Result<Progress, ValueError> {
        self.mesh_publication_demand(plan, grant.maximum_copy_bytes)?;
        self.native.live.reserve_insert_step(plan.handle().unwrap(), grant).map_err(|(error, receipt)| error.with_retained_progress(receipt))
    }

    pub fn publish_mesh(&mut self, plan: &mut MeshPublication, key: String, grant: Grant) -> Result<Progress, (ValueError, String)> {
        if plan.handle() != Some(key.as_str()) { return Err((ValueError::invariant("original mesh publication key changed"), key)); }
        if let Err(error) = self.mesh_publication_demand(plan, grant.maximum_copy_bytes) { return Err((error, key)); }
        let entity = match plan.draft.root() {
            Some(EntityRef::Solid(id)) => Entity::Solid(id, Some(SolidProvenance::PlanarBoundary(None))),
            Some(EntityRef::Shell(id)) => Entity::Shell(id),
            _ => unreachable!("prepared mesh publication has a native root"),
        };
        let (prior, receipt) = self.native.live.insert_reserved(key, entity, grant).map_err(|(error, key, _)| (error, key))?;
        assert!(prior.is_none());
        plan.draft.commit();
        self.native.unpublished_topology -= 1;
        self.native.advance_live_revision();
        Ok(receipt)
    }

    pub fn mesh_publication_close_demand(&self, plan: &MeshPublication, copy: usize) -> Result<RetirementDemand, ValueError> {
        plan.check(self, false)?;
        plan.draft.close_demand(&self.native.body, copy)
    }

    pub fn close_mesh_publication(&mut self, plan: &mut MeshPublication, effect: &mut RetainedAllocationEffect<'_>) -> Result<Step, ValueError> {
        plan.check(self, false)?;
        if plan.terminal_is_empty() { return Ok(Step::Complete(Progress::default())); }
        let result = plan.draft.close(&mut self.native.body, effect);
        if !plan.draft.published() && plan.terminal_is_empty() { self.native.unpublished_topology -= 1; }
        result
    }
}
