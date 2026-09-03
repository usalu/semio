//! 🧪️ `delete-face` fixture — `removes-the-only-face-and-leaves-its-loop-behind`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`: an unknown face id is Error
//! `mutation.target-missing`; otherwise the diff is a bare `faces.removed[id]`. No cascade in
//! either direction — the loop the face bounded stays, and so does the shell that references the
//! face. That deliberate non-cascade is what the single-collection diff pins.
use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::{decode_semio_brep_diff_json, SemioBrepDiff};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::mutations::{decode_semio_brep_mutation_json, SemioBrepMutation};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::{decode_semio_brep_snapshot_json, SemioBrepSnapshot};
use pack::value::ToValue;
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> SemioBrepSnapshot {
    decode_semio_brep_snapshot_json(BEFORE).expect("delete-face before snapshot decodes")
}
fn expected_after() -> SemioBrepSnapshot {
    decode_semio_brep_snapshot_json(AFTER).expect("delete-face after snapshot decodes")
}
fn mutation() -> SemioBrepMutation {
    decode_semio_brep_mutation_json(MUTATION).expect("delete-face mutation decodes")
}

/// ▶️ The face goes; its loop below and the shell above it both remain.
#[semio_framework_async_macros::async_test]
async fn removes_the_face_without_cascading_either_way() {
    let base = before();
    let produced = mutation().diff(&base).diff().apply(&base).expect("delete-face applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "delete-face/removes-the-only-face-and-leaves-its-loop-behind: applied state differs from the committed after-snapshot");
    assert!(produced.faces.is_empty(), "the only face must be gone");
    assert_eq!(produced.loops, base.loops, "delete-face must NOT cascade down into the loop it bounded");
    assert_eq!(produced.shells, base.shells, "delete-face must NOT cascade up into the shell that references it");
}

/// ↩️ The undo re-creates the face with its captured loop, surface and orientation.
#[semio_framework_async_macros::async_test]
async fn the_undo_create_face_restores_the_full_captured_face() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "delete-face of an existing face undoes as exactly one create-face");
    let SemioBrepMutation::CreateFace(recreate) = &undo[0] else { panic!("delete-face must undo as create-face") };
    assert_eq!(recreate.surface, base.faces[0].surface, "the undo must recapture the deleted face's own surface");
    assert_eq!(recreate.orientation, base.faces[0].orientation, "the undo must recapture the deleted face's own orientation");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward delete-face applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo create-face applies");
    }
    assert_eq!(current, base, "delete-face/removes-the-only-face-and-leaves-its-loop-behind: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the `{"DeleteFace":{"id":"f1"}}` payload are canonical fixed points.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded = decode_semio_brep_snapshot_json(text).expect("snapshot decodes");
        let reencoded = pack::json::from_dsl_value(&decoded.to_value());
        let original = pack::json::parse(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "delete-face/removes-the-only-face-and-leaves-its-loop-behind: committed {label} JSON is not canonical");
    }
    let reencoded = pack::json::from_dsl_value(&mutation().to_value());
    let original = pack::json::parse(MUTATION).expect("delete-face mutation reparses");
    assert_eq!(reencoded, original, "delete-face/removes-the-only-face-and-leaves-its-loop-behind: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the face exists, so mutation.target-missing must not fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_as_committed() {
    let outcome = pack::json::parse(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(pack::json::Value::as_str), Some("applied"), "delete-face/removes-the-only-face-and-leaves-its-loop-behind: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "deleting an existing face must raise no diagnostics — this leaf has no cascade to report");
}

/// 🔺️ The sparse delta this mutation produces equals the committed diff. Only `faces.removed`.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioBrepMutation as Mutation<SemioBrepSnapshot>>::diff(&mutation(), &base);
    let produced = pack::json::from_dsl_value(&outcome.diff().to_value());
    let committed = pack::json::parse(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "delete-face/removes-the-only-face-and-leaves-its-loop-behind: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point, and only the collection this mutation is
/// allowed to touch appears in it at all.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded = decode_semio_brep_diff_json(DIFF).expect("committed delete-face diff decodes");

    let faces = decoded.faces.as_ref().expect("delete-face must write the faces triple");
    assert_eq!(faces.removed, vec!["f1".to_string()], "the removal is addressed by face id");
    assert!(faces.modified.is_empty() && faces.added.is_empty(), "a removal neither modifies nor adds");
    assert!(decoded.vertices.is_none() && decoded.edges.is_none() && decoded.loops.is_none() && decoded.shells.is_none() && decoded.solids.is_none(), "delete-face cascades nowhere — no other collection may appear");
    let reencoded = pack::json::from_dsl_value(&decoded.to_value());
    let original = pack::json::parse(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "delete-face/removes-the-only-face-and-leaves-its-loop-behind: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded = decode_semio_brep_diff_json(DIFF).expect("committed delete-face diff decodes");
    let produced = decoded.apply(&before()).expect("committed delete-face diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "delete-face/removes-the-only-face-and-leaves-its-loop-behind: committed diff did not carry before to after");
}
