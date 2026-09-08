
use super::*;
use crate::standards::v1::subsets::base::schema::geometry::SemioTransform;
use crate::standards::v1::subsets::kit::schema::snapshot::{STDIO_SEMIOKIT_DOCUMENT_SCHEMA, SemioKitConnection, SemioKitDesign, SemioKitPiece, SemioKitType};

/// 🌱 A hand-built, non-empty catalog: 2 types, 2 designs (one with 2 pieces + 1 connection,
/// one empty), no children/representations — exercises the real fold without depending on the
/// composite subset's own child-handle demo fixture.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn populated() -> SemioKitSnapshot {
    SemioKitSnapshot {
        schema: STDIO_SEMIOKIT_DOCUMENT_SCHEMA.into(),
        types: vec![SemioKitType { id: "chair".into(), name: "Chair".into(), category: "furniture".into() }, SemioKitType { id: "table".into(), name: "Table".into(), category: "furniture".into() }],
        designs: vec![
            SemioKitDesign {
                id: "living-room".into(),
                name: "Living Room".into(),
                pieces: vec![SemioKitPiece { id: "p1".into(), type_id: "chair".into(), transform: SemioTransform::identity() }, SemioKitPiece { id: "p2".into(), type_id: "chair".into(), transform: SemioTransform::identity() }],
                connections: vec![SemioKitConnection { id: "c1".into(), connecting_piece_id: "p1".into(), connecting_port: "left".into(), connected_piece_id: "p2".into(), connected_port: "right".into() }],
            },
            SemioKitDesign { id: "empty-room".into(), name: "Empty Room".into(), pieces: Vec::new(), connections: Vec::new() },
        ],
        objects: Vec::new(),
        models: Vec::new(),
        properties: None,
        representations: Vec::new(),
    }
}

#[semio_framework_async_macros::async_test]
async fn folds_pieces_and_connections_across_every_design() {
    let entries = compute_semio_kit_entries(&populated());
    assert_eq!(entries.type_count, 2);
    assert_eq!(entries.design_count, 2);
    assert_eq!(entries.piece_count, 2, "sum of pieces across both designs (2 + 0)");
    assert_eq!(entries.connection_count, 1, "sum of connections across both designs (1 + 0)");
    assert_eq!(entries.object_count, 0);
    assert_eq!(entries.model_count, 0);
    assert!(!entries.has_properties);
    assert_eq!(entries.representation_count, 0);
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = populated();
    assert_eq!(compute_semio_kit_entries(&snapshot), compute_semio_kit_entries(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(compute_semio_kit_entries(&SemioKitSnapshot::default()), SemioKitEntries::default());
}
