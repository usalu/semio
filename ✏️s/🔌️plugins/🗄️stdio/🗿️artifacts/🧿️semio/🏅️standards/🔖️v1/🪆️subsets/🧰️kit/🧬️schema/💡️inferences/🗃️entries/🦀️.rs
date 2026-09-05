//! 🗃 `entries` — one named inference: the kit catalog's own census. `objects`/`models`/
//! `properties` are owned CHILD slots (handles only — never embedded content, per this composite
//! subset's own module doc comment), `representations` is a LINK slot; none of the four are
//! honestly resolvable from THIS snapshot alone (resolving a handle is a cross-artifact read, out
//! of scope for a pure snapshot->inference fold), so the honest inference here is a real fold over
//! what IS owned outright: `types`/`designs` (including every design's nested `pieces`/
//! `connections`) plus a plain count/presence read of the four handle slots.

use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Entries
/// 🗃️ Semio kit catalog census.
#[derive(Clone, Copy, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SemioKitEntries {
    pub type_count: u32,
    pub design_count: u32,
    /// 🧩️ Total pieces across EVERY design (a real fold, not a length read of `designs` itself).
    pub piece_count: u32,
    /// 🔌️ Total connections across every design.
    pub connection_count: u32,
    pub object_count: u32,
    pub model_count: u32,
    pub has_properties: bool,
    pub representation_count: u32,
}

/// 🗃️ Computes [`SemioKitEntries`] — pure, total, O(types + designs + pieces + connections +
/// objects + models + representations).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn compute_semio_kit_entries(snapshot: &SemioKitSnapshot) -> SemioKitEntries {
    let piece_count = snapshot.designs.iter().map(|d| d.pieces.len() as u32).sum();
    let connection_count = snapshot.designs.iter().map(|d| d.connections.len() as u32).sum();
    SemioKitEntries {
        type_count: snapshot.types.len() as u32,
        design_count: snapshot.designs.len() as u32,
        piece_count,
        connection_count,
        object_count: snapshot.objects.len() as u32,
        model_count: snapshot.models.len() as u32,
        has_properties: snapshot.properties.is_some(),
        representation_count: snapshot.representations.len() as u32,
    }
}
//#endregion 🔖️Entries

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::base::schema::geometry::SemioTransform;
    use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::{SemioKitConnection, SemioKitDesign, SemioKitPiece, SemioKitType, STDIO_SEMIOKIT_DOCUMENT_SCHEMA};

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
}
//#endregion 🧪️Tests
