//! 💡️ Block3d inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `📦bounds/`).
//!
//! Unlike puzzle3d, block3d has no parent/child object graph — it is a single kind DEFINITION (one
//! `ObjectKind` plus a catalog of rim `Block3dVortexTemplate`s), so the honest whole-snapshot
//! inference here is a geometric bounding box + vertex count over the vortex templates' rim
//! positions, expressed as a plain `Inference` impl (no per-entity `InferredField` caching needed —
//! there is nothing to invalidate incrementally over a flat template list).

use crate::artifacts::block3d::Block3dSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use super::bounds::{compute_block3d_bounds, Block3dBounds};

//#region 🔖️Inference
/// 💡️ Everything inferable from a block3d snapshot. One field per named inference under
/// `💡️inferences/` (currently: `bounds`, backed by the `📦bounds/` slug dir).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.block.block3d.inference")]
pub struct Block3dInference {
    #[state(inferred)]
    pub bounds: Block3dBounds,
}

impl protocol::Inference<Block3dSnapshot> for Block3dInference {
    fn infer(snapshot: &Block3dSnapshot) -> Self {
        Self { bounds: compute_block3d_bounds(snapshot) }
    }
}

impl protocol::InferenceSpec<Block3dSnapshot> for Block3dInference {
    fn inference_schema_id() -> &'static str {
        "s.block.block3d.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.block.block3d.inference.bounds", reads: &["vortices"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::block3d::standards::v1::subsets::any::schema::Block3dBuilder {
    type Snapshot = Block3dSnapshot;
    type Inference = Block3dInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️PuzzleCatalogFragment
/// 🌐️ Resolves the active representation's mesh url — the first representation whose `tags` all
/// appear in `wanted_tags`, or the first representation overall, or `None` for an empty catalog.
pub fn resolve_active_mesh_url<'a>(definition: &'a Block3dSnapshot, wanted_tags: &[&str]) -> Option<&'a str> {
    definition
        .representations
        .iter()
        .find(|representation| wanted_tags.iter().all(|tag| representation.tags.iter().any(|other| other == tag)))
        .or_else(|| definition.representations.first())
        .and_then(|representation| representation.mesh_url.as_deref())
}

/// 🌉️ Maps this `ObjectKind` definition into the `s/plugin/puzzle` 3d catalog shape (`objectKinds`/
/// `vortexKinds`/`cableKinds`/`attractionKinds` — see `Puzzle3dKindCatalogs`), the seam puzzle imports
/// through its `Kit×Type` media port. The active representation's mesh (first row, or the first
/// matching `wanted_tags`) becomes the catalog row's `meshUrl`.
pub fn puzzle3d_catalog_fragment(definition: &Block3dSnapshot, wanted_tags: &[&str]) -> Value {
    let vortices: Vec<Value> = definition.vortices.iter().map(|vortex| json!({ "id": vortex.id, "vortexKind": vortex.vortex_kind, "position": vortex.position, "direction": vortex.direction, "radius": vortex.radius })).collect();
    let object_kind = json!({
        "id": definition.object_kind.id,
        "name": definition.object_kind.name,
        "label": definition.object_kind.label,
        "meshUrl": resolve_active_mesh_url(definition, wanted_tags),
        "vortices": vortices,
    });
    let vortex_kinds: Vec<Value> = definition.vortex_kinds.iter().map(|kind| json!({ "id": kind.id, "name": kind.name, "label": kind.label, "color": kind.color, "defaultCableKind": kind.default_cable_kind })).collect();
    let kind_compatibility: Vec<Value> = definition.compatibility.iter().map(|rule| json!({ "source": rule.source, "target": rule.target, "bidirectional": rule.bidirectional })).collect();
    json!({
        "schema": "manifest",
        "objectKinds": [object_kind],
        "vortexKinds": vortex_kinds,
        "cableKinds": Vec::<Value>::new(),
        "attractionKinds": Vec::<Value>::new(),
        "kindCompatibility": kind_compatibility,
    })
}
//#endregion 🔖️PuzzleCatalogFragment

//#region 🔖️Descriptor
/// 💡️ Registers `s.block.block3d.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `block3d_artifact_schema_descriptor`'s registration.
pub fn block3d_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.block.block3d.inference",
        inference: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::block3d::{Block3dVortexTemplate, BLOCK_3D_SCHEMA};
    use crate::{BlockKindIdentity, BlockRepresentation};
    use protocol::Inference;

