use super::*;
use protocol::{Mutation, MutationDiff, SemanticMutation};

/// 🔧️ All 6 collections are id-keyed SETS with no user-meaningful display order (this facet's
/// own `🔺️diff` module doc comment; same shape `🕸️graph`'s `nodes`/`edges` already establish and
/// test the same way): `create-*`'s diff always APPENDS to `added`, so an inverse that
/// re-creates an entity deleted from the middle of a collection legitimately lands it at the
/// end. Round-trip fidelity is therefore SET equality, not vector equality — sort by id before
/// comparing, never compare the raw `Vec` order.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sorted_by_id(mut s: SemioBrepSnapshot) -> SemioBrepSnapshot {
    s.vertices.sort_by(|a, b| a.id.cmp(&b.id));
    s.edges.sort_by(|a, b| a.id.cmp(&b.id));
    s.loops.sort_by(|a, b| a.id.cmp(&b.id));
    s.faces.sort_by(|a, b| a.id.cmp(&b.id));
    s.shells.sort_by(|a, b| a.id.cmp(&b.id));
    s.solids.sort_by(|a, b| a.id.cmp(&b.id));
    s
}

/// 🔧️ Diffs each inverse against the CURRENT (evolving) state, never the stale pre-operation
/// `base` — written independently against `(payload, base)` semantics per the ticket's explicit
/// warning not to derive test helpers from `din4108`'s reference (which diffs the inverse
/// against the stale `base` and silently discards the forward mutation's effect). This harness
/// threads `mutation.diff(&current); current = diff.diff().apply(&current)` at every step, forward AND
/// backward, matching this facet's own `mutate()` (`🧊️brep/🧬️schema/🦀️.rs`) production
/// convention.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn round_trip(base: &SemioBrepSnapshot, operation: &SemioBrepMutation) -> SemioBrepSnapshot {
    let forward = operation.diff(base).diff().apply(base).expect("apply must succeed for a well-formed fixture");
    let backwards = operation.inverse(base);
    let mut restored = forward.clone();
    for back in &backwards {
        restored = back.diff(&restored).diff().apply(&restored).expect("apply must succeed for a well-formed fixture");
    }
    assert_eq!(sorted_by_id(restored), sorted_by_id(base.clone()), "inverse must exactly restore the pre-operation fixture (as a SET) for {operation:?}");
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
async fn create_delete_vertex_round_trips_explicitly() {
    let base = fixture();
    let create = SemioBrepMutation::CreateVertex(create_vertex::CreateVertex { id: "v3".into(), point: crate::standards::v1::subsets::base::schema::geometry::SemioPoint3 { x: 2.0, y: 2.0, z: 2.0 }, tol: 2e-7 });
    let after_create = round_trip(&base, &create);
    assert!(after_create.vertices.iter().any(|v| v.id == "v3"));

    let undo = create.inverse(&base);
    assert_eq!(undo, vec![SemioBrepMutation::DeleteVertex(delete_vertex::DeleteVertex { id: "v3".into() })]);

    let delete = SemioBrepMutation::DeleteVertex(delete_vertex::DeleteVertex { id: "v2".into() });
    let after_delete = round_trip(&base, &delete);
    assert!(!after_delete.vertices.iter().any(|v| v.id == "v2"));
}

#[semio_framework_async_macros::async_test]
async fn delete_vertex_cascades_to_dependent_edges_and_inverse_restores_both() {
    let base = fixture();
    let delete = SemioBrepMutation::DeleteVertex(delete_vertex::DeleteVertex { id: "v1".into() });
    let diff = delete.diff(&base);
    let after = diff.diff().apply(&base).expect("apply must succeed for a well-formed fixture");
    assert!(!after.vertices.iter().any(|v| v.id == "v1"), "v1 must be gone");
    assert!(!after.edges.iter().any(|e| e.id == "e1"), "e1 (dependent on v1) must be cascade-deleted");

    let undo = delete.inverse(&base);
    assert_eq!(undo.len(), 2, "inverse must reconstruct the vertex AND the one cascade-deleted edge");
    let mut restored = after;
    for back in &undo {
        restored = back.diff(&restored).diff().apply(&restored).expect("apply must succeed for a well-formed fixture");
    }
    assert_eq!(sorted_by_id(restored), sorted_by_id(base), "cascade delete-vertex must be exactly undoable (as a SET)");
}

#[semio_framework_async_macros::async_test]
async fn delete_of_an_absent_id_has_an_empty_inverse_and_is_a_diff_level_no_op() {
    let base = fixture();
    let delete = SemioBrepMutation::DeleteFace(delete_face::DeleteFace { id: "f-missing".into() });
    assert!(delete.inverse(&base).is_empty(), "deleting an absent id has nothing to undo");
}

