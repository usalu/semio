//! 🧪️ `set-snapshot` fixture — `lifts-the-second-point-and-stretches-the-z-bound`.
//!
//! `LasDiff` flattens the whole LAS public header block into top-level scalars and keeps
//! `vlrs`/`points` as index-keyed triples, so `LasDiff::between` compares 24 header fields
//! one by one. Raising one point's Z therefore has to show up as exactly two things: the
//! `maxZ` bounding-box scalar and a single `LasPointModified` whose inner patch names only
//! `z`. Every other header scalar — including the STRUCTURAL ones the encoder recomputes,
//! like `numberOfPointRecords` — must stay absent, and the empty `vlrs` list must not
//! produce a triple at all.
//! `LasPointDiff::gpsTime`/`rgb` are tri-state `Option<Option<_>>` slots; this point-format-0
//! fixture carries neither, so the committed patch simply omits them.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`), every value of which was transcribed from this
//! leaf's own `🔺️diff/🦀️component.rs` oracle. The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::las::standards::v1_0::subsets::any::schema::diff::LasDiff;
use crate::artifacts::las::standards::v1_0::subsets::any::schema::mutations::{apply_las_mutation, LasMutation};
use crate::artifacts::las::standards::v1_0::subsets::any::schema::snapshot::LasSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> LasSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> LasSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> LasMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ `set-snapshot` carries the committed `before` LasSnapshot to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    let outcome = apply_las_mutation(&mut snapshot, &mutation());
    assert!(outcome.messages().is_empty(), "set-snapshot/lifts-the-second-point-and-stretches-the-z-bound: set-snapshot raised diagnostics it should not have");
    assert_eq!(snapshot, expected_after(), "set-snapshot/lifts-the-second-point-and-stretches-the-z-bound: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.points[1].z, 2.5, "set-snapshot/lifts-the-second-point-and-stretches-the-z-bound: the second point must rise to z = 2.5");
    assert_eq!(snapshot.points[0], before().points[0], "set-snapshot/lifts-the-second-point-and-stretches-the-z-bound: the first point is identical on both sides and must survive untouched");
    assert_eq!(snapshot.header.max_z, 2.5, "set-snapshot/lifts-the-second-point-and-stretches-the-z-bound: the header bounding box must grow with the point");
    assert_eq!(snapshot.header.min_z, 0.0, "set-snapshot/lifts-the-second-point-and-stretches-the-z-bound: the lower Z bound does not move");
    assert_eq!(snapshot.header.number_of_point_records, 2, "set-snapshot/lifts-the-second-point-and-stretches-the-z-bound: the point count is unchanged — no point is added or removed");
    assert!(snapshot.vlrs.is_empty(), "set-snapshot/lifts-the-second-point-and-stretches-the-z-bound: this fixture carries no variable-length records on either side");
}

