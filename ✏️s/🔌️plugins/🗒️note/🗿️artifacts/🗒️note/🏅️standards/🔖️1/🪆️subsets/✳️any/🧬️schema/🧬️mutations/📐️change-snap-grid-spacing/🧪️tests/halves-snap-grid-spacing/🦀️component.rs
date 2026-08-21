//! 🧪️ `change-snap-grid-spacing` fixture — `halves-snap-grid-spacing`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::note::schema::mutations::{apply_note_mutation, inverse_note_mutation, NoteMutation};
use crate::artifacts::note::{NoteDiff, NoteSnapshot};
use protocol::Mutation;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> NoteSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> NoteSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> NoteMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ `change-snap-grid-spacing` writes `NoteDiff.snap_grid_spacing` only.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let applied = apply_note_mutation(&before(), &mutation()).expect("change-snap-grid-spacing applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-snap-grid-spacing/halves-snap-grid-spacing: applied state differs from committed after-snapshot");
}

/// ↩️ The inverse restores the base's own `snap_grid_spacing`, here `Some(8.0)`.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let mut snapshot = apply_note_mutation(&base, &forward).expect("change-snap-grid-spacing applies forward");
    let mut undo = inverse_note_mutation(&base, &forward);
    undo.reverse();
    for step in &undo {
        snapshot = apply_note_mutation(&snapshot, step).expect("change-snap-grid-spacing inverse step applies");
    }
    assert_eq!(snapshot, base, "change-snap-grid-spacing/halves-snap-grid-spacing: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: NoteSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-snap-grid-spacing/halves-snap-grid-spacing: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-snap-grid-spacing/halves-snap-grid-spacing: committed mutation JSON is not canonical");
}

/// 🎯️ 4.0 is finite and strictly positive, so the `mutation.invariant` fatal guard does not fire.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    assert_eq!(status, "applied", "change-snap-grid-spacing/halves-snap-grid-spacing: this fixture declares an applied outcome");
    let produced = mutation().diff(&before());
    let blocked = produced.messages().iter().any(|message| matches!(message.level, protocol::Severity::Error | protocol::Severity::Fatal));
    assert!(!blocked, "change-snap-grid-spacing/halves-snap-grid-spacing: declared applied but the diff builder rejected it: {:?}", produced.messages());
    apply_note_mutation(&before(), &mutation()).expect("change-snap-grid-spacing/halves-snap-grid-spacing: declared applied but the diff would not apply");
}

/// 🔺️ Only the scalar `snapGridSpacing` slot is set; the drawn `gridSpacing` stays `None`.
///
/// The single most load-bearing assertion in the fixture: `before`+`after` only prove the end
/// state, whereas this pins WHICH collections and fields this mutation is allowed to touch.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <NoteMutation as protocol::Mutation<NoteSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-snap-grid-spacing/halves-snap-grid-spacing: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff round-trips through the note artifact's own `NoteDiff`: its container is
/// `#[serde(default)]` with no `skip_serializing_if`, so all 23 fields must be present, `null` for
/// every slot `change-snap-grid-spacing` leaves alone.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: NoteDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-snap-grid-spacing/halves-snap-grid-spacing: committed diff JSON is not canonical");
}

/// 🩹 The committed `snapGridSpacing`-only delta carries `before` to `after` on its own.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: NoteDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <NoteDiff as protocol::MutationDiff<NoteSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-snap-grid-spacing/halves-snap-grid-spacing: committed diff did not carry before to after");
}

/// 📐 The SNAP step halves 8→4 while the DRAWN grid spacing stays at 32, and snapping stays off.
#[semio_framework_async_macros::async_test]
async fn snap_step_halves_without_touching_the_drawn_grid() {
    let base = before();
    let applied = apply_note_mutation(&base, &mutation()).expect("change-snap-grid-spacing applies");
    assert_eq!(base.snap_grid_spacing, Some(8.0), "change-snap-grid-spacing/halves-snap-grid-spacing: the base must start at 8.0");
    assert_eq!(applied.snap_grid_spacing, Some(4.0), "change-snap-grid-spacing/halves-snap-grid-spacing: the snap step must halve to 4.0");
    assert_eq!(applied.grid_spacing, Some(32.0), "the drawn grid is a separate setting and must not follow the snap step");
    assert_eq!(applied.snap_enabled, Some(false), "changing the snap step must not enable snapping");
}
