//! 🧪️ `set-snapshot` fixture — `extends-the-readme-and-adds-a-version-member`.
//!
//! ZIP members are NAME-keyed, not index-keyed, so `ZipDiff::between` matches entries by
//! `ZipEntry::name` and needs no position transport: the readme comes back as a
//! `ZipEntryModified` whose inner `ZipEntryDiff` sets only `data` (the name did not move),
//! and the new member is a whole `ZipEntry` in `added`. `ZipDiff::apply` re-sorts the member
//! list by name afterwards, which is why the committed `after` is in name order.
//! Member payloads are decompressed `Vec<u8>` and serde writes them as JSON number arrays,
//! so the fixture keeps them a couple of bytes long and spells the bytes out.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`), every value of which was transcribed from this
//! leaf's own `🔺️diff/🦀️component.rs` oracle. The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::zip::standards::v2_0::subsets::any::schema::diff::ZipDiff;
use crate::artifacts::zip::standards::v2_0::subsets::any::schema::mutations::{apply_zip_mutation, ZipMutation};
use crate::artifacts::zip::standards::v2_0::subsets::any::schema::snapshot::ZipSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> ZipSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> ZipSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> ZipMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ `set-snapshot` carries the committed `before` ZipSnapshot to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    let outcome = apply_zip_mutation(&mut snapshot, &mutation());
    assert!(outcome.messages().is_empty(), "set-snapshot/extends-the-readme-and-adds-a-version-member: set-snapshot raised diagnostics it should not have");
    assert_eq!(snapshot, expected_after(), "set-snapshot/extends-the-readme-and-adds-a-version-member: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.entries.len(), 2, "set-snapshot/extends-the-readme-and-adds-a-version-member: the archive must end up with both members");
    assert_eq!(snapshot.entries[0].name, "doc/readme.txt", "set-snapshot/extends-the-readme-and-adds-a-version-member: ZipDiff::apply sorts members by name, so the readme comes first");
    assert_eq!(snapshot.entries[0].data, vec![104u8, 105, 33], "set-snapshot/extends-the-readme-and-adds-a-version-member: the readme payload must gain the trailing exclamation byte");
    assert_eq!(snapshot.entries[1].data, vec![49u8], "set-snapshot/extends-the-readme-and-adds-a-version-member: the new version member carries its own one-byte payload");
    assert_eq!(snapshot.comment, "semio", "set-snapshot/extends-the-readme-and-adds-a-version-member: the EOCD archive comment must land too");
}

/// ↩️ `set-snapshot`'s inverse is a single `SetSnapshot` carrying the pre-state ZipSnapshot back, so
/// forward-then-undo restores `before` byte for byte.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <ZipMutation as protocol::Mutation<ZipSnapshot>>::inverse(&mutation, &base);
    assert_eq!(inverse.len(), 1, "set-snapshot/extends-the-readme-and-adds-a-version-member: undoing a whole-snapshot replacement is exactly one step");
    assert!(matches!(inverse[0], ZipMutation::SetSnapshot { .. }), "set-snapshot/extends-the-readme-and-adds-a-version-member: the undo step must itself be a SetSnapshot carrying the pre-state");
    let mut snapshot = base.clone();
    apply_zip_mutation(&mut snapshot, &mutation);
    for step in &inverse {
        apply_zip_mutation(&mut snapshot, step);
    }
    assert_eq!(snapshot, base, "set-snapshot/extends-the-readme-and-adds-a-version-member: inverse did not restore the before-snapshot");
    assert_eq!(snapshot.entries.len(), 1, "set-snapshot/extends-the-readme-and-adds-a-version-member: the undo must drop the added member again");
    assert_eq!(snapshot.comment, "", "set-snapshot/extends-the-readme-and-adds-a-version-member: the undo must clear the archive comment back to empty");
}

/// 🔣️ Both committed ZipSnapshot snapshots and this leaf's committed mutation payload are already
/// canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: ZipSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "set-snapshot/extends-the-readme-and-adds-a-version-member: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "set-snapshot/extends-the-readme-and-adds-a-version-member: committed mutation JSON is not canonical");
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
    let raised = <ZipMutation as protocol::Mutation<ZipSnapshot>>::diff(&mutation(), &before());
    let produced: Vec<(String, String)> = raised
        .messages()
        .iter()
        .map(|message| {
            let level = serde_json::to_value(message.level).expect("severity encodes");
            (level.as_str().unwrap_or_default().to_string(), message.code.0.clone())
        })
        .collect();
    assert_eq!(produced, declared, "set-snapshot/extends-the-readme-and-adds-a-version-member: raised diagnostics differ from the committed 🎯️outcome messages");
    let mut snapshot = before();
    apply_zip_mutation(&mut snapshot, &mutation());
    match status {
        "applied" => assert_ne!(snapshot, before(), "set-snapshot/extends-the-readme-and-adds-a-version-member: declared applied but the snapshot came back unchanged"),
        "rejected" => assert_eq!(snapshot, before(), "set-snapshot/extends-the-readme-and-adds-a-version-member: a rejected mutation must leave the snapshot untouched"),
        other => panic!("set-snapshot/extends-the-readme-and-adds-a-version-member: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The sparse delta this leaf produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: `set-snapshot` has NO whole-snapshot replacement slot
/// in ZipDiff, so the delta must name only the fields that actually differ.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let raised = <ZipMutation as protocol::Mutation<ZipSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(raised.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "set-snapshot/extends-the-readme-and-adds-a-version-member: produced diff differs from the committed 🔺️diff/🔣️component.json");
    assert_eq!(raised.diff().comment.as_deref(), Some("semio"), "set-snapshot/extends-the-readme-and-adds-a-version-member: the archive comment is a flat scalar slot on ZipDiff");
    let entries = raised.diff().entries.as_ref().expect("set-snapshot/extends-the-readme-and-adds-a-version-member: the entries triple must be present");
    assert!(entries.removed.is_empty(), "set-snapshot/extends-the-readme-and-adds-a-version-member: no member disappears from the archive");
    assert_eq!(entries.modified.len(), 1, "set-snapshot/extends-the-readme-and-adds-a-version-member: only the readme is patched in place");
    assert_eq!(entries.modified[0].name, "doc/readme.txt", "set-snapshot/extends-the-readme-and-adds-a-version-member: ZipEntryModified is keyed by the member's BASE name");
    assert!(entries.modified[0].diff.name.is_none(), "set-snapshot/extends-the-readme-and-adds-a-version-member: the readme is not renamed, so the name sub-slot must stay unset");
    assert_eq!(entries.added.len(), 1, "set-snapshot/extends-the-readme-and-adds-a-version-member: exactly one new member");
}

/// 🔣️ The committed diff is itself canonical and decodes to ZipDiff.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: ZipDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "set-snapshot/extends-the-readme-and-adds-a-version-member: committed diff JSON is not canonical");
    assert_eq!(decoded.entries.as_ref().expect("entries triple").added[0].data, vec![49u8], "set-snapshot/extends-the-readme-and-adds-a-version-member: member payloads decode from JSON number arrays — a base64 string would mean the committed diff was written against the DSL codec instead");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is
/// a complete description of what this `set-snapshot` changed, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: ZipDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <ZipDiff as protocol::MutationDiff<ZipSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "set-snapshot/extends-the-readme-and-adds-a-version-member: committed diff did not carry before to after");
}
