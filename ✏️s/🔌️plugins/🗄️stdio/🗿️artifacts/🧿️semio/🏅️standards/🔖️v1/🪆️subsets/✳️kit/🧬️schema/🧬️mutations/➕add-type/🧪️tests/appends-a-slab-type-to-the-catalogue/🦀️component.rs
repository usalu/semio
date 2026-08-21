//! 🧪️ `add-type` fixture — `appends-a-slab-type-to-the-catalogue`.
//!
//! Transcribed from `../../🔺️diff/🦀️component.rs`: a duplicate type id is FATAL
//! `mutation.duplicate-id`; otherwise the whole `types` list is rebuilt from `base` with the new
//! `SemioKitType` PUSHED at the end, and every other one of the six kit slots is left `None`.
//! `SemioKitDiff` is a per-field diff over six independent slots, so "types and only types" is a
//! claim the committed diff can actually carry.

use crate::artifacts::semio::standards::v1::subsets::kit::schema::diff::SemioKitDiff;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::mutations::SemioKitMutation;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> SemioKitSnapshot {
    serde_json::from_str(BEFORE).expect("add-type before snapshot decodes")
}
fn expected_after() -> SemioKitSnapshot {
    serde_json::from_str(AFTER).expect("add-type after snapshot decodes")
}
fn mutation() -> SemioKitMutation {
    serde_json::from_str(MUTATION).expect("add-type mutation decodes")
}

/// ▶️ A third type appears at the end of the catalogue; nothing else in the kit moves.
#[semio_framework_async_macros::async_test]
async fn appends_the_slab_type_at_the_end_of_the_catalogue() {
    let base = before();
    let produced = mutation().diff(&base).diff().apply(&base).expect("add-type applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "add-type/appends-a-slab-type-to-the-catalogue: applied state differs from the committed after-snapshot");
    assert_eq!(produced.types.len(), base.types.len() + 1, "add-type adds exactly one type");
    assert_eq!(produced.types.last().expect("the new type is pushed at the end").id, "t3", "the type keeps the id the payload named");
    assert_eq!(&produced.types[..base.types.len()], &base.types[..], "the pre-existing types must be byte-identical and keep their order");
    assert_eq!((produced.designs, produced.objects, produced.models), (base.designs, base.objects, base.models), "adding a type touches no design and no child collection");
}

/// ↩️ `add-type`'s undo is a single `remove-type` for the same id.
#[semio_framework_async_macros::async_test]
async fn the_undo_remove_type_takes_the_slab_type_back_out() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "add-type undoes as exactly one remove-type");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward add-type applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo remove-type applies");
    }
    assert_eq!(current, base, "add-type/appends-a-slab-type-to-the-catalogue: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the `{"AddType":{"id":"t3","name":"Slab","category":"structure"}}` payload are canonical — the kit snapshot always emits `types`/`designs`/`objects`/`models`/`representations` (no `skip_serializing_if`) but omits `properties` when unset.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioKitSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "add-type/appends-a-slab-type-to-the-catalogue: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("add-type mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("add-type mutation reparses");
    assert_eq!(reencoded, original, "add-type/appends-a-slab-type-to-the-catalogue: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: no type with id t3 exists, so the FATAL mutation.duplicate-id branch must not fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_as_committed() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "add-type/appends-a-slab-type-to-the-catalogue: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "adding a type with a fresh id must raise no diagnostics");
}

/// 🔺️ The sparse delta this mutation produces equals the committed diff. Only the `types` slot.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioKitMutation as Mutation<SemioKitSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "add-type/appends-a-slab-type-to-the-catalogue: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point, and only the slot this mutation is
/// allowed to touch appears in it at all.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded: SemioKitDiff = serde_json::from_str(DIFF).expect("committed add-type diff decodes");
    assert_eq!(decoded.types.as_ref().map(|list| list.values.len()), Some(3), "the diff carries the whole rebuilt type catalogue");
    assert!(decoded.designs.is_none() && decoded.objects.is_none() && decoded.models.is_none() && decoded.properties.is_none() && decoded.representations.is_none(), "no other kit slot may appear in the diff");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "add-type/appends-a-slab-type-to-the-catalogue: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioKitDiff = serde_json::from_str(DIFF).expect("committed add-type diff decodes");
    let produced = decoded.apply(&before()).expect("committed add-type diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "add-type/appends-a-slab-type-to-the-catalogue: committed diff did not carry before to after");
}
