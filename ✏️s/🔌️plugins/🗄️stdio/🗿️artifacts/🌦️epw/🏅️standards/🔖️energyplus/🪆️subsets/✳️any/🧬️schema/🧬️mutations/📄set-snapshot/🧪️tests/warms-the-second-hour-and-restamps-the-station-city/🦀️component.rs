//! 🧪️ `set-snapshot` fixture — `warms-the-second-hour-and-restamps-the-station-city`.
//!
//! EPW's eight header lines are modelled at two different fidelities and the delta has to
//! respect both: `location` and `dataPeriods` are structured sub-documents replaced whole
//! (they are rarely mutated and have no collection worth an algebra), while the other six
//! header lines are verbatim-retained strings with their own flat slots. The hourly records
//! are the opposite — an index-keyed triple whose per-record `EpwRecordDiff` is genuinely
//! sparse across all 35 spec columns.
//! So changing the station city plus one column of hour 2 must produce exactly one whole
//! `EpwLocation` and one record patch naming `dryBulbTemp` and nothing else; the other 34
//! columns of that record, and hour 1 entirely, must be absent.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`), every value of which was transcribed from this
//! leaf's own `🔺️diff/🦀️component.rs` oracle. The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::epw::standards::energyplus::subsets::any::schema::diff::EpwDiff;
use crate::artifacts::epw::standards::energyplus::subsets::any::schema::mutations::{apply_epw_mutation, EpwMutation};
use crate::artifacts::epw::standards::energyplus::subsets::any::schema::snapshot::EpwSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> EpwSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> EpwSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> EpwMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ `set-snapshot` carries the committed `before` EpwSnapshot to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    let outcome = apply_epw_mutation(&mut snapshot, &mutation());
    assert!(outcome.messages().is_empty(), "set-snapshot/warms-the-second-hour-and-restamps-the-station-city: set-snapshot raised diagnostics it should not have");
    assert_eq!(snapshot, expected_after(), "set-snapshot/warms-the-second-hour-and-restamps-the-station-city: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.location.city, "Hannover-Langenhagen", "set-snapshot/warms-the-second-hour-and-restamps-the-station-city: the LOCATION city token must be restamped");
    assert_eq!(snapshot.location.wmo, "103380", "set-snapshot/warms-the-second-hour-and-restamps-the-station-city: the rest of the LOCATION line rides along unchanged inside the whole-substruct slot");
    assert_eq!(snapshot.records[1].dry_bulb_temp, "5.6", "set-snapshot/warms-the-second-hour-and-restamps-the-station-city: hour 2's dry-bulb column must warm to 5.6");
    assert_eq!(snapshot.records[1].dew_point_temp, before().records[1].dew_point_temp, "set-snapshot/warms-the-second-hour-and-restamps-the-station-city: hour 2's dew-point column is untouched");
    assert_eq!(snapshot.records[0], before().records[0], "set-snapshot/warms-the-second-hour-and-restamps-the-station-city: hour 1 is identical on both sides and must survive untouched");
    assert_eq!(snapshot.comments_1, "COMMENTS 1,semio fixture", "set-snapshot/warms-the-second-hour-and-restamps-the-station-city: the verbatim-retained COMMENTS 1 header line does not move");
}

