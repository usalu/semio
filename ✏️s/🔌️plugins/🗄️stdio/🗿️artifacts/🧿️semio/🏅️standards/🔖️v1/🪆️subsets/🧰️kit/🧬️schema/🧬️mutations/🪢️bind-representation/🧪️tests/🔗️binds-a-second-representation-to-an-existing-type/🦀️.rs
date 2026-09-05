//! 🧪️ `bind-representation` fixture — `🔗️binds-a-second-representation-to-an-existing-type`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`: `role` must name an EXISTING type or the
//! outcome is Error `mutation.target-missing` (the role is a type id, not free text); an identical
//! link already present is Warning `mutation.no-op` (whole-value `contains`, target + pin + role);
//! otherwise the new `store::ArtifactLink` is pushed onto `representations`. A LINK is not a child:
//! it carries a `pin`, has an independent lifecycle, and never nests inline.

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
    serde_json::from_str(BEFORE).expect("bind-representation before snapshot decodes")
}
fn expected_after() -> SemioKitSnapshot {
    serde_json::from_str(AFTER).expect("bind-representation after snapshot decodes")
}
fn mutation() -> SemioKitMutation {
    serde_json::from_str(MUTATION).expect("bind-representation mutation decodes")
}

/// ▶️ A second link appears for the same role, pointing at a different target.
#[semio_framework_async_macros::async_test]
async fn binds_the_second_representation_for_an_existing_type_role() {
    let base = before();
    let produced = mutation().diff(&base).diff().apply(&base).expect("bind-representation applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "bind-representation/binds-a-second-representation-to-an-existing-type: applied state differs from the committed after-snapshot");
    assert_eq!(produced.representations.len(), base.representations.len() + 1, "bind-representation adds exactly one link");
    let bound = produced.representations.last().expect("the new link is pushed at the end");
    assert!(produced.types.iter().any(|kind| kind.id == bound.role), "the role must name a real type — that is the leaf's own target-missing guard");
    assert_ne!(bound.target, base.representations[0].target, "the second link points somewhere else — two representations may share one role");
    assert_eq!(produced.types, base.types, "binding a representation must not touch the type catalogue it references");
}

/// ↩️ The undo is an `unbind-representation` at the index the new link landed at — which is
/// `base.representations.len()`, i.e. the FINAL-state position.
#[semio_framework_async_macros::async_test]
async fn the_undo_unbind_representation_targets_the_appended_index() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    let SemioKitMutation::UnbindRepresentation(unbind) = &undo[0] else { panic!("bind-representation must undo as unbind-representation") };
    assert_eq!(unbind.index, base.representations.len(), "the undo addresses the index the link was appended AT, not the base length minus one");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward bind-representation applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo unbind-representation applies");
    }
    assert_eq!(current, base, "bind-representation/binds-a-second-representation-to-an-existing-type: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the payload are canonical — `LinkPin` is internally tagged on `kind`, so an unpinned link encodes as `{"kind":"head"}` rather than a bare string.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioKitSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "bind-representation/binds-a-second-representation-to-an-existing-type: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("bind-representation mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("bind-representation mutation reparses");
    assert_eq!(reencoded, original, "bind-representation/binds-a-second-representation-to-an-existing-type: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the role names an existing type and no identical link is present, so neither target-missing nor no-op may fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_as_committed() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "bind-representation/binds-a-second-representation-to-an-existing-type: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "binding a genuinely new representation to an existing role must raise no diagnostics");
}

/// 🔺️ The sparse delta this mutation produces equals the committed diff. Only the `representations` slot.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioKitMutation as Mutation<SemioKitSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "bind-representation/binds-a-second-representation-to-an-existing-type: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point, and only the slot this mutation is
/// allowed to touch appears in it at all.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded: SemioKitDiff = serde_json::from_str(DIFF).expect("committed bind-representation diff decodes");
    assert_eq!(decoded.representations.as_ref().map(|list| list.values.len()), Some(2), "the diff carries the whole rebuilt link list");
    assert!(decoded.types.is_none() && decoded.designs.is_none() && decoded.objects.is_none() && decoded.models.is_none() && decoded.properties.is_none(), "no other kit slot may appear in the diff");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "bind-representation/binds-a-second-representation-to-an-existing-type: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioKitDiff = serde_json::from_str(DIFF).expect("committed bind-representation diff decodes");
    let produced = decoded.apply(&before()).expect("committed bind-representation diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "bind-representation/binds-a-second-representation-to-an-existing-type: committed diff did not carry before to after");
}
