//! 🧪️ `move-vertex` fixture — `lifts-the-third-corner-off-the-base-plane`.
//!
//! Transcribed from `../../🔺️diff/🦀️component.rs`, guards in order: unknown id ⇒ Error
//! `mutation.target-missing`; a non-finite component ⇒ FATAL `mutation.invariant`; an unchanged
//! point ⇒ Warning `mutation.no-op`. The diff is a `vertices.modified` entry whose per-vertex diff
//! sets `point` — a vertex is a strong entity, so it is diffed per field, never removed and
//! re-added. Moving a vertex deliberately does NOT re-fit the curves of the edges that reference
//! it; the diff proves that by never mentioning `edges`.
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
    serde_json::from_str(BEFORE).expect("move-vertex before snapshot decodes")
}
fn expected_after() -> SemioBrepSnapshot {
    serde_json::from_str(AFTER).expect("move-vertex after snapshot decodes")
}
fn mutation() -> SemioBrepMutation {
    serde_json::from_str(MUTATION).expect("move-vertex mutation decodes")
}

/// ▶️ Only `v3`'s point changes — the edges that reference it keep their existing curves.
#[semio_framework_async_macros::async_test]
async fn moves_only_the_addressed_vertex() {
    let base = before();
    let produced = mutation().diff(&base).diff().apply(&base).expect("move-vertex applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "move-vertex/lifts-the-third-corner-off-the-base-plane: applied state differs from the committed after-snapshot");
    let moved = produced.vertices.iter().find(|vertex| vertex.id == "v3").expect("the moved vertex is still there — a move is not a delete");
    assert_eq!((moved.point.x, moved.point.y, moved.point.z), (2.0, 1.0, 0.5), "the point must become the payload's absolute coordinates");
    assert_eq!(produced.vertices.len(), base.vertices.len(), "move-vertex may never add or drop a vertex");
    assert_eq!(produced.edges, base.edges, "moving a vertex must not silently re-fit the curves of the edges that reference it");
}

/// ↩️ The undo is a `move-vertex` back to BASE's own point.
#[semio_framework_async_macros::async_test]
async fn the_undo_move_vertex_restores_the_original_point() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "move-vertex of an existing vertex undoes as exactly one move-vertex");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward move-vertex applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo move-vertex applies");
    }
    assert_eq!(current, base, "move-vertex/lifts-the-third-corner-off-the-base-plane: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the `{"MoveVertex":{"vertex_id":"v3","new_point":{…}}}` payload are canonical — every coordinate is dyadic so decode→encode is exact.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioBrepSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "move-vertex/lifts-the-third-corner-off-the-base-plane: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("move-vertex mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("move-vertex mutation reparses");
    assert_eq!(reencoded, original, "move-vertex/lifts-the-third-corner-off-the-base-plane: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the vertex exists, the coordinates are finite and genuinely different, so none of target-missing/invariant/no-op may fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_as_committed() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "move-vertex/lifts-the-third-corner-off-the-base-plane: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "a finite, genuinely-different point must raise no diagnostics");
}

/// 🔺️ The sparse delta this mutation produces equals the committed diff. Only `vertices.modified`, keyed by vertex id.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioBrepMutation as Mutation<SemioBrepSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "move-vertex/lifts-the-third-corner-off-the-base-plane: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioBrepDiff = serde_json::from_str(DIFF).expect("committed move-vertex diff decodes");
    let produced = decoded.apply(&before()).expect("committed move-vertex diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "move-vertex/lifts-the-third-corner-off-the-base-plane: committed diff did not carry before to after");
}

/// 🔣️ The committed diff is a decode→encode fixed point, and only the collections the mutation is
/// allowed to touch appear in it at all.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded: SemioBrepDiff = serde_json::from_str(DIFF).expect("committed move-vertex diff decodes");

    let vertices = decoded.vertices.as_ref().expect("move-vertex must write the vertices triple");
    assert!(vertices.removed.is_empty() && vertices.added.is_empty(), "a move is a per-field modification, never a remove-and-re-add");
    assert_eq!(vertices.modified.len(), 1, "exactly one vertex is modified");
    assert_eq!(vertices.modified[0].key, "v3", "the modification is keyed by vertex id");
    assert!(decoded.edges.is_none() && decoded.loops.is_none() && decoded.faces.is_none() && decoded.shells.is_none() && decoded.solids.is_none(), "no other collection may appear in the diff");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "move-vertex/lifts-the-third-corner-off-the-base-plane: committed diff JSON is not canonical");
}
