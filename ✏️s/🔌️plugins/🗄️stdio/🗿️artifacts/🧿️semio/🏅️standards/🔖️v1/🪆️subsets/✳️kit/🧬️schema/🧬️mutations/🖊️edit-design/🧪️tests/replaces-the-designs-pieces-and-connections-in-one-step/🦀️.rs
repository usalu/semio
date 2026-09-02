//! 🧪️ `edit-design` fixture — `replaces-the-designs-pieces-and-connections-in-one-step`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`: unknown id ⇒ Error `mutation.target-missing`;
//! pieces AND connections both already equal ⇒ Warning `mutation.no-op` (the guard is a
//! conjunction, so changing either one alone still applies). Otherwise BOTH nested collections are
//! replaced wholesale on the matched design — a design's content is edited as one value, never
//! piece-by-piece, and `id`/`name` are left alone.

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
    serde_json::from_str(BEFORE).expect("edit-design before snapshot decodes")
}
fn expected_after() -> SemioKitSnapshot {
    serde_json::from_str(AFTER).expect("edit-design after snapshot decodes")
}
fn mutation() -> SemioKitMutation {
    serde_json::from_str(MUTATION).expect("edit-design mutation decodes")
}

/// ▶️ Both nested collections are replaced at once; the design keeps its identity and name.
#[semio_framework_async_macros::async_test]
async fn replaces_both_nested_collections_and_keeps_the_designs_identity() {
    let base = before();
    let produced = mutation().diff(&base).diff().apply(&base).expect("edit-design applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "edit-design/replaces-the-designs-pieces-and-connections-in-one-step: applied state differs from the committed after-snapshot");
    assert_eq!(produced.designs[0].pieces.len(), 2, "the payload's piece list replaces the design's own wholesale");
    assert_eq!(produced.designs[0].connections.len(), 1, "the payload's connection list lands in the same step");
    assert_eq!(produced.designs[0].id, base.designs[0].id, "edit-design must not re-key the design");
    assert_eq!(produced.designs[0].name, base.designs[0].name, "edit-design must not rename the design");
    assert_eq!(produced.types, base.types, "editing a design must not touch the type catalogue its pieces reference");
}

/// ↩️ The undo is an `edit-design` carrying BASE's captured pieces and connections.
#[semio_framework_async_macros::async_test]
async fn the_undo_edit_design_restores_the_captured_content() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "edit-design of an existing design undoes as exactly one edit-design");
    let SemioKitMutation::EditDesign(restore) = &undo[0] else { panic!("edit-design must undo as edit-design") };
    assert_eq!(restore.pieces, base.designs[0].pieces, "the undo must recapture BASE's own pieces");
    assert_eq!(restore.connections, base.designs[0].connections, "the undo must recapture BASE's own connections");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward edit-design applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo edit-design applies");
    }
    assert_eq!(current, base, "edit-design/replaces-the-designs-pieces-and-connections-in-one-step: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the payload are canonical — the payload embeds whole `SemioKitPiece`/`SemioKitConnection` values, so their camelCase field spellings appear inside a mutation whose own keys are snake_case.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioKitSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "edit-design/replaces-the-designs-pieces-and-connections-in-one-step: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("edit-design mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("edit-design mutation reparses");
    assert_eq!(reencoded, original, "edit-design/replaces-the-designs-pieces-and-connections-in-one-step: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the design exists and its content genuinely differs, so neither target-missing nor the conjunction no-op may fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_as_committed() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "edit-design/replaces-the-designs-pieces-and-connections-in-one-step: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "replacing a design content with genuinely different content must raise no diagnostics");
}

/// 🔺️ The sparse delta this mutation produces equals the committed diff. Only the `designs` slot.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioKitMutation as Mutation<SemioKitSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "edit-design/replaces-the-designs-pieces-and-connections-in-one-step: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point, and only the slot this mutation is
/// allowed to touch appears in it at all.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded: SemioKitDiff = serde_json::from_str(DIFF).expect("committed edit-design diff decodes");
    let designs = decoded.designs.as_ref().expect("edit-design must write the designs slot");
    assert_eq!(designs.values[0].pieces.len(), 2, "the diff itself must already carry the new piece list");
    assert_eq!(designs.values[0].connections.len(), 1, "and the new connection list");
    assert!(decoded.types.is_none() && decoded.objects.is_none() && decoded.models.is_none() && decoded.properties.is_none() && decoded.representations.is_none(), "no other kit slot may appear in the diff");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "edit-design/replaces-the-designs-pieces-and-connections-in-one-step: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioKitDiff = serde_json::from_str(DIFF).expect("committed edit-design diff decodes");
    let produced = decoded.apply(&before()).expect("committed edit-design diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "edit-design/replaces-the-designs-pieces-and-connections-in-one-step: committed diff did not carry before to after");
}
