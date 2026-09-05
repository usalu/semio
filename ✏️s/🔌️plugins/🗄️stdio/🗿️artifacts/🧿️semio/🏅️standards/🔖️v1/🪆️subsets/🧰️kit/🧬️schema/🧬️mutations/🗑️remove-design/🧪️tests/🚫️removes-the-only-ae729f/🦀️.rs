//! 🧪️ `remove-design` fixture — `🚫️removes-the-only-design-together-with-its-pieces`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`: an unknown design id is Error
//! `mutation.target-missing`; otherwise the `designs` list is rebuilt by filtering that id out —
//! which takes the design's pieces and connections with it, since they live INSIDE the design.
//! That is why `↩️inverse/🦀️.rs` needs TWO steps: `add-design` only restores the empty
//! shell, so an `edit-design` carrying the captured content has to follow it.

use crate::artifacts::semio::standards::v1::subsets::kit::schema::diff::SemioKitDiff;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::mutations::SemioKitMutation;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> SemioKitSnapshot {
    serde_json::from_str(BEFORE).expect("remove-design before snapshot decodes")
}
fn expected_after() -> SemioKitSnapshot {
    serde_json::from_str(AFTER).expect("remove-design after snapshot decodes")
}
fn mutation() -> SemioKitMutation {
    serde_json::from_str(MUTATION).expect("remove-design mutation decodes")
}

/// ▶️ The design and everything nested inside it disappear together.
#[semio_framework_async_macros::async_test]
async fn removes_the_design_and_the_pieces_nested_inside_it() {
    let base = before();
    assert!(!base.designs[0].pieces.is_empty(), "the fixture needs a non-empty design for the two-step inverse to matter");
    let produced = mutation().diff(&base).diff().apply(&base).expect("remove-design applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "remove-design/removes-the-only-design-together-with-its-pieces: applied state differs from the committed after-snapshot");
    assert!(produced.designs.is_empty(), "the only design must be gone");
    assert_eq!(produced.types, base.types, "removing a design must not touch the type catalogue it referenced");
}

/// ↩️ The undo is TWO steps — `add-design` recreates the empty shell, `edit-design` refills it.
#[semio_framework_async_macros::async_test]
async fn the_undo_readds_the_design_then_refills_its_content() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 2, "an empty add-design alone would lose the pieces — the inverse needs an edit-design too");
    assert!(matches!(undo[0], SemioKitMutation::AddDesign(_)), "the shell must be recreated first");
    let SemioKitMutation::EditDesign(refill) = &undo[1] else { panic!("the second undo step must be edit-design") };
    assert_eq!(refill.pieces, base.designs[0].pieces, "the refill must recapture the removed design's own pieces");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward remove-design applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("each undo step applies to the running state");
    }
    assert_eq!(current, base, "remove-design/removes-the-only-design-together-with-its-pieces: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the `{"RemoveDesign":{"id":"d1"}}` payload are canonical fixed points.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioKitSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "remove-design/removes-the-only-design-together-with-its-pieces: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("remove-design mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("remove-design mutation reparses");
    assert_eq!(reencoded, original, "remove-design/removes-the-only-design-together-with-its-pieces: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the design exists, so mutation.target-missing must not fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_as_committed() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "remove-design/removes-the-only-design-together-with-its-pieces: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "removing an existing design must raise no diagnostics");
}

/// 🔺️ The sparse delta this mutation produces equals the committed diff. Only the `designs` slot, carrying the emptied list.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioKitMutation as Mutation<SemioKitSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "remove-design/removes-the-only-design-together-with-its-pieces: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point, and only the slot this mutation is
/// allowed to touch appears in it at all.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded: SemioKitDiff = serde_json::from_str(DIFF).expect("committed remove-design diff decodes");
    assert_eq!(decoded.designs.as_ref().map(|list| list.values.len()), Some(0), "the diff carries the emptied design list, not a removal marker");
    assert!(decoded.types.is_none() && decoded.objects.is_none() && decoded.models.is_none() && decoded.properties.is_none() && decoded.representations.is_none(), "no other kit slot may appear in the diff");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "remove-design/removes-the-only-design-together-with-its-pieces: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioKitDiff = serde_json::from_str(DIFF).expect("committed remove-design diff decodes");
    let produced = decoded.apply(&before()).expect("committed remove-design diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "remove-design/removes-the-only-design-together-with-its-pieces: committed diff did not carry before to after");
}
