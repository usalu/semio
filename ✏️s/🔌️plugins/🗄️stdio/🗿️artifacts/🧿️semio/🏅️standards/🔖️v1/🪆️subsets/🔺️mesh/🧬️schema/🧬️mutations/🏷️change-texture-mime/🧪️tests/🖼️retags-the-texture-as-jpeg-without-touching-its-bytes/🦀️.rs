//! 🧪️ `change-texture-mime` fixture — `🖼️retags-the-texture-as-jpeg-without-touching-its-bytes`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`: unknown id ⇒ Error `mutation.target-missing`,
//! an unchanged mime ⇒ Warning `mutation.no-op`. `mime` and `bytes` were deliberately decomposed
//! into two independent triads (out of an older bundled `SetTextureBytes{mime, bytes}`), so the
//! per-texture diff must write `mime` and leave `bytes` at `None` — retagging a payload is not the
//! same operation as replacing it.

use crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::SemioMeshDiff;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::SemioMeshMutation;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> SemioMeshSnapshot {
    serde_json::from_str(BEFORE).expect("change-texture-mime before snapshot decodes")
}
fn expected_after() -> SemioMeshSnapshot {
    serde_json::from_str(AFTER).expect("change-texture-mime after snapshot decodes")
}
fn mutation() -> SemioMeshMutation {
    serde_json::from_str(MUTATION).expect("change-texture-mime mutation decodes")
}

/// ▶️ The declared mime changes; the payload bytes are not re-encoded.
#[semio_framework_async_macros::async_test]
async fn retags_the_mime_without_re_encoding_the_payload() {
    let base = before();
    let produced = mutation().diff(&base).diff().apply(&base).expect("change-texture-mime applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "change-texture-mime/retags-the-texture-as-jpeg-without-touching-its-bytes: applied state differs from the committed after-snapshot");
    assert_eq!(produced.textures[0].mime, "image/jpeg", "the mime must become the payload's value");
    assert_eq!(produced.textures[0].bytes, base.textures[0].bytes, "retagging must NOT touch the bytes — that is replace-texture-bytes' job");
    assert_eq!(produced.textures.len(), base.textures.len(), "retagging may never add or drop a texture");
}

/// ↩️ The undo is a `change-texture-mime` carrying BASE's captured mime.
#[semio_framework_async_macros::async_test]
async fn the_undo_change_texture_mime_restores_the_original_mime() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "change-texture-mime undoes as exactly one change-texture-mime");
    let SemioMeshMutation::ChangeTextureMime(restore) = &undo[0] else { panic!("change-texture-mime must undo as itself") };
    assert_eq!(restore.new_mime, base.textures[0].mime, "the undo must recapture BASE's own mime");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward change-texture-mime applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo change-texture-mime applies");
    }
    assert_eq!(current, base, "change-texture-mime/retags-the-texture-as-jpeg-without-touching-its-bytes: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the `{"ChangeTextureMime":{"id":"tex-a","new_mime":"image/jpeg"}}` payload are canonical fixed points.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioMeshSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-texture-mime/retags-the-texture-as-jpeg-without-touching-its-bytes: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-texture-mime mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-texture-mime mutation reparses");
    assert_eq!(reencoded, original, "change-texture-mime/retags-the-texture-as-jpeg-without-touching-its-bytes: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the texture exists and the new mime genuinely differs, so neither target-missing nor no-op may fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_as_committed() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-texture-mime/retags-the-texture-as-jpeg-without-touching-its-bytes: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "retagging to a genuinely different mime must raise no diagnostics");
}

/// 🔺️ The sparse delta this mutation produces equals the committed diff. Only `textures.modified`, carrying `mime` alone.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioMeshMutation as Mutation<SemioMeshSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-texture-mime/retags-the-texture-as-jpeg-without-touching-its-bytes: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point, and only the collection this mutation is
/// allowed to touch appears in it at all.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded: SemioMeshDiff = serde_json::from_str(DIFF).expect("committed change-texture-mime diff decodes");
    let textures = decoded.textures.as_ref().expect("change-texture-mime must write the textures triple");
    assert!(textures.removed.is_empty() && textures.added.is_empty(), "a retag is a per-field modification");
    let tdiff = &textures.modified[0].diff;
    assert_eq!(tdiff.mime.as_deref(), Some("image/jpeg"), "the mime must be written");
    assert!(tdiff.bytes.is_none(), "the bytes field must stay unwritten — the diff proves the payload was not touched");
    assert!(decoded.meshes.is_none() && decoded.materials.is_none(), "no mesh or material slot may appear in the diff");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-texture-mime/retags-the-texture-as-jpeg-without-touching-its-bytes: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioMeshDiff = serde_json::from_str(DIFF).expect("committed change-texture-mime diff decodes");
    let produced = decoded.apply(&before()).expect("committed change-texture-mime diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-texture-mime/retags-the-texture-as-jpeg-without-touching-its-bytes: committed diff did not carry before to after");
}
