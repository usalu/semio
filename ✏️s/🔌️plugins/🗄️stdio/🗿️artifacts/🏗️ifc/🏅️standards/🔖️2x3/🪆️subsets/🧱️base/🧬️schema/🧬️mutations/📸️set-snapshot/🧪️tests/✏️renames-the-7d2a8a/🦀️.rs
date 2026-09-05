//! 🧪️ `set-snapshot` fixture — `✏️renames-the-ifcproject-instance`.
//!
//! IFC2X3 keeps the lossless generic Part-21 instance graph, and `Part21Instance` already
//! carries a stable `u64` id — so `Ifc2x3Diff::between` is an id-keyed set/map merge with no
//! position-transport algebra at all: a changed instance appears whole in
//! `upsertedInstances`, and `instanceOrder` stays absent because the id sequence did not
//! move. `Part21Value` is an externally tagged enum with no serde rename, so its arms encode
//! as `{\"Str\": ...}` / `\"Unset\"`, and `Part21Header`'s fields keep their snake_case
//! names — neither follows this artifact's own camelCase envelope.
//! `edmPreamble` is a tri-state `Option<Option<Ifc2x3EdmPreamble>>` whose `Some(None)`
//! 'preamble cleared' state cannot survive a JSON round trip; this payload leaves it alone.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`), every value of which was transcribed from this
//! leaf's own `🔺️diff/🦀️.rs` oracle. The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::ifc::standards::v2x3::subsets::base::schema::diff::Ifc2x3Diff;
use crate::artifacts::ifc::standards::v2x3::subsets::base::schema::mutations::{apply_ifc2x3_mutation, Ifc2x3Mutation};
use crate::artifacts::ifc::standards::v2x3::subsets::base::schema::snapshot::Ifc2x3Snapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> Ifc2x3Snapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> Ifc2x3Snapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> Ifc2x3Mutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ `set-snapshot` carries the committed `before` Ifc2x3Snapshot to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    let outcome = apply_ifc2x3_mutation(&mut snapshot, &mutation());
    assert!(outcome.messages().is_empty(), "set-snapshot/renames-the-ifcproject-instance: set-snapshot raised diagnostics it should not have");
    assert_eq!(snapshot, expected_after(), "set-snapshot/renames-the-ifcproject-instance: applied state differs from committed after-snapshot");
    let args = snapshot.document.instances[0].entity("IFCPROJECT").expect("set-snapshot/renames-the-ifcproject-instance: the instance must still be an IFCPROJECT").clone();
    assert_eq!(args[2].as_str(), Some("Nakagin Capsule Tower"), "set-snapshot/renames-the-ifcproject-instance: the project long name must land on the full tower name");
    assert_eq!(args[0].as_str(), Some("0Xk$Q1nMz9wgLp2vBcRt4A"), "set-snapshot/renames-the-ifcproject-instance: the IFC GlobalId argument is untouched");
    assert!(args[1].is_unset(), "set-snapshot/renames-the-ifcproject-instance: the OwnerHistory argument stays the Part-21 unset marker");
    assert_eq!(snapshot.document.header, before().document.header, "set-snapshot/renames-the-ifcproject-instance: the Part-21 HEADER section is equal on both sides and must survive untouched");
    assert_eq!(snapshot.schema, "stdio.ifc.2x3", "set-snapshot/renames-the-ifcproject-instance: 2x3 keeps its own envelope id, distinct from IFC4's stdio.ifc");
}

