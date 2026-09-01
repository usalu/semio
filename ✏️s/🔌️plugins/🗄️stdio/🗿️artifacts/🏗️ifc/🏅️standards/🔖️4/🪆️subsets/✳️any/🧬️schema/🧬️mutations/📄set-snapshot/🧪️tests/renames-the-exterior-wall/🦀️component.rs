//! 🧪️ `set-snapshot` fixture — `renames-the-exterior-wall`.
//!
//! IFC4 keeps its OWN typed graph (never step's `Part21Document`), and `IfcDiff` splits the
//! HEADER into three independent whole-tuple slots plus an id-keyed `entities` triple whose
//! per-entity patch descends into a POSITIONAL `IfcArgsDiff`. Renaming a wall therefore has
//! to reach argument slot 2 of entity `#1` and nothing else — not the entity keyword, not the
//! sibling IFCPROJECT, and none of the three HEADER records.
//! `IfcValue` is adjacently tagged (`{\"kind\":…,\"value\":…}`), which is what
//! distinguishes this artifact's committed JSON from IFC2X3's externally tagged
//! `Part21Value` even though both formats are ISO 10303-21 syntax on the wire.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`), every value of which was transcribed from this
//! leaf's own `🔺️diff/🦀️component.rs` oracle. The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::ifc::standards::v4::subsets::any::schema::diff::IfcDiff;
use crate::artifacts::ifc::standards::v4::subsets::any::schema::mutations::{apply_ifc_mutation, IfcMutation};
use crate::artifacts::ifc::standards::v4::subsets::any::schema::snapshot::IfcSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> IfcSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> IfcSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> IfcMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ `set-snapshot` carries the committed `before` IfcSnapshot to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    let outcome = apply_ifc_mutation(&mut snapshot, &mutation());
    assert!(outcome.messages().is_empty(), "set-snapshot/renames-the-exterior-wall: set-snapshot raised diagnostics it should not have");
    assert_eq!(snapshot, expected_after(), "set-snapshot/renames-the-exterior-wall: applied state differs from committed after-snapshot");
    assert_eq!(
        snapshot.entities[0].args[2],
        crate::artifacts::ifc::standards::v4::subsets::any::schema::snapshot::IfcValue::String("Exterior Wall".into()),
        "set-snapshot/renames-the-exterior-wall: the wall's Name attribute must land on 'Exterior Wall'"
    );
    assert_eq!(snapshot.entities[0].name, "IFCWALL", "set-snapshot/renames-the-exterior-wall: the EXPRESS entity keyword is untouched");
    assert!(matches!(snapshot.entities[0].args[1], crate::artifacts::ifc::standards::v4::subsets::any::schema::snapshot::IfcValue::Unset), "set-snapshot/renames-the-exterior-wall: the OwnerHistory attribute stays the Part-21 unset marker");
    assert_eq!(snapshot.entities[1], before().entities[1], "set-snapshot/renames-the-exterior-wall: the IFCPROJECT instance is identical on both sides and must survive untouched");
    assert_eq!(snapshot.header, before().header, "set-snapshot/renames-the-exterior-wall: all three HEADER records are equal on both sides");
}

