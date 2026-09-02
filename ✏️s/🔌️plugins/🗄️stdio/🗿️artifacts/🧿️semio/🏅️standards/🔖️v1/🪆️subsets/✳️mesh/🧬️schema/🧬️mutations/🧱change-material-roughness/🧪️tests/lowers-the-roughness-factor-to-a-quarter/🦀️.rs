//! 🧪️ `change-material-roughness` fixture — `lowers-the-roughness-factor-to-a-quarter`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`: the exact mirror of
//! `change-material-metallic` — unknown id ⇒ Error `mutation.target-missing`, unchanged factor ⇒
//! Warning `mutation.no-op`, non-finite factor ⇒ FATAL `mutation.invariant`. Committing both halves
//! of the pair is what proves the decomposition is real: this diff must mention `roughness` and
//! NOT `metallic`.

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
    serde_json::from_str(BEFORE).expect("change-material-roughness before snapshot decodes")
}
fn expected_after() -> SemioMeshSnapshot {
    serde_json::from_str(AFTER).expect("change-material-roughness after snapshot decodes")
}
fn mutation() -> SemioMeshMutation {
    serde_json::from_str(MUTATION).expect("change-material-roughness mutation decodes")
}

/// ▶️ Only the roughness factor moves; metallic and the base color stay.
#[semio_framework_async_macros::async_test]
async fn lowers_roughness_without_touching_metallic() {
    let base = before();
    let produced = mutation().diff(&base).diff().apply(&base).expect("change-material-roughness applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "change-material-roughness/lowers-the-roughness-factor-to-a-quarter: applied state differs from the committed after-snapshot");
    assert_eq!(produced.materials[0].roughness, 0.25, "the roughness factor must take the payload's absolute value");
    assert_eq!(produced.materials[0].metallic, base.materials[0].metallic, "the sibling metallic factor is a SEPARATE triad and must not move");
    assert_eq!(produced.materials[0].base_color, base.materials[0].base_color, "changing roughness must not repaint the material");
}

/// ↩️ The undo is a `change-material-roughness` carrying BASE's captured factor.
#[semio_framework_async_macros::async_test]
async fn the_undo_change_material_roughness_restores_the_original_factor() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "change-material-roughness undoes as exactly one change-material-roughness");
    let SemioMeshMutation::ChangeMaterialRoughness(restore) = &undo[0] else { panic!("change-material-roughness must undo as itself") };
    assert_eq!(restore.new_roughness, base.materials[0].roughness, "the undo must recapture BASE's own roughness factor");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward change-material-roughness applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo change-material-roughness applies");
    }
    assert_eq!(current, base, "change-material-roughness/lowers-the-roughness-factor-to-a-quarter: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the `{"ChangeMaterialRoughness":{"id":"mat-a","new_roughness":0.25}}` payload are canonical — 0.25 is exactly representable as an `f32`.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioMeshSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-material-roughness/lowers-the-roughness-factor-to-a-quarter: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-material-roughness mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-material-roughness mutation reparses");
    assert_eq!(reencoded, original, "change-material-roughness/lowers-the-roughness-factor-to-a-quarter: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the material exists, the factor genuinely differs and is finite, so none of the three guards may fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_as_committed() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-material-roughness/lowers-the-roughness-factor-to-a-quarter: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "lowering the roughness factor to a finite, genuinely different value must raise no diagnostics");
}

/// 🔺️ The sparse delta this mutation produces equals the committed diff. Only `materials.modified`, carrying `roughness` alone.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioMeshMutation as Mutation<SemioMeshSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-material-roughness/lowers-the-roughness-factor-to-a-quarter: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point, and only the collection this mutation is
/// allowed to touch appears in it at all.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded: SemioMeshDiff = serde_json::from_str(DIFF).expect("committed change-material-roughness diff decodes");
    let materials = decoded.materials.as_ref().expect("change-material-roughness must write the materials triple");
    let mdiff = &materials.modified[0].diff;
    assert_eq!(mdiff.roughness, Some(0.25), "the roughness factor must be written");
    assert!(mdiff.metallic.is_none() && mdiff.base_color.is_none(), "the sibling factor and the color must stay unwritten");
    assert!(decoded.meshes.is_none() && decoded.textures.is_none(), "no mesh or texture slot may appear in the diff");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-material-roughness/lowers-the-roughness-factor-to-a-quarter: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioMeshDiff = serde_json::from_str(DIFF).expect("committed change-material-roughness diff decodes");
    let produced = decoded.apply(&before()).expect("committed change-material-roughness diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-material-roughness/lowers-the-roughness-factor-to-a-quarter: committed diff did not carry before to after");
}
