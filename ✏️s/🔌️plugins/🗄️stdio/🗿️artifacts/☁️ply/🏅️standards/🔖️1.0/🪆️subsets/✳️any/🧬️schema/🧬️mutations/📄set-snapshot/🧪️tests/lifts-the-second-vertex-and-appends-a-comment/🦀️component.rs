//! 🧪️ `set-snapshot` fixture — `lifts-the-second-vertex-and-appends-a-comment`.
//!
//! PLY elements are NAME-keyed and their rows index-keyed, so `PlyDiff::between` matches the
//! `vertex` element by name and then recurses into `rows_between`. Because the property
//! declarations are identical on both sides, `PlyElementDiff::properties` must stay unset and
//! the row patch is a `PlyRowFieldChange` addressed by PROPERTY NAME (`z`) rather than by
//! cell position — that name-addressing is what survives a later property reorder.
//! `comments` is a whole-value `Option<Vec<String>>` slot (comment position is meaningful in
//! PLY), so appending one re-states the entire list; `PlyValue` is adjacently tagged, so a
//! cell encodes as `{\"kind\":\"float\",\"value\":…}`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`), every value of which was transcribed from this
//! leaf's own `🔺️diff/🦀️component.rs` oracle. The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::ply::standards::v1_0::subsets::any::schema::diff::PlyDiff;
use crate::artifacts::ply::standards::v1_0::subsets::any::schema::mutations::{apply_ply_mutation, PlyMutation};
use crate::artifacts::ply::standards::v1_0::subsets::any::schema::snapshot::PlySnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> PlySnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> PlySnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> PlyMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ `set-snapshot` carries the committed `before` PlySnapshot to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    let outcome = apply_ply_mutation(&mut snapshot, &mutation());
    assert!(outcome.messages().is_empty(), "set-snapshot/lifts-the-second-vertex-and-appends-a-comment: set-snapshot raised diagnostics it should not have");
    assert_eq!(snapshot, expected_after(), "set-snapshot/lifts-the-second-vertex-and-appends-a-comment: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.elements[0].rows[1].values[2], crate::artifacts::ply::standards::v1_0::subsets::any::schema::snapshot::PlyValue::Float(2.0), "set-snapshot/lifts-the-second-vertex-and-appends-a-comment: the second vertex's z cell must land on 2.0");
    assert_eq!(snapshot.elements[0].rows[0], before().elements[0].rows[0], "set-snapshot/lifts-the-second-vertex-and-appends-a-comment: the first vertex row is identical on both sides and must survive untouched");
    assert_eq!(snapshot.elements[0].count, 2, "set-snapshot/lifts-the-second-vertex-and-appends-a-comment: apply_element_diff keeps count synced to rows.len(), and no row was added or removed");
    assert_eq!(snapshot.elements[0].properties, before().elements[0].properties, "set-snapshot/lifts-the-second-vertex-and-appends-a-comment: the property declarations are untouched");
    assert_eq!(snapshot.comments, vec!["semio fixture".to_string(), "raised the second vertex".to_string()], "set-snapshot/lifts-the-second-vertex-and-appends-a-comment: the comment list must gain the second entry, in order");
}

