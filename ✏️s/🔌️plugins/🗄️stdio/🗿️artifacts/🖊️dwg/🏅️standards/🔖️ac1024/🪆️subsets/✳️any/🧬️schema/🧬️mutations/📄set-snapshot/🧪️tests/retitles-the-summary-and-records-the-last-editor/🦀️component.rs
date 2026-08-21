//! 🧪️ `📄set-snapshot` fixture — `retitles-the-summary-and-records-the-last-editor` (AC1024).
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`.
//!
//! 🖊️ The case edits three `SummaryInfo` strings of an AC1024 drawing — title, `lastSavedBy` and
//! revision number — while the version preamble, the two logical layers, the header variables and
//! the auxiliary/application blocks stay put. `DwgDiff` is a per-top-level-field WHOLE-VALUE delta
//! (no sub-field patching inside `summary`), so the committed diff must be exactly one key.

use crate::artifacts::dwg::schema::diff::DwgDiff;
use crate::artifacts::dwg::schema::mutations::{apply_dwg_mutation, DwgMutation};
use crate::artifacts::dwg::schema::snapshot::DwgSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> DwgSnapshot {
    serde_json::from_str(BEFORE).expect("before AC1024 snapshot decodes")
}
fn expected_after() -> DwgSnapshot {
    serde_json::from_str(AFTER).expect("after AC1024 snapshot decodes")
}
fn mutation() -> DwgMutation {
    serde_json::from_str(MUTATION).expect("set-snapshot mutation decodes")
}

/// ▶️ `set-snapshot` carries the drawing to exactly the committed `after`: the summary block now
/// names the plan and its last editor.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    let outcome = apply_dwg_mutation(&mut snapshot, &mutation());
    assert!(outcome.messages().is_empty(), "dwg-ac1024/set-snapshot: a genuinely changed drawing must not raise any message");
    assert_eq!(snapshot.summary.title, "Ground Floor Plan", "dwg-ac1024/set-snapshot: the summary title must be rewritten");
    assert_eq!(snapshot.summary.last_saved_by, "ueli", "dwg-ac1024/set-snapshot: the last editor must be recorded");
    assert_eq!(snapshot.version, "AC1024", "dwg-ac1024/set-snapshot: a summary edit must not disturb the version preamble");
    assert_eq!(snapshot, expected_after(), "dwg-ac1024/set-snapshot: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse of `set-snapshot` is `set-snapshot(base)` — it must restore the untitled summary
/// and the drafting-bot editor.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <DwgMutation as protocol::Mutation<DwgSnapshot>>::inverse(&mutation, &base);
    let mut snapshot = base.clone();
    apply_dwg_mutation(&mut snapshot, &mutation);
    for step in &inverse {
        apply_dwg_mutation(&mut snapshot, step);
    }
    assert_eq!(snapshot, base, "dwg-ac1024/set-snapshot: inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed AC1024 snapshots and the mutation are already canonical: `DwgSnapshot` has no
/// `skip_serializing_if` anywhere, so every header sub-block — every zeroed dimension scalar and
/// every `null` UCS relation handle — is spelled out in full.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: DwgSnapshot = serde_json::from_str(text).expect("AC1024 snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("AC1024 snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("AC1024 snapshot reparses");
        assert_eq!(reencoded, original, "dwg-ac1024/set-snapshot: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("set-snapshot mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("set-snapshot mutation reparses");
    assert_eq!(reencoded, original, "dwg-ac1024/set-snapshot: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome is `applied` — the drawing really moves, so no diagnostic is raised.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    assert_eq!(status, "applied", "dwg-ac1024/set-snapshot: this fixture declares an applied outcome");
    let mut snapshot = before();
    let produced = apply_dwg_mutation(&mut snapshot, &mutation());
    assert!(produced.messages().is_empty(), "dwg-ac1024/set-snapshot: declared applied, so no diagnostic may be raised");
    assert_ne!(snapshot, before(), "dwg-ac1024/set-snapshot: an applied set-snapshot must actually move the drawing");
}

/// 🔺️ The sparse `DwgDiff` this mutation produces is exactly the committed diff — the load-bearing
/// assertion: only `summary` may appear. `drawing`, `header`, `classes`, `dependencies`,
/// `application`, `template`, `auxiliaryHeader`, `revisionHistory`, `preview` and
/// `applicationHistory` are each their own whole-value slot and must stay absent.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <DwgMutation as protocol::Mutation<DwgSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced DWG diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed DWG diff decodes");
    assert_eq!(produced, committed, "dwg-ac1024/set-snapshot: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to `DwgDiff` carrying the summary block
/// alone.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: DwgDiff = serde_json::from_str(DIFF).expect("committed DWG diff decodes");
    assert!(decoded.version.is_none() && decoded.maintenance_version.is_none() && decoded.codepage.is_none(), "dwg-ac1024/set-snapshot: the version preamble must stay out of the delta");
    assert!(decoded.drawing.is_none() && decoded.header.is_none() && decoded.auxiliary_header.is_none() && decoded.application.is_none(), "dwg-ac1024/set-snapshot: no block other than summary may be re-emitted");
    assert_eq!(decoded.summary.as_ref().map(|summary| summary.revision_number.as_str()), Some("2"), "dwg-ac1024/set-snapshot: the whole-value summary must carry the bumped revision number");
    let reencoded = serde_json::to_value(&decoded).expect("DWG diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed DWG diff reparses");
    assert_eq!(reencoded, original, "dwg-ac1024/set-snapshot: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields the committed `after` — the
/// whole-value summary slot is a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: DwgDiff = serde_json::from_str(DIFF).expect("committed DWG diff decodes");
    let produced = <DwgDiff as protocol::MutationDiff<DwgSnapshot>>::apply(&decoded, &before()).expect("committed DWG diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "dwg-ac1024/set-snapshot: committed diff did not carry before to after");
}
