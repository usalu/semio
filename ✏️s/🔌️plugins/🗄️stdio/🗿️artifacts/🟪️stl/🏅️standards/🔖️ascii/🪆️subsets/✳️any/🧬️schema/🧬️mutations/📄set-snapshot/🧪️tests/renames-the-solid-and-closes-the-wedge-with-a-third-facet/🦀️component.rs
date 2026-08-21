//! 🧪️ `📄set-snapshot` fixture — `renames-the-solid-and-closes-the-wedge-with-a-third-facet`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`.
//!
//! 🔺️ The case renames the `solid`/`endsolid` token and appends a third facet to the triangle
//! soup. The two existing facets are byte-identical across the change, so `StlDiff::between` must
//! produce a `triangles` triple whose `modified` and `removed` lists are BOTH empty — an
//! index-keyed append, never a re-listing of the whole soup.

use crate::artifacts::stl::schema::diff::StlDiff;
use crate::artifacts::stl::schema::mutations::{apply_stl_mutation, StlMutation};
use crate::artifacts::stl::schema::snapshot::StlSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> StlSnapshot {
    serde_json::from_str(BEFORE).expect("before STL snapshot decodes")
}
fn expected_after() -> StlSnapshot {
    serde_json::from_str(AFTER).expect("after STL snapshot decodes")
}
fn mutation() -> StlMutation {
    serde_json::from_str(MUTATION).expect("set-snapshot mutation decodes")
}

/// ▶️ `set-snapshot` carries the wedge to exactly the committed `after`: a renamed solid with three
/// facets.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    let outcome = apply_stl_mutation(&mut snapshot, &mutation());
    assert!(outcome.messages().is_empty(), "stl/set-snapshot: a genuinely changed solid must not raise any message");
    assert_eq!(snapshot.solid_name, "wedge-v2", "stl/set-snapshot: the solid/endsolid name token must be rewritten");
    assert_eq!(snapshot.triangles.len(), 3, "stl/set-snapshot: the third facet must be appended");
    assert_eq!(snapshot.triangles[0], before().triangles[0], "stl/set-snapshot: the pre-existing facets must be carried over untouched");
    assert_eq!(snapshot, expected_after(), "stl/set-snapshot: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse of `set-snapshot` is `set-snapshot(base)` — it must drop the appended facet and
/// restore the original solid name.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <StlMutation as protocol::Mutation<StlSnapshot>>::inverse(&mutation, &base);
    let mut snapshot = base.clone();
    apply_stl_mutation(&mut snapshot, &mutation);
    for step in &inverse {
        apply_stl_mutation(&mut snapshot, step);
    }
    assert_eq!(snapshot, base, "stl/set-snapshot: inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed STL snapshots and the mutation are already canonical: every facet spells out
/// its persisted `normal` (never recomputed from winding) and its own three self-contained
/// vertices as a nested `[[f64; 3]; 3]` array — STL has no shared vertex index space.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: StlSnapshot = serde_json::from_str(text).expect("STL snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("STL snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("STL snapshot reparses");
        assert_eq!(reencoded, original, "stl/set-snapshot: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("set-snapshot mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("set-snapshot mutation reparses");
    assert_eq!(reencoded, original, "stl/set-snapshot: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome is `applied` — the solid really moves, so no diagnostic is raised.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    assert_eq!(status, "applied", "stl/set-snapshot: this fixture declares an applied outcome");
    let mut snapshot = before();
    let produced = apply_stl_mutation(&mut snapshot, &mutation());
    assert!(produced.messages().is_empty(), "stl/set-snapshot: declared applied, so no diagnostic may be raised");
    assert_ne!(snapshot, before(), "stl/set-snapshot: an applied set-snapshot must actually move the solid");
}

/// 🔺️ The sparse `StlDiff` this mutation produces is exactly the committed diff — the load-bearing
/// assertion: appending a facet must not report the two unchanged facets as `modified`, and the
/// appended entry's `index` is its position in the FINAL sequence.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <StlMutation as protocol::Mutation<StlSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced STL diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed STL diff decodes");
    assert_eq!(produced, committed, "stl/set-snapshot: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to `StlDiff`: a renamed solid plus a
/// pure append.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: StlDiff = serde_json::from_str(DIFF).expect("committed STL diff decodes");
    assert_eq!(decoded.solid_name.as_deref(), Some("wedge-v2"), "stl/set-snapshot: the committed diff must carry the new solid name");
    let triangles = decoded.triangles.as_ref().expect("the committed diff carries a triangles triple");
    assert!(triangles.removed.is_empty() && triangles.modified.is_empty(), "stl/set-snapshot: an append must neither remove nor patch an existing facet");
    assert_eq!(triangles.added.len(), 1, "stl/set-snapshot: exactly one facet is appended");
    assert_eq!(triangles.added[0].index, 2, "stl/set-snapshot: an added index refers to the FINAL sequence position");
    let reencoded = serde_json::to_value(&decoded).expect("STL diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed STL diff reparses");
    assert_eq!(reencoded, original, "stl/set-snapshot: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields the committed `after` — the rename
/// plus append is a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: StlDiff = serde_json::from_str(DIFF).expect("committed STL diff decodes");
    let produced = <StlDiff as protocol::MutationDiff<StlSnapshot>>::apply(&decoded, &before()).expect("committed STL diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "stl/set-snapshot: committed diff did not carry before to after");
}
