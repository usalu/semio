//! 🧪️ `replace-primitive-geometry` fixture — `swaps-the-triangle-for-a-textured-quad`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`: an absent mesh/primitive pair is Error
//! `mutation.target-missing`; the no-op guard is a FIVE-way conjunction (all of positions, normals,
//! uvs, colors, indices unchanged), so changing any single buffer still applies. The per-primitive
//! diff writes all five buffers as whole values — they are weak parallel data, never sub-diffed per
//! vertex — while `topology` and `material_id` stay `None`.

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
    serde_json::from_str(BEFORE).expect("replace-primitive-geometry before snapshot decodes")
}
fn expected_after() -> SemioMeshSnapshot {
    serde_json::from_str(AFTER).expect("replace-primitive-geometry after snapshot decodes")
}
fn mutation() -> SemioMeshMutation {
    serde_json::from_str(MUTATION).expect("replace-primitive-geometry mutation decodes")
}

/// ▶️ All five buffers are replaced together; the draw mode and material binding survive.
#[semio_framework_async_macros::async_test]
async fn replaces_all_five_buffers_and_keeps_topology_and_material() {
    let base = before();
    let produced = mutation().diff(&base).diff().apply(&base).expect("replace-primitive-geometry applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "replace-primitive-geometry/swaps-the-triangle-for-a-textured-quad: applied state differs from the committed after-snapshot");
    let edited = &produced.meshes[0].primitives[0];
    assert_eq!(edited.positions.len(), 4, "the triangle's three vertices become the quad's four");
    assert_eq!(edited.normals.len(), 4, "the normal buffer stays parallel to positions");
    assert_eq!(edited.uvs.len(), 4, "the uv buffer stays parallel to positions");
    assert_eq!(edited.colors.len(), 4, "the color buffer stays parallel to positions");
    assert_eq!(edited.indices, vec![0u32, 1, 2, 1, 3, 2], "the index buffer describes the two triangles of the quad");
    assert_eq!(edited.topology, base.meshes[0].primitives[0].topology, "replacing geometry must not change the draw mode");
    assert_eq!(edited.material_id, base.meshes[0].primitives[0].material_id, "replacing geometry must not rebind the material");
}

/// ↩️ The undo is a `replace-primitive-geometry` carrying all five of BASE's captured buffers.
#[semio_framework_async_macros::async_test]
async fn the_undo_replace_primitive_geometry_restores_all_five_buffers() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "replace-primitive-geometry undoes as exactly one replace-primitive-geometry");
    let SemioMeshMutation::ReplacePrimitiveGeometry(restore) = &undo[0] else { panic!("replace-primitive-geometry must undo as itself") };
    assert_eq!(restore.positions, base.meshes[0].primitives[0].positions, "the undo must recapture BASE's own position buffer");
    assert_eq!(restore.indices, base.meshes[0].primitives[0].indices, "and BASE's own index buffer");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward replace-primitive-geometry applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo replace-primitive-geometry applies");
    }
    assert_eq!(current, base, "replace-primitive-geometry/swaps-the-triangle-for-a-textured-quad: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the payload are canonical — every coordinate and uv is dyadic, and `SemioUv` encodes as `{"u":…,"v":…}` rather than a bare pair.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioMeshSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "replace-primitive-geometry/swaps-the-triangle-for-a-textured-quad: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("replace-primitive-geometry mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("replace-primitive-geometry mutation reparses");
    assert_eq!(reencoded, original, "replace-primitive-geometry/swaps-the-triangle-for-a-textured-quad: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the primitive exists and at least one buffer genuinely differs, so neither target-missing nor the five-way no-op may fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_as_committed() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "replace-primitive-geometry/swaps-the-triangle-for-a-textured-quad: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "replacing geometry with genuinely different buffers must raise no diagnostics");
}

/// 🔺️ The sparse delta this mutation produces equals the committed diff. A doubly-nested `primitives.modified` entry carrying exactly the five buffer fields.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioMeshMutation as Mutation<SemioMeshSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "replace-primitive-geometry/swaps-the-triangle-for-a-textured-quad: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point, and only the collection this mutation is
/// allowed to touch appears in it at all.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded: SemioMeshDiff = serde_json::from_str(DIFF).expect("committed replace-primitive-geometry diff decodes");
    let meshes = decoded.meshes.as_ref().expect("the meshes triple must be present");
    assert!(meshes.removed.is_empty() && meshes.added.is_empty(), "the mesh is modified, never removed or re-added");
    let nested = meshes.modified[0].diff.primitives.as_ref().expect("the per-mesh diff must carry a primitives triple");
    assert!(nested.removed.is_empty() && nested.added.is_empty(), "the primitive is modified per field, never removed and re-added");
    let pdiff = &nested.modified[0].diff;
    assert!(pdiff.positions.is_some() && pdiff.normals.is_some() && pdiff.uvs.is_some() && pdiff.colors.is_some() && pdiff.indices.is_some(), "all five buffers travel together in one diff");
    assert!(pdiff.topology.is_none() && pdiff.material_id.is_none(), "neither the draw mode nor the material binding may be written");
    assert!(decoded.materials.is_none() && decoded.textures.is_none(), "no material or texture slot may appear in the diff");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "replace-primitive-geometry/swaps-the-triangle-for-a-textured-quad: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioMeshDiff = serde_json::from_str(DIFF).expect("committed replace-primitive-geometry diff decodes");
    let produced = decoded.apply(&before()).expect("committed replace-primitive-geometry diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "replace-primitive-geometry/swaps-the-triangle-for-a-textured-quad: committed diff did not carry before to after");
}
