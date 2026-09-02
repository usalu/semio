//! 🧪️ `set-primitive-material` fixture — `binds-the-primitive-to-the-existing-material`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`: an absent mesh/primitive pair is Error
//! `mutation.target-missing`, an unchanged binding is Warning `mutation.no-op`. Note there is
//! deliberately NO referential check that the material exists — the fixture binds a real one
//! anyway. `SemioPrimitiveDiff::material_id` is a genuine TRI-STATE `Option<Option<String>>`; this
//! case takes the SET arm (`Some(Some(id))`), whose inner value is a real string, so the committed
//! diff survives a JSON round trip (the `null`/unbind arm would not).

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
    serde_json::from_str(BEFORE).expect("set-primitive-material before snapshot decodes")
}
fn expected_after() -> SemioMeshSnapshot {
    serde_json::from_str(AFTER).expect("set-primitive-material after snapshot decodes")
}
fn mutation() -> SemioMeshMutation {
    serde_json::from_str(MUTATION).expect("set-primitive-material mutation decodes")
}

/// ▶️ The primitive gains a material binding; its buffers and draw mode are untouched.
#[semio_framework_async_macros::async_test]
async fn binds_the_material_without_touching_the_geometry() {
    let base = before();
    assert!(base.meshes[0].primitives[0].material_id.is_none(), "the fixture starts from an unbound primitive");
    let produced = mutation().diff(&base).diff().apply(&base).expect("set-primitive-material applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "set-primitive-material/binds-the-primitive-to-the-existing-material: applied state differs from the committed after-snapshot");
    let edited = &produced.meshes[0].primitives[0];
    assert_eq!(edited.material_id.as_deref(), Some("mat-a"), "the binding must become the payload's material id");
    assert!(produced.materials.iter().any(|material| Some(material.id.as_str()) == edited.material_id.as_deref()), "the fixture binds a REAL material even though this leaf does not check that");
    assert_eq!(edited.positions, base.meshes[0].primitives[0].positions, "binding a material must not touch the geometry");
    assert_eq!(produced.materials, base.materials, "binding a material must not rewrite the material itself");
}

/// ↩️ The undo is a `set-primitive-material` carrying BASE's own binding — here `None`, i.e. unbind.
#[semio_framework_async_macros::async_test]
async fn the_undo_set_primitive_material_unbinds_it_again() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "set-primitive-material undoes as exactly one set-primitive-material");
    let SemioMeshMutation::SetPrimitiveMaterial(restore) = &undo[0] else { panic!("set-primitive-material must undo as itself") };
    assert!(restore.material_id.is_none(), "the undo carries BASE's own None — the mutation payload is the FINAL value, not a tri-state");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward set-primitive-material applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo set-primitive-material applies");
    }
    assert_eq!(current, base, "set-primitive-material/binds-the-primitive-to-the-existing-material: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the payload are canonical — the SNAPSHOT always emits `materialId` (explicit `null` when unbound) while the DIFF omits the key entirely when the binding is untouched.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioMeshSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "set-primitive-material/binds-the-primitive-to-the-existing-material: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("set-primitive-material mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("set-primitive-material mutation reparses");
    assert_eq!(reencoded, original, "set-primitive-material/binds-the-primitive-to-the-existing-material: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the primitive exists and the new binding genuinely differs, so neither target-missing nor no-op may fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_as_committed() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "set-primitive-material/binds-the-primitive-to-the-existing-material: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "binding a genuinely different material must raise no diagnostics");
}

/// 🔺️ The sparse delta this mutation produces equals the committed diff. A doubly-nested `primitives.modified` entry carrying `materialId` alone, as a real string.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioMeshMutation as Mutation<SemioMeshSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "set-primitive-material/binds-the-primitive-to-the-existing-material: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point, and only the collection this mutation is
/// allowed to touch appears in it at all.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded: SemioMeshDiff = serde_json::from_str(DIFF).expect("committed set-primitive-material diff decodes");
    let meshes = decoded.meshes.as_ref().expect("the meshes triple must be present");
    assert!(meshes.removed.is_empty() && meshes.added.is_empty(), "the mesh is modified, never removed or re-added");
    let nested = meshes.modified[0].diff.primitives.as_ref().expect("the per-mesh diff must carry a primitives triple");
    assert!(nested.removed.is_empty() && nested.added.is_empty(), "the primitive is modified per field, never removed and re-added");
    let pdiff = &nested.modified[0].diff;
    assert!(matches!(pdiff.material_id, Some(Some(_))), "the binding slot must decode as Some(Some(id)) — bound, not cleared");
    assert!(pdiff.topology.is_none() && pdiff.positions.is_none() && pdiff.indices.is_none(), "neither the draw mode nor any buffer may be written");
    assert!(decoded.materials.is_none() && decoded.textures.is_none(), "no material or texture slot may appear in the diff");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "set-primitive-material/binds-the-primitive-to-the-existing-material: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioMeshDiff = serde_json::from_str(DIFF).expect("committed set-primitive-material diff decodes");
    let produced = decoded.apply(&before()).expect("committed set-primitive-material diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "set-primitive-material/binds-the-primitive-to-the-existing-material: committed diff did not carry before to after");
}
