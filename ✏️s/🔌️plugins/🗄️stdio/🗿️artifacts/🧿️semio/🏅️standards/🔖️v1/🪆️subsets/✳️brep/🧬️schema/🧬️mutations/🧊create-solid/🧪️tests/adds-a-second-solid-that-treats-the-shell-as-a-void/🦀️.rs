//! 🧪️ `create-solid` fixture — `adds-a-second-solid-that-treats-the-shell-as-a-void`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`: a duplicate solid id is FATAL
//! `mutation.duplicate-id`, and that is the only guard. `shells` on the payload is a
//! `Vec<BrepSolidShell>` — shell id plus an `is_void` flag distinguishing an internal cavity from
//! the outer boundary — and the second solid takes the same shell as a VOID, so that flag carries
//! the whole semantic difference between the two solids.
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
    serde_json::from_str(BEFORE).expect("create-solid before snapshot decodes")
}
fn expected_after() -> SemioBrepSnapshot {
    serde_json::from_str(AFTER).expect("create-solid after snapshot decodes")
}
fn mutation() -> SemioBrepMutation {
    serde_json::from_str(MUTATION).expect("create-solid mutation decodes")
}

/// ▶️ A second solid appears over the same shell, this time as a void.
#[semio_framework_async_macros::async_test]
async fn adds_the_second_solid_treating_the_shell_as_a_void() {
    let base = before();
    let produced = mutation().diff(&base).diff().apply(&base).expect("create-solid applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "create-solid/adds-a-second-solid-that-treats-the-shell-as-a-void: applied state differs from the committed after-snapshot");
    assert_eq!(produced.solids.len(), base.solids.len() + 1, "create-solid adds exactly one solid");
    let created = produced.solids.last().expect("the created solid is appended — id-keyed collections have no insertion index");
    assert_eq!(created.shells[0].shell, base.solids[0].shells[0].shell, "the new solid deliberately reuses the existing shell");
    assert!(created.shells[0].is_void, "the payload's own is_void flag must land — that is the whole difference from the first solid");
    assert!(!base.solids[0].shells[0].is_void, "the pre-existing solid takes the same shell as its OUTER boundary");
    assert_eq!(produced.shells, base.shells, "creating a solid must not rewrite the shell it references");
}

/// ↩️ `create-solid`'s undo is a single `delete-solid` for the same id.
#[semio_framework_async_macros::async_test]
async fn the_undo_delete_solid_removes_the_second_solid_again() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "create-solid undoes as exactly one delete-solid");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward create-solid applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo delete-solid applies");
    }
    assert_eq!(current, base, "create-solid/adds-a-second-solid-that-treats-the-shell-as-a-void: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the payload are canonical — a solid member encodes as `{"shell":…,"isVoid":…}` (camelCase on the ENTITY) while the payload key stays `shells`.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioBrepSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "create-solid/adds-a-second-solid-that-treats-the-shell-as-a-void: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("create-solid mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("create-solid mutation reparses");
    assert_eq!(reencoded, original, "create-solid/adds-a-second-solid-that-treats-the-shell-as-a-void: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the solid id is free, so the FATAL mutation.duplicate-id branch must not fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_as_committed() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "create-solid/adds-a-second-solid-that-treats-the-shell-as-a-void: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "creating a solid with a fresh id must raise no diagnostics");
}

/// 🔺️ The sparse delta this mutation produces equals the committed diff. Only `solids.added`.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioBrepMutation as Mutation<SemioBrepSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "create-solid/adds-a-second-solid-that-treats-the-shell-as-a-void: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point, and only the collection this mutation is
/// allowed to touch appears in it at all.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded: SemioBrepDiff = serde_json::from_str(DIFF).expect("committed create-solid diff decodes");

    let solids = decoded.solids.as_ref().expect("create-solid must write the solids triple");
    assert_eq!(solids.added.len(), 1, "exactly one solid is added");
    assert!(solids.removed.is_empty() && solids.modified.is_empty(), "a create neither removes nor modifies");
    assert!(decoded.vertices.is_none() && decoded.edges.is_none() && decoded.loops.is_none() && decoded.faces.is_none() && decoded.shells.is_none(), "no other collection may appear in the diff");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "create-solid/adds-a-second-solid-that-treats-the-shell-as-a-void: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioBrepDiff = serde_json::from_str(DIFF).expect("committed create-solid diff decodes");
    let produced = decoded.apply(&before()).expect("committed create-solid diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "create-solid/adds-a-second-solid-that-treats-the-shell-as-a-void: committed diff did not carry before to after");
}
