//! 🧪️ `set-snapshot` fixture — `recolours-the-circle-fill-to-crimson`.
//!
//! `SvgSnapshot` embeds xml's `XmlDocument` node model but declares its OWN diff types, and
//! `SvgDiff::between` walks that tree: the `<svg>` root keeps its name and its `viewBox`, so
//! the delta nests one `SvgChildrenDiff` entry down to the `<circle>` and stops at a
//! name-keyed `SvgAttributesDiff` with a single `modified` fill. Nothing about `r` or about
//! the root's own attributes may appear.
//! `SvgDiff::declaration`/`doctype` are tri-state `Option<Option<_>>` slots whose
//! `Some(None)` 'cleared' state cannot survive a JSON round trip; this payload leaves both
//! absent, and the fixture pins that rather than asserting a fidelity the encoding lacks.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`), every value of which was transcribed from this
//! leaf's own `🔺️diff/🦀️component.rs` oracle. The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::svg::standards::v1_1::subsets::any::schema::diff::SvgDiff;
use crate::artifacts::svg::standards::v1_1::subsets::any::schema::mutations::{apply_svg_mutation, SvgMutation};
use crate::artifacts::svg::standards::v1_1::subsets::any::schema::snapshot::SvgSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> SvgSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> SvgSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> SvgMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ `set-snapshot` carries the committed `before` SvgSnapshot to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    let outcome = apply_svg_mutation(&mut snapshot, &mutation());
    assert!(outcome.messages().is_empty(), "set-snapshot/recolours-the-circle-fill-to-crimson: set-snapshot raised diagnostics it should not have");
    assert_eq!(snapshot, expected_after(), "set-snapshot/recolours-the-circle-fill-to-crimson: applied state differs from committed after-snapshot");
    let circle = match snapshot.doc.root.as_ref().expect("set-snapshot/recolours-the-circle-fill-to-crimson: the svg root element must still be present") {
        crate::artifacts::xml::schema::snapshot::XmlNode::Element { children, .. } => children[0].clone(),
        other => panic!("set-snapshot/recolours-the-circle-fill-to-crimson: the svg root must stay an element, got {other:?}"),
    };
    match &circle {
        crate::artifacts::xml::schema::snapshot::XmlNode::Element { name, attrs, .. } => {
            assert_eq!(name, "circle", "set-snapshot/recolours-the-circle-fill-to-crimson: the only child stays a circle");
            assert_eq!(attrs[0].value, "4", "set-snapshot/recolours-the-circle-fill-to-crimson: the radius attribute is untouched");
            assert_eq!(attrs[1].value, "crimson", "set-snapshot/recolours-the-circle-fill-to-crimson: the fill attribute must land on crimson, keeping its original position in the attribute list");
        }
        other => panic!("set-snapshot/recolours-the-circle-fill-to-crimson: the circle child must stay an element, got {other:?}"),
    }
}

/// ↩️ `set-snapshot`'s inverse is a single `SetSnapshot` carrying the pre-state SvgSnapshot back, so
/// forward-then-undo restores `before` byte for byte.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <SvgMutation as protocol::Mutation<SvgSnapshot>>::inverse(&mutation, &base);
    assert_eq!(inverse.len(), 1, "set-snapshot/recolours-the-circle-fill-to-crimson: undoing a whole-snapshot replacement is exactly one step");
    assert!(matches!(inverse[0], SvgMutation::SetSnapshot { .. }), "set-snapshot/recolours-the-circle-fill-to-crimson: the undo step must itself be a SetSnapshot carrying the pre-state");
    let mut snapshot = base.clone();
    apply_svg_mutation(&mut snapshot, &mutation);
    for step in &inverse {
        apply_svg_mutation(&mut snapshot, step);
    }
    assert_eq!(snapshot, base, "set-snapshot/recolours-the-circle-fill-to-crimson: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed SvgSnapshot snapshots and this leaf's committed mutation payload are already
/// canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SvgSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "set-snapshot/recolours-the-circle-fill-to-crimson: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "set-snapshot/recolours-the-circle-fill-to-crimson: committed mutation JSON is not canonical");
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
    let raised = <SvgMutation as protocol::Mutation<SvgSnapshot>>::diff(&mutation(), &before());
    let produced: Vec<(String, String)> = raised
        .messages()
        .iter()
        .map(|message| {
            let level = serde_json::to_value(message.level).expect("severity encodes");
            (level.as_str().unwrap_or_default().to_string(), message.code.0.clone())
        })
        .collect();
    assert_eq!(produced, declared, "set-snapshot/recolours-the-circle-fill-to-crimson: raised diagnostics differ from the committed 🎯️outcome messages");
    let mut snapshot = before();
    apply_svg_mutation(&mut snapshot, &mutation());
    match status {
        "applied" => assert_ne!(snapshot, before(), "set-snapshot/recolours-the-circle-fill-to-crimson: declared applied but the snapshot came back unchanged"),
        "rejected" => assert_eq!(snapshot, before(), "set-snapshot/recolours-the-circle-fill-to-crimson: a rejected mutation must leave the snapshot untouched"),
        other => panic!("set-snapshot/recolours-the-circle-fill-to-crimson: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The sparse delta this leaf produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: `set-snapshot` has NO whole-snapshot replacement slot
/// in SvgDiff, so the delta must name only the fields that actually differ.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let raised = <SvgMutation as protocol::Mutation<SvgSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(raised.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "set-snapshot/recolours-the-circle-fill-to-crimson: produced diff differs from the committed 🔺️diff/🔣️component.json");
    assert!(raised.diff().declaration.is_none() && raised.diff().doctype.is_none() && raised.diff().prolog.is_none(), "set-snapshot/recolours-the-circle-fill-to-crimson: an attribute edit deep in the tree must never reach the document-prolog slots");
    let root = raised.diff().root.as_ref().expect("set-snapshot/recolours-the-circle-fill-to-crimson: the root node diff must be present");
    let element = match root {
        crate::artifacts::svg::standards::v1_1::subsets::any::schema::diff::SvgNodeDiff::Element(element) => element,
        other => panic!("set-snapshot/recolours-the-circle-fill-to-crimson: the svg root diff must stay kind-shaped, got {other:?}"),
    };
    assert!(element.name.is_none() && element.attributes.is_none(), "set-snapshot/recolours-the-circle-fill-to-crimson: the svg root keeps its name and its viewBox — neither may appear in the delta");
    let children = element.children.as_ref().expect("set-snapshot/recolours-the-circle-fill-to-crimson: the children triple must be present");
    assert!(children.removed.is_empty() && children.added.is_empty(), "set-snapshot/recolours-the-circle-fill-to-crimson: recolouring a circle never adds or removes a child node");
    assert_eq!(children.modified[0].index, 0, "set-snapshot/recolours-the-circle-fill-to-crimson: SvgChildModified indices are BASE-state indices");
}

/// 🔣️ The committed diff is itself canonical and decodes to SvgDiff.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: SvgDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "set-snapshot/recolours-the-circle-fill-to-crimson: committed diff JSON is not canonical");
    assert!(decoded.declaration.is_none() && decoded.doctype.is_none(), "set-snapshot/recolours-the-circle-fill-to-crimson: both tri-state prolog slots must round-trip as absent — a committed null would collapse the Some(None) 'cleared' state that Option<Option<_>> cannot express in JSON");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is
/// a complete description of what this `set-snapshot` changed, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SvgDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <SvgDiff as protocol::MutationDiff<SvgSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "set-snapshot/recolours-the-circle-fill-to-crimson: committed diff did not carry before to after");
}
