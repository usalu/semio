//! 🧪️ `create-edge` fixture — `adds-a-diagonal-edge-across-the-square`.
//!
//! Transcribed from `../../🔺️diff/🦀️component.rs`: a duplicate edge id is FATAL
//! `mutation.duplicate-id` and that is the ONLY guard — note there is deliberately no
//! referential-integrity check on `start_vertex`/`end_vertex` here, unlike `✳️graph`'s own
//! `create-edge`. The diff is the `edges` triple's `added` arm alone.
use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::SemioBrepDiff;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::mutations::SemioBrepMutation;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> SemioBrepSnapshot {
    serde_json::from_str(BEFORE).expect("create-edge before snapshot decodes")
}
fn expected_after() -> SemioBrepSnapshot {
    serde_json::from_str(AFTER).expect("create-edge after snapshot decodes")
}
fn mutation() -> SemioBrepMutation {
    serde_json::from_str(MUTATION).expect("create-edge mutation decodes")
}

/// ▶️ A fifth edge appears between two EXISTING vertices; nothing above it in the topology moves.
#[semio_framework_async_macros::async_test]
async fn adds_the_diagonal_between_two_existing_vertices() {
    let base = before();
    let produced = mutation().diff(&base).diff().apply(&base).expect("create-edge applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "create-edge/adds-a-diagonal-edge-across-the-square: applied state differs from the committed after-snapshot");
    assert_eq!(produced.edges.len(), base.edges.len() + 1, "create-edge adds exactly one edge");
    let created = produced.edges.last().expect("the created edge is appended — id-keyed collections have no insertion index");
    assert_eq!((created.start_vertex.as_str(), created.end_vertex.as_str()), ("v1", "v3"), "the endpoints come straight from the payload");
    assert!(produced.vertices.iter().any(|vertex| vertex.id == created.start_vertex), "the fixture keeps the endpoints real even though this leaf does not check them");
    assert_eq!(produced.vertices, base.vertices, "creating an edge must not touch a vertex");
    assert_eq!(produced.loops, base.loops, "creating a free edge must not enrol it in any loop");
}

/// ↩️ `create-edge`'s undo is a single `delete-edge` for the same id.
#[semio_framework_async_macros::async_test]
async fn the_undo_delete_edge_removes_the_diagonal_again() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "create-edge undoes as exactly one delete-edge");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward create-edge applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo delete-edge applies");
    }
    assert_eq!(current, base, "create-edge/adds-a-diagonal-edge-across-the-square: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the payload are canonical — the ENTITY spells its endpoints `startVertex`/`endVertex` (camelCase) while the PAYLOAD spells them `start_vertex`/`end_vertex`.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioBrepSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "create-edge/adds-a-diagonal-edge-across-the-square: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("create-edge mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("create-edge mutation reparses");
    assert_eq!(reencoded, original, "create-edge/adds-a-diagonal-edge-across-the-square: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the edge id is free, so the FATAL mutation.duplicate-id branch must not fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_as_committed() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "create-edge/adds-a-diagonal-edge-across-the-square: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "creating an edge with a fresh id must raise no diagnostics");
}

/// 🔺️ The sparse delta this mutation produces equals the committed diff. Only `edges.added`.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioBrepMutation as Mutation<SemioBrepSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "create-edge/adds-a-diagonal-edge-across-the-square: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point, and only the collection this mutation is
/// allowed to touch appears in it at all.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded: SemioBrepDiff = serde_json::from_str(DIFF).expect("committed create-edge diff decodes");

    let edges = decoded.edges.as_ref().expect("create-edge must write the edges triple");
    assert_eq!(edges.added.len(), 1, "exactly one edge is added");
    assert!(edges.removed.is_empty() && edges.modified.is_empty(), "a create neither removes nor modifies");
    assert!(decoded.vertices.is_none() && decoded.loops.is_none() && decoded.faces.is_none() && decoded.shells.is_none() && decoded.solids.is_none(), "no other collection may appear in the diff");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "create-edge/adds-a-diagonal-edge-across-the-square: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioBrepDiff = serde_json::from_str(DIFF).expect("committed create-edge diff decodes");
    let produced = decoded.apply(&before()).expect("committed create-edge diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "create-edge/adds-a-diagonal-edge-across-the-square: committed diff did not carry before to after");
}
