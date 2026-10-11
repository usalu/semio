//! 📬️ Paged structural edit for one mesh vertex move.

use super::*;
use crate::{
    standards::v1::subsets::{base::schema::geometry::SemioPoint3, mesh::schema::mutations::move_vertex},
};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_value::{ValueError, ValueRefusalKind};
use semio_s_artifact_stdio_contract::editing::NativeEditPreparationRoute;
use semio_framework_plugin::plugin_app_close_prelude::store::{self as app_store, PagedOneItemEdit, PagedOneItemEditStep};
use std::sync::Arc;
use std::mem::size_of;

const PREFIX: &str = "stdio-semio-mesh-set-vertex";

pub(super) fn route(_prefix: &'static str) -> Option<NativeEditPreparationRoute<SemioMeshSnapshot, SemioMeshMutation>> {
    Some(NativeEditPreparationRoute::new(MeshVertexEdit::recognizes, Arc::new(app_store::PagedOneItemPreparationFactory::<SemioMeshSnapshot, SemioMeshMutation, MeshVertexEdit>::default())))
}

#[derive(Default)]
pub(super) struct MeshVertexEdit {
    mesh_index: usize,
    primitive_index: usize,
    mesh_matches: usize,
    primitive_matches: usize,
    located: Option<(usize, usize)>,
    scanned: bool,
}

impl MeshVertexEdit {
    fn target(mutation: &SemioMeshMutation) -> Result<&move_vertex::MoveVertex, ValueError> {
        let SemioMeshMutation::MoveVertex(value) = mutation else { return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "stdio-semio-mesh-set-vertex-mutation")) };
        Ok(value)
    }

    fn scan(&mut self, post: &SemioMeshSnapshot, target: &move_vertex::MoveVertex, grant: RetainedCloneGrant) -> Result<PagedOneItemEditStep<SemioMeshMutation>, ValueError> {
        let unit = |bytes: usize| RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() };
        let Some(mesh) = post.meshes.get(self.mesh_index) else {
            if self.mesh_matches != 1 || self.primitive_matches != 1 {
                return Err(ValueError::new(ValueRefusalKind::InvalidValue, format!("{PREFIX}-target-count-{}-{}", self.mesh_matches, self.primitive_matches)));
            }
            self.scanned = true;
            return Ok(PagedOneItemEditStep::Progress(unit(0)));
        };
        if mesh.id != target.mesh_id {
            let bytes = mesh.id.len();
            if grant.maximum_copy_bytes < bytes {
                return Ok(PagedOneItemEditStep::Progress(RetainedCloneProgress::default()));
            }
            self.mesh_index += 1;
            return Ok(PagedOneItemEditStep::Progress(unit(bytes)));
        }
        let Some(primitive) = mesh.primitives.get(self.primitive_index) else {
            self.mesh_matches += 1;
            self.mesh_index += 1;
            self.primitive_index = 0;
            return Ok(PagedOneItemEditStep::Progress(unit(0)));
        };
        let bytes = primitive.id.len();
        if grant.maximum_copy_bytes < bytes {
            return Ok(PagedOneItemEditStep::Progress(RetainedCloneProgress::default()));
        }
        if primitive.id == target.primitive_id {
            self.primitive_matches += 1;
            self.located = Some((self.mesh_index, self.primitive_index));
        }
        self.primitive_index += 1;
        Ok(PagedOneItemEditStep::Progress(unit(bytes)))
    }

    fn apply(&mut self, post: &mut SemioMeshSnapshot, target: &move_vertex::MoveVertex, grant: RetainedCloneGrant) -> Result<PagedOneItemEditStep<SemioMeshMutation>, ValueError> {
        let copied = size_of::<SemioPoint3>();
        let retained = target.mesh_id.len().checked_add(target.primitive_id.len()).ok_or_else(|| ValueError::literal(ValueRefusalKind::OwnershipLimit, "stdio-semio-mesh-set-vertex-inverse-overflow"))?;
        if grant.maximum_copy_bytes < copied || grant.maximum_capacity_bytes < retained {
            return Ok(PagedOneItemEditStep::Progress(RetainedCloneProgress::default()));
        }
        let (mesh_index, primitive_index) = self.located.ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "stdio-semio-mesh-set-vertex-located"))?;
        let slot = post
            .meshes
            .get_mut(mesh_index)
            .and_then(|mesh| mesh.primitives.get_mut(primitive_index))
            .and_then(|primitive| primitive.positions.get_mut(target.vertex_index))
            .ok_or_else(|| ValueError::literal(ValueRefusalKind::InvalidValue, "stdio-semio-mesh-set-vertex-vertex-index"))?;
        let old_point = std::mem::replace(slot, target.new_point);
        let inverse = SemioMeshMutation::MoveVertex(move_vertex::MoveVertex { mesh_id: target.mesh_id.clone(), primitive_id: target.primitive_id.clone(), vertex_index: target.vertex_index, new_point: old_point });
        Ok(PagedOneItemEditStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: copied, retained_capacity_bytes: retained, released_bytes: 0 }, inverse))
    }
}

impl PagedOneItemEdit<SemioMeshSnapshot, SemioMeshMutation> for MeshVertexEdit {
    const PREFIX: &'static str = PREFIX;

    fn recognizes(mutation: &SemioMeshMutation) -> bool {
        matches!(mutation, SemioMeshMutation::MoveVertex(_))
    }

