//! 🧪️ `set-icc` fixture — `attaches-an-icc-profile-where-there-was-none`.
//!
//! Transcribed from `../../🔺️diff/🦀️component.rs`: an ICC value already equal to `base`'s is
//! Warning `mutation.no-op`, otherwise the enum arm emits `icc: Some(Some(bytes))`.
//! `SemioImageDiff::icc` is a genuine TRI-STATE `Option<Option<Vec<u8>>>` — absent = unchanged,
//! `null` = cleared, bytes = set. This case is deliberately the SET arm: its inner value is a real
//! array, so the committed diff survives a JSON round trip (the `null`/clear arm would not, since
//! `Option<Option<T>>` decodes `null` back to the outer `None`).
use crate::artifacts::semio::standards::v1::subsets::image::schema::diff::SemioImageDiff;
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::{apply_semio_image_mutation, SemioImageMutation};
use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use protocol::{Mutation, MutationDiff};

/// 🔗️ This leaf's own `🔺️diff` oracle, mounted directly: the enum-level `Mutation::diff` arm
/// deliberately carries NO guard branches — every `mutation.no-op`/`mutation.clamped`/
/// `mutation.target-missing`/`mutation.invariant` decision for `set-icc` lives in that file, so the
/// fixture asserts against it rather than against the guardless enum arm.
#[path = "../../🔺️diff/🦀️component.rs"]
mod leaf_diff;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> SemioImageSnapshot {
    serde_json::from_str(BEFORE).expect("set-icc before snapshot decodes")
}
fn expected_after() -> SemioImageSnapshot {
    serde_json::from_str(AFTER).expect("set-icc after snapshot decodes")
}
fn mutation() -> SemioImageMutation {
    serde_json::from_str(MUTATION).expect("set-icc mutation decodes")
}
fn leaf_outcome() -> protocol::MutationOutcome<SemioImageDiff> {
    let SemioImageMutation::SetIcc { icc } = mutation() else { panic!("set-icc/attaches-an-icc-profile-where-there-was-none: the committed mutation must be the set-icc variant") };
    leaf_diff::diff(&before(), icc)
}

/// ▶️ The profile bytes are stored verbatim; nothing else moves.
#[semio_framework_async_macros::async_test]
async fn attaches_the_profile_bytes_verbatim() {
    let base = before();
    assert!(base.icc.is_none(), "the fixture's whole point is a base carrying no embedded profile");
    let produced = leaf_outcome().diff().apply(&base).expect("set-icc applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "set-icc/attaches-an-icc-profile-where-there-was-none: applied state differs from the committed after-snapshot");
    assert_eq!(produced.icc.as_deref(), Some([0u8, 1, 2, 3].as_slice()), "the profile bytes must be stored byte-for-byte, never re-encoded");
    assert_eq!(produced.frames, base.frames, "attaching a profile must not touch a frame");
    let mut in_place = before();
    apply_semio_image_mutation(&mut in_place, &mutation());
    assert_eq!(in_place, expected_after(), "the subset's own apply entry point must reach the same state as the leaf diff");
}

/// ↩️ The undo is a `set-icc` carrying BASE's own value — here `None`, i.e. clear it again.
#[semio_framework_async_macros::async_test]
async fn the_undo_set_icc_clears_the_profile_again() {
    let base = before();
    let mutation = mutation();
    let undo = <SemioImageMutation as Mutation<SemioImageSnapshot>>::inverse(&mutation, &base);
    assert_eq!(undo, vec![SemioImageMutation::SetIcc { icc: None }], "the undo of an attach is a set-icc carrying BASE's own None — the mutation payload is the FINAL value, not a tri-state");
    let mut current = before();
    apply_semio_image_mutation(&mut current, &mutation);
    for step in &undo {
        apply_semio_image_mutation(&mut current, step);
    }
    assert_eq!(current, base, "set-icc/attaches-an-icc-profile-where-there-was-none: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the `{"mutation":"setIcc","icc":[0,1,2,3]}` payload are canonical — the SNAPSHOT's `icc` has no `skip_serializing_if`, so an absent profile is an explicit `null` there, whereas the DIFF omits the key entirely when unchanged.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioImageSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "set-icc/attaches-an-icc-profile-where-there-was-none: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("set-icc mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("set-icc mutation reparses");
    assert_eq!(reencoded, original, "set-icc/attaches-an-icc-profile-where-there-was-none: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the new profile genuinely differs from the base's absent one, so mutation.no-op must not fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_with_no_guard_branch_firing() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "set-icc/attaches-an-icc-profile-where-there-was-none: this case is declared applied");
    assert!(leaf_outcome().messages().is_empty(), "the new profile genuinely differs from the base's absent one, so mutation.no-op must not fire");
}

/// 🔺️ The delta the leaf's own diff builder produces equals the committed diff. Exactly one key, whose value is the byte array (the Some(Some(..)) arm of the tri-state).
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = leaf_outcome();
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "set-icc/attaches-an-icc-profile-where-there-was-none: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioImageDiff = serde_json::from_str(DIFF).expect("committed set-icc diff decodes");
    let produced = decoded.apply(&before()).expect("committed set-icc diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "set-icc/attaches-an-icc-profile-where-there-was-none: committed diff did not carry before to after");
}

/// 🔣️ The committed diff is canonical, and its `icc` value is a real array — NOT `null`, which is
/// what keeps this case a decode→encode fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_uses_the_set_arm_of_the_tri_state() {
    let decoded: SemioImageDiff = serde_json::from_str(DIFF).expect("committed set-icc diff decodes");
    assert!(matches!(decoded.icc, Some(Some(_))), "the icc slot must decode as Some(Some(bytes)) — set, not cleared");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert!(committed.get("icc").expect("the committed diff names the icc slot").is_array(), "a SET profile is an array; a CLEARED one would be null and would not survive the Option<Option<..>> round trip");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    assert_eq!(reencoded, committed, "set-icc/attaches-an-icc-profile-where-there-was-none: committed diff JSON is not canonical");
}
