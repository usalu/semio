//! 🧪️ `create-face` fixture — `adds-an-opposing-face-over-the-same-loop`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`: a duplicate face id is FATAL
//! `mutation.duplicate-id`, and that is the only guard — the referenced `outer_loop` is NOT
//! validated here. The new face reuses the existing loop with an opposite surface normal and
//! `orientation: false`, which is what makes `orientation` a load-bearing payload field rather
//! than a default.
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
    serde_json::from_str(BEFORE).expect("create-face before snapshot decodes")
}
fn expected_after() -> SemioBrepSnapshot {
    serde_json::from_str(AFTER).expect("create-face after snapshot decodes")
}
fn mutation() -> SemioBrepMutation {
    serde_json::from_str(MUTATION).expect("create-face mutation decodes")
}

/// ▶️ A second face appears over the SAME loop, with the opposite sense.
#[semio_framework_async_macros::async_test]
async fn adds_the_opposing_face_over_the_existing_loop() {
    let base = before();
    let produced = mutation().diff(&base).diff().apply(&base).expect("create-face applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "create-face/adds-an-opposing-face-over-the-same-loop: applied state differs from the committed after-snapshot");
    assert_eq!(produced.faces.len(), base.faces.len() + 1, "create-face adds exactly one face");
    let created = produced.faces.last().expect("the created face is appended — id-keyed collections have no insertion index");
    assert_eq!(created.outer_loop, base.faces[0].outer_loop, "the new face deliberately reuses the existing loop");
    assert!(!created.orientation, "the payload's own orientation flag must land, not a default");
    assert!(created.inner_loops.is_empty(), "an empty inner_loops vector round-trips as an empty array, never as a missing key");
    assert_eq!(produced.loops, base.loops, "creating a face must not rewrite the loop it references");
}

/// ↩️ `create-face`'s undo is a single `delete-face` for the same id.
#[semio_framework_async_macros::async_test]
async fn the_undo_delete_face_removes_the_opposing_face_again() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "create-face undoes as exactly one delete-face");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward create-face applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo delete-face applies");
    }
    assert_eq!(current, base, "create-face/adds-an-opposing-face-over-the-same-loop: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the payload are canonical — the ENTITY spells `outerLoop`/`innerLoops`, the PAYLOAD spells `outer_loop`/`inner_loops`, and `BrepSurface` is internally tagged on `kind`.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioBrepSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "create-face/adds-an-opposing-face-over-the-same-loop: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("create-face mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("create-face mutation reparses");
    assert_eq!(reencoded, original, "create-face/adds-an-opposing-face-over-the-same-loop: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the face id is free, so the FATAL mutation.duplicate-id branch must not fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_as_committed() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "create-face/adds-an-opposing-face-over-the-same-loop: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "creating a face with a fresh id must raise no diagnostics");
}

/// 🔺️ The sparse delta this mutation produces equals the committed diff. Only `faces.added`.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioBrepMutation as Mutation<SemioBrepSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "create-face/adds-an-opposing-face-over-the-same-loop: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point, and only the collection this mutation is
/// allowed to touch appears in it at all.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded: SemioBrepDiff = serde_json::from_str(DIFF).expect("committed create-face diff decodes");

    let faces = decoded.faces.as_ref().expect("create-face must write the faces triple");
    assert_eq!(faces.added.len(), 1, "exactly one face is added");
    assert!(faces.removed.is_empty() && faces.modified.is_empty(), "a create neither removes nor modifies");
    assert!(decoded.vertices.is_none() && decoded.edges.is_none() && decoded.loops.is_none() && decoded.shells.is_none() && decoded.solids.is_none(), "no other collection may appear in the diff");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "create-face/adds-an-opposing-face-over-the-same-loop: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioBrepDiff = serde_json::from_str(DIFF).expect("committed create-face diff decodes");
    let produced = decoded.apply(&before()).expect("committed create-face diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "create-face/adds-an-opposing-face-over-the-same-loop: committed diff did not carry before to after");
}
