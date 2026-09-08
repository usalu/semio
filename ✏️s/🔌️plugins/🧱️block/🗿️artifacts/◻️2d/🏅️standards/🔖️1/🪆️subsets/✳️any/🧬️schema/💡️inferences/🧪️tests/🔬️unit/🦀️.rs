
use super::*;
use crate::BlockKindIdentity;
use crate::{BLOCK_2D_SCHEMA, Block2dHandleKind, Block2dHandleTemplate};
use protocol::Inference;
use std::f64::consts::FRAC_PI_2;

//#region 🧸️Fixtures
fn handle(id: &str, angle: f64, radius: f64) -> Block2dHandleTemplate {
    Block2dHandleTemplate { id: id.into(), handle_kind: "wire".into(), angle, radius }
}

fn snapshot_with_handles(handles: Vec<Block2dHandleTemplate>) -> Block2dSnapshot {
    Block2dSnapshot { node_kind: BlockKindIdentity { id: "square".into(), name: "square".into(), label: "Square".into(), ..Default::default() }, handles, ..Block2dSnapshot::default() }
}
//#endregion 🧸️Fixtures

//#region 🧪️InferenceLaws
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = snapshot_with_handles(vec![handle("h0", 0.0, 1.0), handle("h1", FRAC_PI_2, 2.0)]);
    assert_eq!(Block2dInference::infer(&snapshot), Block2dInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(Block2dInference::infer(&Block2dSnapshot::default()), Block2dInference::default());
}

#[semio_framework_async_macros::async_test]
async fn bounds_convert_polar_handles_to_cartesian() {
    let snapshot = snapshot_with_handles(vec![handle("h0", 0.0, 1.0), handle("h1", FRAC_PI_2, 2.0)]);
    let inferred = Block2dInference::infer(&snapshot);
    let bounds = inferred.bounds.bounding_box.expect("non-empty handles produce a bounding box");
    assert!((bounds.min[0] - 0.0).abs() < 1e-9, "min x should be 0.0 (h0 at angle 0)");
    assert!((bounds.max[0] - 1.0).abs() < 1e-9, "max x should be 1.0 (h0 at angle 0, radius 1)");
    assert!((bounds.min[1] - 0.0).abs() < 1e-9, "min y should be 0.0 (h0 at angle 0)");
    assert!((bounds.max[1] - 2.0).abs() < 1e-9, "max y should be 2.0 (h1 at angle pi/2, radius 2)");
    assert_eq!(inferred.bounds.vertex_count, 2);
}
//#endregion 🧪️InferenceLaws

//#region 🧪️PuzzleCatalogFragment
#[semio_framework_async_macros::async_test]
async fn puzzle2d_manifest_fragment_maps_kind_identity_and_handles() {
    let mut definition = Block2dSnapshot { schema: BLOCK_2D_SCHEMA.into(), node_kind: BlockKindIdentity { id: "left".into(), name: "left".into(), label: "Left".into(), ..Default::default() }, ..Block2dSnapshot::default() };
    definition.handle_kinds.push(Block2dHandleKind { id: "b-l".into(), name: "b-l".into(), label: "b-l".into(), color: "hsl(206 52% 48%)".into(), default_wire_kind: "cable.link".into() });
    definition.handles.push(Block2dHandleTemplate { id: "h0".into(), handle_kind: "b-l".into(), angle: -1.57, radius: 0.36 });
    let fragment = puzzle2d_manifest_fragment(&definition);
    assert_eq!(fragment["nodeKinds"][0]["id"], "left");
    assert_eq!(fragment["nodeKinds"][0]["presentation"]["handles"][0]["handleKind"], "b-l");
    assert_eq!(fragment["portKinds"][0]["id"], "b-l");
}
//#endregion 🧪️PuzzleCatalogFragment
