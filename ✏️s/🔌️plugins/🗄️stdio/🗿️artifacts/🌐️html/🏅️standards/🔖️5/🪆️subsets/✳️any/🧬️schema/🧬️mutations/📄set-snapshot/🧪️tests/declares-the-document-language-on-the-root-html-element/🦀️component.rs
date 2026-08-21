//! 🧪️ `set-snapshot` fixture — `declares-the-document-language-on-the-root-html-element`.
//!
//! `HtmlDiff::between` descends the real `HtmlNode` tree: the root element keeps its name and
//! its whole `<body>` subtree, so the only thing the delta may carry is a name-keyed
//! `HtmlAttributesDiff` with one `added` entry. `HtmlAttrAdded` also records the FINAL
//! position the attribute takes, because HTML attribute order carries no spec meaning but is
//! significant for byte-preserving round-trips.
//! `HtmlDiff::doctype` is a tri-state `Option<Option<String>>` and a cleared doctype
//! (`Some(None)`) would serialise to `null` and decode back as `None` — a real JSON
//! round-trip limitation. This payload leaves the doctype alone, so the field is simply
//! absent and the fixed point holds; the limitation is pinned, not asserted away.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`), every value of which was transcribed from this
//! leaf's own `🔺️diff/🦀️component.rs` oracle. The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::html::standards::v5::subsets::any::schema::diff::HtmlDiff;
use crate::artifacts::html::standards::v5::subsets::any::schema::mutations::{apply_html_mutation, HtmlMutation};
use crate::artifacts::html::standards::v5::subsets::any::schema::snapshot::HtmlSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> HtmlSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> HtmlSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> HtmlMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ `set-snapshot` carries the committed `before` HtmlSnapshot to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    let outcome = apply_html_mutation(&mut snapshot, &mutation());
    assert!(outcome.messages().is_empty(), "set-snapshot/declares-the-document-language-on-the-root-html-element: set-snapshot raised diagnostics it should not have");
    assert_eq!(snapshot, expected_after(), "set-snapshot/declares-the-document-language-on-the-root-html-element: applied state differs from committed after-snapshot");
    let root_attributes = match &snapshot.root {
        crate::artifacts::html::standards::v5::subsets::any::schema::snapshot::HtmlNode::Element { attributes, .. } => attributes.clone(),
        other => panic!("set-snapshot/declares-the-document-language-on-the-root-html-element: the root must stay an element, got {other:?}"),
    };
    assert_eq!(root_attributes.len(), 1, "set-snapshot/declares-the-document-language-on-the-root-html-element: the root element must end up carrying exactly the one new attribute");
    assert_eq!(root_attributes[0].name, "lang", "set-snapshot/declares-the-document-language-on-the-root-html-element: the added attribute is lang");
    assert_eq!(root_attributes[0].value.as_deref(), Some("de"), "set-snapshot/declares-the-document-language-on-the-root-html-element: lang is a valued attribute, not a valueless boolean one");
    assert_eq!(snapshot.doctype.as_deref(), Some("DOCTYPE html"), "set-snapshot/declares-the-document-language-on-the-root-html-element: the doctype is equal on both sides and must survive untouched");
}

