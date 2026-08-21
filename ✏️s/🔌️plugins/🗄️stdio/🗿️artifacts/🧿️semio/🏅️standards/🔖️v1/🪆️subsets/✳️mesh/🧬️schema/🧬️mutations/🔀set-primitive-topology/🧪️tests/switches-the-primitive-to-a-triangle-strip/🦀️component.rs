//! 🧪️ `set-primitive-topology` fixture — `switches-the-primitive-to-a-triangle-strip`.
//!
//! Transcribed from `../../🔺️diff/🦀️component.rs`: an absent mesh/primitive pair is Error
//! `mutation.target-missing`, an unchanged topology is Warning `mutation.no-op`. The per-primitive
//! diff sets `topology` and leaves ALL SIX buffer fields plus `material_id` at `None` — switching
//! draw mode must not touch a single vertex, which is exactly what those `None`s encode.

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
    serde_json::from_str(BEFORE).expect("set-primitive-topology before snapshot decodes")
}
fn expected_after() -> SemioMeshSnapshot {
    serde_json::from_str(AFTER).expect("set-primitive-topology after snapshot decodes")
}
fn mutation() -> SemioMeshMutation {
    serde_json::from_str(MUTATION).expect("set-primitive-topology mutation decodes")
}

/// ▶️ Only the draw mode changes; every vertex buffer and the material binding survive.
#[semio_framework_async_macros::async_test]
async fn switches_the_draw_mode_without_touching_a_buffer() {
    let base = before();
    let produced = mutation().diff(&base).diff().apply(&base).expect("set-primitive-topology applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "set-primitive-topology/switches-the-primitive-to-a-triangle-strip: applied state differs from the committed after-snapshot");
    let edited = &produced.meshes[0].primitives[0];
    assert_ne!(edited.topology, base.meshes[0].primitives[0].topology, "the topology really must have changed");
    assert_eq!(edited.positions, base.meshes[0].primitives[0].positions, "changing draw mode must not rewrite the position buffer");
    assert_eq!(edited.indices, base.meshes[0].primitives[0].indices, "changing draw mode must not rewrite the index buffer");
    assert_eq!(edited.material_id, base.meshes[0].primitives[0].material_id, "changing draw mode must not rebind the material");
}

/// ↩️ The undo is a `set-primitive-topology` carrying BASE's captured topology.
#[semio_framework_async_macros::async_test]
async fn the_undo_set_primitive_topology_restores_the_triangle_mode() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "set-primitive-topology undoes as exactly one set-primitive-topology");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward set-primitive-topology applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo set-primitive-topology applies");
    }
    assert_eq!(current, base, "set-primitive-topology/switches-the-primitive-to-a-triangle-strip: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the payload are canonical — `SemioTopology` is `rename_all = "camelCase"`, so `TriangleStrip` encodes as `"triangleStrip"`.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioMeshSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "set-primitive-topology/switches-the-primitive-to-a-triangle-strip: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("set-primitive-topology mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("set-primitive-topology mutation reparses");
    assert_eq!(reencoded, original, "set-primitive-topology/switches-the-primitive-to-a-triangle-strip: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the primitive exists and the new topology genuinely differs, so neither target-missing nor no-op may fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_as_committed() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "set-primitive-topology/switches-the-primitive-to-a-triangle-strip: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "switching to a genuinely different draw mode must raise no diagnostics");
}

/// 🔺️ The sparse delta this mutation produces equals the committed diff. A doubly-nested `primitives.modified` entry whose per-primitive diff carries `topology` alone.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioMeshMutation as Mutation<SemioMeshSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "set-primitive-topology/switches-the-primitive-to-a-triangle-strip: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point, and only the collection this mutation is
/// allowed to touch appears in it at all.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded: SemioMeshDiff = serde_json::from_str(DIFF).expect("committed set-primitive-topology diff decodes");
    let meshes = decoded.meshes.as_ref().expect("the meshes triple must be present");
    assert!(meshes.removed.is_empty() && meshes.added.is_empty(), "the mesh is modified, never removed or re-added");
    let nested = meshes.modified[0].diff.primitives.as_ref().expect("the per-mesh diff must carry a primitives triple");
    assert!(nested.removed.is_empty() && nested.added.is_empty(), "the primitive is modified per field, never removed and re-added");
    let pdiff = &nested.modified[0].diff;
    assert!(pdiff.topology.is_some(), "the topology field must be written");
    assert!(pdiff.positions.is_none() && pdiff.normals.is_none() && pdiff.uvs.is_none() && pdiff.colors.is_none() && pdiff.indices.is_none(), "no buffer field may be written");
    assert!(pdiff.material_id.is_none(), "the material binding must stay untouched");
    assert!(decoded.materials.is_none() && decoded.textures.is_none(), "no material or texture slot may appear in the diff");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "set-primitive-topology/switches-the-primitive-to-a-triangle-strip: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioMeshDiff = serde_json::from_str(DIFF).expect("committed set-primitive-topology diff decodes");
    let produced = decoded.apply(&before()).expect("committed set-primitive-topology diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "set-primitive-topology/switches-the-primitive-to-a-triangle-strip: committed diff did not carry before to after");
}
