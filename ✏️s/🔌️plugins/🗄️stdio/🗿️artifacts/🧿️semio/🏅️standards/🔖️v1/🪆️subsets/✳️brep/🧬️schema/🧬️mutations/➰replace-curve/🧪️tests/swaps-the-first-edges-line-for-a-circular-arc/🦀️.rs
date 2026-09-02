//! 🧪️ `replace-curve` fixture — `swaps-the-first-edges-line-for-a-circular-arc`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`: unknown edge id ⇒ Error
//! `mutation.target-missing`, an identical curve ⇒ Warning `mutation.no-op`. The diff is an
//! `edges.modified` entry whose `BrepEdgeDiff` sets `curve` and leaves `start_vertex`/`end_vertex`
//! at `None` — the geometry is replaced while the topology (which vertices the edge runs between)
//! is explicitly left alone.
use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::SemioBrepDiff;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::mutations::SemioBrepMutation;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> SemioBrepSnapshot {
    serde_json::from_str(BEFORE).expect("replace-curve before snapshot decodes")
}
fn expected_after() -> SemioBrepSnapshot {
    serde_json::from_str(AFTER).expect("replace-curve after snapshot decodes")
}
fn mutation() -> SemioBrepMutation {
    serde_json::from_str(MUTATION).expect("replace-curve mutation decodes")
}

/// ▶️ The edge's curve becomes a circle; its endpoints and every sibling edge stay put.
#[semio_framework_async_macros::async_test]
async fn replaces_the_geometry_without_moving_the_topology() {
    let base = before();
    let produced = mutation().diff(&base).diff().apply(&base).expect("replace-curve applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "replace-curve/swaps-the-first-edges-line-for-a-circular-arc: applied state differs from the committed after-snapshot");
    let edited = produced.edges.iter().find(|edge| edge.id == "e1").expect("the edge is still there — a replace is not a delete");
    assert_ne!(edited.curve, base.edges[0].curve, "the curve really must have changed");
    assert_eq!((edited.start_vertex.as_str(), edited.end_vertex.as_str()), ("v1", "v2"), "replace-curve must not touch which vertices the edge runs between");
    assert_eq!(&produced.edges[1..], &base.edges[1..], "the sibling edges must be byte-identical");
    assert_eq!(produced.vertices, base.vertices, "replacing a curve must not move a vertex");
}

/// ↩️ The undo is a `replace-curve` carrying BASE's captured curve.
#[semio_framework_async_macros::async_test]
async fn the_undo_replace_curve_restores_the_original_line() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "replace-curve of an existing edge undoes as exactly one replace-curve");
    let SemioBrepMutation::ReplaceCurve(restore) = &undo[0] else { panic!("replace-curve must undo as replace-curve") };
    assert_eq!(restore.new_curve, base.edges[0].curve, "the undo must recapture BASE's own curve");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward replace-curve applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo replace-curve applies");
    }
    assert_eq!(current, base, "replace-curve/swaps-the-first-edges-line-for-a-circular-arc: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the payload are canonical — `BrepCurve` is internally tagged on `kind` and carries `rename_all_fields`, so both its VARIANTS and its members are camelCase (`radiusMajor`, `controlPoints`), the spelling `📸️snapshot/🔣️.json` declares. This vector's `circle` arm has only single-word members, so it reads the same either way; the mismatch the missing `rename_all_fields` used to cause was invisible here and was found by the cross-language differential in `🧪️tests/mutate-semio-brep`.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioBrepSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "replace-curve/swaps-the-first-edges-line-for-a-circular-arc: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("replace-curve mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("replace-curve mutation reparses");
    assert_eq!(reencoded, original, "replace-curve/swaps-the-first-edges-line-for-a-circular-arc: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the edge exists and the new curve genuinely differs, so neither target-missing nor no-op may fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_as_committed() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "replace-curve/swaps-the-first-edges-line-for-a-circular-arc: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "replacing a curve with a genuinely different one must raise no diagnostics");
}

/// 🔺️ The sparse delta this mutation produces equals the committed diff. Only `edges.modified`, and its per-edge diff must carry `curve` alone.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioBrepMutation as Mutation<SemioBrepSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "replace-curve/swaps-the-first-edges-line-for-a-circular-arc: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point, and only the collection this mutation is
/// allowed to touch appears in it at all.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded: SemioBrepDiff = serde_json::from_str(DIFF).expect("committed replace-curve diff decodes");

    let edges = decoded.edges.as_ref().expect("replace-curve must write the edges triple");
    assert!(edges.removed.is_empty() && edges.added.is_empty(), "a replace is a per-field modification, never a remove-and-re-add");
    assert_eq!(edges.modified.len(), 1, "exactly one edge is modified");
    assert!(edges.modified[0].diff.start_vertex.is_none() && edges.modified[0].diff.end_vertex.is_none(), "the per-edge diff must leave both endpoint fields unset");
    assert!(decoded.vertices.is_none() && decoded.loops.is_none() && decoded.faces.is_none() && decoded.shells.is_none() && decoded.solids.is_none(), "no other collection may appear in the diff");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "replace-curve/swaps-the-first-edges-line-for-a-circular-arc: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioBrepDiff = serde_json::from_str(DIFF).expect("committed replace-curve diff decodes");
    let produced = decoded.apply(&before()).expect("committed replace-curve diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "replace-curve/swaps-the-first-edges-line-for-a-circular-arc: committed diff did not carry before to after");
}
