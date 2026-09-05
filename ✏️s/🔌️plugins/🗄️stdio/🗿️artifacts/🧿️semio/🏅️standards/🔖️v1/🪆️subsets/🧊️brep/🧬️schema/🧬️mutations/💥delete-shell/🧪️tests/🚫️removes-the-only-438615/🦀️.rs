//! 🧪️ `delete-shell` fixture — `🚫️removes-the-only-shell-and-leaves-its-faces-behind`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`: an unknown shell id is Error
//! `mutation.target-missing`; otherwise the diff is a bare `shells.removed[id]`. No cascade: the
//! faces the shell gathered stay, and so does the solid that references it — leaving that solid
//! pointing at a shell that no longer exists is the code's own choice, and the diff's
//! single-collection shape is what records it.
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
    decode_semio_brep_snapshot_json(BEFORE).expect("delete-shell before snapshot decodes")
}
fn expected_after() -> SemioBrepSnapshot {
    decode_semio_brep_snapshot_json(AFTER).expect("delete-shell after snapshot decodes")
}
fn mutation() -> SemioBrepMutation {
    decode_semio_brep_mutation_json(MUTATION).expect("delete-shell mutation decodes")
}

/// ▶️ The shell goes; its faces below and the solid above it both remain.
#[semio_framework_async_macros::async_test]
async fn removes_the_shell_without_cascading_either_way() {
    let base = before();
    let produced = mutation().diff(&base).diff().apply(&base).expect("delete-shell applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "delete-shell/removes-the-only-shell-and-leaves-its-faces-behind: applied state differs from the committed after-snapshot");
    assert!(produced.shells.is_empty(), "the only shell must be gone");
    assert_eq!(produced.faces, base.faces, "delete-shell must NOT cascade down into the faces it gathered");
    assert_eq!(produced.solids, base.solids, "delete-shell must NOT cascade up into the solid that references it");
}

/// ↩️ The undo re-creates the shell with its captured face list, orientations included.
#[semio_framework_async_macros::async_test]
async fn the_undo_create_shell_restores_the_captured_face_list() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "delete-shell of an existing shell undoes as exactly one create-shell");
    let SemioBrepMutation::CreateShell(recreate) = &undo[0] else { panic!("delete-shell must undo as create-shell") };
    assert_eq!(recreate.faces, base.shells[0].faces, "the undo must recapture the deleted shell's own face list verbatim");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward delete-shell applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo create-shell applies");
    }
    assert_eq!(current, base, "delete-shell/removes-the-only-shell-and-leaves-its-faces-behind: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the `{"DeleteShell":{"id":"s1"}}` payload are canonical fixed points.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded = decode_semio_brep_snapshot_json(text).expect("snapshot decodes");
        let reencoded = pack::json::from_dsl_value(&decoded.to_value());
        let original = pack::json::parse(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "delete-shell/removes-the-only-shell-and-leaves-its-faces-behind: committed {label} JSON is not canonical");
    }
    let reencoded = pack::json::from_dsl_value(&mutation().to_value());
    let original = pack::json::parse(MUTATION).expect("delete-shell mutation reparses");
    assert_eq!(reencoded, original, "delete-shell/removes-the-only-shell-and-leaves-its-faces-behind: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the shell exists, so mutation.target-missing must not fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_as_committed() {
    let outcome = pack::json::parse(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(pack::json::Value::as_str), Some("applied"), "delete-shell/removes-the-only-shell-and-leaves-its-faces-behind: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "deleting an existing shell must raise no diagnostics — this leaf has no cascade to report");
}

/// 🔺️ The sparse delta this mutation produces equals the committed diff. Only `shells.removed`.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioBrepMutation as Mutation<SemioBrepSnapshot>>::diff(&mutation(), &base);
    let produced = pack::json::from_dsl_value(&outcome.diff().to_value());
    let committed = pack::json::parse(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "delete-shell/removes-the-only-shell-and-leaves-its-faces-behind: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point, and only the collection this mutation is
/// allowed to touch appears in it at all.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded = decode_semio_brep_diff_json(DIFF).expect("committed delete-shell diff decodes");

    let shells = decoded.shells.as_ref().expect("delete-shell must write the shells triple");
    assert_eq!(shells.removed, vec!["s1".to_string()], "the removal is addressed by shell id");
    assert!(shells.modified.is_empty() && shells.added.is_empty(), "a removal neither modifies nor adds");
    assert!(decoded.vertices.is_none() && decoded.edges.is_none() && decoded.loops.is_none() && decoded.faces.is_none() && decoded.solids.is_none(), "delete-shell cascades nowhere — no other collection may appear");
    let reencoded = pack::json::from_dsl_value(&decoded.to_value());
    let original = pack::json::parse(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "delete-shell/removes-the-only-shell-and-leaves-its-faces-behind: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded = decode_semio_brep_diff_json(DIFF).expect("committed delete-shell diff decodes");
    let produced = decoded.apply(&before()).expect("committed delete-shell diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "delete-shell/removes-the-only-shell-and-leaves-its-faces-behind: committed diff did not carry before to after");
}