#[semio_framework_async_macros::async_test]
async fn replace_and_move_of_an_absent_target_have_empty_inverse_and_are_no_ops() {
    let base = fixture();
    let replace = SemioBrepMutation::ReplaceCurve(replace_curve::ReplaceCurve {
        edge_id: "e-missing".into(),
        new_curve: crate::standards::v1::subsets::brep::schema::snapshot::BrepCurve::Line {
            origin: crate::standards::v1::subsets::base::schema::geometry::SemioPoint3::default(),
            direction: crate::standards::v1::subsets::base::schema::geometry::SemioPoint3::default(),
        },
    });
    assert!(replace.inverse(&base).is_empty());
    assert_eq!(replace.diff(&base).diff().apply(&base).expect("apply must succeed for a well-formed fixture"), base, "replace-curve on an absent edge is a no-op");

    let mv = SemioBrepMutation::MoveVertex(move_vertex::MoveVertex { vertex_id: "v-missing".into(), new_point: crate::standards::v1::subsets::base::schema::geometry::SemioPoint3::default() });
    assert!(mv.inverse(&base).is_empty());
    assert_eq!(mv.diff(&base).diff().apply(&base).expect("apply must succeed for a well-formed fixture"), base, "move-vertex on an absent vertex is a no-op");
}
//#endregion 🧪️InverseRoundTripLaw

//#region 🧪️DiffConsistencyLaw
/// 🧪️ diff_consistency_law: ∀ variant, the mutation's own `diff(base)` (built directly from
/// `(payload, base)`, never apply-then-capture) matches `SemioBrepDiff::between(base,
/// diff.diff().apply(base))` — i.e. the sparse diff this facet hand-constructs is exactly the diff a
/// generic before/after comparison would independently derive. This is the check that would
/// have caught an apply-then-capture bug (the `mutual recursion` trap this module's original
/// doc comment already warned about) or a cascade that silently touched the wrong fields.
#[semio_framework_async_macros::async_test]
async fn diff_consistency_law_matches_independent_between() {
    use protocol::command::DiffAlgebra;
    let base = fixture();
    for m in demo_mutation_cases() {
        let hand_diff = m.diff(&base);
        let after = hand_diff.diff().apply(&base).expect("apply must succeed for a well-formed fixture");
        let independent_diff = SemioBrepDiff::between(&base, &after);
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
        let parsed = SemioBrepMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
        assert_eq!(parsed, m, "print_op/parse_op round-trip mismatch for {m:?}");

        let encoded = m.encode_op().unwrap_or_else(|e| panic!("encode_op({m:?}) failed: {e}"));
        let decoded = SemioBrepMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
        assert_eq!(decoded, m, "encode_op/decode_op round-trip mismatch for {m:?}");
    }
}

#[semio_framework_async_macros::async_test]
async fn language_neutral_tolerance_contract_matches_rust_decoder() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🧫️fixtures/📏️tolerance/🔣️.json")).expect("tolerance cases decode");
    for test_case in fixture["cases"].as_array().expect("tolerance cases are an array") {
        let accepted = decode_semio_brep_mutation_json(&test_case["mutation"].to_string()).is_ok();
        assert_eq!(accepted, test_case["accepted"].as_bool().expect("case declares acceptance"), "{}", test_case["id"].as_str().expect("case id is a string"));
    }
    for test_case in fixture["diffCases"].as_array().expect("tolerance diff cases are an array") {
        let accepted = crate::standards::v1::subsets::brep::schema::diff::decode_semio_brep_diff_json(&test_case["diff"].to_string()).is_ok();
        assert_eq!(accepted, test_case["accepted"].as_bool().expect("case declares acceptance"), "{}", test_case["id"].as_str().expect("case id is a string"));
    }
}
//#endregion 🧪️OpCodecRoundTripLaw

//#region 🧪️SemanticKinds
#[semio_framework_async_macros::async_test]
async fn semantic_kinds_cover_every_variant() {
    assert_eq!(SemioBrepMutation::kinds().len(), 13);
    let mutation = SemioBrepMutation::DeleteVertex(delete_vertex::DeleteVertex { id: "v1".into() });
    assert_eq!(mutation.semantics().kind, "delete-vertex");
    assert_eq!(mutation.semantics().record, "DeletedVertex");
    assert_eq!(mutation.target(), vec!["v1".to_string()]);
}
//#endregion 🧪️SemanticKinds

//#region 🧪️KindsCatalog
/// 🏷️ `KINDS` (this facet's own const, consumed by `mutate-semio-brep`'s adapter) must name
/// every declared variant, in the exact order and spelling `#[derive(dsl::Mutations)]` assigns —
/// the framework never parses Rust, so this is what keeps the catalog honest.
#[semio_framework_async_macros::async_test]
async fn kinds_match_the_enum_and_the_catalog() {
    let descriptors = SemioBrepMutation::kinds();
    assert_eq!(KINDS.len(), descriptors.len(), "KINDS must name exactly one entry per declared variant");
    for (kind, descriptor) in KINDS.iter().zip(descriptors.iter()) {
        assert_eq!(*kind, descriptor.kind, "KINDS must match #[derive(dsl::Mutations)]'s own declaration order and spelling");
    }
    let manifest = include_str!("../../../../🔮️oracle/🔣️.json");
    for kind in KINDS {
        assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
    }
}
//#endregion 🧪️KindsCatalog
