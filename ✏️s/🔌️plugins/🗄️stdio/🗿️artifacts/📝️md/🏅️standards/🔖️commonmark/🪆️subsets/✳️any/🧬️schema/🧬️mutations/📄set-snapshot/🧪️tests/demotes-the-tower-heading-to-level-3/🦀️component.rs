//! 🧪️ `set-snapshot` fixture — `demotes-the-tower-heading-to-level-3`.
//!
//! `MdSnapshot` is a real CommonMark block tree, not a `body: String`, so `between_block`
//! keeps a heading's KIND and emits a kind-shaped `MdBlockDiff::Heading` whose `inlines`
//! slot stays `None` because the inline run is untouched — only `level` moves. That is what
//! this fixture pins: a heading-level change must not drag the block's inlines through the
//! delta, and the sibling paragraph must not appear in it at all.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`), every value of which was transcribed from this
//! leaf's own `🔺️diff/🦀️component.rs` oracle. The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::md::standards::v_commonmark::subsets::any::schema::diff::MdDiff;
use crate::artifacts::md::standards::v_commonmark::subsets::any::schema::mutations::{apply_md_mutation, MdMutation};
use crate::artifacts::md::standards::v_commonmark::subsets::any::schema::snapshot::MdSnapshot;
use crate::artifacts::md::standards::v_commonmark::subsets::any::schema::diff::MdBlockDiff;
use crate::artifacts::md::standards::v_commonmark::subsets::any::schema::snapshot::{MdBlock, MdInline};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> MdSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> MdSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> MdMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ `set-snapshot` carries the committed `before` MdSnapshot to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    let outcome = apply_md_mutation(&mut snapshot, &mutation());
    assert!(outcome.messages().is_empty(), "set-snapshot/demotes-the-tower-heading-to-level-3: set-snapshot raised diagnostics it should not have");
    assert_eq!(snapshot, expected_after(), "set-snapshot/demotes-the-tower-heading-to-level-3: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.blocks[0], MdBlock::Heading { level: 3, inlines: vec![MdInline::Text { text: "Capsule Tower".into() }] }, "set-snapshot/demotes-the-tower-heading-to-level-3: the heading must keep its inline run and only change level");
    assert_eq!(snapshot.blocks[1], before().blocks[1], "set-snapshot/demotes-the-tower-heading-to-level-3: the sibling paragraph is identical on both sides and must survive untouched");
}

/// ↩️ `set-snapshot`'s inverse is a single `SetSnapshot` carrying the pre-state MdSnapshot back, so
/// forward-then-undo restores `before` byte for byte.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <MdMutation as protocol::Mutation<MdSnapshot>>::inverse(&mutation, &base);
    assert_eq!(inverse.len(), 1, "set-snapshot/demotes-the-tower-heading-to-level-3: undoing a whole-snapshot replacement is exactly one step");
    assert!(matches!(inverse[0], MdMutation::SetSnapshot { .. }), "set-snapshot/demotes-the-tower-heading-to-level-3: the undo step must itself be a SetSnapshot carrying the pre-state");
    let mut snapshot = base.clone();
    apply_md_mutation(&mut snapshot, &mutation);
    for step in &inverse {
        apply_md_mutation(&mut snapshot, step);
    }
    assert_eq!(snapshot, base, "set-snapshot/demotes-the-tower-heading-to-level-3: inverse did not restore the before-snapshot");
    assert_eq!(snapshot.blocks[0], MdBlock::Heading { level: 2, inlines: vec![MdInline::Text { text: "Capsule Tower".into() }] }, "set-snapshot/demotes-the-tower-heading-to-level-3: the undo must put the level-2 heading back");
}

/// 🔣️ Both committed MdSnapshot snapshots and this leaf's committed mutation payload are already
/// canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: MdSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "set-snapshot/demotes-the-tower-heading-to-level-3: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "set-snapshot/demotes-the-tower-heading-to-level-3: committed mutation JSON is not canonical");
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
    let raised = <MdMutation as protocol::Mutation<MdSnapshot>>::diff(&mutation(), &before());
    let produced: Vec<(String, String)> = raised
        .messages()
        .iter()
        .map(|message| {
            let level = serde_json::to_value(message.level).expect("severity encodes");
            (level.as_str().unwrap_or_default().to_string(), message.code.0.clone())
        })
        .collect();
    assert_eq!(produced, declared, "set-snapshot/demotes-the-tower-heading-to-level-3: raised diagnostics differ from the committed 🎯️outcome messages");
    let mut snapshot = before();
    apply_md_mutation(&mut snapshot, &mutation());
    match status {
        "applied" => assert_ne!(snapshot, before(), "set-snapshot/demotes-the-tower-heading-to-level-3: declared applied but the snapshot came back unchanged"),
        "rejected" => assert_eq!(snapshot, before(), "set-snapshot/demotes-the-tower-heading-to-level-3: a rejected mutation must leave the snapshot untouched"),
        other => panic!("set-snapshot/demotes-the-tower-heading-to-level-3: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The sparse delta this leaf produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: `set-snapshot` has NO whole-snapshot replacement slot
/// in MdDiff, so the delta must name only the fields that actually differ.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let raised = <MdMutation as protocol::Mutation<MdSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(raised.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "set-snapshot/demotes-the-tower-heading-to-level-3: produced diff differs from the committed 🔺️diff/🔣️component.json");
    let blocks = raised.diff().blocks.as_ref().expect("set-snapshot/demotes-the-tower-heading-to-level-3: the top-level blocks triple must be present");
    assert!(blocks.removed.is_empty() && blocks.added.is_empty(), "set-snapshot/demotes-the-tower-heading-to-level-3: the block sequence keeps its length and kinds, so nothing is removed or added");
    assert_eq!(blocks.modified.len(), 1, "set-snapshot/demotes-the-tower-heading-to-level-3: only the heading block is patched — the paragraph must not appear in the delta at all");
    assert_eq!(blocks.modified[0].index, 0, "set-snapshot/demotes-the-tower-heading-to-level-3: MdBlockModified indices are BASE-state indices");
    assert!(matches!(blocks.modified[0].diff, MdBlockDiff::Heading { level: Some(3), inlines: None }), "set-snapshot/demotes-the-tower-heading-to-level-3: a level-only edit must keep the kind-shaped Heading variant with inlines unset — an MdBlockDiff::Replace here would mean between_block lost the kind match");
}

/// 🔣️ The committed diff is itself canonical and decodes to MdDiff.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: MdDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "set-snapshot/demotes-the-tower-heading-to-level-3: committed diff JSON is not canonical");
    assert!(matches!(decoded.blocks.as_ref().expect("blocks triple").modified[0].diff, MdBlockDiff::Heading { .. }), "set-snapshot/demotes-the-tower-heading-to-level-3: the committed diff must decode to the kind-tagged Heading variant, not to Replace");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is
/// a complete description of what this `set-snapshot` changed, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: MdDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <MdDiff as protocol::MutationDiff<MdSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "set-snapshot/demotes-the-tower-heading-to-level-3: committed diff did not carry before to after");
}
