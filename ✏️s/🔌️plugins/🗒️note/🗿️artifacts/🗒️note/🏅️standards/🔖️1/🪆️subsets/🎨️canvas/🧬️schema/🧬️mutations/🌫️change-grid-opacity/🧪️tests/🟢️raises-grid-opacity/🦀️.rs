//! 🧪️ `change-grid-opacity` fixture — `🟢️raises-grid-opacity`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::note::schema::mutations::{apply_note_mutation, inverse_note_mutation, NoteMutation};
use crate::artifacts::note::{NoteDiff, NoteSnapshot};
use protocol::Mutation;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> NoteSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> NoteSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> NoteMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ `change-grid-opacity` writes `NoteDiff.grid_opacity` only.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let applied = apply_note_mutation(&before(), &mutation()).expect("change-grid-opacity applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-grid-opacity/raises-grid-opacity: applied state differs from committed after-snapshot");
}

/// ↩️ The inverse restores the base's own `grid_opacity`, here `Some(0.35)`.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let mut snapshot = apply_note_mutation(&base, &forward).expect("change-grid-opacity applies forward");
    let mut undo = inverse_note_mutation(&base, &forward);
    undo.reverse();
    for step in &undo {
        snapshot = apply_note_mutation(&snapshot, step).expect("change-grid-opacity inverse step applies");
    }
    assert_eq!(snapshot, base, "change-grid-opacity/raises-grid-opacity: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: NoteSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-grid-opacity/raises-grid-opacity: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-grid-opacity/raises-grid-opacity: committed mutation JSON is not canonical");
}

/// 🎯️ 0.75 lies inside this leaf's closed `0.0..=1.0` band, so the `mutation.invariant` fatal guard does not fire.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    assert_eq!(status, "applied", "change-grid-opacity/raises-grid-opacity: this fixture declares an applied outcome");
    let produced = mutation().diff(&before());
    let blocked = produced.messages().iter().any(|message| matches!(message.level, protocol::Severity::Error | protocol::Severity::Fatal));
    assert!(!blocked, "change-grid-opacity/raises-grid-opacity: declared applied but the diff builder rejected it: {:?}", produced.messages());
    apply_note_mutation(&before(), &mutation()).expect("change-grid-opacity/raises-grid-opacity: declared applied but the diff would not apply");
}

/// 🔺️ Only the scalar `gridOpacity` slot is set; `gridVisible` stays `None`, so fading and hiding remain independent.
///
/// The single most load-bearing assertion in the fixture: `before`+`after` only prove the end
/// state, whereas this pins WHICH collections and fields this mutation is allowed to touch.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <NoteMutation as protocol::Mutation<NoteSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-grid-opacity/raises-grid-opacity: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff round-trips through the note artifact's own `NoteDiff`: its container is
/// `#[serde(default)]` with no `skip_serializing_if`, so all 23 fields must be present, `null` for
/// every slot `change-grid-opacity` leaves alone.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: NoteDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-grid-opacity/raises-grid-opacity: committed diff JSON is not canonical");
}

/// 🩹 The committed `gridOpacity`-only delta carries `before` to `after` on its own.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: NoteDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <NoteDiff as protocol::MutationDiff<NoteSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-grid-opacity/raises-grid-opacity: committed diff did not carry before to after");
}

/// 🌫️ Opacity rises 0.35→0.75 and stays inside the closed 0..=1 band this leaf alone enforces.
#[semio_framework_async_macros::async_test]
async fn opacity_rises_and_stays_inside_the_unit_band() {
    let base = before();
    let applied = apply_note_mutation(&base, &mutation()).expect("change-grid-opacity applies");
    assert_eq!(base.grid_opacity, Some(0.35), "change-grid-opacity/raises-grid-opacity: the base must start at 0.35");
    assert_eq!(applied.grid_opacity, Some(0.75), "change-grid-opacity/raises-grid-opacity: opacity must rise to 0.75");
    let opacity = applied.grid_opacity.expect("opacity is set");
    assert!((0.0..=1.0).contains(&opacity), "the applied opacity must satisfy this leaf's own 0..=1 band");
    assert_eq!(applied.grid_visible, Some(true), "fading the grid is not the same as hiding it");
    assert_eq!(applied.grid_spacing, Some(32.0), "fading the grid must not resize it");
}