/// ↩️ `set-snapshot`'s inverse is a single `SetSnapshot` carrying the pre-state PlySnapshot back, so
/// forward-then-undo restores `before` byte for byte.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <PlyMutation as protocol::Mutation<PlySnapshot>>::inverse(&mutation, &base);
    assert_eq!(inverse.len(), 1, "set-snapshot/lifts-the-second-vertex-and-appends-a-comment: undoing a whole-snapshot replacement is exactly one step");
    assert!(matches!(inverse[0], PlyMutation::SetSnapshot { .. }), "set-snapshot/lifts-the-second-vertex-and-appends-a-comment: the undo step must itself be a SetSnapshot carrying the pre-state");
    let mut snapshot = base.clone();
    apply_ply_mutation(&mut snapshot, &mutation);
    for step in &inverse {
        apply_ply_mutation(&mut snapshot, step);
    }
    assert_eq!(snapshot, base, "set-snapshot/lifts-the-second-vertex-and-appends-a-comment: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed PlySnapshot snapshots and this leaf's committed mutation payload are already
/// canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: PlySnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "set-snapshot/lifts-the-second-vertex-and-appends-a-comment: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "set-snapshot/lifts-the-second-vertex-and-appends-a-comment: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome — status AND every diagnostic this leaf's own diff builder raises for
/// this payload — matches what the mutation actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let declared: Vec<(String, String)> = outcome
        .get("messages")
        .and_then(serde_json::Value::as_array)
        .map(|rows| rows.iter().map(|row| (row["level"].as_str().unwrap_or_default().to_string(), row["code"].as_str().unwrap_or_default().to_string())).collect())
        .unwrap_or_default();
    let raised = <PlyMutation as protocol::Mutation<PlySnapshot>>::diff(&mutation(), &before());
    let produced: Vec<(String, String)> = raised
        .messages()
        .iter()
        .map(|message| {
            let level = serde_json::to_value(message.level).expect("severity encodes");
            (level.as_str().unwrap_or_default().to_string(), message.code.0.clone())
        })
        .collect();
    assert_eq!(produced, declared, "set-snapshot/lifts-the-second-vertex-and-appends-a-comment: raised diagnostics differ from the committed 🎯️outcome messages");
    let mut snapshot = before();
    apply_ply_mutation(&mut snapshot, &mutation());
    match status {
        "applied" => assert_ne!(snapshot, before(), "set-snapshot/lifts-the-second-vertex-and-appends-a-comment: declared applied but the snapshot came back unchanged"),
        "rejected" => assert_eq!(snapshot, before(), "set-snapshot/lifts-the-second-vertex-and-appends-a-comment: a rejected mutation must leave the snapshot untouched"),
        other => panic!("set-snapshot/lifts-the-second-vertex-and-appends-a-comment: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The sparse delta this leaf produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: `set-snapshot` has NO whole-snapshot replacement slot
/// in PlyDiff, so the delta must name only the fields that actually differ.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let raised = <PlyMutation as protocol::Mutation<PlySnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(raised.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "set-snapshot/lifts-the-second-vertex-and-appends-a-comment: produced diff differs from the committed 🔺️diff/🔣️component.json");
    assert!(raised.diff().format.is_none(), "set-snapshot/lifts-the-second-vertex-and-appends-a-comment: the wire format stays ascii and must not be re-stated");
    assert_eq!(raised.diff().comments.as_ref().expect("comments slot").len(), 2, "set-snapshot/lifts-the-second-vertex-and-appends-a-comment: comments is a whole-value slot, so it carries the complete new list");
    let elements = raised.diff().elements.as_ref().expect("set-snapshot/lifts-the-second-vertex-and-appends-a-comment: the elements triple must be present");
    assert!(elements.removed.is_empty() && elements.added.is_empty(), "set-snapshot/lifts-the-second-vertex-and-appends-a-comment: the vertex element is matched by name, not replaced");
    assert_eq!(elements.modified[0].name, "vertex", "set-snapshot/lifts-the-second-vertex-and-appends-a-comment: PlyElementModified is keyed by element NAME");
    assert!(elements.modified[0].diff.properties.is_none(), "set-snapshot/lifts-the-second-vertex-and-appends-a-comment: identical property declarations must never trigger the whole-rows-replace fallback");
    let rows = elements.modified[0].diff.rows.as_ref().expect("set-snapshot/lifts-the-second-vertex-and-appends-a-comment: the rows triple must be present");
    assert_eq!(rows.modified[0].index, 1, "set-snapshot/lifts-the-second-vertex-and-appends-a-comment: PlyRowModified indices are BASE-state row positions");
    assert_eq!(rows.modified[0].diff.fields.len(), 1, "set-snapshot/lifts-the-second-vertex-and-appends-a-comment: only the z column changed — x and y must not appear");
    assert_eq!(rows.modified[0].diff.fields[0].name, "z", "set-snapshot/lifts-the-second-vertex-and-appends-a-comment: a row field change is addressed by PROPERTY NAME, not by cell index");
}

/// 🔣️ The committed diff is itself canonical and decodes to PlyDiff.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: PlyDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "set-snapshot/lifts-the-second-vertex-and-appends-a-comment: committed diff JSON is not canonical");
    assert_eq!(decoded.format, None, "set-snapshot/lifts-the-second-vertex-and-appends-a-comment: the format slot must round-trip as absent, not as a defaulted PlyFormat::Ascii");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is
/// a complete description of what this `set-snapshot` changed, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: PlyDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <PlyDiff as protocol::MutationDiff<PlySnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "set-snapshot/lifts-the-second-vertex-and-appends-a-comment: committed diff did not carry before to after");
}
