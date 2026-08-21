//! 🧪️ `create-shell` fixture — `adds-a-second-shell-that-reuses-the-face-with-flipped-sense`.
//!
//! Transcribed from `../../🔺️diff/🦀️component.rs`: a duplicate shell id is FATAL
//! `mutation.duplicate-id`, and that is the only guard. `faces` on the payload is a
//! `Vec<BrepShellFace>` — face id PLUS traversal orientation, a named weak struct rather than a
//! bare `(String, bool)` — and the second shell reuses the same face with the opposite sense, so
//! the orientation flag is doing real work here.
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
    serde_json::from_str(BEFORE).expect("create-shell before snapshot decodes")
}
fn expected_after() -> SemioBrepSnapshot {
    serde_json::from_str(AFTER).expect("create-shell after snapshot decodes")
}
fn mutation() -> SemioBrepMutation {
    serde_json::from_str(MUTATION).expect("create-shell mutation decodes")
}

/// ▶️ A second shell appears over the same face, with the opposite sense.
#[semio_framework_async_macros::async_test]
async fn adds_the_second_shell_over_the_existing_face() {
    let base = before();
    let produced = mutation().diff(&base).diff().apply(&base).expect("create-shell applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "create-shell/adds-a-second-shell-that-reuses-the-face-with-flipped-sense: applied state differs from the committed after-snapshot");
    assert_eq!(produced.shells.len(), base.shells.len() + 1, "create-shell adds exactly one shell");
    let created = produced.shells.last().expect("the created shell is appended — id-keyed collections have no insertion index");
    assert_eq!(created.faces[0].face, base.shells[0].faces[0].face, "the new shell deliberately reuses the existing face");
    assert_ne!(created.faces[0].orientation, base.shells[0].faces[0].orientation, "with the opposite traversal sense — that flag is a real payload field");
    assert_eq!(produced.faces, base.faces, "creating a shell must not rewrite the face it references");
}

/// ↩️ `create-shell`'s undo is a single `delete-shell` for the same id.
#[semio_framework_async_macros::async_test]
async fn the_undo_delete_shell_removes_the_second_shell_again() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "create-shell undoes as exactly one delete-shell");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward create-shell applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo delete-shell applies");
    }
    assert_eq!(current, base, "create-shell/adds-a-second-shell-that-reuses-the-face-with-flipped-sense: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the payload are canonical — a shell member encodes as `{"face":…,"orientation":…}`, never as a bare pair.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioBrepSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "create-shell/adds-a-second-shell-that-reuses-the-face-with-flipped-sense: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("create-shell mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("create-shell mutation reparses");
    assert_eq!(reencoded, original, "create-shell/adds-a-second-shell-that-reuses-the-face-with-flipped-sense: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the shell id is free, so the FATAL mutation.duplicate-id branch must not fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_as_committed() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "create-shell/adds-a-second-shell-that-reuses-the-face-with-flipped-sense: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "creating a shell with a fresh id must raise no diagnostics");
}

/// 🔺️ The sparse delta this mutation produces equals the committed diff. Only `shells.added`.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioBrepMutation as Mutation<SemioBrepSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "create-shell/adds-a-second-shell-that-reuses-the-face-with-flipped-sense: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point, and only the collection this mutation is
/// allowed to touch appears in it at all.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded: SemioBrepDiff = serde_json::from_str(DIFF).expect("committed create-shell diff decodes");

    let shells = decoded.shells.as_ref().expect("create-shell must write the shells triple");
    assert_eq!(shells.added.len(), 1, "exactly one shell is added");
    assert!(shells.removed.is_empty() && shells.modified.is_empty(), "a create neither removes nor modifies");
    assert!(decoded.vertices.is_none() && decoded.edges.is_none() && decoded.loops.is_none() && decoded.faces.is_none() && decoded.solids.is_none(), "no other collection may appear in the diff");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "create-shell/adds-a-second-shell-that-reuses-the-face-with-flipped-sense: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioBrepDiff = serde_json::from_str(DIFF).expect("committed create-shell diff decodes");
    let produced = decoded.apply(&before()).expect("committed create-shell diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "create-shell/adds-a-second-shell-that-reuses-the-face-with-flipped-sense: committed diff did not carry before to after");
}
