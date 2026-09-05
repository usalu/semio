//! 🧪️ `change-material-metallic` fixture — `🪙️raises-the-metallic-factor-to-fully-metallic`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`: unknown id ⇒ Error `mutation.target-missing`,
//! an unchanged factor ⇒ Warning `mutation.no-op`, a non-finite factor ⇒ FATAL `mutation.invariant`.
//! `metallic` and `roughness` are two INDEPENDENT scalar triads (decomposed from an older bundled
//! `SetMaterialPbr`), so the whole point of this case is that the diff mentions `metallic` and not
//! `roughness`.

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
    serde_json::from_str(BEFORE).expect("change-material-metallic before snapshot decodes")
}
fn expected_after() -> SemioMeshSnapshot {
    serde_json::from_str(AFTER).expect("change-material-metallic after snapshot decodes")
}
fn mutation() -> SemioMeshMutation {
    serde_json::from_str(MUTATION).expect("change-material-metallic mutation decodes")
}

/// ▶️ Only the metallic factor moves; roughness and the base color stay.
#[semio_framework_async_macros::async_test]
async fn raises_metallic_without_touching_roughness() {
    let base = before();
    let produced = mutation().diff(&base).diff().apply(&base).expect("change-material-metallic applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "change-material-metallic/raises-the-metallic-factor-to-fully-metallic: applied state differs from the committed after-snapshot");
    assert_eq!(produced.materials[0].metallic, 1.0, "the metallic factor must take the payload's absolute value");
    assert_eq!(produced.materials[0].roughness, base.materials[0].roughness, "the sibling roughness factor is a SEPARATE triad and must not move");
    assert_eq!(produced.materials[0].base_color, base.materials[0].base_color, "changing metallic must not repaint the material");
}

/// ↩️ The undo is a `change-material-metallic` carrying BASE's captured factor.
#[semio_framework_async_macros::async_test]
async fn the_undo_change_material_metallic_restores_the_original_factor() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "change-material-metallic undoes as exactly one change-material-metallic");
    let SemioMeshMutation::ChangeMaterialMetallic(restore) = &undo[0] else { panic!("change-material-metallic must undo as itself") };
    assert_eq!(restore.new_metallic, base.materials[0].metallic, "the undo must recapture BASE's own metallic factor");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward change-material-metallic applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo change-material-metallic applies");
    }
    assert_eq!(current, base, "change-material-metallic/raises-the-metallic-factor-to-fully-metallic: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the `{"ChangeMaterialMetallic":{"id":"mat-a","new_metallic":1.0}}` payload are canonical — the factor is an `f32` written as a dyadic literal.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioMeshSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-material-metallic/raises-the-metallic-factor-to-fully-metallic: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-material-metallic mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-material-metallic mutation reparses");
    assert_eq!(reencoded, original, "change-material-metallic/raises-the-metallic-factor-to-fully-metallic: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the material exists, the factor genuinely differs and is finite, so none of the three guards may fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_as_committed() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-material-metallic/raises-the-metallic-factor-to-fully-metallic: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "raising the metallic factor to a finite, genuinely different value must raise no diagnostics");
}

/// 🔺️ The sparse delta this mutation produces equals the committed diff. Only `materials.modified`, carrying `metallic` alone.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioMeshMutation as Mutation<SemioMeshSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-material-metallic/raises-the-metallic-factor-to-fully-metallic: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point, and only the collection this mutation is
/// allowed to touch appears in it at all.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded: SemioMeshDiff = serde_json::from_str(DIFF).expect("committed change-material-metallic diff decodes");
    let materials = decoded.materials.as_ref().expect("change-material-metallic must write the materials triple");
    let mdiff = &materials.modified[0].diff;
    assert_eq!(mdiff.metallic, Some(1.0), "the metallic factor must be written");
    assert!(mdiff.roughness.is_none() && mdiff.base_color.is_none(), "the sibling factor and the color must stay unwritten");
    assert!(decoded.meshes.is_none() && decoded.textures.is_none(), "no mesh or texture slot may appear in the diff");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-material-metallic/raises-the-metallic-factor-to-fully-metallic: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioMeshDiff = serde_json::from_str(DIFF).expect("committed change-material-metallic diff decodes");
    let produced = decoded.apply(&before()).expect("committed change-material-metallic diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-material-metallic/raises-the-metallic-factor-to-fully-metallic: committed diff did not carry before to after");
}
