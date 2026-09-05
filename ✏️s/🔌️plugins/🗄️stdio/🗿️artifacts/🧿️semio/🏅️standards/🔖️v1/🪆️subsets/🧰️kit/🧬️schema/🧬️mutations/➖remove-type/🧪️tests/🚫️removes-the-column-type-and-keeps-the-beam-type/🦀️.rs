//! 🧪️ `remove-type` fixture — `🚫️removes-the-column-type-and-keeps-the-beam-type`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`: an unknown type id is Error
//! `mutation.target-missing`; otherwise the `types` list is rebuilt by FILTERING that id out, which
//! preserves the order of everything else. There is deliberately no cascade — a design piece whose
//! `typeId` names the removed type keeps naming it, and the diff proves that by never touching
//! `designs`.

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
    serde_json::from_str(BEFORE).expect("remove-type before snapshot decodes")
}
fn expected_after() -> SemioKitSnapshot {
    serde_json::from_str(AFTER).expect("remove-type after snapshot decodes")
}
fn mutation() -> SemioKitMutation {
    serde_json::from_str(MUTATION).expect("remove-type mutation decodes")
}

/// ▶️ Only the column type goes; the beam type and every design keep their place.
#[semio_framework_async_macros::async_test]
async fn removes_the_column_type_without_cascading_into_designs() {
    let base = before();
    let produced = mutation().diff(&base).diff().apply(&base).expect("remove-type applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "remove-type/removes-the-column-type-and-keeps-the-beam-type: applied state differs from the committed after-snapshot");
    assert!(!produced.types.iter().any(|kind| kind.id == "t2"), "the named type must be gone");
    assert_eq!(produced.types, vec![base.types[0].clone()], "the surviving type keeps its position");
    assert_eq!(produced.designs, base.designs, "remove-type must NOT cascade into the designs whose pieces reference a type");
}

/// ↩️ The undo re-adds the type with its captured name AND category.
#[semio_framework_async_macros::async_test]
async fn the_undo_add_type_restores_the_full_captured_type() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "remove-type of an existing type undoes as exactly one add-type");
    let SemioKitMutation::AddType(readd) = &undo[0] else { panic!("remove-type must undo as add-type") };
    assert_eq!((readd.name.as_str(), readd.category.as_str()), ("Column", "structure"), "the undo must recapture the removed type's own name and category from base");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward remove-type applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo add-type applies");
    }
    assert_eq!(current, base, "remove-type/removes-the-column-type-and-keeps-the-beam-type: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the `{"RemoveType":{"id":"t2"}}` payload are canonical fixed points.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioKitSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "remove-type/removes-the-column-type-and-keeps-the-beam-type: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("remove-type mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("remove-type mutation reparses");
    assert_eq!(reencoded, original, "remove-type/removes-the-column-type-and-keeps-the-beam-type: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the type exists, so mutation.target-missing must not fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_as_committed() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "remove-type/removes-the-column-type-and-keeps-the-beam-type: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "removing an existing type must raise no diagnostics — this leaf has no cascade to report");
}

/// 🔺️ The sparse delta this mutation produces equals the committed diff. Only the `types` slot, carrying the SHORTENED list.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioKitMutation as Mutation<SemioKitSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "remove-type/removes-the-column-type-and-keeps-the-beam-type: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point, and only the slot this mutation is
/// allowed to touch appears in it at all.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded: SemioKitDiff = serde_json::from_str(DIFF).expect("committed remove-type diff decodes");
    assert_eq!(decoded.types.as_ref().map(|list| list.values.len()), Some(1), "the diff carries the shortened type catalogue, not a removal marker");
    assert!(decoded.designs.is_none() && decoded.objects.is_none() && decoded.models.is_none() && decoded.properties.is_none() && decoded.representations.is_none(), "no other kit slot may appear in the diff");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "remove-type/removes-the-column-type-and-keeps-the-beam-type: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioKitDiff = serde_json::from_str(DIFF).expect("committed remove-type diff decodes");
    let produced = decoded.apply(&before()).expect("committed remove-type diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "remove-type/removes-the-column-type-and-keeps-the-beam-type: committed diff did not carry before to after");
}
