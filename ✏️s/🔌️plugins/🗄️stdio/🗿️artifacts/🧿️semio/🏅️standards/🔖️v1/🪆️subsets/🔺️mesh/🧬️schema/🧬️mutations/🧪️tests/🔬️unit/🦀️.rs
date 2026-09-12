use super::*;
use protocol::command::DiffAlgebra;
use protocol::{Mutation, MutationDiff, SemanticMutation};

/// 🔧️ All 4 collections are id-keyed SETS with no user-meaningful display order (this facet's
/// own `🔺️diff` module doc comment): `create-*`'s diff always APPENDS to `added`, so an
/// inverse that re-creates an entity deleted from the middle of a collection legitimately
/// lands it at the end UNLESS the position-preserving inverse (used by every `delete-*` triad
/// here) restores it. Both `delete-mesh`/`delete-material`/`delete-texture`/`delete-primitive`
/// ARE position-preserving, so plain round-trip equality (not merely set equality) is expected
/// and asserted directly.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn round_trip(base: &SemioMeshSnapshot, operation: &SemioMeshMutation) -> SemioMeshSnapshot {
    let forward = operation.diff(base).diff().apply(base).expect("apply must succeed for a well-formed fixture");
    let backwards = operation.inverse(base);
    let mut restored = forward.clone();
    for back in &backwards {
        restored = back.diff(&restored).diff().apply(&restored).expect("apply must succeed for a well-formed fixture");
    }
    assert_eq!(restored, base.clone(), "inverse must exactly restore the pre-operation fixture for {operation:?}");
    forward
}

//#region 🧪️InverseRoundTripLaw
#[semio_framework_async_macros::async_test]
async fn inverse_round_trip_law_covers_every_variant() {
    let base = fixture();
    for m in demo_mutation_cases() {
        let _ = round_trip(&base, &m);
    }
}

#[semio_framework_async_macros::async_test]
async fn create_delete_mesh_round_trips_explicitly() {
    let base = fixture();
    let create = SemioMeshMutation::CreateMesh(create_mesh::CreateMesh { mesh: SemioMesh { id: "mesh-c".into(), primitives: vec![] } });
    let after_create = round_trip(&base, &create);
    assert!(after_create.meshes.iter().any(|m| m.id == "mesh-c"));

    let undo = create.inverse(&base);
    assert_eq!(undo, vec![SemioMeshMutation::DeleteMesh(delete_mesh::DeleteMesh { id: "mesh-c".into() })]);

    let delete = SemioMeshMutation::DeleteMesh(delete_mesh::DeleteMesh { id: "mesh-a".into() });
    let after_delete = round_trip(&base, &delete);
    assert!(!after_delete.meshes.iter().any(|m| m.id == "mesh-a"));
}

#[semio_framework_async_macros::async_test]
async fn delete_of_an_absent_id_has_an_empty_inverse_and_is_a_diff_level_no_op() {
    let base = fixture();
    let delete = SemioMeshMutation::DeleteMaterial(delete_material::DeleteMaterial { id: "mat-missing".into() });
    assert!(delete.inverse(&base).is_empty(), "deleting an absent id has nothing to undo");
    assert!(delete.diff(&base).diff().is_empty(), "deleting an absent id must diff empty, not merely be harmless to apply");
}

#[semio_framework_async_macros::async_test]
async fn set_change_replace_move_of_an_absent_target_have_empty_inverse_and_are_no_ops() {
    let base = fixture();

    let topo = SemioMeshMutation::SetPrimitiveTopology(set_primitive_topology::SetPrimitiveTopology { mesh_id: "mesh-missing".into(), primitive_id: "prim-missing".into(), topology: SemioTopology::Lines });
    assert!(topo.inverse(&base).is_empty());
    assert_eq!(topo.diff(&base).diff().apply(&base).expect("apply must succeed for a well-formed fixture"), base, "set-primitive-topology on an absent target is a no-op");

    let geom = SemioMeshMutation::ReplacePrimitiveGeometry(replace_primitive_geometry::ReplacePrimitiveGeometry {
        mesh_id: "mesh-missing".into(),
        primitive_id: "prim-missing".into(),
        positions: vec![],
        normals: vec![],
        uvs: vec![],
        colors: vec![],
        indices: vec![],
    });
    assert!(geom.inverse(&base).is_empty());
    assert_eq!(geom.diff(&base).diff().apply(&base).expect("apply must succeed for a well-formed fixture"), base, "replace-primitive-geometry on an absent target is a no-op");

    let color = SemioMeshMutation::ChangeMaterialBaseColor(change_material_base_color::ChangeMaterialBaseColor { id: "mat-missing".into(), new_base_color: SemioRgba::default() });
    assert!(color.inverse(&base).is_empty());
    assert_eq!(color.diff(&base).diff().apply(&base).expect("apply must succeed for a well-formed fixture"), base, "change-material-base-color on an absent target is a no-op");

    let mv = SemioMeshMutation::MoveVertex(move_vertex::MoveVertex { mesh_id: "mesh-a".into(), primitive_id: "prim-a".into(), vertex_index: 999, new_point: SemioPoint3::default() });
    assert!(mv.inverse(&base).is_empty());
    assert_eq!(mv.diff(&base).diff().apply(&base).expect("apply must succeed for a well-formed fixture"), base, "move-vertex at an out-of-bounds index is a no-op");
}
//#endregion 🧪️InverseRoundTripLaw

