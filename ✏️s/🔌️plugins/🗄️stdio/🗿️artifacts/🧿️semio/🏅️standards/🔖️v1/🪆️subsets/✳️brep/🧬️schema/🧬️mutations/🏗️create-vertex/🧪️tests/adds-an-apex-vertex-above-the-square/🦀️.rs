//! 🧪️ `create-vertex` fixture — `adds-an-apex-vertex-above-the-square`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`: a duplicate id is FATAL
//! `mutation.duplicate-id`; otherwise the diff is a `vertices` `NamedTripleDiff` carrying the new
//! `BrepVertex` in `added` and NOTHING else — `removed`/`modified` are empty vectors, which
//! `skip_serializing_if = "Vec::is_empty"` drops from the JSON entirely. All six brep collections
//! are id-keyed, so the new vertex is appended by `apply_named` rather than positioned.
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
    serde_json::from_str(BEFORE).expect("create-vertex before snapshot decodes")
}
fn expected_after() -> SemioBrepSnapshot {
    serde_json::from_str(AFTER).expect("create-vertex after snapshot decodes")
}
fn mutation() -> SemioBrepMutation {
    serde_json::from_str(MUTATION).expect("create-vertex mutation decodes")
}

/// ▶️ A fifth vertex appears; the four existing ones and every other collection stay put.
#[semio_framework_async_macros::async_test]
async fn adds_the_apex_vertex_without_touching_any_other_collection() {
    let base = before();
    let produced = mutation().diff(&base).diff().apply(&base).expect("create-vertex applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "create-vertex/adds-an-apex-vertex-above-the-square: applied state differs from the committed after-snapshot");
    assert_eq!(produced.vertices.len(), base.vertices.len() + 1, "create-vertex adds exactly one vertex");
    let created = produced.vertices.last().expect("the created vertex is appended — id-keyed collections have no insertion index");
    assert_eq!(created.id, "v5", "the vertex keeps the id the payload named");
    assert_eq!(created.point.z, 1.0, "the vertex carries the payload's own point");
    assert_eq!((produced.edges, produced.loops, produced.faces, produced.shells, produced.solids), (base.edges, base.loops, base.faces, base.shells, base.solids), "an isolated vertex touches no topology above it");
}

/// ↩️ `create-vertex`'s undo is a single `delete-vertex` for the same id.
#[semio_framework_async_macros::async_test]
async fn the_undo_delete_vertex_removes_the_apex_again() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "creating a vertex nothing references undoes as exactly one delete-vertex");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward create-vertex applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo delete-vertex applies");
    }
    assert_eq!(current, base, "create-vertex/adds-an-apex-vertex-above-the-square: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the `{"CreateVertex":{"id":"v5","point":{…}}}` payload are canonical — brep entity structs are camelCase (`startVertex`, `outerLoop`, `isVoid`) while the mutation payloads carry no `rename_all` at all.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioBrepSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "create-vertex/adds-an-apex-vertex-above-the-square: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("create-vertex mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("create-vertex mutation reparses");
    assert_eq!(reencoded, original, "create-vertex/adds-an-apex-vertex-above-the-square: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: no vertex with id v5 exists, so the FATAL mutation.duplicate-id branch must not fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_as_committed() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "create-vertex/adds-an-apex-vertex-above-the-square: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "creating a vertex with a fresh id must raise no diagnostics");
}

/// 🔺️ The sparse delta this mutation produces equals the committed diff. Only the `vertices` triple, and only its `added` arm.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioBrepMutation as Mutation<SemioBrepSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "create-vertex/adds-an-apex-vertex-above-the-square: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioBrepDiff = serde_json::from_str(DIFF).expect("committed create-vertex diff decodes");
    let produced = decoded.apply(&before()).expect("committed create-vertex diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "create-vertex/adds-an-apex-vertex-above-the-square: committed diff did not carry before to after");
}

/// 🔣️ The committed diff is a decode→encode fixed point, and only the collections the mutation is
/// allowed to touch appear in it at all.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded: SemioBrepDiff = serde_json::from_str(DIFF).expect("committed create-vertex diff decodes");

    let vertices = decoded.vertices.as_ref().expect("create-vertex must write the vertices triple");
    assert_eq!(vertices.added.len(), 1, "exactly one vertex is added");
    assert!(vertices.removed.is_empty() && vertices.modified.is_empty(), "a create neither removes nor modifies");
    assert!(decoded.edges.is_none() && decoded.loops.is_none() && decoded.faces.is_none() && decoded.shells.is_none() && decoded.solids.is_none(), "no other collection may appear in the diff");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "create-vertex/adds-an-apex-vertex-above-the-square: committed diff JSON is not canonical");
}
