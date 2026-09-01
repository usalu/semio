//! 🧪️ `set-snapshot` fixture — `restamps-the-product-long-name`.
//!
//! AP214 entities carry a stable `#N` instance number, so `StepEntitiesDiff` is id-keyed and
//! needs no position transport — but each entity's ARGUMENT list is positional, so the edit
//! lands one level deeper as an index-keyed `StepArgsDiff`. That two-level shape is what this
//! fixture pins: the delta names entity `#1` by id and argument slot 1 by position, and must
//! not restate the entity's other three arguments, the two untouched instances, or the typed
//! HEADER triple.
//! `StepValue` is an externally tagged enum, so its arms encode as `{\"string\": …}` /
//! `{\"reference\": …}` — deliberately different from ifc4's adjacently tagged `IfcValue`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`), every value of which was transcribed from this
//! leaf's own `🔺️diff/🦀️component.rs` oracle. The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::step::standards::v_ap214::subsets::any::schema::diff::StepDiff;
use crate::artifacts::step::standards::v_ap214::subsets::any::schema::mutations::{apply_step_mutation, StepMutation};
use crate::artifacts::step::standards::v_ap214::subsets::any::schema::snapshot::StepSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> StepSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> StepSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> StepMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ `set-snapshot` carries the committed `before` StepSnapshot to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    let outcome = apply_step_mutation(&mut snapshot, &mutation());
    assert!(outcome.messages().is_empty(), "set-snapshot/restamps-the-product-long-name: set-snapshot raised diagnostics it should not have");
    assert_eq!(snapshot, expected_after(), "set-snapshot/restamps-the-product-long-name: applied state differs from committed after-snapshot");
    assert_eq!(
        snapshot.entities[0].args[1],
        crate::artifacts::step::standards::v_ap214::subsets::any::schema::snapshot::StepValue::String("Capsule Unit A".into()),
        "set-snapshot/restamps-the-product-long-name: the PRODUCT long name must land on 'Capsule Unit A'"
    );
    assert_eq!(snapshot.entities[0].name, "PRODUCT", "set-snapshot/restamps-the-product-long-name: the entity keyword is untouched");
    assert_eq!(snapshot.entities[0].args.len(), 4, "set-snapshot/restamps-the-product-long-name: no argument is inserted or dropped");
    assert_eq!(snapshot.entities[1], before().entities[1], "set-snapshot/restamps-the-product-long-name: #2 is identical on both sides and must survive untouched");
    assert_eq!(snapshot.header, before().header, "set-snapshot/restamps-the-product-long-name: the typed HEADER triple is equal on both sides");
}

/// ↩️ `set-snapshot`'s inverse is a single `SetSnapshot` carrying the pre-state StepSnapshot back, so
/// forward-then-undo restores `before` byte for byte.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <StepMutation as protocol::Mutation<StepSnapshot>>::inverse(&mutation, &base);
    assert_eq!(inverse.len(), 1, "set-snapshot/restamps-the-product-long-name: undoing a whole-snapshot replacement is exactly one step");
    assert!(matches!(inverse[0], StepMutation::SetSnapshot(_)), "set-snapshot/restamps-the-product-long-name: the undo step must itself be a SetSnapshot carrying the pre-state");
    let mut snapshot = base.clone();
    apply_step_mutation(&mut snapshot, &mutation);
    for step in &inverse {
        apply_step_mutation(&mut snapshot, step);
    }
    assert_eq!(snapshot, base, "set-snapshot/restamps-the-product-long-name: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed StepSnapshot snapshots and this leaf's committed mutation payload are already
/// canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: StepSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "set-snapshot/restamps-the-product-long-name: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "set-snapshot/restamps-the-product-long-name: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome — status AND every diagnostic this leaf's own diff builder raises for
/// this payload — matches what the mutation actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let declared: Vec<(String, String)> =
        outcome.get("messages").and_then(serde_json::Value::as_array).map(|rows| rows.iter().map(|row| (row["level"].as_str().unwrap_or_default().to_string(), row["code"].as_str().unwrap_or_default().to_string())).collect()).unwrap_or_default();
    let raised = <StepMutation as protocol::Mutation<StepSnapshot>>::diff(&mutation(), &before());
    let produced: Vec<(String, String)> = raised
        .messages()
        .iter()
        .map(|message| {
            let level = serde_json::to_value(message.level).expect("severity encodes");
            (level.as_str().unwrap_or_default().to_string(), message.code.0.clone())
        })
        .collect();
    assert_eq!(produced, declared, "set-snapshot/restamps-the-product-long-name: raised diagnostics differ from the committed 🎯️outcome messages");
    let mut snapshot = before();
    apply_step_mutation(&mut snapshot, &mutation());
    match status {
        "applied" => assert_ne!(snapshot, before(), "set-snapshot/restamps-the-product-long-name: declared applied but the snapshot came back unchanged"),
        "rejected" => assert_eq!(snapshot, before(), "set-snapshot/restamps-the-product-long-name: a rejected mutation must leave the snapshot untouched"),
        other => panic!("set-snapshot/restamps-the-product-long-name: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The sparse delta this leaf produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: `set-snapshot` has NO whole-snapshot replacement slot
/// in StepDiff, so the delta must name only the fields that actually differ.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let raised = <StepMutation as protocol::Mutation<StepSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(raised.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "set-snapshot/restamps-the-product-long-name: produced diff differs from the committed 🔺️diff/🔣️component.json");
    assert!(
        raised.diff().file_description.is_none() && raised.diff().file_name.is_none() && raised.diff().file_schema.is_none(),
        "set-snapshot/restamps-the-product-long-name: the three HEADER records are weak whole-replace slots and must stay absent here"
    );
    let entities = raised.diff().entities.as_ref().expect("set-snapshot/restamps-the-product-long-name: the entities triple must be present");
    assert!(entities.removed.is_empty() && entities.added.is_empty(), "set-snapshot/restamps-the-product-long-name: no instance enters or leaves the graph");
    assert_eq!(entities.modified[0].id, 1, "set-snapshot/restamps-the-product-long-name: StepEntityModified is keyed by the stable #N instance number, not by position");
    assert!(entities.modified[0].diff.name.is_none() && entities.modified[0].diff.complex.is_none(), "set-snapshot/restamps-the-product-long-name: the entity keyword and its (empty) complex-type list must not be rewritten");
    let args = entities.modified[0].diff.args.as_ref().expect("set-snapshot/restamps-the-product-long-name: the args triple must be present");
    assert_eq!(args.modified.len(), 1, "set-snapshot/restamps-the-product-long-name: exactly one argument slot moves");
    assert_eq!(args.modified[0].index, 1, "set-snapshot/restamps-the-product-long-name: argument lists are POSITIONAL, so the inner key is an index");
}

/// 🔣️ The committed diff is itself canonical and decodes to StepDiff.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: StepDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "set-snapshot/restamps-the-product-long-name: committed diff JSON is not canonical");
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(DIFF).expect("diff reparses").pointer("/entities/modified/0/diff/args/modified/0/value").and_then(|v| v.get("string")).and_then(serde_json::Value::as_str),
        Some("Capsule Unit A"),
        "set-snapshot/restamps-the-product-long-name: StepValue is externally tagged, so the committed argument reads {{\"string\": …}} — an adjacently tagged {{\"kind\",\"value\"}} pair would be ifc4's encoding, not step's"
    );
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is
/// a complete description of what this `set-snapshot` changed, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: StepDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <StepDiff as protocol::MutationDiff<StepSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "set-snapshot/restamps-the-product-long-name: committed diff did not carry before to after");
}
