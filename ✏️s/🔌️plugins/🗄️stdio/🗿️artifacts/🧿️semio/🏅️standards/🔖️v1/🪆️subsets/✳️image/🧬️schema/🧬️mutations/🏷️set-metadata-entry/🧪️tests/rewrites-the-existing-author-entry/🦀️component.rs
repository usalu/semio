//! 🧪️ `set-metadata-entry` fixture — `rewrites-the-existing-author-entry`.
//!
//! Transcribed from `../../🔺️diff/🦀️component.rs` AND the enum arm it delegates to: an entry that
//! already holds this exact `key`/`value` pair is Warning `mutation.no-op`, and the arm then
//! BRANCHES on whether the key exists — present ⇒ `metadata.modified[{key, diff}]`, absent ⇒
//! `metadata.added[entry]`. This case takes the MODIFIED branch (the key exists with a different
//! value); `metadata` is a weak, name-keyed collection, so its per-entry "diff" is just the whole
//! new value string.
use crate::artifacts::semio::standards::v1::subsets::image::schema::diff::SemioImageDiff;
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::{apply_semio_image_mutation, SemioImageMutation};
use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use protocol::{Mutation, MutationDiff};

/// 🔗️ This leaf's own `🔺️diff` oracle, mounted directly: the enum-level `Mutation::diff` arm
/// deliberately carries NO guard branches — every `mutation.no-op`/`mutation.target-missing`
/// decision for `set-metadata-entry` lives in that file, so the fixture asserts against it rather than against
/// the guardless enum arm.
#[path = "../../🔺️diff/🦀️component.rs"]
mod leaf_diff;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> SemioImageSnapshot {
    serde_json::from_str(BEFORE).expect("set-metadata-entry before snapshot decodes")
}
fn expected_after() -> SemioImageSnapshot {
    serde_json::from_str(AFTER).expect("set-metadata-entry after snapshot decodes")
}
fn mutation() -> SemioImageMutation {
    serde_json::from_str(MUTATION).expect("set-metadata-entry mutation decodes")
}
fn leaf_outcome() -> protocol::MutationOutcome<SemioImageDiff> {
    let SemioImageMutation::SetMetadataEntry { key, value } = mutation() else { panic!("set-metadata-entry/rewrites-the-existing-author-entry: the committed mutation must be the set-metadata-entry variant") };
    leaf_diff::diff(&before(), key, value)
}

/// ▶️ The existing entry's value is rewritten in place — no second `Author` entry appears.
#[semio_framework_async_macros::async_test]
async fn rewrites_the_author_entry_in_place() {
    let base = before();
    let produced = leaf_outcome().diff().apply(&base).expect("set-metadata-entry applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "set-metadata-entry/rewrites-the-existing-author-entry: applied state differs from the committed after-snapshot");
    assert_eq!(produced.metadata.len(), base.metadata.len(), "rewriting an EXISTING key must not append a duplicate entry");
    assert_eq!(produced.metadata[0].key, "Author", "the entry keeps its name key");
    assert_eq!(produced.metadata[0].value, "Ueli Saluz", "the entry's value must become the payload's value");
    assert_eq!(produced.frames, base.frames, "metadata edits must not touch a frame");
    let mut in_place = before();
    apply_semio_image_mutation(&mut in_place, &mutation());
    assert_eq!(in_place, expected_after(), "the subset's own apply entry point must reach the same state as the leaf diff");
}

/// ↩️ Because the key EXISTED in base, the undo is another `set-metadata-entry` carrying the old
/// value — not a `remove-metadata-entry`, which is what the absent-key branch would produce.
#[semio_framework_async_macros::async_test]
async fn the_undo_set_metadata_entry_restores_the_old_value() {
    let base = before();
    let mutation = mutation();
    let undo = <SemioImageMutation as Mutation<SemioImageSnapshot>>::inverse(&mutation, &base);
    assert_eq!(undo, vec![SemioImageMutation::SetMetadataEntry { key: "Author".to_string(), value: "semio".to_string() }], "an existing key undoes as a set carrying BASE's own value, never as a remove");
    let mut current = before();
    apply_semio_image_mutation(&mut current, &mutation);
    for step in &undo {
        apply_semio_image_mutation(&mut current, step);
    }
    assert_eq!(current, base, "set-metadata-entry/rewrites-the-existing-author-entry: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the `{"mutation":"setMetadataEntry","key":"Author","value":"Ueli Saluz"}` payload are canonical fixed points.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioImageSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "set-metadata-entry/rewrites-the-existing-author-entry: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("set-metadata-entry mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("set-metadata-entry mutation reparses");
    assert_eq!(reencoded, original, "set-metadata-entry/rewrites-the-existing-author-entry: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the key exists but with a different value, so mutation.no-op must not fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_with_no_guard_branch_firing() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "set-metadata-entry/rewrites-the-existing-author-entry: this case is declared applied");
    assert!(leaf_outcome().messages().is_empty(), "the key exists but with a different value, so mutation.no-op must not fire");
}

/// 🔺️ The delta the leaf's own diff builder produces equals the committed diff. Only `modified` may be present — the ADDED branch would mean the key had been absent.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = leaf_outcome();
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "set-metadata-entry/rewrites-the-existing-author-entry: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point and is scoped as narrowly as the leaf builds it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded: SemioImageDiff = serde_json::from_str(DIFF).expect("committed set-metadata-entry diff decodes");

    let metadata = decoded.metadata.as_ref().expect("set-metadata-entry must write the metadata slot");
    assert!(metadata.removed.is_empty() && metadata.added.is_empty(), "an in-place rewrite neither adds nor removes an entry");
    assert_eq!(metadata.modified.len(), 1, "exactly one entry is modified");
    assert_eq!(metadata.modified[0].key, "Author", "the modification is keyed by the entry name, not an index");
    assert!(decoded.frames.is_none() && decoded.width.is_none(), "no other slot may be touched");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "set-metadata-entry/rewrites-the-existing-author-entry: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioImageDiff = serde_json::from_str(DIFF).expect("committed set-metadata-entry diff decodes");
    let produced = decoded.apply(&before()).expect("committed set-metadata-entry diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "set-metadata-entry/rewrites-the-existing-author-entry: committed diff did not carry before to after");
}
