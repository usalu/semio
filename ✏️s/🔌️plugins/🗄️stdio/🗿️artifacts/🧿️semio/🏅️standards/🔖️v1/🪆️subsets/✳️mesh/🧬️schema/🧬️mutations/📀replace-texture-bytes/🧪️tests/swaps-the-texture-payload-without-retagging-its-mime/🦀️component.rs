//! 🧪️ `replace-texture-bytes` fixture — `swaps-the-texture-payload-without-retagging-its-mime`.
//!
//! Transcribed from `../../🔺️diff/🦀️component.rs`: unknown id ⇒ Error `mutation.target-missing`,
//! unchanged bytes ⇒ Warning `mutation.no-op`. The exact mirror of `change-texture-mime`: raw image
//! bytes are the large swapped payload, so the per-texture diff writes `bytes` whole and leaves
//! `mime` at `None`. Committing both halves is what proves the decomposition is real rather than
//! decorative.

use crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::SemioMeshDiff;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::SemioMeshMutation;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> SemioMeshSnapshot {
    serde_json::from_str(BEFORE).expect("replace-texture-bytes before snapshot decodes")
}
fn expected_after() -> SemioMeshSnapshot {
    serde_json::from_str(AFTER).expect("replace-texture-bytes after snapshot decodes")
}
fn mutation() -> SemioMeshMutation {
    serde_json::from_str(MUTATION).expect("replace-texture-bytes mutation decodes")
}

/// ▶️ The payload bytes are replaced wholesale; the declared mime is untouched.
#[semio_framework_async_macros::async_test]
async fn swaps_the_payload_without_retagging_the_mime() {
    let base = before();
    let produced = mutation().diff(&base).diff().apply(&base).expect("replace-texture-bytes applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "replace-texture-bytes/swaps-the-texture-payload-without-retagging-its-mime: applied state differs from the committed after-snapshot");
    assert_eq!(produced.textures[0].bytes, vec![10u8, 20, 30, 40, 50], "the bytes must become the payload's buffer, verbatim");
    assert_ne!(produced.textures[0].bytes.len(), base.textures[0].bytes.len(), "the buffer really is replaced wholesale, not patched in place");
    assert_eq!(produced.textures[0].mime, base.textures[0].mime, "replacing bytes must NOT retag the mime — that is change-texture-mime's job");
}

/// ↩️ The undo is a `replace-texture-bytes` carrying BASE's captured buffer.
#[semio_framework_async_macros::async_test]
async fn the_undo_replace_texture_bytes_restores_the_captured_buffer() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "replace-texture-bytes undoes as exactly one replace-texture-bytes");
    let SemioMeshMutation::ReplaceTextureBytes(restore) = &undo[0] else { panic!("replace-texture-bytes must undo as itself") };
    assert_eq!(restore.new_bytes, base.textures[0].bytes, "the undo must recapture BASE's own byte buffer");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward replace-texture-bytes applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo replace-texture-bytes applies");
    }
    assert_eq!(current, base, "replace-texture-bytes/swaps-the-texture-payload-without-retagging-its-mime: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the payload are canonical — the new buffer travels as a plain JSON number array under the snake_case key `new_bytes`.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioMeshSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "replace-texture-bytes/swaps-the-texture-payload-without-retagging-its-mime: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("replace-texture-bytes mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("replace-texture-bytes mutation reparses");
    assert_eq!(reencoded, original, "replace-texture-bytes/swaps-the-texture-payload-without-retagging-its-mime: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the texture exists and the new bytes genuinely differ, so neither target-missing nor no-op may fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_as_committed() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "replace-texture-bytes/swaps-the-texture-payload-without-retagging-its-mime: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "replacing a payload with genuinely different bytes must raise no diagnostics");
}

/// 🔺️ The sparse delta this mutation produces equals the committed diff. Only `textures.modified`, carrying `bytes` alone.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioMeshMutation as Mutation<SemioMeshSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "replace-texture-bytes/swaps-the-texture-payload-without-retagging-its-mime: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point, and only the collection this mutation is
/// allowed to touch appears in it at all.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded: SemioMeshDiff = serde_json::from_str(DIFF).expect("committed replace-texture-bytes diff decodes");
    let textures = decoded.textures.as_ref().expect("replace-texture-bytes must write the textures triple");
    assert!(textures.removed.is_empty() && textures.added.is_empty(), "a replace is a per-field modification");
    let tdiff = &textures.modified[0].diff;
    assert_eq!(tdiff.bytes.as_deref(), Some([10u8, 20, 30, 40, 50].as_slice()), "the new buffer must be written verbatim");
    assert!(tdiff.mime.is_none(), "the mime field must stay unwritten — the diff proves the tag was not touched");
    assert!(decoded.meshes.is_none() && decoded.materials.is_none(), "no mesh or material slot may appear in the diff");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "replace-texture-bytes/swaps-the-texture-payload-without-retagging-its-mime: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioMeshDiff = serde_json::from_str(DIFF).expect("committed replace-texture-bytes diff decodes");
    let produced = decoded.apply(&before()).expect("committed replace-texture-bytes diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "replace-texture-bytes/swaps-the-texture-payload-without-retagging-its-mime: committed diff did not carry before to after");
}