//#region 🧪️DiffConsistencyLaw
/// 🧪️ diff_consistency_law: ∀ variant, the mutation's own `diff(base)` (built directly from
/// `(payload, base)`, never apply-then-capture) matches `SemioMeshDiff::between(base,
/// diff.diff().apply(base))` — the sparse diff this facet hand-constructs is exactly the diff a
/// generic before/after comparison would independently derive.
#[semio_framework_async_macros::async_test]
async fn diff_consistency_law_matches_independent_between() {
    use protocol::command::DiffAlgebra;
    let base = fixture();
    for m in demo_mutation_cases() {
        let hand_diff = m.diff(&base);
        let after = hand_diff.diff().apply(&base).expect("apply must succeed for a well-formed fixture");
        let independent_diff = SemioMeshDiff::between(&base, &after);
        assert_eq!(
            hand_diff.diff().apply(&base).expect("apply must succeed for a well-formed fixture"),
            independent_diff.apply(&base).expect("apply must succeed for a well-formed fixture"),
            "diff({m:?}) must match an independent before/after comparison"
        );
    }
}
//#endregion 🧪️DiffConsistencyLaw

//#region 🧪️DeterminismLaw
#[semio_framework_async_macros::async_test]
async fn determinism_law_diff_and_inverse_are_pure_functions_of_payload_and_base() {
    let base = fixture();
    for m in demo_mutation_cases() {
        assert_eq!(m.diff(&base), m.diff(&base), "diff({m:?}) must be deterministic");
        assert_eq!(m.inverse(&base), m.inverse(&base), "inverse({m:?}) must be deterministic");
    }
}
//#endregion 🧪️DeterminismLaw

//#region 🧪️OpCodecRoundTripLaw
#[semio_framework_async_macros::async_test]
async fn op_text_binary_roundtrip_law() {
    for m in demo_mutation_cases() {
        let printed = m.print_op();
        assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
        let parsed = SemioMeshMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
        assert_eq!(parsed, m, "print_op/parse_op round-trip mismatch for {m:?} (printed {printed:?})");

        let encoded = m.encode_op().unwrap_or_else(|e| panic!("encode_op({m:?}) failed: {e}"));
        let decoded = SemioMeshMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
        assert_eq!(decoded, m, "encode_op/decode_op round-trip mismatch for {m:?}");
    }
}
//#endregion 🧪️OpCodecRoundTripLaw

//#region 🧪️SemanticKinds
#[semio_framework_async_macros::async_test]
async fn semantic_kinds_cover_every_variant() {
    assert_eq!(SemioMeshMutation::kinds().len(), 17);
    let mutation = SemioMeshMutation::DeleteMesh(delete_mesh::DeleteMesh { id: "mesh-a".into() });
    assert_eq!(mutation.semantics().kind, "delete-mesh");
    assert_eq!(mutation.semantics().record, "DeletedMesh");
    assert_eq!(mutation.target(), vec!["mesh-a".to_string()]);
}
//#endregion 🧪️SemanticKinds

//#region 🧪️KindsCatalog
/// 🏷️ `KINDS` (this facet's own const, consumed by `mutate-semio-mesh`'s adapter) must name
/// every declared variant, in the exact order and spelling `#[derive(dsl::Mutations)]` assigns —
/// the framework never parses Rust, so this is what keeps the catalog honest.
#[semio_framework_async_macros::async_test]
async fn kinds_match_the_enum_and_the_catalog() {
    let descriptors = SemioMeshMutation::kinds();
    assert_eq!(KINDS.len(), descriptors.len(), "KINDS must name exactly one entry per declared variant");
    for (kind, descriptor) in KINDS.iter().zip(descriptors.iter()) {
        assert_eq!(*kind, descriptor.kind, "KINDS must match #[derive(dsl::Mutations)]'s own declaration order and spelling");
    }
    let manifest = include_str!("../../../../🔮️oracles/🔣️.json");
    for kind in KINDS {
        assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
    }
}
//#endregion 🧪️KindsCatalog