    fn preflight(mutation: &SemioMeshMutation) -> Result<usize, String> {
        let value = Self::target(mutation).map_err(|_| format!("{PREFIX}-mutation"))?;
        value.mesh_id.len().checked_add(value.primitive_id.len()).and_then(|bytes| bytes.checked_add(size_of::<usize>() + size_of::<SemioPoint3>())).ok_or_else(|| format!("{PREFIX}-payload-overflow"))
    }

    fn advance(&mut self, post: &mut SemioMeshSnapshot, mutation: &SemioMeshMutation, grant: RetainedCloneGrant) -> Result<PagedOneItemEditStep<SemioMeshMutation>, ValueError> {
        let target = Self::target(mutation)?;
        if self.scanned { self.apply(post, target, grant) } else { self.scan(post, target, grant) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::standards::v1::subsets::mesh::schema::snapshot::{SemioMaterial, SemioMesh, SemioPrimitive, SemioTexture};

    fn fixture() -> serde_json::Value {
        serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../📇️registry/🧬️contract/✏️editing/🧫️fixtures/🧵️retained-native/🔣️.json"))).expect("retained native fixture")
    }

    fn snapshot() -> SemioMeshSnapshot {
        let vertex_count = fixture()["structuralCopy"]["meshVertexCount"].as_u64().expect("mesh vertex count") as usize;
        SemioMeshSnapshot {
            schema: SEMIO_MESH_DOCUMENT_SCHEMA.into(),
            meshes: vec![
                SemioMesh { id: "other".into(), primitives: vec![SemioPrimitive { id: "primitive".into(), positions: vec![SemioPoint3 { x: 9.0, y: 9.0, z: 9.0 }], ..Default::default() }] },
                SemioMesh { id: "mesh".into(), primitives: vec![SemioPrimitive { id: "primitive".into(), positions: (0..vertex_count).map(|index| SemioPoint3 { x: index as f64, y: 1.0, z: 2.0 }).collect(), ..Default::default() }] },
            ],
            materials: vec![SemioMaterial { id: "material".into(), ..Default::default() }],
            textures: vec![SemioTexture { id: "texture".into(), mime: "application/octet-stream".into(), bytes: vec![7; 64] }],
        }
    }

    fn mutation(snapshot: &SemioMeshSnapshot, mesh_id: &str) -> SemioMeshMutation {
        SemioMeshMutation::MoveVertex(move_vertex::MoveVertex { mesh_id: mesh_id.into(), primitive_id: "primitive".into(), vertex_index: snapshot.meshes[1].primitives[0].positions.len() - 1, new_point: SemioPoint3 { x: -1.0, y: -2.0, z: -3.0 } })
    }

    fn grant() -> RetainedCloneGrant {
        RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 4_096, maximum_capacity_bytes: 4_096, maximum_release_bytes: 4_096, maximum_depth: 8 }
    }

    #[test]
    fn edit_scans_one_unit_per_turn_and_moves_exactly_the_addressed_vertex() {
        let source = snapshot();
        let mutation = mutation(&source, "mesh");
        let mut post = source.clone();
        let mut edit = MeshVertexEdit::default();
        let mut turns = 0;
        let (progress, inverse) = loop {
            match edit.advance(&mut post, &mutation, grant()).expect("mesh edit turn") {
                PagedOneItemEditStep::Progress(progress) => assert!(progress.fits(grant()) && progress.copied_items <= 1),
                PagedOneItemEditStep::Complete(progress, inverse) => break (progress, inverse),
            }
            turns += 1;
            assert!(turns < 10_000);
        };
        assert!(progress.fits(grant()));
        assert_eq!(post.textures, source.textures);
        assert_eq!(post.materials, source.materials);
        assert_eq!(post.meshes[0], source.meshes[0]);
        assert_eq!(post.meshes[1].primitives[0].positions.last(), Some(&SemioPoint3 { x: -1.0, y: -2.0, z: -3.0 }));
        assert_eq!(post.meshes[1].primitives[0].positions.len(), source.meshes[1].primitives[0].positions.len());
        let SemioMeshMutation::MoveVertex(inverse) = inverse else { panic!("mesh inverse") };
        assert_eq!(inverse.new_point, *source.meshes[1].primitives[0].positions.last().expect("source target"));
    }

    #[test]
    fn edit_refuses_an_absent_target_before_touching_any_vertex() {
        let source = snapshot();
        let mutation = mutation(&source, "absent");
        let mut post = source.clone();
        let mut edit = MeshVertexEdit::default();
        let error = loop {
            match edit.advance(&mut post, &mutation, grant()) {
                Ok(PagedOneItemEditStep::Progress(_)) => {}
                Ok(PagedOneItemEditStep::Complete(..)) => panic!("absent mesh must not complete"),
                Err(error) => break error,
            }
        };
        assert!(format!("{error:?}").contains("target-count-0-0"));
        assert_eq!(post, source);
    }

    #[test]
    fn edit_waits_without_progress_when_the_grant_cannot_cover_one_turn() {
        let source = snapshot();
        let mutation = mutation(&source, "mesh");
        let mut post = source.clone();
        let mut edit = MeshVertexEdit::default();
        let starved = RetainedCloneGrant { maximum_copy_bytes: 0, ..grant() };
        match edit.advance(&mut post, &mutation, starved).expect("starved mesh edit turn") {
            PagedOneItemEditStep::Progress(progress) => assert_eq!(progress, RetainedCloneProgress::default()),
            PagedOneItemEditStep::Complete(..) => panic!("starved grant must not complete"),
        }
        assert_eq!(post, source);
    }
}
