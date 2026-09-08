
use super::*;
use crate::{BLOCK_3D_SCHEMA, Block3dVortexTemplate};
use crate::{BlockKindIdentity, BlockRepresentation};
use protocol::Inference;

//#region 🧸️Fixtures
fn vortex(id: &str, position: [f64; 3], radius: f64) -> Block3dVortexTemplate {
    Block3dVortexTemplate { id: id.into(), vortex_kind: "door".into(), position, direction: [0.0, 1.0, 0.0], radius, label: None }
}

fn snapshot_with_vortices(vortices: Vec<Block3dVortexTemplate>) -> Block3dSnapshot {
    Block3dSnapshot { object_kind: BlockKindIdentity { id: "capsule".into(), name: "capsule".into(), label: "Capsule".into(), ..Default::default() }, vortices, ..Block3dSnapshot::default() }
}
//#endregion 🧸️Fixtures

//#region 🧪️InferenceLaws
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = snapshot_with_vortices(vec![vortex("v0", [1.0, 2.0, 3.0], 0.5), vortex("v1", [-1.0, 0.0, 4.0], 0.25)]);
    assert_eq!(Block3dInference::infer(&snapshot), Block3dInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(Block3dInference::infer(&Block3dSnapshot::default()), Block3dInference::default());
}

#[semio_framework_async_macros::async_test]
async fn bounds_match_vortex_positions_inflated_by_radius() {
    let snapshot = snapshot_with_vortices(vec![vortex("v0", [1.0, 2.0, 3.0], 0.5), vortex("v1", [-1.0, 0.0, 4.0], 0.25)]);
    let inferred = Block3dInference::infer(&snapshot);
    let bounds = inferred.bounds.bounding_box.expect("non-empty vortices produce a bounding box");
    assert_eq!(bounds.min, [-1.25, -0.25, 2.5]);
    assert_eq!(bounds.max, [1.5, 2.5, 4.25]);
    assert_eq!(inferred.bounds.vertex_count, 2);
}
//#endregion 🧪️InferenceLaws

//#region 🧪️PuzzleCatalogFragment
#[semio_framework_async_macros::async_test]
async fn resolve_active_mesh_url_prefers_matching_tags() {
    let mut definition = Block3dSnapshot::default();
    definition.representations.push(BlockRepresentation { id: "r0".into(), name: "1:500".into(), mesh_url: Some("/mesh/low.glb".into()), tags: vec!["1to500".into()], lod: None, description: String::new(), attributes: Vec::new() });
    definition.representations.push(BlockRepresentation { id: "r1".into(), name: "full".into(), mesh_url: Some("/mesh/full.glb".into()), tags: vec!["full".into()], lod: None, description: String::new(), attributes: Vec::new() });
    assert_eq!(resolve_active_mesh_url(&definition, &["full"]), Some("/mesh/full.glb"));
    assert_eq!(resolve_active_mesh_url(&definition, &["missing"]), Some("/mesh/low.glb"));
}

#[semio_framework_async_macros::async_test]
async fn puzzle3d_catalog_fragment_maps_vortices() {
    let mut definition = Block3dSnapshot { schema: BLOCK_3D_SCHEMA.into(), object_kind: BlockKindIdentity { id: "capsule".into(), name: "capsule".into(), label: "Capsule".into(), ..Default::default() }, ..Block3dSnapshot::default() };
    definition.vortices.push(Block3dVortexTemplate { id: "v0".into(), vortex_kind: "door".into(), position: [0.0, 0.0, 0.0], direction: [0.0, 1.0, 0.0], radius: 0.3, label: None });
    let fragment = puzzle3d_catalog_fragment(&definition, &[]);
    assert_eq!(fragment["objectKinds"][0]["id"], "capsule");
    assert_eq!(fragment["objectKinds"][0]["vortices"][0]["vortexKind"], "door");
}
//#endregion 🧪️PuzzleCatalogFragment
