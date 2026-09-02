//! 🧪️ `change-representation-pin` fixture — `repins-the-representation-from-head-to-a-checkpoint`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`: an out-of-range index is Error
//! `mutation.target-missing`, a pin already equal is Warning `mutation.no-op`; otherwise ONLY the
//! addressed link's `pin` is assigned. `target` and `role` are explicitly untouched — repinning
//! changes WHICH VERSION of a representation the kit is frozen to, never which document or what
//! for.

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
    serde_json::from_str(BEFORE).expect("change-representation-pin before snapshot decodes")
}
fn expected_after() -> SemioKitSnapshot {
    serde_json::from_str(AFTER).expect("change-representation-pin after snapshot decodes")
}
fn mutation() -> SemioKitMutation {
    serde_json::from_str(MUTATION).expect("change-representation-pin mutation decodes")
}

/// ▶️ The link stops tracking the target's live tip and freezes to a checkpoint instead.
#[semio_framework_async_macros::async_test]
async fn repins_the_link_without_retargeting_or_rerolling_it() {
    let base = before();
    let produced = mutation().diff(&base).diff().apply(&base).expect("change-representation-pin applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "change-representation-pin/repins-the-representation-from-head-to-a-checkpoint: applied state differs from the committed after-snapshot");
    assert_ne!(produced.representations[0].pin, base.representations[0].pin, "the pin really must have changed");
    assert_eq!(produced.representations[0].target, base.representations[0].target, "repinning must not retarget the link");
    assert_eq!(produced.representations[0].role, base.representations[0].role, "repinning must not change what the link is a representation FOR");
    assert_eq!(produced.representations.len(), base.representations.len(), "repinning may never add or drop a link");
}

/// ↩️ The undo is a `change-representation-pin` carrying BASE's captured pin.
#[semio_framework_async_macros::async_test]
async fn the_undo_change_representation_pin_restores_the_head_pin() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "change-representation-pin of an existing link undoes as exactly one change-representation-pin");
    let SemioKitMutation::ChangeRepresentationPin(restore) = &undo[0] else { panic!("change-representation-pin must undo as change-representation-pin") };
    assert_eq!(restore.pin, base.representations[0].pin, "the undo must recapture BASE's own pin");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward change-representation-pin applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo change-representation-pin applies");
    }
    assert_eq!(current, base, "change-representation-pin/repins-the-representation-from-head-to-a-checkpoint: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the payload are canonical — `LinkPin::Checkpoint` encodes as `{"kind":"checkpoint","id":…}`, the internally-tagged shape that lets a unit variant (`head`) and a struct variant share one field.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioKitSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-representation-pin/repins-the-representation-from-head-to-a-checkpoint: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-representation-pin mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-representation-pin mutation reparses");
    assert_eq!(reencoded, original, "change-representation-pin/repins-the-representation-from-head-to-a-checkpoint: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the index is in range and the new pin genuinely differs, so neither target-missing nor no-op may fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_as_committed() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-representation-pin/repins-the-representation-from-head-to-a-checkpoint: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "repinning to a genuinely different pin must raise no diagnostics");
}

/// 🔺️ The sparse delta this mutation produces equals the committed diff. Only the `representations` slot.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioKitMutation as Mutation<SemioKitSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-representation-pin/repins-the-representation-from-head-to-a-checkpoint: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point, and only the slot this mutation is
/// allowed to touch appears in it at all.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded: SemioKitDiff = serde_json::from_str(DIFF).expect("committed change-representation-pin diff decodes");
    let links = decoded.representations.as_ref().expect("change-representation-pin must write the representations slot");
    assert_eq!(links.values.len(), 1, "a repin never changes how many links there are");
    assert_eq!(links.values[0].role, before().representations[0].role, "the diff carries the whole link, role unchanged");
    assert!(decoded.types.is_none() && decoded.designs.is_none() && decoded.objects.is_none() && decoded.models.is_none() && decoded.properties.is_none(), "no other kit slot may appear in the diff");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-representation-pin/repins-the-representation-from-head-to-a-checkpoint: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioKitDiff = serde_json::from_str(DIFF).expect("committed change-representation-pin diff decodes");
    let produced = decoded.apply(&before()).expect("committed change-representation-pin diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-representation-pin/repins-the-representation-from-head-to-a-checkpoint: committed diff did not carry before to after");
}
