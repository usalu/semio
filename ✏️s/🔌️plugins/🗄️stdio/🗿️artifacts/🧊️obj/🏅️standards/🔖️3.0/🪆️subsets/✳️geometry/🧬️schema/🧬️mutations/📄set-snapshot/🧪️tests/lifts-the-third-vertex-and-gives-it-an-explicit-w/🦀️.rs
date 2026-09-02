//! 🧪️ `📄set-snapshot` fixture — `lifts-the-third-vertex-and-gives-it-an-explicit-w`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`.
//!
//! 📍 The case lifts the third `v` line off the z=0 plane and, at the same time, writes the
//! optional fourth component the source had omitted. `ObjVertexDiff::w` is a tri-state
//! `Option<Option<f64>>`; this fixture deliberately exercises only its ROUND-TRIPPABLE direction
//! (`Some(Some(1.0))`, serialized as `1.0`). The opposite direction — dropping `w` back to absent —
//! is `Some(None)`, which serde writes as bare `null` and reads back as `None` (= unchanged), so no
//! committed fixture may express it; the same caveat applies to `ObjDiff::mtllib`, kept `None` on
//! both sides here.
//!
//! Faces, normals, the `shell` group, the `tri` object, the `usemtl` range and the retained
//! comment line are all identical across the change, so none of them may appear in the diff.

use crate::artifacts::obj::schema::diff::ObjDiff;
use crate::artifacts::obj::schema::mutations::{apply_obj_mutation, ObjMutation};
use crate::artifacts::obj::schema::snapshot::ObjSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> ObjSnapshot {
    serde_json::from_str(BEFORE).expect("before OBJ snapshot decodes")
}
fn expected_after() -> ObjSnapshot {
    serde_json::from_str(AFTER).expect("after OBJ snapshot decodes")
}
fn mutation() -> ObjMutation {
    serde_json::from_str(MUTATION).expect("set-snapshot mutation decodes")
}

/// ▶️ `set-snapshot` carries the single-triangle mesh to exactly the committed `after`: vertex 2 is
/// lifted and now carries an explicit `w`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    let outcome = apply_obj_mutation(&mut snapshot, &mutation());
    assert!(outcome.messages().is_empty(), "obj/set-snapshot: a genuinely changed mesh must not raise any message");
    assert_eq!(snapshot.vertices[2].z, 1.5, "obj/set-snapshot: the third v line must be lifted off the z=0 plane");
    assert_eq!(snapshot.vertices[2].w, Some(1.0), "obj/set-snapshot: the omitted fourth component must become explicit");
    assert_eq!(snapshot.faces, before().faces, "obj/set-snapshot: moving a vertex must not disturb the f line that references it");
    assert_eq!(snapshot, expected_after(), "obj/set-snapshot: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse of `set-snapshot` is `set-snapshot(base)` — it must drop `w` back to absent and
/// return the vertex to the z=0 plane.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <ObjMutation as protocol::Mutation<ObjSnapshot>>::inverse(&mutation, &base);
    let mut snapshot = base.clone();
    apply_obj_mutation(&mut snapshot, &mutation);
    for step in &inverse {
        apply_obj_mutation(&mut snapshot, step);
    }
    assert_eq!(snapshot.vertices[2].w, None, "obj/set-snapshot: the inverse must make the fourth component implicit again");
    assert_eq!(snapshot, base, "obj/set-snapshot: inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed OBJ snapshots and the mutation are already canonical: an omitted `w`/
/// `texcoord`/`mtllib` is SKIPPED (never written as `null`), while empty collections such as
/// `texcoords` and `smoothingGroups` are still written as `[]` because the snapshot's `Vec` fields
/// carry no `skip_serializing_if`.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: ObjSnapshot = serde_json::from_str(text).expect("OBJ snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("OBJ snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("OBJ snapshot reparses");
        assert_eq!(reencoded, original, "obj/set-snapshot: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("set-snapshot mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("set-snapshot mutation reparses");
    assert_eq!(reencoded, original, "obj/set-snapshot: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome is `applied` — the mesh really moves, so no diagnostic is raised.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    assert_eq!(status, "applied", "obj/set-snapshot: this fixture declares an applied outcome");
    let mut snapshot = before();
    let produced = apply_obj_mutation(&mut snapshot, &mutation());
    assert!(produced.messages().is_empty(), "obj/set-snapshot: declared applied, so no diagnostic may be raised");
    assert_ne!(snapshot, before(), "obj/set-snapshot: an applied set-snapshot must actually move the mesh");
}

/// 🔺️ The sparse `ObjDiff` this mutation produces is exactly the committed diff — the load-bearing
/// assertion: nine of the ten top-level slots must stay absent, vertices 0 and 1 must not appear
/// in `vertices.modified`, and the patched vertex must carry only `z` and `w`.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <ObjMutation as protocol::Mutation<ObjSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced OBJ diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed OBJ diff decodes");
    assert_eq!(produced, committed, "obj/set-snapshot: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to `ObjDiff`: one vertex patched in
/// place, `w` surviving the round trip as `Some(Some(1.0))`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: ObjDiff = serde_json::from_str(DIFF).expect("committed OBJ diff decodes");
    assert!(decoded.faces.is_none() && decoded.normals.is_none() && decoded.groups.is_none() && decoded.objects.is_none(), "obj/set-snapshot: no collection other than vertices may be touched");
    assert!(decoded.mtllib.is_none() && decoded.usemtl.is_none() && decoded.smoothing_groups.is_none() && decoded.unknown_statements.is_none(), "obj/set-snapshot: the material, smoothing and raw-retention slots must stay absent");
    let vertices = decoded.vertices.as_ref().expect("the committed diff carries a vertices triple");
    assert!(vertices.removed.is_empty() && vertices.added.is_empty() && vertices.modified.len() == 1 && vertices.modified[0].index == 2, "obj/set-snapshot: exactly vertex 2 may be patched in place");
    assert_eq!(vertices.modified[0].diff.w, Some(Some(1.0)), "obj/set-snapshot: the tri-state w must decode as 'set to 1.0', not as 'unchanged'");
    assert!(vertices.modified[0].diff.x.is_none() && vertices.modified[0].diff.y.is_none(), "obj/set-snapshot: the untouched x/y components must stay absent");
    let reencoded = serde_json::to_value(&decoded).expect("OBJ diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed OBJ diff reparses");
    assert_eq!(reencoded, original, "obj/set-snapshot: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields the committed `after` — the two
/// vertex fields are a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: ObjDiff = serde_json::from_str(DIFF).expect("committed OBJ diff decodes");
    let produced = <ObjDiff as protocol::MutationDiff<ObjSnapshot>>::apply(&decoded, &before()).expect("committed OBJ diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "obj/set-snapshot: committed diff did not carry before to after");
}
