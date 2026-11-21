//! 📥️ Private native import custody and atomic publication of a shell or solid root.

use super::*;
use semio_framework_3d_brep_native_registry::primitive_handle;
use semio_framework_value::{RetainedAllocationEffect, RetainedCloneGrant as Grant, RetainedCloneProgress as Progress, RetainedCloneStep as Step, RetirementDemand, ValueError};
use semio_framework_value::retained_clone::retained_clone_grant_admits;

/// 🧾️ Owns imported topology until the matching registry publishes its single root.
pub struct MeshPublication {
    cursor: MeshImportCursor,
    owner: usize,
    revision: u64,
    key: Option<[u8; 64]>,
    published: bool,
}

impl MeshPublication {
    pub fn handle(&self) -> Option<&str> { self.key.as_ref().map(|key| std::str::from_utf8(key).expect("mesh digest is ASCII")) }
    pub fn kind(&self) -> &'static str { if matches!(self.cursor.original_import_root(), Some(EntityRef::Solid(_))) { "solid" } else { "shell" } }
    pub fn terminal_is_empty(&self) -> bool { self.cursor.cleanup_complete() }

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
        Ok(MeshPublication { cursor, owner: std::ptr::from_ref(&self.native) as usize, revision: self.native.shape_mutation_revision,
            key: None, published: false })
    }

    pub fn mesh_preparation_demand(&self, plan: &MeshPublication) -> Result<RetirementDemand, ValueError> {
        plan.check(self, true)?;
        let b = &self.native.body;
        let depth = match plan.cursor.original_import_root() {
            Some(EntityRef::Solid(id)) => b.solids.next_remove_depth_demand(id)?,
            Some(EntityRef::Shell(id)) => b.shells.next_remove_depth_demand(id)?,
            _ => return plan.cursor.original_import_demand(b),
        };
        Ok(RetirementDemand { depth, copy_bytes: 64, ..Default::default() })
    }

    pub fn step_mesh_preparation(&mut self, plan: &mut MeshPublication, effect: &mut RetainedAllocationEffect<'_>) -> Result<Progress, ValueError> {
        let quote = self.mesh_preparation_demand(plan)?;
        let Some(root) = plan.cursor.original_import_root() else { return plan.cursor.step_original_import(&mut self.native.body, effect); };
        if effect.grant().maximum_items == 0 || !retained_clone_grant_admits(effect.grant(), quote) || !effect.performed(1)? {
            return Ok(Progress::default());
        }
        let b = &self.native.body;
        plan.key = match root {
            EntityRef::Solid(id) => b.solids.get(id).map(|solid| primitive_handle(b"Solid:", solid.label.0)),
            EntityRef::Shell(id) => b.shells.get(id).map(|shell| primitive_handle(b"Shell:", shell.label.0)),
            _ => None,
        };
        if plan.key.is_none() { return Err(ValueError::invariant("original mesh root disappeared")); }
        Ok(Progress { copied_items: 1, copied_bytes: quote.copy_bytes, ..Default::default() })
    }

    pub fn mesh_publication_demand(&self, plan: &MeshPublication, copy: usize) -> Result<RetirementDemand, ValueError> {
        plan.check(self, true)?;
        let key = plan.handle().filter(|key| !plan.published && !self.native.live.contains_key(*key))
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
        let entity = match plan.cursor.original_import_root() {
            Some(EntityRef::Solid(id)) => Entity::Solid(id, Some(SolidProvenance::PlanarBoundary(None))),
            Some(EntityRef::Shell(id)) => Entity::Shell(id),
            _ => unreachable!("prepared mesh publication has a native root"),
        };
        let (prior, receipt) = self.native.live.insert_reserved(key, entity, grant).map_err(|(error, key, _)| (error, key))?;
        assert!(prior.is_none());
        plan.cursor.commit_original_import();
        plan.published = true;
        self.native.unpublished_topology -= 1;
        self.native.advance_live_revision();
        Ok(receipt)
    }

    pub fn mesh_publication_close_demand(&self, plan: &MeshPublication, copy: usize) -> Result<RetirementDemand, ValueError> {
        plan.check(self, false)?;
        let b = &self.native.body;
        Ok(RetirementDemand { copy_bytes: plan.cursor.next_close_copy_byte_demand(b)?,
            capacity_bytes: plan.cursor.next_close_capacity_byte_demand(b, copy)?, release_bytes: plan.cursor.next_close_release_byte_demand(b)?,
            depth: plan.cursor.next_close_depth_demand(b)? })
    }

    pub fn close_mesh_publication(&mut self, plan: &mut MeshPublication, effect: &mut RetainedAllocationEffect<'_>) -> Result<Step, ValueError> {
        plan.check(self, false)?;
        if plan.terminal_is_empty() { return Ok(Step::Complete(Progress::default())); }
        if !plan.published { plan.cursor.cancel(); }
        let outcome = plan.cursor.close_original_step(&mut self.native.body, effect);
        if !plan.published && plan.terminal_is_empty() { self.native.unpublished_topology -= 1; }
        outcome.result
    }
}