/// ↩️ `set-snapshot`'s inverse is a single `SetSnapshot` carrying the pre-state LasSnapshot back, so
/// forward-then-undo restores `before` byte for byte.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <LasMutation as protocol::Mutation<LasSnapshot>>::inverse(&mutation, &base);
    assert_eq!(inverse.len(), 1, "set-snapshot/lifts-the-second-point-and-stretches-the-z-bound: undoing a whole-snapshot replacement is exactly one step");
    assert!(matches!(inverse[0], LasMutation::SetSnapshot { .. }), "set-snapshot/lifts-the-second-point-and-stretches-the-z-bound: the undo step must itself be a SetSnapshot carrying the pre-state");
    let mut snapshot = base.clone();
    apply_las_mutation(&mut snapshot, &mutation);
    for step in &inverse {
        apply_las_mutation(&mut snapshot, step);
    }
    assert_eq!(snapshot, base, "set-snapshot/lifts-the-second-point-and-stretches-the-z-bound: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed LasSnapshot snapshots and this leaf's committed mutation payload are already
/// canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: LasSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "set-snapshot/lifts-the-second-point-and-stretches-the-z-bound: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "set-snapshot/lifts-the-second-point-and-stretches-the-z-bound: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome — status AND every diagnostic this leaf's own diff builder raises for
/// this payload — matches what the mutation actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let declared: Vec<(String, String)> =
        outcome.get("messages").and_then(serde_json::Value::as_array).map(|rows| rows.iter().map(|row| (row["level"].as_str().unwrap_or_default().to_string(), row["code"].as_str().unwrap_or_default().to_string())).collect()).unwrap_or_default();
    let raised = <LasMutation as protocol::Mutation<LasSnapshot>>::diff(&mutation(), &before());
    let produced: Vec<(String, String)> = raised
        .messages()
        .iter()
        .map(|message| {
            let level = serde_json::to_value(message.level).expect("severity encodes");
            (level.as_str().unwrap_or_default().to_string(), message.code.0.clone())
        })
        .collect();
    assert_eq!(produced, declared, "set-snapshot/lifts-the-second-point-and-stretches-the-z-bound: raised diagnostics differ from the committed 🎯️outcome messages");
    let mut snapshot = before();
    apply_las_mutation(&mut snapshot, &mutation());
    match status {
        "applied" => assert_ne!(snapshot, before(), "set-snapshot/lifts-the-second-point-and-stretches-the-z-bound: declared applied but the snapshot came back unchanged"),
        "rejected" => assert_eq!(snapshot, before(), "set-snapshot/lifts-the-second-point-and-stretches-the-z-bound: a rejected mutation must leave the snapshot untouched"),
        other => panic!("set-snapshot/lifts-the-second-point-and-stretches-the-z-bound: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The sparse delta this leaf produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: `set-snapshot` has NO whole-snapshot replacement slot
/// in LasDiff, so the delta must name only the fields that actually differ.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let raised = <LasMutation as protocol::Mutation<LasSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(raised.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "set-snapshot/lifts-the-second-point-and-stretches-the-z-bound: produced diff differs from the committed 🔺️diff/🔣️component.json");
    assert!(raised.diff().vlrs.is_none(), "set-snapshot/lifts-the-second-point-and-stretches-the-z-bound: an empty VLR list on both sides must not produce a triple at all");
    assert!(raised.diff().number_of_point_records.is_none(), "set-snapshot/lifts-the-second-point-and-stretches-the-z-bound: the STRUCTURAL point count is equal on both sides and must stay out of the delta");
    assert!(
        raised.diff().min_z.is_none() && raised.diff().max_x.is_none() && raised.diff().max_y.is_none(),
        "set-snapshot/lifts-the-second-point-and-stretches-the-z-bound: only the maxZ bound moves — the other five bounding-box scalars must stay absent"
    );
    assert_eq!(raised.diff().max_z, Some(2.5), "set-snapshot/lifts-the-second-point-and-stretches-the-z-bound: maxZ is the one header scalar this payload moves");
    let points = raised.diff().points.as_ref().expect("set-snapshot/lifts-the-second-point-and-stretches-the-z-bound: the points triple must be present");
    assert!(points.removed.is_empty() && points.added.is_empty(), "set-snapshot/lifts-the-second-point-and-stretches-the-z-bound: raising a point never adds or drops one");
    assert_eq!(points.modified[0].index, 1, "set-snapshot/lifts-the-second-point-and-stretches-the-z-bound: LasPointModified indices are BASE-state indices");
    assert_eq!(points.modified[0].diff.z, Some(2.5), "set-snapshot/lifts-the-second-point-and-stretches-the-z-bound: the point patch names z");
    assert!(
        points.modified[0].diff.x.is_none() && points.modified[0].diff.y.is_none() && points.modified[0].diff.intensity.is_none(),
        "set-snapshot/lifts-the-second-point-and-stretches-the-z-bound: X, Y and intensity are unchanged on that point and must not be rewritten"
    );
}

/// 🔣️ The committed diff is itself canonical and decodes to LasDiff.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: LasDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "set-snapshot/lifts-the-second-point-and-stretches-the-z-bound: committed diff JSON is not canonical");
    let point_patch = &decoded.points.as_ref().expect("points triple").modified[0].diff;
    assert!(
        point_patch.gps_time.is_none() && point_patch.rgb.is_none(),
        "set-snapshot/lifts-the-second-point-and-stretches-the-z-bound: the tri-state gpsTime/rgb slots must round-trip as absent — a committed null would collapse the Some(None) 'field cleared' state that Option<Option<_>> cannot express in JSON"
    );
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is
/// a complete description of what this `set-snapshot` changed, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: LasDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <LasDiff as protocol::MutationDiff<LasSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "set-snapshot/lifts-the-second-point-and-stretches-the-z-bound: committed diff did not carry before to after");
}