/// ↩️ `set-snapshot`'s inverse is a single `SetSnapshot` carrying the pre-state EpwSnapshot back, so
/// forward-then-undo restores `before` byte for byte.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <EpwMutation as protocol::Mutation<EpwSnapshot>>::inverse(&mutation, &base);
    assert_eq!(inverse.len(), 1, "set-snapshot/warms-the-second-hour-and-restamps-the-station-city: undoing a whole-snapshot replacement is exactly one step");
    assert!(matches!(inverse[0], EpwMutation::SetSnapshot { .. }), "set-snapshot/warms-the-second-hour-and-restamps-the-station-city: the undo step must itself be a SetSnapshot carrying the pre-state");
    let mut snapshot = base.clone();
    apply_epw_mutation(&mut snapshot, &mutation);
    for step in &inverse {
        apply_epw_mutation(&mut snapshot, step);
    }
    assert_eq!(snapshot, base, "set-snapshot/warms-the-second-hour-and-restamps-the-station-city: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed EpwSnapshot snapshots and this leaf's committed mutation payload are already
/// canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: EpwSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "set-snapshot/warms-the-second-hour-and-restamps-the-station-city: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "set-snapshot/warms-the-second-hour-and-restamps-the-station-city: committed mutation JSON is not canonical");
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
    let raised = <EpwMutation as protocol::Mutation<EpwSnapshot>>::diff(&mutation(), &before());
    let produced: Vec<(String, String)> = raised
        .messages()
        .iter()
        .map(|message| {
            let level = serde_json::to_value(message.level).expect("severity encodes");
            (level.as_str().unwrap_or_default().to_string(), message.code.0.clone())
        })
        .collect();
    assert_eq!(produced, declared, "set-snapshot/warms-the-second-hour-and-restamps-the-station-city: raised diagnostics differ from the committed 🎯️outcome messages");
    let mut snapshot = before();
    apply_epw_mutation(&mut snapshot, &mutation());
    match status {
        "applied" => assert_ne!(snapshot, before(), "set-snapshot/warms-the-second-hour-and-restamps-the-station-city: declared applied but the snapshot came back unchanged"),
        "rejected" => assert_eq!(snapshot, before(), "set-snapshot/warms-the-second-hour-and-restamps-the-station-city: a rejected mutation must leave the snapshot untouched"),
        other => panic!("set-snapshot/warms-the-second-hour-and-restamps-the-station-city: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The sparse delta this leaf produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: `set-snapshot` has NO whole-snapshot replacement slot
/// in EpwDiff, so the delta must name only the fields that actually differ.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let raised = <EpwMutation as protocol::Mutation<EpwSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(raised.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "set-snapshot/warms-the-second-hour-and-restamps-the-station-city: produced diff differs from the committed 🔺️diff/🔣️component.json");
    assert!(raised.diff().design_conditions.is_none() && raised.diff().typical_extreme_periods.is_none() && raised.diff().ground_temperatures.is_none() && raised.diff().holidays_dst.is_none() && raised.diff().comments_1.is_none() && raised.diff().comments_2.is_none(), "set-snapshot/warms-the-second-hour-and-restamps-the-station-city: the six verbatim-retained header lines are equal on both sides and must stay out of the delta");
    assert!(raised.diff().data_periods.is_none(), "set-snapshot/warms-the-second-hour-and-restamps-the-station-city: the DATA PERIODS header line is unchanged");
    assert_eq!(raised.diff().location.as_ref().expect("location slot").city, "Hannover-Langenhagen", "set-snapshot/warms-the-second-hour-and-restamps-the-station-city: EpwLocation is a whole-substruct replace slot, so the delta carries the complete new header line");
    let records = raised.diff().records.as_ref().expect("set-snapshot/warms-the-second-hour-and-restamps-the-station-city: the records triple must be present");
    assert!(records.removed.is_empty() && records.added.is_empty(), "set-snapshot/warms-the-second-hour-and-restamps-the-station-city: no hourly record is added or dropped");
    assert_eq!(records.modified[0].index, 1, "set-snapshot/warms-the-second-hour-and-restamps-the-station-city: EpwRecordModified indices are BASE-state indices");
    assert_eq!(records.modified[0].diff.dry_bulb_temp.as_deref(), Some("5.6"), "set-snapshot/warms-the-second-hour-and-restamps-the-station-city: the record patch names the dry-bulb column");
    assert!(records.modified[0].diff.year.is_none() && records.modified[0].diff.hour.is_none() && records.modified[0].diff.wind_speed.is_none(), "set-snapshot/warms-the-second-hour-and-restamps-the-station-city: the other 34 spec columns of that record are unchanged and must not be rewritten");
}

/// 🔣️ The committed diff is itself canonical and decodes to EpwDiff.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: EpwDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "set-snapshot/warms-the-second-hour-and-restamps-the-station-city: committed diff JSON is not canonical");
    assert_eq!(serde_json::from_str::<serde_json::Value>(DIFF).expect("diff reparses").get("records").and_then(|r| r.get("modified")).and_then(|m| m.get(0)).and_then(|m| m.get("diff")).and_then(serde_json::Value::as_object).map(|o| o.len()), Some(1), "set-snapshot/warms-the-second-hour-and-restamps-the-station-city: exactly one of EpwRecordDiff's 35 column slots may appear in the committed record patch");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is
/// a complete description of what this `set-snapshot` changed, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: EpwDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <EpwDiff as protocol::MutationDiff<EpwSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "set-snapshot/warms-the-second-hour-and-restamps-the-station-city: committed diff did not carry before to after");
}