/// ↩️ `set-snapshot`'s inverse is a single `SetSnapshot` carrying the pre-state HtmlSnapshot back, so
/// forward-then-undo restores `before` byte for byte.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <HtmlMutation as protocol::Mutation<HtmlSnapshot>>::inverse(&mutation, &base);
    assert_eq!(inverse.len(), 1, "set-snapshot/declares-the-document-language-on-the-root-html-element: undoing a whole-snapshot replacement is exactly one step");
    assert!(matches!(inverse[0], HtmlMutation::SetSnapshot { .. }), "set-snapshot/declares-the-document-language-on-the-root-html-element: the undo step must itself be a SetSnapshot carrying the pre-state");
    let mut snapshot = base.clone();
    apply_html_mutation(&mut snapshot, &mutation);
    for step in &inverse {
        apply_html_mutation(&mut snapshot, step);
    }
    assert_eq!(snapshot, base, "set-snapshot/declares-the-document-language-on-the-root-html-element: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed HtmlSnapshot snapshots and this leaf's committed mutation payload are already
/// canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: HtmlSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "set-snapshot/declares-the-document-language-on-the-root-html-element: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "set-snapshot/declares-the-document-language-on-the-root-html-element: committed mutation JSON is not canonical");
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
    let raised = <HtmlMutation as protocol::Mutation<HtmlSnapshot>>::diff(&mutation(), &before());
    let produced: Vec<(String, String)> = raised
        .messages()
        .iter()
        .map(|message| {
            let level = serde_json::to_value(message.level).expect("severity encodes");
            (level.as_str().unwrap_or_default().to_string(), message.code.0.clone())
        })
        .collect();
    assert_eq!(produced, declared, "set-snapshot/declares-the-document-language-on-the-root-html-element: raised diagnostics differ from the committed 🎯️outcome messages");
    let mut snapshot = before();
    apply_html_mutation(&mut snapshot, &mutation());
    match status {
        "applied" => assert_ne!(snapshot, before(), "set-snapshot/declares-the-document-language-on-the-root-html-element: declared applied but the snapshot came back unchanged"),
        "rejected" => assert_eq!(snapshot, before(), "set-snapshot/declares-the-document-language-on-the-root-html-element: a rejected mutation must leave the snapshot untouched"),
        other => panic!("set-snapshot/declares-the-document-language-on-the-root-html-element: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The sparse delta this leaf produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: `set-snapshot` has NO whole-snapshot replacement slot
/// in HtmlDiff, so the delta must name only the fields that actually differ.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let raised = <HtmlMutation as protocol::Mutation<HtmlSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(raised.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "set-snapshot/declares-the-document-language-on-the-root-html-element: produced diff differs from the committed 🔺️diff/🔣️component.json");
    assert!(raised.diff().doctype.is_none(), "set-snapshot/declares-the-document-language-on-the-root-html-element: an attribute edit must never reach the document-level doctype slot");
    let root = raised.diff().root.as_ref().expect("set-snapshot/declares-the-document-language-on-the-root-html-element: the root node diff must be present");
    let element = match root {
        crate::artifacts::html::standards::v5::subsets::any::schema::diff::HtmlNodeDiff::Element(element) => element,
        other => panic!("set-snapshot/declares-the-document-language-on-the-root-html-element: adding an attribute must keep the kind-shaped Element diff, got {other:?}"),
    };
    assert!(element.name.is_none(), "set-snapshot/declares-the-document-language-on-the-root-html-element: the root element keeps the name html");
    assert!(element.children.is_none(), "set-snapshot/declares-the-document-language-on-the-root-html-element: the whole body subtree is untouched and must not appear in the delta");
    let attributes = element.attributes.as_ref().expect("set-snapshot/declares-the-document-language-on-the-root-html-element: the attributes triple must be present");
    assert!(attributes.removed.is_empty() && attributes.modified.is_empty(), "set-snapshot/declares-the-document-language-on-the-root-html-element: nothing is removed or re-valued, only added");
    assert_eq!(attributes.added[0].index, 0, "set-snapshot/declares-the-document-language-on-the-root-html-element: HtmlAttrAdded carries the FINAL attribute position");
}

/// 🔣️ The committed diff is itself canonical and decodes to HtmlDiff.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: HtmlDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "set-snapshot/declares-the-document-language-on-the-root-html-element: committed diff JSON is not canonical");
    assert!(decoded.doctype.is_none(), "set-snapshot/declares-the-document-language-on-the-root-html-element: doctype must round-trip as absent — a committed null here would be indistinguishable from the Some(None) 'doctype cleared' state that Option<Option<String>> cannot express in JSON");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is
/// a complete description of what this `set-snapshot` changed, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: HtmlDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <HtmlDiff as protocol::MutationDiff<HtmlSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "set-snapshot/declares-the-document-language-on-the-root-html-element: committed diff did not carry before to after");
}
