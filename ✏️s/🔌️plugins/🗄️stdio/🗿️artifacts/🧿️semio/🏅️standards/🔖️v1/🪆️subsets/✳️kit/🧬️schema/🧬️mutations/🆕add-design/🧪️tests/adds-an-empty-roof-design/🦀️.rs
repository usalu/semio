//! 🧪️ `add-design` fixture — `adds-an-empty-roof-design`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`: a duplicate design id is FATAL
//! `mutation.duplicate-id`; otherwise a `SemioKitDesign` with EMPTY `pieces` and `connections` is
//! pushed. The payload carries only `id`/`name` — content arrives later through `edit-design` —
//! so this case pins that a freshly added design starts empty rather than cloned from anything.

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
    serde_json::from_str(BEFORE).expect("add-design before snapshot decodes")
}
fn expected_after() -> SemioKitSnapshot {
    serde_json::from_str(AFTER).expect("add-design after snapshot decodes")
}
fn mutation() -> SemioKitMutation {
    serde_json::from_str(MUTATION).expect("add-design mutation decodes")
}

/// ▶️ A second, empty design appears; the existing one keeps its pieces.
#[semio_framework_async_macros::async_test]
async fn adds_the_roof_design_with_no_pieces_and_no_connections() {
    let base = before();
    let produced = mutation().diff(&base).diff().apply(&base).expect("add-design applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "add-design/adds-an-empty-roof-design: applied state differs from the committed after-snapshot");
    assert_eq!(produced.designs.len(), base.designs.len() + 1, "add-design adds exactly one design");
    let created = produced.designs.last().expect("the new design is pushed at the end");
    assert_eq!(created.name, "Roof", "the design carries the payload's name");
    assert!(created.pieces.is_empty() && created.connections.is_empty(), "a freshly added design starts EMPTY — content comes later via edit-design");
    assert_eq!(produced.designs[0], base.designs[0], "the pre-existing design keeps its own pieces untouched");
}

/// ↩️ `add-design`'s undo is a single `remove-design` for the same id.
#[semio_framework_async_macros::async_test]
async fn the_undo_remove_design_takes_the_roof_design_back_out() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "add-design undoes as exactly one remove-design");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward add-design applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo remove-design applies");
    }
    assert_eq!(current, base, "add-design/adds-an-empty-roof-design: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the `{"AddDesign":{"id":"d2","name":"Roof"}}` payload are canonical — a design piece spells its type reference `typeId` and a connection spells `connectingPieceId`/`connectedPort`, all camelCase.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioKitSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "add-design/adds-an-empty-roof-design: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("add-design mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("add-design mutation reparses");
    assert_eq!(reencoded, original, "add-design/adds-an-empty-roof-design: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: no design with id d2 exists, so the FATAL mutation.duplicate-id branch must not fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_as_committed() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "add-design/adds-an-empty-roof-design: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "adding a design with a fresh id must raise no diagnostics");
}

/// 🔺️ The sparse delta this mutation produces equals the committed diff. Only the `designs` slot.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioKitMutation as Mutation<SemioKitSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "add-design/adds-an-empty-roof-design: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point, and only the slot this mutation is
/// allowed to touch appears in it at all.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded: SemioKitDiff = serde_json::from_str(DIFF).expect("committed add-design diff decodes");
    assert_eq!(decoded.designs.as_ref().map(|list| list.values.len()), Some(2), "the diff carries the whole rebuilt design list");
    assert!(decoded.types.is_none() && decoded.objects.is_none() && decoded.models.is_none() && decoded.properties.is_none() && decoded.representations.is_none(), "no other kit slot may appear in the diff");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "add-design/adds-an-empty-roof-design: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioKitDiff = serde_json::from_str(DIFF).expect("committed add-design diff decodes");
    let produced = decoded.apply(&before()).expect("committed add-design diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "add-design/adds-an-empty-roof-design: committed diff did not carry before to after");
}
