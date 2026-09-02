//! 🧪️ `change-material-base-color` fixture — `repaints-the-material-from-red-to-blue`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`, guards in order: unknown id ⇒ Error
//! `mutation.target-missing`; an unchanged color ⇒ Warning `mutation.no-op`; a non-finite CHANNEL ⇒
//! FATAL `mutation.invariant`. A color is treated as ONE cohesive value (never edited channel by
//! channel from outside), so the per-material diff writes `base_color` whole and leaves `metallic`
//! and `roughness` at `None`.

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
    serde_json::from_str(BEFORE).expect("change-material-base-color before snapshot decodes")
}
fn expected_after() -> SemioMeshSnapshot {
    serde_json::from_str(AFTER).expect("change-material-base-color after snapshot decodes")
}
fn mutation() -> SemioMeshMutation {
    serde_json::from_str(MUTATION).expect("change-material-base-color mutation decodes")
}

/// ▶️ The base color is replaced wholesale; both PBR factors survive.
#[semio_framework_async_macros::async_test]
async fn repaints_the_material_and_keeps_both_pbr_factors() {
    let base = before();
    let produced = mutation().diff(&base).diff().apply(&base).expect("change-material-base-color applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "change-material-base-color/repaints-the-material-from-red-to-blue: applied state differs from the committed after-snapshot");
    let edited = &produced.materials[0];
    assert_eq!((edited.base_color.r, edited.base_color.b), (0.0, 1.0), "the color is replaced as ONE value — every channel takes the payload's");
    assert_eq!(edited.metallic, base.materials[0].metallic, "repainting must not touch the metallic factor");
    assert_eq!(edited.roughness, base.materials[0].roughness, "repainting must not touch the roughness factor");
    assert_eq!(produced.meshes, base.meshes, "repainting a material must not touch a primitive");
}

/// ↩️ The undo is a `change-material-base-color` carrying BASE's captured color.
#[semio_framework_async_macros::async_test]
async fn the_undo_change_material_base_color_restores_the_red() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "change-material-base-color undoes as exactly one change-material-base-color");
    let SemioMeshMutation::ChangeMaterialBaseColor(restore) = &undo[0] else { panic!("change-material-base-color must undo as itself") };
    assert_eq!(restore.new_base_color, base.materials[0].base_color, "the undo must recapture BASE's own color");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward change-material-base-color applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo change-material-base-color applies");
    }
    assert_eq!(current, base, "change-material-base-color/repaints-the-material-from-red-to-blue: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the payload are canonical — `SemioRgba` encodes as four named `f32` channels, and the diff key is the camelCase `baseColor` while the payload key is the snake_case `new_base_color`.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioMeshSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-material-base-color/repaints-the-material-from-red-to-blue: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-material-base-color mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-material-base-color mutation reparses");
    assert_eq!(reencoded, original, "change-material-base-color/repaints-the-material-from-red-to-blue: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the material exists, the color genuinely differs and every channel is finite, so none of the three guards may fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_as_committed() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-material-base-color/repaints-the-material-from-red-to-blue: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "repainting to a finite, genuinely different color must raise no diagnostics");
}

/// 🔺️ The sparse delta this mutation produces equals the committed diff. Only `materials.modified`, carrying `baseColor` alone.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioMeshMutation as Mutation<SemioMeshSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-material-base-color/repaints-the-material-from-red-to-blue: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point, and only the collection this mutation is
/// allowed to touch appears in it at all.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded: SemioMeshDiff = serde_json::from_str(DIFF).expect("committed change-material-base-color diff decodes");
    let materials = decoded.materials.as_ref().expect("change-material-base-color must write the materials triple");
    assert!(materials.removed.is_empty() && materials.added.is_empty(), "a change is a per-field modification");
    let mdiff = &materials.modified[0].diff;
    assert!(mdiff.base_color.is_some(), "the base color must be written");
    assert!(mdiff.metallic.is_none() && mdiff.roughness.is_none(), "neither PBR factor may be written");
    assert!(decoded.meshes.is_none() && decoded.textures.is_none(), "no mesh or texture slot may appear in the diff");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-material-base-color/repaints-the-material-from-red-to-blue: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioMeshDiff = serde_json::from_str(DIFF).expect("committed change-material-base-color diff decodes");
    let produced = decoded.apply(&before()).expect("committed change-material-base-color diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-material-base-color/repaints-the-material-from-red-to-blue: committed diff did not carry before to after");
}
