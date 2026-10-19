//! 🏷️ Validation gate and labels: the validation gate, the disposal, the persistent labels and handles, the shells, compounds and explosion and the edge enumeration. See [`super`].

use super::*;

// #region 🔖️SyncApi
impl Brep {
    /// 🩺️ The structured validation gate every preview tessellation runs first: `Ok(())` when THE
    /// SHAPE ASKED ABOUT carries no ERROR-class issue, `Err(issues)` otherwise. Advisory `warning-`
    /// codes never block. Unlike [`Brep::validate_sync`] this returns the typed issues instead of a
    /// JSON string, so a host can route them into a typed diagnostic rather than re-parsing prose.
    ///
    /// 🎯️ The verdict is SCOPED to the entities `shape` reaches. [`validate_body`] walks the whole
    /// arena, and this kernel is process-wide: one invalid solid anywhere — a boolean result whose
    /// void shell is not inverted, say — used to block the tessellation of every OTHER live handle
    /// for as long as it stayed live. In the procedural playground that read as six of eight
    /// examples settling on an empty preview payload reported as `idle`, with the geometry simply
    /// gone (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
    /// `📓️slider-reevaluation-correctness-2026-09-15.md`). An issue whose label names no entity this
    /// shape reaches is somebody else's problem; an issue whose label this rule cannot read at all
    /// still blocks, because an unreadable diagnostic is not evidence of health.
    pub fn validate_gate_sync(&self, shape: &GeometryHandle) -> Result<(), Vec<ValidationIssue>> {
        let Ok(entity) = self.entity(shape) else {
            return Err(vec![ValidationIssue { entity: shape.as_str().to_string(), code: "unknown-handle", message: format!("handle {} is not live in this kernel", shape.as_str()) }]);
        };
        if !matches!(entity, Entity::Solid(_) | Entity::Shell(_) | Entity::Compound(_, _)) {
            return Ok(());
        }
        let reach = self.body.reachable_from(&entity_roots(entity));
        let blocking: Vec<ValidationIssue> = validate_body(&self.body).into_iter().filter(|issue| !issue.code.starts_with("warning-") && !(matches!(entity,Entity::Shell(_)) && issue.code=="shell-not-closed")).filter(|issue| issue_reaches(&reach, &issue.entity)).collect();
        if blocking.is_empty() {
            Ok(())
        } else {
            Err(blocking)
        }
    }

    /// ♻️ Reclaims the handle and, if no other live handle still reaches the underlying entity,
    /// runs the arena GC (see [`Brep::compact_unreachable`]) — dispose is not merely a registry
    /// removal, it is the operation that actually frees the topology.
    pub fn dispose_sync(&mut self, shape: &GeometryHandle) -> usize {
        let removed = self.live.remove(shape.as_str()).is_some();
        if removed{self.advance_live_revision();}
        if removed {
            self.compact_unreachable();
        }
        usize::from(removed)
    }
    /// 🏷️ The [`PersistentLabel`] `handle` currently resolves to, or `None` for an unknown handle.
    pub fn label_of(&self, handle: &GeometryHandle) -> Option<PersistentLabel> {
        self.live.get(handle.as_str()).and_then(|e| label_of_entity(&self.body, e))
    }
    /// 🏷️ The handle for `label`, searching every arena store in turn — `None` if no live entity
    /// carries that label. Re-mints (deterministically, see [`Brep::mint`]) rather than requiring
    /// the caller to have kept the original handle string around.
    pub fn handle_for_label(&mut self, label: PersistentLabel) -> Option<GeometryHandle> {
        let vertex = self.body.vertices.iter().find(|(_, v)| v.label == label).map(|(id, _)| id);
        if let Some(id) = vertex {
            return Some(self.mint(GeometryKind::Vertex, Entity::Vertex(id)));
        }
        let edge = self.body.edges.iter().find(|(_, e)| e.label == label).map(|(id, _)| id);
        if let Some(id) = edge {
            return Some(self.mint(GeometryKind::Edge, Entity::Edge(id)));
        }
        let face = self.body.faces.iter().find(|(_, f)| f.label == label).map(|(id, _)| id);
        if let Some(id) = face {
            return Some(self.mint(GeometryKind::Face, Entity::Face(id)));
        }
        let shell = self.body.shells.iter().find(|(_, s)| s.label == label).map(|(id, _)| id);
        if let Some(id) = shell {
            return Some(self.mint(GeometryKind::Shell, Entity::Shell(id)));
        }
        let solid = self.body.solids.iter().find(|(_, s)| s.label == label).map(|(id, _)| id);
        if let Some(id) = solid {
            return Some(self.mint(GeometryKind::Solid, Entity::Solid(id)));
        }
        None
    }
    pub fn solid_shells_sync(&mut self, shape: &GeometryHandle) -> Result<Vec<GeometryHandle>, BrepError> {
        let solid = self.solid_id(shape)?;
        Ok(self.body.solid_shells(solid).into_iter().map(|s| self.register_shell(s)).collect())
    }
    pub fn compound_sync(&mut self, solids: &[GeometryHandle]) -> Result<GeometryHandle, BrepError> {
        let mut ids = Vec::with_capacity(solids.len());
        for s in solids {
            ids.push(self.solid_id(s)?);
        }
        Ok(self.register_compound(ids))
    }
    pub fn explode_sync(&mut self, compound: &GeometryHandle) -> Result<Vec<GeometryHandle>, BrepError> {
        match self.entity(compound)?.clone() {
            Entity::Compound(solids, _) => Ok(solids.into_iter().map(|s| self.register_solid(s)).collect()),
            _ => Err(BrepError::InvalidInput(format!("{} is not a compound", compound.as_str()))),
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn all_edges(body: &Body, solid: SolidId) -> Vec<EdgeId> {
    let mut set = std::collections::BTreeSet::new();
    for face in body.solid_faces(solid) {
        for cid in body.face_coedges(face) {
            if let Some(c) = body.coedges.get(cid) {
                set.insert(c.edge);
            }
        }
    }
    set.into_iter().collect()
}
// #endregion 🔖️SyncApi