/// ↩️ `set-snapshot`'s inverse is a single `SetSnapshot` carrying the pre-state Ifc2x3Snapshot back, so
/// forward-then-undo restores `before` byte for byte.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <Ifc2x3Mutation as protocol::Mutation<Ifc2x3Snapshot>>::inverse(&mutation, &base);
    assert_eq!(inverse.len(), 1, "set-snapshot/renames-the-ifcproject-instance: undoing a whole-snapshot replacement is exactly one step");
    assert!(matches!(inverse[0], Ifc2x3Mutation::SetSnapshot(..)), "set-snapshot/renames-the-ifcproject-instance: the undo step must itself be a SetSnapshot carrying the pre-state");
    let mut snapshot = base.clone();
    apply_ifc2x3_mutation(&mut snapshot, &mutation);
    for step in &inverse {
        apply_ifc2x3_mutation(&mut snapshot, step);
    }
    assert_eq!(snapshot, base, "set-snapshot/renames-the-ifcproject-instance: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed Ifc2x3Snapshot snapshots and this leaf's committed mutation payload are already
/// canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Ifc2x3Snapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "set-snapshot/renames-the-ifcproject-instance: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "set-snapshot/renames-the-ifcproject-instance: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome — status AND every diagnostic this leaf's own diff builder raises for
/// this payload — matches what the mutation actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let declared: Vec<(String, String)> =
        outcome.get("messages").and_then(serde_json::Value::as_array).map(|rows| rows.iter().map(|row| (row["level"].as_str().unwrap_or_default().to_string(), row["code"].as_str().unwrap_or_default().to_string())).collect()).unwrap_or_default();
    let raised = <Ifc2x3Mutation as protocol::Mutation<Ifc2x3Snapshot>>::diff(&mutation(), &before());
    let produced: Vec<(String, String)> = raised
        .messages()
        .iter()
        .map(|message| {
            let level = serde_json::to_value(message.level).expect("severity encodes");
            (level.as_str().unwrap_or_default().to_string(), message.code.0.clone())
        })
        .collect();
    assert_eq!(produced, declared, "set-snapshot/renames-the-ifcproject-instance: raised diagnostics differ from the committed 🎯️outcome messages");
    let mut snapshot = before();
    apply_ifc2x3_mutation(&mut snapshot, &mutation());
    match status {
        "applied" => assert_ne!(snapshot, before(), "set-snapshot/renames-the-ifcproject-instance: declared applied but the snapshot came back unchanged"),
        "rejected" => assert_eq!(snapshot, before(), "set-snapshot/renames-the-ifcproject-instance: a rejected mutation must leave the snapshot untouched"),
        other => panic!("set-snapshot/renames-the-ifcproject-instance: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The sparse delta this leaf produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: `set-snapshot` has NO whole-snapshot replacement slot
/// in Ifc2x3Diff, so the delta must name only the fields that actually differ.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let raised = <Ifc2x3Mutation as protocol::Mutation<Ifc2x3Snapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(raised.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "set-snapshot/renames-the-ifcproject-instance: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert!(raised.diff().schema.is_none(), "set-snapshot/renames-the-ifcproject-instance: the envelope id is equal on both sides");
    assert!(raised.diff().header.is_none(), "set-snapshot/renames-the-ifcproject-instance: the HEADER section is equal on both sides and must not be re-written");
    assert!(raised.diff().removed_instances.is_empty(), "set-snapshot/renames-the-ifcproject-instance: no instance leaves the graph");
    assert!(raised.diff().instance_order.is_none(), "set-snapshot/renames-the-ifcproject-instance: the id sequence is unchanged, so no explicit instance order is needed");
    assert!(raised.diff().edm_preamble.is_none(), "set-snapshot/renames-the-ifcproject-instance: the EDM production preamble is absent on both sides");
    assert_eq!(raised.diff().upserted_instances.len(), 1, "set-snapshot/renames-the-ifcproject-instance: exactly one instance is upserted");
    assert_eq!(raised.diff().upserted_instances[0].id, 1, "set-snapshot/renames-the-ifcproject-instance: Part21Instance ids are the diff's own keys — no index transport is involved");
}

/// 🔣️ The committed diff is itself canonical and decodes to Ifc2x3Diff.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: Ifc2x3Diff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "set-snapshot/renames-the-ifcproject-instance: committed diff JSON is not canonical");
    assert!(
        decoded.edm_preamble.is_none(),
        "set-snapshot/renames-the-ifcproject-instance: edmPreamble must round-trip as absent — a committed null would collapse the Some(None) 'preamble cleared' state that Option<Option<Ifc2x3EdmPreamble>> cannot express in JSON"
    );
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is
/// a complete description of what this `set-snapshot` changed, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: Ifc2x3Diff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <Ifc2x3Diff as protocol::MutationDiff<Ifc2x3Snapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "set-snapshot/renames-the-ifcproject-instance: committed diff did not carry before to after");
}