/// ↩️ `set-snapshot`'s inverse is a single `SetSnapshot` carrying the pre-state IfcSnapshot back, so
/// forward-then-undo restores `before` byte for byte.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <IfcMutation as protocol::Mutation<IfcSnapshot>>::inverse(&mutation, &base);
    assert_eq!(inverse.len(), 1, "set-snapshot/renames-the-exterior-wall: undoing a whole-snapshot replacement is exactly one step");
    assert!(matches!(inverse[0], IfcMutation::SetSnapshot(..)), "set-snapshot/renames-the-exterior-wall: the undo step must itself be a SetSnapshot carrying the pre-state");
    let mut snapshot = base.clone();
    apply_ifc_mutation(&mut snapshot, &mutation);
    for step in &inverse {
        apply_ifc_mutation(&mut snapshot, step);
    }
    assert_eq!(snapshot, base, "set-snapshot/renames-the-exterior-wall: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed IfcSnapshot snapshots and this leaf's committed mutation payload are already
/// canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: IfcSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "set-snapshot/renames-the-exterior-wall: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "set-snapshot/renames-the-exterior-wall: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome — status AND every diagnostic this leaf's own diff builder raises for
/// this payload — matches what the mutation actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let declared: Vec<(String, String)> =
        outcome.get("messages").and_then(serde_json::Value::as_array).map(|rows| rows.iter().map(|row| (row["level"].as_str().unwrap_or_default().to_string(), row["code"].as_str().unwrap_or_default().to_string())).collect()).unwrap_or_default();
    let raised = <IfcMutation as protocol::Mutation<IfcSnapshot>>::diff(&mutation(), &before());
    let produced: Vec<(String, String)> = raised
        .messages()
        .iter()
        .map(|message| {
            let level = serde_json::to_value(message.level).expect("severity encodes");
            (level.as_str().unwrap_or_default().to_string(), message.code.0.clone())
        })
        .collect();
    assert_eq!(produced, declared, "set-snapshot/renames-the-exterior-wall: raised diagnostics differ from the committed 🎯️outcome messages");
    let mut snapshot = before();
    apply_ifc_mutation(&mut snapshot, &mutation());
    match status {
        "applied" => assert_ne!(snapshot, before(), "set-snapshot/renames-the-exterior-wall: declared applied but the snapshot came back unchanged"),
        "rejected" => assert_eq!(snapshot, before(), "set-snapshot/renames-the-exterior-wall: a rejected mutation must leave the snapshot untouched"),
        other => panic!("set-snapshot/renames-the-exterior-wall: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The sparse delta this leaf produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: `set-snapshot` has NO whole-snapshot replacement slot
/// in IfcDiff, so the delta must name only the fields that actually differ.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let raised = <IfcMutation as protocol::Mutation<IfcSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(raised.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "set-snapshot/renames-the-exterior-wall: produced diff differs from the committed 🔺️diff/🔣️component.json");
    assert!(
        raised.diff().file_description.is_none() && raised.diff().file_name.is_none() && raised.diff().file_schema.is_none(),
        "set-snapshot/renames-the-exterior-wall: the three HEADER slots are independent whole-tuple replacements and must stay absent here"
    );
    let entities = raised.diff().entities.as_ref().expect("set-snapshot/renames-the-exterior-wall: the entities triple must be present");
    assert!(entities.removed.is_empty() && entities.added.is_empty(), "set-snapshot/renames-the-exterior-wall: renaming a wall never adds or removes an instance");
    assert_eq!(entities.modified[0].id, 1, "set-snapshot/renames-the-exterior-wall: IfcEntityModified is keyed by the #N instance id");
    assert!(entities.modified[0].diff.name.is_none(), "set-snapshot/renames-the-exterior-wall: IFCWALL stays IFCWALL — the entity keyword must not appear");
    let args = entities.modified[0].diff.args.as_ref().expect("set-snapshot/renames-the-exterior-wall: the args triple must be present");
    assert_eq!(args.modified.len(), 1, "set-snapshot/renames-the-exterior-wall: exactly one EXPRESS attribute moves");
    assert_eq!(args.modified[0].index, 2, "set-snapshot/renames-the-exterior-wall: Name is EXPRESS attribute position 2 on IfcRoot's subtypes");
}

/// 🔣️ The committed diff is itself canonical and decodes to IfcDiff.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: IfcDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "set-snapshot/renames-the-exterior-wall: committed diff JSON is not canonical");
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(DIFF).expect("diff reparses").pointer("/entities/modified/0/diff/args/modified/0/value/kind").and_then(serde_json::Value::as_str),
        Some("string"),
        "set-snapshot/renames-the-exterior-wall: IfcValue is adjacently tagged, so the committed argument carries a separate kind/value pair — step's externally tagged spelling would be wrong here"
    );
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is
/// a complete description of what this `set-snapshot` changed, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: IfcDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <IfcDiff as protocol::MutationDiff<IfcSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "set-snapshot/renames-the-exterior-wall: committed diff did not carry before to after");
}
