//! 🧪️ `📸️set-snapshot` fixture — `🔢️bumps-the-auxiliary-save-counter` (AC1018).
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`.
//!
//! 🗓️ The case models one more save of an AC1018 drawing: `AuxHeader`'s `totalSaves`,
//! `saveGeneration` and `terminalSaveGeneration` all advance together, while the `revisionHistory`
//! block, the single logical layer and the AC1018 version preamble stay put. This tree's own
//! mutation vocabulary is a plain re-export of AC1024's, so the fixture also exercises that the
//! re-export really resolves from the AC1018 module path.

use crate::artifacts::dwg::standards::v_ac1018::subsets::any::schema::mutations::{apply_dwg_mutation, DwgMutation};
use crate::artifacts::dwg::{DwgDiff, DwgSnapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> DwgSnapshot {
    serde_json::from_str(BEFORE).expect("before AC1018 snapshot decodes")
}
fn expected_after() -> DwgSnapshot {
    serde_json::from_str(AFTER).expect("after AC1018 snapshot decodes")
}
fn mutation() -> DwgMutation {
    serde_json::from_str(MUTATION).expect("set-snapshot mutation decodes")
}

/// ▶️ `set-snapshot` carries the AC1018 drawing to exactly the committed `after`: the eighth save
/// is recorded and the save generation advances with it.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    let outcome = apply_dwg_mutation(&mut snapshot, &mutation());
    assert!(outcome.messages().is_empty(), "dwg-ac1018/set-snapshot: a genuinely changed drawing must not raise any message");
    assert_eq!(snapshot.auxiliary_header.total_saves, 8, "dwg-ac1018/set-snapshot: the save counter must advance");
    assert_eq!(snapshot.auxiliary_header.save_generation, u32::from(snapshot.auxiliary_header.terminal_save_generation), "dwg-ac1018/set-snapshot: the terminal save generation must track the current one");
    assert_eq!(snapshot.revision_history.revisions, vec![7], "dwg-ac1018/set-snapshot: the revision history is a separate block and must not follow the save counter");
    assert_eq!(snapshot, expected_after(), "dwg-ac1018/set-snapshot: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse of `set-snapshot` is `set-snapshot(base)` — it must wind the auxiliary header
/// back to seven saves and generation four.
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
    assert_eq!(snapshot, base, "dwg-ac1018/set-snapshot: inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed AC1018 snapshots and the mutation are already canonical: `DwgSnapshot` has no
/// `skip_serializing_if` anywhere, so the auxiliary header's two legacy version stamps and its
/// julian created/updated dates are spelled out even when they are all zeroes.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: DwgSnapshot = serde_json::from_str(text).expect("AC1018 snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("AC1018 snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("AC1018 snapshot reparses");
        assert_eq!(reencoded, original, "dwg-ac1018/set-snapshot: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("set-snapshot mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("set-snapshot mutation reparses");
    assert_eq!(reencoded, original, "dwg-ac1018/set-snapshot: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome is `applied` — the drawing really moves, so no diagnostic is raised.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    assert_eq!(status, "applied", "dwg-ac1018/set-snapshot: this fixture declares an applied outcome");
    let mut snapshot = before();
    let produced = apply_dwg_mutation(&mut snapshot, &mutation());
    assert!(produced.messages().is_empty(), "dwg-ac1018/set-snapshot: declared applied, so no diagnostic may be raised");
    assert_ne!(snapshot, before(), "dwg-ac1018/set-snapshot: an applied set-snapshot must actually move the drawing");
}

/// 🔺️ The sparse `DwgDiff` this mutation produces is exactly the committed diff — the load-bearing
/// assertion: only `auxiliaryHeader` may appear. In particular `revisionHistory` — the block a
/// naive "a save happened" delta would drag along — must stay absent.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <DwgMutation as protocol::Mutation<DwgSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced DWG diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed DWG diff decodes");
    assert_eq!(produced, committed, "dwg-ac1018/set-snapshot: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to `DwgDiff` carrying the auxiliary
/// header alone.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: DwgDiff = serde_json::from_str(DIFF).expect("committed DWG diff decodes");
    assert!(decoded.revision_history.is_none() && decoded.summary.is_none() && decoded.drawing.is_none() && decoded.header.is_none(), "dwg-ac1018/set-snapshot: no block other than the auxiliary header may be re-emitted");
    assert_eq!(decoded.auxiliary_header.as_ref().map(|aux| aux.total_saves), Some(8), "dwg-ac1018/set-snapshot: the whole-value auxiliary header must carry the bumped save count");
    let reencoded = serde_json::to_value(&decoded).expect("DWG diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed DWG diff reparses");
    assert_eq!(reencoded, original, "dwg-ac1018/set-snapshot: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields the committed `after` — the
/// whole-value auxiliary-header slot is a complete description of the save, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: DwgDiff = serde_json::from_str(DIFF).expect("committed DWG diff decodes");
    let produced = <DwgDiff as protocol::MutationDiff<DwgSnapshot>>::apply(&decoded, &before()).expect("committed DWG diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "dwg-ac1018/set-snapshot: committed diff did not carry before to after");
}
