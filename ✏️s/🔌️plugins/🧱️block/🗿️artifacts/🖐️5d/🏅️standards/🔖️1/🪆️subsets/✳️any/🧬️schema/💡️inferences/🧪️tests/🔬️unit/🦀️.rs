
use super::*;
use crate::BlockKindIdentity;
use crate::{BLOCK_5D_SCHEMA, Block5dGripTemplate};
use protocol::Inference;

//#region 🧸️Fixtures
fn grip(id: &str, position: [f64; 3], radius_3d: f64) -> Block5dGripTemplate {
    Block5dGripTemplate { id: id.into(), grip_kind: "rope".into(), angle: 0.0, radius_2d: 0.0, position, direction: [0.0, 1.0, 0.0], radius_3d }
}

fn snapshot_with_grips(grips: Vec<Block5dGripTemplate>) -> Block5dSnapshot {
    Block5dSnapshot { part_kind: BlockKindIdentity { id: "capsule".into(), name: "capsule".into(), label: "Capsule".into(), ..Default::default() }, grips, ..Block5dSnapshot::default() }
}
//#endregion 🧸️Fixtures

//#region 🧪️InferenceLaws
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = snapshot_with_grips(vec![grip("g0", [1.0, 2.0, 3.0], 0.5), grip("g1", [-1.0, 0.0, 4.0], 0.25)]);
    assert_eq!(Block5dInference::infer(&snapshot), Block5dInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(Block5dInference::infer(&Block5dSnapshot::default()), Block5dInference::default());
}

#[semio_framework_async_macros::async_test]
async fn bounds_match_grip_positions_inflated_by_radius_3d() {
    let snapshot = snapshot_with_grips(vec![grip("g0", [1.0, 2.0, 3.0], 0.5), grip("g1", [-1.0, 0.0, 4.0], 0.25)]);
    let inferred = Block5dInference::infer(&snapshot);
    let bounds = inferred.bounds.bounding_box.expect("non-empty grips produce a bounding box");
    assert_eq!(bounds.min, [-1.25, -0.25, 2.5]);
    assert_eq!(bounds.max, [1.5, 2.5, 4.25]);
    assert_eq!(inferred.bounds.vertex_count, 2);
}
//#endregion 🧪️InferenceLaws

//#region 🧪️PuzzleCatalogFragment
#[semio_framework_async_macros::async_test]
async fn puzzle5d_catalog_fragment_maps_grips() {
    let mut definition = Block5dSnapshot { schema: BLOCK_5D_SCHEMA.into(), part_kind: BlockKindIdentity { id: "left".into(), name: "left".into(), label: "Left".into(), ..Default::default() }, ..Block5dSnapshot::default() };
    definition.grips.push(Block5dGripTemplate { id: "g0".into(), grip_kind: "b-l".into(), angle: -1.57, radius_2d: 0.36, position: [4.05, 4.68, 3.0], direction: [0.0, 1.0, 0.0], radius_3d: 0.36 });
    let fragment = puzzle5d_catalog_fragment(&definition);
    assert_eq!(fragment["parts"][0]["id"], "left");
    assert_eq!(fragment["parts"][0]["grips"][0]["gripKind"], "b-l");
}
//#endregion 🧪️PuzzleCatalogFragment
