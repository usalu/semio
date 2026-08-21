//! 🧪️ `📄set-snapshot` fixture — `retags-the-catalog-revision-and-rewrites-an-item-label`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`.
//!
//! 🌳️ The case bumps the root element's `revision` attribute and rewrites the literal text inside
//! `catalog/item`, leaving the XML declaration, the `xmlns` attribute and the trailing comment
//! node alone — so `XmlDiff::between` must produce a single recursive `root` chain (attribute
//! `modified` at the top, one `children.modified` step down into `item`, one `Text` leaf) and
//! must NOT touch `prolog`, `declaration` or `doctype`.

use crate::artifacts::xml::schema::diff::{XmlDiff, XmlNodeDiff};
use crate::artifacts::xml::schema::mutations::{apply_xml_mutation, XmlMutation};
use crate::artifacts::xml::XmlSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> XmlSnapshot {
    serde_json::from_str(BEFORE).expect("before XML snapshot decodes")
}
fn expected_after() -> XmlSnapshot {
    serde_json::from_str(AFTER).expect("after XML snapshot decodes")
}
fn mutation() -> XmlMutation {
    serde_json::from_str(MUTATION).expect("set-snapshot mutation decodes")
}

/// ▶️ `set-snapshot` carries the catalog document to exactly the committed `after`: `revision="2"`
/// on the root and the relabelled item text.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    let outcome = apply_xml_mutation(&mut snapshot, &mutation());
    assert!(outcome.messages().is_empty(), "xml/set-snapshot: a genuinely changed document must not raise any message");
    assert!(snapshot.doc.declaration.is_some(), "xml/set-snapshot: the XML declaration must survive a whole-document set");
    assert_eq!(snapshot, expected_after(), "xml/set-snapshot: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse of `set-snapshot` is `set-snapshot(base)` — it must restore `revision="1"` and
/// the original item text, comment node and declaration included.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <XmlMutation as protocol::Mutation<XmlSnapshot>>::inverse(&mutation, &base);
    let mut snapshot = base.clone();
    apply_xml_mutation(&mut snapshot, &mutation);
    for step in &inverse {
        apply_xml_mutation(&mut snapshot, step);
    }
    assert_eq!(snapshot, base, "xml/set-snapshot: inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed XML snapshots and the mutation are already canonical: `XmlDocument` skips an
/// absent `doctype` and an empty `prolog` entirely, while `XmlNode::Element` always spells out its
/// `attrs`/`children` arrays and each node carries its `kind` tag.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: XmlSnapshot = serde_json::from_str(text).expect("XML snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("XML snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("XML snapshot reparses");
        assert_eq!(reencoded, original, "xml/set-snapshot: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("set-snapshot mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("set-snapshot mutation reparses");
    assert_eq!(reencoded, original, "xml/set-snapshot: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome is `applied` — the document really moves, so the `mutation.no-op`
/// warning that an identical set-snapshot would raise never appears.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    assert_eq!(status, "applied", "xml/set-snapshot: this fixture declares an applied outcome");
    let mut snapshot = before();
    let produced = apply_xml_mutation(&mut snapshot, &mutation());
    assert!(produced.messages().is_empty(), "xml/set-snapshot: declared applied, so no diagnostic may be raised");
    assert_ne!(snapshot, before(), "xml/set-snapshot: an applied set-snapshot must actually move the document");
}

/// 🔺️ The sparse `XmlDiff` this mutation produces is exactly the committed diff — the load-bearing
/// assertion: it pins that the delta is a narrow recursive path into `catalog/item`, never a
/// wholesale `XmlNodeDiff::Replace` of the root subtree.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <XmlMutation as protocol::Mutation<XmlSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced XML diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed XML diff decodes");
    assert_eq!(produced, committed, "xml/set-snapshot: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to `XmlDiff`: `prolog`/`declaration`/
/// `doctype` stay `None` (and are therefore omitted, never spelled as `null` — which for the
/// tri-state `Option<Option<..>>` declaration/doctype fields would decode back as "unchanged" and
/// silently break the round trip), and `root` is an `Element` chain rather than a `Replace`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: XmlDiff = serde_json::from_str(DIFF).expect("committed XML diff decodes");
    assert!(decoded.prolog.is_none() && decoded.declaration.is_none() && decoded.doctype.is_none(), "xml/set-snapshot: the committed diff must leave prolog, declaration and doctype untouched");
    assert!(matches!(decoded.root, Some(XmlNodeDiff::Element(_))), "xml/set-snapshot: the committed diff must patch the root element, never replace it wholesale");
    let reencoded = serde_json::to_value(&decoded).expect("XML diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed XML diff reparses");
    assert_eq!(reencoded, original, "xml/set-snapshot: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields the committed `after` — the nested
/// attribute + text delta is a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: XmlDiff = serde_json::from_str(DIFF).expect("committed XML diff decodes");
    let produced = <XmlDiff as protocol::MutationDiff<XmlSnapshot>>::apply(&decoded, &before()).expect("committed XML diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "xml/set-snapshot: committed diff did not carry before to after");
}
