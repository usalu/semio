//! 🧪️ `change-catalog-generation` fixture — `bumps-the-catalog-generation-to-7`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.

use crate::artifacts::home::diff::SHomeDiff;
use crate::artifacts::home::mutations::SHomeMutation;
use crate::artifacts::home::SHomeSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> SHomeSnapshot {
    serde_json::from_str(BEFORE).expect("before launcher document decodes")
}
fn expected_after() -> SHomeSnapshot {
    serde_json::from_str(AFTER).expect("after launcher document decodes")
}
fn mutation() -> SHomeMutation {
    serde_json::from_str(MUTATION).expect("change-catalog-generation mutation decodes")
}
fn built_outcome() -> protocol::MutationOutcome<SHomeDiff> {
    <SHomeMutation as protocol::Mutation<SHomeSnapshot>>::diff(&mutation(), &before())
}

/// ▶️ Pinning the counter to `7` moves `catalogGeneration` from `3` and leaves the launcher's
/// `schema` field alone — this is a setter, not an increment.
#[semio_framework_async_macros::async_test]
async fn pins_the_counter_of_the_committed_after() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("change-catalog-generation applies to its committed before-document");
    assert_eq!(applied, expected_after(), "change-catalog-generation/bumps-the-catalog-generation-to-7: the bumped document differs from the committed after-snapshot");
    assert_eq!(applied.catalog_generation, 7, "change-catalog-generation/bumps-the-catalog-generation-to-7: the counter must land on the payload's value, not on before + 1");
}

/// ↩️ The inverse re-pins the OLD counter read out of BASE — `3`, never a structural inversion of
/// the diff.
#[semio_framework_async_macros::async_test]
async fn repinning_the_old_counter_restores_before() {
    let base = before();
    let forward = <SHomeMutation as protocol::Mutation<SHomeSnapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("forward change-catalog-generation applies");
    let inverse = <SHomeMutation as protocol::Mutation<SHomeSnapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-catalog-generation/bumps-the-catalog-generation-to-7: the inverse of one counter pin is exactly one counter pin back");
    for step in &inverse {
        let undo = <SHomeMutation as protocol::Mutation<SHomeSnapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the change-catalog-generation inverse step applies");
    }
    assert_eq!(snapshot, base, "change-catalog-generation/bumps-the-catalog-generation-to-7: re-pinning generation 3 did not restore the before-document");
}

/// 🔣️ Both committed launcher snapshots and the `changeCatalogGeneration` payload are canonical.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SHomeSnapshot = serde_json::from_str(text).expect("launcher snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("launcher snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("launcher snapshot reparses");
        assert_eq!(reencoded, original, "change-catalog-generation/bumps-the-catalog-generation-to-7: committed {label} launcher JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("changeCatalogGeneration payload encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("changeCatalogGeneration payload reparses");
    assert_eq!(reencoded, original, "change-catalog-generation/bumps-the-catalog-generation-to-7: committed changeCatalogGeneration JSON is not canonical");
}

/// 🎯️ The only guard this mutation has is the equal-counter `mutation.no-op` warning; `3 != 7`, so
/// the declared `applied` outcome must be message-free.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-catalog-generation/bumps-the-catalog-generation-to-7: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), None, "change-catalog-generation/bumps-the-catalog-generation-to-7: pinning a different value must not raise mutation.no-op");
    assert!(produced.messages().is_empty(), "change-catalog-generation/bumps-the-catalog-generation-to-7: an accepted counter pin emits no diagnostics");
}

/// 🔺️ `SHomeDiff` carries four optional fields; this mutation is allowed to set exactly one of
/// them — `catalogGeneration` — and must leave `schema`, `activePanelTab` and `locale` null.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("produced change-catalog-generation diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-catalog-generation/bumps-the-catalog-generation-to-7: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff decodes to `SHomeDiff` and re-encodes unchanged — including the three
/// nulls, which `SHomeDiff` emits because no field carries `skip_serializing_if`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: SHomeDiff = serde_json::from_str(DIFF).expect("committed change-catalog-generation diff decodes");
    assert_eq!(decoded.catalog_generation, Some(7), "change-catalog-generation/bumps-the-catalog-generation-to-7: the committed diff must set the counter");
    assert!(decoded.active_panel_tab.is_none() && decoded.locale.is_none(), "change-catalog-generation/bumps-the-catalog-generation-to-7: an artifact-lane counter pin must not reach into the config lane");
    let reencoded = serde_json::to_value(&decoded).expect("committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-catalog-generation/bumps-the-catalog-generation-to-7: committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-document to the after-document.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SHomeDiff = serde_json::from_str(DIFF).expect("committed change-catalog-generation diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("committed diff applies to the before-document");
    assert_eq!(produced, expected_after(), "change-catalog-generation/bumps-the-catalog-generation-to-7: committed diff did not carry before to after");
}
