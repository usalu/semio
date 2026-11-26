//! 📥️ Private draft custody for a retained mesh import and its publication phase.

use super::MeshImportCursor;
use semio_framework_3d_brep_representation::topology::{Body, EntityRef};
use semio_framework_value::{RetainedAllocationEffect, RetainedCloneProgress as Progress, RetainedCloneStep as Step, RetirementDemand, ValueError};
use semio_framework_value::retained_clone::retained_clone_grant_admits;

/// 🧾️ Owns the topology draft and key until the registry commits the root.
pub struct MeshImportDraft {
    cursor: MeshImportCursor,
    key: Option<[u8; 64]>,
    published: bool,
}

impl MeshImportDraft {
    pub fn new(cursor: MeshImportCursor) -> Self {
        Self { cursor, key: None, published: false }
    }

    pub fn handle(&self) -> Option<&str> { self.key.as_ref().map(|key| std::str::from_utf8(key).expect("mesh digest is ASCII")) }
    pub fn kind(&self) -> &'static str { if matches!(self.root(), Some(EntityRef::Solid(_))) { "solid" } else { "shell" } }
    pub fn terminal_is_empty(&self) -> bool { self.cursor.cleanup_complete() }
    pub fn published(&self) -> bool { self.published }
    pub fn root(&self) -> Option<EntityRef> { self.cursor.original_import_root() }

    pub fn preparation_demand(&self, body: &Body) -> Result<RetirementDemand, ValueError> {
        let depth = match self.root() {
            Some(EntityRef::Solid(id)) => body.solids.next_remove_depth_demand(id)?,
            Some(EntityRef::Shell(id)) => body.shells.next_remove_depth_demand(id)?,
            _ => return self.cursor.original_import_demand(body),
        };
        Ok(RetirementDemand { depth, copy_bytes: 64, ..Default::default() })
    }

    pub fn step_preparation(&mut self, body: &mut Body, key_for_label: fn(&[u8], u64) -> [u8; 64], effect: &mut RetainedAllocationEffect<'_>) -> Result<Progress, ValueError> {
        let quote = self.preparation_demand(body)?;
        let Some(root) = self.root() else { return self.cursor.step_original_import(body, effect); };
        if effect.grant().maximum_items == 0 || !retained_clone_grant_admits(effect.grant(), quote) || !effect.performed(1)? {
            return Ok(Progress::default());
        }
        self.key = match root {
            EntityRef::Solid(id) => body.solids.get(id).map(|solid| key_for_label(b"Solid:", solid.label.0)),
            EntityRef::Shell(id) => body.shells.get(id).map(|shell| key_for_label(b"Shell:", shell.label.0)),
            _ => None,
        };
        if self.key.is_none() { return Err(ValueError::invariant("original mesh root disappeared")); }
        Ok(Progress { copied_items: 1, copied_bytes: quote.copy_bytes, ..Default::default() })
    }

    pub fn commit(&mut self) {
        self.cursor.commit_original_import();
        self.published = true;
    }

    pub fn close_demand(&self, body: &Body, copy: usize) -> Result<RetirementDemand, ValueError> {
        Ok(RetirementDemand { copy_bytes: self.cursor.next_close_copy_byte_demand(body)?,
            capacity_bytes: self.cursor.next_close_capacity_byte_demand(body, copy)?, release_bytes: self.cursor.next_close_release_byte_demand(body)?,
            depth: self.cursor.next_close_depth_demand(body)? })
    }

    pub fn close(&mut self, body: &mut Body, effect: &mut RetainedAllocationEffect<'_>) -> Result<Step, ValueError> {
        if !self.published { self.cursor.cancel(); }
        self.cursor.close_original_step(body, effect).result
    }
}
