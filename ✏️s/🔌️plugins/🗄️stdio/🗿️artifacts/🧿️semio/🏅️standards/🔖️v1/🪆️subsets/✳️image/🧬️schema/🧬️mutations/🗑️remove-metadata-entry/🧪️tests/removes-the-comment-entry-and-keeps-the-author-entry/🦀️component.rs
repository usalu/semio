//! 🧪️ `remove-metadata-entry` fixture — `removes-the-comment-entry-and-keeps-the-author-entry`.
//!
//! Transcribed from `../../🔺️diff/🦀️component.rs`: a key absent from `base.metadata` is Error
//! `mutation.target-missing`; otherwise the diff is a bare `metadata.removed[key]` — name-keyed,
//! so no index is transported and the surviving entries keep their order. The two-entry
//! before-snapshot is what makes "keeps the author entry" a real claim rather than a tautology.
use crate::artifacts::semio::standards::v1::subsets::image::schema::diff::SemioImageDiff;
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::{apply_semio_image_mutation, SemioImageMutation};
use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use protocol::{Mutation, MutationDiff};
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::set_metadata_entry;
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::remove_metadata_entry;

/// 🔗️ This leaf's own `🔺️diff` oracle, mounted directly: the enum-level `Mutation::diff` arm
/// deliberately carries NO guard branches — every `mutation.no-op`/`mutation.target-missing`
/// decision for `remove-metadata-entry` lives in that file, so the fixture asserts against it rather than against
/// the guardless enum arm.
#[path = "../../🔺️diff/🦀️component.rs"]
mod leaf_diff;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> SemioImageSnapshot {
    serde_json::from_str(BEFORE).expect("remove-metadata-entry before snapshot decodes")
}
fn expected_after() -> SemioImageSnapshot {
    serde_json::from_str(AFTER).expect("remove-metadata-entry after snapshot decodes")
}
fn mutation() -> SemioImageMutation {
    serde_json::from_str(MUTATION).expect("remove-metadata-entry mutation decodes")
}
fn leaf_outcome() -> protocol::MutationOutcome<SemioImageDiff> {
    let SemioImageMutation::RemoveMetadataEntry(remove_metadata_entry::RemoveMetadataEntry { key }) = mutation() else { panic!("remove-metadata-entry/removes-the-comment-entry-and-keeps-the-author-entry: the committed mutation must be the remove-metadata-entry variant") };
    leaf_diff::diff(&before(), key)
}

/// ▶️ Only the `Comment` entry goes; `Author` survives untouched.
#[semio_framework_async_macros::async_test]
async fn removes_only_the_comment_entry() {
    let base = before();
    assert_eq!(base.metadata.len(), 2, "the fixture needs a sibling entry for the claim to mean anything");
    let produced = leaf_outcome().diff().apply(&base).expect("remove-metadata-entry applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "remove-metadata-entry/removes-the-comment-entry-and-keeps-the-author-entry: applied state differs from the committed after-snapshot");
    assert!(!produced.metadata.iter().any(|entry| entry.key == "Comment"), "the addressed entry must be gone");
    assert_eq!(produced.metadata, vec![base.metadata[0].clone()], "the sibling Author entry must survive byte-identical");
    assert_eq!(produced.frames, base.frames, "removing a metadata entry must not touch a frame");
    let mut in_place = before();
    apply_semio_image_mutation(&mut in_place, &mutation());
    assert_eq!(in_place, expected_after(), "the subset's own apply entry point must reach the same state as the leaf diff");
}

/// ↩️ The undo re-sets the entry from BASE's captured value — the diff itself never carried it.
#[semio_framework_async_macros::async_test]
async fn the_undo_set_metadata_entry_restores_the_captured_comment() {
    let base = before();
    let mutation = mutation();
    let undo = <SemioImageMutation as Mutation<SemioImageSnapshot>>::inverse(&mutation, &base);
    assert_eq!(undo, vec![SemioImageMutation::SetMetadataEntry(set_metadata_entry::SetMetadataEntry { key: "Comment".to_string(), value: "draft".to_string() })], "the undo must recapture the removed entry's value from base");
    let mut current = before();
    apply_semio_image_mutation(&mut current, &mutation);
    for step in &undo {
        apply_semio_image_mutation(&mut current, step);
    }
    assert_eq!(current, base, "remove-metadata-entry/removes-the-comment-entry-and-keeps-the-author-entry: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the `{"mutation":"removeMetadataEntry","key":"Comment"}` payload are canonical fixed points.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioImageSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "remove-metadata-entry/removes-the-comment-entry-and-keeps-the-author-entry: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("remove-metadata-entry mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("remove-metadata-entry mutation reparses");
    assert_eq!(reencoded, original, "remove-metadata-entry/removes-the-comment-entry-and-keeps-the-author-entry: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the key exists in the base metadata, so mutation.target-missing must not fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_with_no_guard_branch_firing() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "remove-metadata-entry/removes-the-comment-entry-and-keeps-the-author-entry: this case is declared applied");
    assert!(leaf_outcome().messages().is_empty(), "the key exists in the base metadata, so mutation.target-missing must not fire");
}

/// 🔺️ The delta the leaf's own diff builder produces equals the committed diff. Only `removed` may be present, and it carries the KEY, not an index.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = leaf_outcome();
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "remove-metadata-entry/removes-the-comment-entry-and-keeps-the-author-entry: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point and is scoped as narrowly as the leaf builds it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded: SemioImageDiff = serde_json::from_str(DIFF).expect("committed remove-metadata-entry diff decodes");

    let metadata = decoded.metadata.as_ref().expect("remove-metadata-entry must write the metadata slot");
    assert_eq!(metadata.removed, vec!["Comment".to_string()], "the removal is addressed by name key");
    assert!(metadata.modified.is_empty() && metadata.added.is_empty(), "a removal neither modifies nor adds");
    assert!(decoded.frames.is_none() && decoded.width.is_none(), "no other slot may be touched");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "remove-metadata-entry/removes-the-comment-entry-and-keeps-the-author-entry: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioImageDiff = serde_json::from_str(DIFF).expect("committed remove-metadata-entry diff decodes");
    let produced = decoded.apply(&before()).expect("committed remove-metadata-entry diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "remove-metadata-entry/removes-the-comment-entry-and-keeps-the-author-entry: committed diff did not carry before to after");
}