    //#region 🧸️Fixtures
    fn vortex(id: &str, position: [f64; 3], radius: f64) -> Block3dVortexTemplate {
        Block3dVortexTemplate { id: id.into(), vortex_kind: "door".into(), position, direction: [0.0, 1.0, 0.0], radius, label: None }
    }

    fn snapshot_with_vortices(vortices: Vec<Block3dVortexTemplate>) -> Block3dSnapshot {
        Block3dSnapshot {
            object_kind: BlockKindIdentity { id: "capsule".into(), name: "capsule".into(), label: "Capsule".into(), ..Default::default() },
            vortices,
            ..Block3dSnapshot::default()
        }
    }
    //#endregion 🧸️Fixtures

    //#region 🧪️InferenceLaws
    #[test]
    fn inference_determinism_law() {
        let snapshot = snapshot_with_vortices(vec![vortex("v0", [1.0, 2.0, 3.0], 0.5), vortex("v1", [-1.0, 0.0, 4.0], 0.25)]);
        assert_eq!(Block3dInference::infer(&snapshot), Block3dInference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(Block3dInference::infer(&Block3dSnapshot::default()), Block3dInference::default());
    }

    #[test]
    fn bounds_match_vortex_positions_inflated_by_radius() {
        let snapshot = snapshot_with_vortices(vec![vortex("v0", [1.0, 2.0, 3.0], 0.5), vortex("v1", [-1.0, 0.0, 4.0], 0.25)]);
        let inferred = Block3dInference::infer(&snapshot);
        let bounds = inferred.bounds.bounding_box.expect("non-empty vortices produce a bounding box");
        assert_eq!(bounds.min, [-1.25, -0.5, 2.5]);
        assert_eq!(bounds.max, [1.5, 2.5, 4.25]);
        assert_eq!(inferred.bounds.vertex_count, 2);
    }
    //#endregion 🧪️InferenceLaws

    //#region 🧪️PuzzleCatalogFragment
    #[test]
    fn resolve_active_mesh_url_prefers_matching_tags() {
        let mut definition = Block3dSnapshot::default();
        definition.representations.push(BlockRepresentation { id: "r0".into(), name: "1:500".into(), mesh_url: Some("/mesh/low.glb".into()), tags: vec!["1to500".into()], lod: None, description: String::new(), attributes: Vec::new() });
        definition.representations.push(BlockRepresentation { id: "r1".into(), name: "full".into(), mesh_url: Some("/mesh/full.glb".into()), tags: vec!["full".into()], lod: None, description: String::new(), attributes: Vec::new() });
        assert_eq!(resolve_active_mesh_url(&definition, &["full"]), Some("/mesh/full.glb"));
        assert_eq!(resolve_active_mesh_url(&definition, &["missing"]), Some("/mesh/low.glb"));
    }

    #[test]
    fn puzzle3d_catalog_fragment_maps_vortices() {
        let mut definition = Block3dSnapshot { schema: BLOCK_3D_SCHEMA.into(), object_kind: BlockKindIdentity { id: "capsule".into(), name: "capsule".into(), label: "Capsule".into(), ..Default::default() }, ..Block3dSnapshot::default() };
        definition.vortices.push(Block3dVortexTemplate { id: "v0".into(), vortex_kind: "door".into(), position: [0.0, 0.0, 0.0], direction: [0.0, 1.0, 0.0], radius: 0.3, label: None });
        let fragment = puzzle3d_catalog_fragment(&definition, &[]);
        assert_eq!(fragment["objectKinds"][0]["id"], "capsule");
        assert_eq!(fragment["objectKinds"][0]["vortices"][0]["vortexKind"], "door");
    }
    //#endregion 🧪️PuzzleCatalogFragment
}
//#endregion 🧪️Tests
