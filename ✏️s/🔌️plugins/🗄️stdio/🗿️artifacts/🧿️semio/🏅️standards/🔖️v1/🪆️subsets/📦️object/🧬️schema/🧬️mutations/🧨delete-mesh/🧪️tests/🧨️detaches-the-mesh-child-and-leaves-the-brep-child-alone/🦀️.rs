//! 🧪️ `delete-mesh` fixture — `🧨️detaches-the-mesh-child-and-leaves-the-brep-child-alone`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`: an object with no mesh child is Error
//! `mutation.target-missing`; otherwise the diff sets `mesh = Some(None)` — outer `Some` = "this
//! diff writes the slot", inner `None` = "clear it".
//!
//! `Some(None)` encodes as `{"mesh": null}`, and the value codec decodes an explicit `null` back
//! into the INNER `None` (an absent key stays the outer `None`, "untouched"), so the committed
//! `🔺️diff/🔣️.json` is a decode→encode fixed point that clears the slot when applied.

use crate::standards::v1::subsets::object::schema::diff::SemioObjectDiff;
use crate::standards::v1::subsets::object::schema::mutations::SemioObjectMutation;
use crate::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧨delete-mesh/🧨️detaches-the-mesh-child-and-leaves-the-brep-child-alone/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧨delete-mesh/🧨️detaches-the-mesh-child-and-leaves-the-brep-child-alone/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧨delete-mesh/🧨️detaches-the-mesh-child-and-leaves-the-brep-child-alone/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧨delete-mesh/🧨️detaches-the-mesh-child-and-leaves-the-brep-child-alone/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧨delete-mesh/🧨️detaches-the-mesh-child-and-leaves-the-brep-child-alone/🎯️outcome/🔣️.json");

fn before() -> SemioObjectSnapshot {
    dsl::json::from_json_str(BEFORE).expect("delete-mesh before snapshot decodes")
}
fn expected_after() -> SemioObjectSnapshot {
    dsl::json::from_json_str(AFTER).expect("delete-mesh after snapshot decodes")
}
fn mutation() -> SemioObjectMutation {
    dsl::json::from_json_str(MUTATION).expect("delete-mesh mutation decodes")
}

/// ▶️ The mesh handle is cleared and the sibling brep handle is deliberately left in place — the
/// slots are independent, and a delete that cleared both would still reach an "empty" object.
#[semio_framework_async_macros::async_test]
async fn clears_the_mesh_slot_and_leaves_the_brep_slot_alone() {
    let base = before();
    assert!(base.mesh.is_some() && base.brep.is_some(), "the fixture needs BOTH slots populated for the independence claim to mean anything");
    let produced = mutation().diff(&base).diff().apply(&base).expect("delete-mesh applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "delete-mesh/detaches-the-mesh-child-and-leaves-the-brep-child-alone: applied state differs from the committed after-snapshot");
    assert!(produced.mesh.is_none(), "the mesh slot must be empty afterwards");
    assert_eq!(produced.brep, base.brep, "the sibling brep handle must survive untouched");
    assert_eq!(produced.transform, base.transform, "delete-mesh must not touch the object's placement");
}

/// ↩️ The undo re-creates the captured handle — the same `childId` and the same target, not a
/// fresh one.
#[semio_framework_async_macros::async_test]
async fn the_undo_create_mesh_reattaches_the_captured_handle() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "deleting an existing child undoes as exactly one CreateMesh");
    let SemioObjectMutation::CreateMesh(recreate) = &undo[0] else { panic!("delete-mesh must undo as CreateMesh") };
    assert_eq!(recreate.child_id, "kitchen-sink-mesh", "the undo must recapture the ORIGINAL child id from base");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward delete-mesh applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo CreateMesh applies to the cleared object");
    }
    assert_eq!(current, base, "delete-mesh/detaches-the-mesh-child-and-leaves-the-brep-child-alone: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the empty-payload `{"DeleteMesh":{}}` mutation are canonical — the after-snapshot
/// OMITS the `mesh` key entirely (the snapshot field is a plain `Option` with
/// `skip_serializing_if`), which is a different encoding from the diff's `null`.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioObjectSnapshot = dsl::json::from_json_str(text).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&decoded)).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "delete-mesh/detaches-the-mesh-child-and-leaves-the-brep-child-alone: committed {label} JSON is not canonical");
    }
    let after_json: serde_json::Value = serde_json::from_str(AFTER).expect("after reparses");
    assert!(after_json.get("mesh").is_none(), "a cleared snapshot slot is an ABSENT key, never an explicit null");
    let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&(mutation()))).expect("delete-mesh mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("delete-mesh mutation reparses");
    assert_eq!(reencoded, original, "delete-mesh/detaches-the-mesh-child-and-leaves-the-brep-child-alone: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the base really has a mesh child, so the `mutation.target-missing`
/// rejection must not fire.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_without_a_target_missing_rejection() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "delete-mesh/detaches-the-mesh-child-and-leaves-the-brep-child-alone: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "clearing a populated slot must raise no diagnostics");
}

/// 🔺️ The produced delta ENCODES to exactly the committed diff: `{"mesh": null}`, and nothing
/// else. This is the assertion that pins "clear the slot" as distinct from "leave it alone".
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioObjectMutation as Mutation<SemioObjectSnapshot>>::diff(&mutation(), &base);
    assert!(matches!(outcome.diff().mesh, Some(None)), "the in-memory diff must be Some(None) — write the slot, clear it");
    let produced = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(outcome.diff())).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "delete-mesh/detaches-the-mesh-child-and-leaves-the-brep-child-alone: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert!(committed.get("mesh").expect("the committed diff names the mesh slot").is_null(), "a cleared DIFF slot is an explicit null, never an absent key");
}

/// 🔣️ The committed diff is a decode→encode fixed point that keeps the clear intent: an explicit
/// `{"mesh":null}` decodes to `Some(None)` — write the slot, clear it — never to the outer
/// "untouched" `None`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_json_decodes_to_the_explicit_clear() {
    let decoded: SemioObjectDiff = dsl::json::from_json_str(DIFF).expect("committed delete-mesh diff decodes");
    assert!(matches!(decoded.mesh, Some(None)), "decoding {{\"mesh\":null}} must yield Some(None) — the clear intent survives the JSON round trip");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&decoded)).expect("re-encode"), committed, "the committed diff is a decode→encode fixed point");
    let authored = SemioObjectDiff { mesh: Some(None), ..Default::default() };
    assert_eq!(decoded, authored, "the decoded diff is exactly the authored Some(None) diff");
}

/// 🩹 The diff carries `before` to `after` — both the in-memory `Some(None)` diff and the one decoded
/// from the committed JSON.
#[semio_framework_async_macros::async_test]
async fn authored_and_decoded_diffs_apply_to_after() {
    let authored = SemioObjectDiff { mesh: Some(None), ..Default::default() };
    let produced = authored.apply(&before()).expect("the Some(None) diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "delete-mesh/detaches-the-mesh-child-and-leaves-the-brep-child-alone: the Some(None) diff did not carry before to after");
    let decoded: SemioObjectDiff = dsl::json::from_json_str(DIFF).expect("committed delete-mesh diff decodes");
    let applied = decoded.apply(&before()).expect("the decoded diff applies to the before-snapshot");
    assert_eq!(applied, expected_after(), "delete-mesh/detaches-the-mesh-child-and-leaves-the-brep-child-alone: the JSON-decoded diff did not carry before to after");
}
