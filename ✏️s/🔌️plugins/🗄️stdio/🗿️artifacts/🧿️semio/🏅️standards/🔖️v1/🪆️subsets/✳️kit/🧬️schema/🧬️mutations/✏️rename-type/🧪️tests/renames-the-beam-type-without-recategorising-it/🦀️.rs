//! 🧪️ `rename-type` fixture — `renames-the-beam-type-without-recategorising-it`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`: unknown id ⇒ Error `mutation.target-missing`,
//! a name already equal ⇒ Warning `mutation.no-op`. Otherwise ONLY `name` is assigned on the
//! matched type — `id` stays the identity (this is a rename of the DISPLAY name, not a re-keying)
//! and `category` is explicitly untouched.

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
    serde_json::from_str(BEFORE).expect("rename-type before snapshot decodes")
}
fn expected_after() -> SemioKitSnapshot {
    serde_json::from_str(AFTER).expect("rename-type after snapshot decodes")
}
fn mutation() -> SemioKitMutation {
    serde_json::from_str(MUTATION).expect("rename-type mutation decodes")
}

/// ▶️ The display name changes; the id that everything else references and the category do not.
#[semio_framework_async_macros::async_test]
async fn renames_the_display_name_and_keeps_id_and_category() {
    let base = before();
    let produced = mutation().diff(&base).diff().apply(&base).expect("rename-type applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "rename-type/renames-the-beam-type-without-recategorising-it: applied state differs from the committed after-snapshot");
    assert_eq!(produced.types[0].name, "Girder", "the type's display name must become new_name");
    assert_eq!(produced.types[0].id, base.types[0].id, "a rename must NOT re-key the type — pieces reference it by id");
    assert_eq!(produced.types[0].category, base.types[0].category, "a rename must not recategorise the type");
    assert_eq!(produced.types[1], base.types[1], "the untargeted type must be byte-identical");
}

/// ↩️ The undo is a `rename-type` carrying BASE's captured name.
#[semio_framework_async_macros::async_test]
async fn the_undo_rename_type_restores_the_original_name() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "rename-type of an existing type undoes as exactly one rename-type");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward rename-type applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo rename-type applies");
    }
    assert_eq!(current, base, "rename-type/renames-the-beam-type-without-recategorising-it: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the `{"RenameType":{"id":"t1","new_name":"Girder"}}` payload are canonical — the payload field stays snake_case because kit payload structs carry no `rename_all`.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioKitSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "rename-type/renames-the-beam-type-without-recategorising-it: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("rename-type mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("rename-type mutation reparses");
    assert_eq!(reencoded, original, "rename-type/renames-the-beam-type-without-recategorising-it: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the type exists and the new name genuinely differs, so neither target-missing nor no-op may fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_as_committed() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "rename-type/renames-the-beam-type-without-recategorising-it: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "renaming to a genuinely different name must raise no diagnostics");
}

/// 🔺️ The sparse delta this mutation produces equals the committed diff. Only the `types` slot.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioKitMutation as Mutation<SemioKitSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "rename-type/renames-the-beam-type-without-recategorising-it: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point, and only the slot this mutation is
/// allowed to touch appears in it at all.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded: SemioKitDiff = serde_json::from_str(DIFF).expect("committed rename-type diff decodes");
    let types = decoded.types.as_ref().expect("rename-type must write the types slot");
    assert_eq!(types.values[0].name, "Girder", "the diff itself must already carry the renamed type");
    assert_eq!(types.values.len(), 2, "a rename never changes how many types there are");
    assert!(decoded.designs.is_none() && decoded.objects.is_none() && decoded.models.is_none() && decoded.properties.is_none() && decoded.representations.is_none(), "no other kit slot may appear in the diff");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "rename-type/renames-the-beam-type-without-recategorising-it: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioKitDiff = serde_json::from_str(DIFF).expect("committed rename-type diff decodes");
    let produced = decoded.apply(&before()).expect("committed rename-type diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "rename-type/renames-the-beam-type-without-recategorising-it: committed diff did not carry before to after");
}
