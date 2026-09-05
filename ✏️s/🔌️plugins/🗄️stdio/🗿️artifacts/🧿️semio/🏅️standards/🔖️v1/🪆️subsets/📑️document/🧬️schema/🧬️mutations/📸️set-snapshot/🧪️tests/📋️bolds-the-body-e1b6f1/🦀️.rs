//! 🧪️ `📸️set-snapshot` fixture — `📋️bolds-the-body-paragraph-and-finalizes-its-copy`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`.
//!
//! ✍️ The case rewrites the body paragraph's single run and switches it to bold. The `Overview`
//! heading, the `Body` named style and the (empty) image store are untouched — so
//! `SemioDocumentDiff` must fill its index-keyed `blocks` slot alone, descending
//! `blocks.modified[1] → runs.modified[0] → style.bold` and stopping there. In particular it must
//! be a `DocBlockDiff::Paragraph` patch, never the `Replace` a block-KIND change would produce.

use crate::artifacts::semio::standards::v1::subsets::document::schema::diff::{DocBlockDiff, SemioDocumentDiff};
use crate::artifacts::semio::standards::v1::subsets::document::schema::mutations::{apply_semio_document_mutation, SemioDocumentMutation};
use crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::SemioDocumentSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> SemioDocumentSnapshot {
    serde_json::from_str(BEFORE).expect("before document snapshot decodes")
}
fn expected_after() -> SemioDocumentSnapshot {
    serde_json::from_str(AFTER).expect("after document snapshot decodes")
}
fn mutation() -> SemioDocumentMutation {
    serde_json::from_str(MUTATION).expect("set-snapshot mutation decodes")
}

/// ▶️ `set-snapshot` carries the document to exactly the committed `after`: a bold, finalized body
/// paragraph under an unchanged heading.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    let outcome = apply_semio_document_mutation(&mut snapshot, &mutation());
    assert!(outcome.messages().is_empty(), "semio-document/set-snapshot: a genuinely changed document must not raise any message");
    assert_eq!(snapshot.blocks[0], before().blocks[0], "semio-document/set-snapshot: the heading block must be carried over untouched");
    assert_eq!(snapshot.styles, before().styles, "semio-document/set-snapshot: editing a run must not touch the named style table");
    assert_eq!(snapshot, expected_after(), "semio-document/set-snapshot: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse of `set-snapshot` is `set-snapshot(base)` — it must restore the draft copy and
/// unbold the run.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <SemioDocumentMutation as protocol::Mutation<SemioDocumentSnapshot>>::inverse(&mutation, &base);
    let mut snapshot = base.clone();
    apply_semio_document_mutation(&mut snapshot, &mutation);
    for step in &inverse {
        apply_semio_document_mutation(&mut snapshot, step);
    }
    assert_eq!(snapshot, base, "semio-document/set-snapshot: inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed documents and the mutation are already canonical. The document-specific trap
/// pinned here: `DocBlock` is an ENUM, so its container `rename_all` renames only the variant and
/// `style_id` stays snake_case on the wire — while `DocParagraphDiff` is a plain STRUCT, so the
/// very same field is written `styleId` inside the diff. `RunStyle`'s optional fields carry
/// `#[serde(default)]` but no `skip_serializing_if`, so every one is spelled out as `null`.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioDocumentSnapshot = serde_json::from_str(text).expect("document snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("document snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("document snapshot reparses");
        assert_eq!(reencoded, original, "semio-document/set-snapshot: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("set-snapshot mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("set-snapshot mutation reparses");
    assert_eq!(reencoded, original, "semio-document/set-snapshot: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome is `applied` — the document really moves, so the `mutation.no-op`
/// warning an identical set-snapshot would raise never appears.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    assert_eq!(status, "applied", "semio-document/set-snapshot: this fixture declares an applied outcome");
    let mut snapshot = before();
    let produced = apply_semio_document_mutation(&mut snapshot, &mutation());
    assert!(produced.messages().is_empty(), "semio-document/set-snapshot: declared applied, so no diagnostic may be raised");
    assert_ne!(snapshot, before(), "semio-document/set-snapshot: an applied set-snapshot must actually move the document");
}

/// 🔺️ The sparse `SemioDocumentDiff` this mutation produces is exactly the committed diff — the
/// load-bearing assertion: `styles` and `images` must stay absent, the heading must not appear in
/// `blocks.modified`, the paragraph's tri-state `styleId` slot must stay unset, and the run's style
/// patch must carry `bold` alone (not the six other `RunStyle` fields the snapshot spells out).
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioDocumentMutation as protocol::Mutation<SemioDocumentSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced document diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed document diff decodes");
    assert_eq!(produced, committed, "semio-document/set-snapshot: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to `SemioDocumentDiff` as a kind-
/// preserving `Paragraph` patch.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: SemioDocumentDiff = serde_json::from_str(DIFF).expect("committed document diff decodes");
    assert!(decoded.styles.is_none() && decoded.images.is_none(), "semio-document/set-snapshot: neither the style table nor the image store may be re-emitted");
    let blocks = decoded.blocks.as_ref().expect("the committed diff carries a blocks triple");
    assert!(blocks.removed.is_empty() && blocks.added.is_empty() && blocks.modified.len() == 1 && blocks.modified[0].index == 1, "semio-document/set-snapshot: exactly the body paragraph may be patched in place");
    let DocBlockDiff::Paragraph(paragraph) = &blocks.modified[0].diff else {
        panic!("semio-document/set-snapshot: the block delta must stay a Paragraph patch, never the Replace a kind change would produce");
    };
    assert!(paragraph.style_id.is_none(), "semio-document/set-snapshot: the paragraph's tri-state style reference did not move and must stay absent");
    let runs = paragraph.runs.as_ref().expect("the paragraph patch carries a runs triple");
    assert!(runs.removed.is_empty() && runs.added.is_empty() && runs.modified.len() == 1, "semio-document/set-snapshot: the single run must be patched, never removed and re-added");
    let style = runs.modified[0].diff.style.as_ref().expect("the run patch carries a style delta");
    assert_eq!(style.bold, Some(true), "semio-document/set-snapshot: the style delta must set bold");
    assert!(style.italic.is_none() && style.underline.is_none() && style.size.is_none() && style.font.is_none() && style.color.is_none() && style.link.is_none(), "semio-document/set-snapshot: no other RunStyle field moved, so none may appear");
    let reencoded = serde_json::to_value(&decoded).expect("document diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed document diff reparses");
    assert_eq!(reencoded, original, "semio-document/set-snapshot: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields the committed `after` — the run text
/// plus the single style flag is a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioDocumentDiff = serde_json::from_str(DIFF).expect("committed document diff decodes");
    let produced = <SemioDocumentDiff as protocol::MutationDiff<SemioDocumentSnapshot>>::apply(&decoded, &before()).expect("committed document diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "semio-document/set-snapshot: committed diff did not carry before to after");
}
