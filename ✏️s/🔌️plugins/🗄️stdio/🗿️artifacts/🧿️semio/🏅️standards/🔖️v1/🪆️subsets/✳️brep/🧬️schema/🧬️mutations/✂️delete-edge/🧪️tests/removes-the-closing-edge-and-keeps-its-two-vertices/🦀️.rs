//! 🧪️ `delete-edge` fixture — `removes-the-closing-edge-and-keeps-its-two-vertices`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`: an unknown edge id is Error
//! `mutation.target-missing`; otherwise the diff is a bare `edges.removed[id]`. Note the asymmetry
//! with `delete-vertex`, which DOES cascade: deleting an edge cascades nowhere — not into its
//! vertices below it, and not into the loops above it — which is exactly what the committed diff's
//! single-collection shape pins.
use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::SemioBrepDiff;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::mutations::SemioBrepMutation;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> SemioBrepSnapshot {
    serde_json::from_str(BEFORE).expect("delete-edge before snapshot decodes")
}
fn expected_after() -> SemioBrepSnapshot {
    serde_json::from_str(AFTER).expect("delete-edge after snapshot decodes")
}
fn mutation() -> SemioBrepMutation {
    serde_json::from_str(MUTATION).expect("delete-edge mutation decodes")
}

/// ▶️ Only `e4` goes; both of its vertices and the loop that referenced it survive.
#[semio_framework_async_macros::async_test]
async fn removes_only_the_addressed_edge() {
    let base = before();
    let produced = mutation().diff(&base).diff().apply(&base).expect("delete-edge applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "delete-edge/removes-the-closing-edge-and-keeps-its-two-vertices: applied state differs from the committed after-snapshot");
    assert!(!produced.edges.iter().any(|edge| edge.id == "e4"), "the addressed edge must be gone");
    assert_eq!(produced.edges.len(), base.edges.len() - 1, "exactly one edge is removed");
    assert_eq!(produced.vertices, base.vertices, "deleting an edge must NOT cascade down into its vertices");
    assert_eq!(produced.loops, base.loops, "deleting an edge must NOT cascade up into the loops that reference it");
}

/// ↩️ The undo re-creates the edge with its captured endpoints AND its captured curve.
#[semio_framework_async_macros::async_test]
async fn the_undo_create_edge_restores_the_full_captured_edge() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "delete-edge of an existing edge undoes as exactly one create-edge");
    let SemioBrepMutation::CreateEdge(recreate) = &undo[0] else { panic!("delete-edge must undo as create-edge") };
    assert_eq!((recreate.start_vertex.as_str(), recreate.end_vertex.as_str()), ("v4", "v1"), "the undo must recapture the deleted edge's own endpoints from base");
    assert_eq!(recreate.curve, base.edges[3].curve, "the undo must recapture the deleted edge's own curve, not a default one");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward delete-edge applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo create-edge applies");
    }
    assert_eq!(current, base, "delete-edge/removes-the-closing-edge-and-keeps-its-two-vertices: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the `{"DeleteEdge":{"id":"e4"}}` payload are canonical fixed points.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioBrepSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "delete-edge/removes-the-closing-edge-and-keeps-its-two-vertices: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("delete-edge mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("delete-edge mutation reparses");
    assert_eq!(reencoded, original, "delete-edge/removes-the-closing-edge-and-keeps-its-two-vertices: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the edge exists, so mutation.target-missing must not fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_as_committed() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "delete-edge/removes-the-closing-edge-and-keeps-its-two-vertices: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "deleting an existing edge must raise no diagnostics — this leaf has no cascade to report");
}

/// 🔺️ The sparse delta this mutation produces equals the committed diff. Only `edges.removed`, carrying the ID, not the content.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioBrepMutation as Mutation<SemioBrepSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "delete-edge/removes-the-closing-edge-and-keeps-its-two-vertices: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point, and only the collection this mutation is
/// allowed to touch appears in it at all.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded: SemioBrepDiff = serde_json::from_str(DIFF).expect("committed delete-edge diff decodes");

    let edges = decoded.edges.as_ref().expect("delete-edge must write the edges triple");
    assert_eq!(edges.removed, vec!["e4".to_string()], "the removal is addressed by edge id");
    assert!(edges.modified.is_empty() && edges.added.is_empty(), "a removal neither modifies nor adds");
    assert!(decoded.vertices.is_none() && decoded.loops.is_none() && decoded.faces.is_none() && decoded.shells.is_none() && decoded.solids.is_none(), "delete-edge cascades nowhere — no other collection may appear");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "delete-edge/removes-the-closing-edge-and-keeps-its-two-vertices: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioBrepDiff = serde_json::from_str(DIFF).expect("committed delete-edge diff decodes");
    let produced = decoded.apply(&before()).expect("committed delete-edge diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "delete-edge/removes-the-closing-edge-and-keeps-its-two-vertices: committed diff did not carry before to after");
}
