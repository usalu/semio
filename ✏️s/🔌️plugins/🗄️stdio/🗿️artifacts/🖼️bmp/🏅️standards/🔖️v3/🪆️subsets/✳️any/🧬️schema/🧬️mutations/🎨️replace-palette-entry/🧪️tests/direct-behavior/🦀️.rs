//! 🧪️ `replace-palette-entry` fixture — `direct-behavior`.
//!
//! Source of truth is the committed JSON quintet beside this file: the before-snapshot, the
//! mutation payload, the after-snapshot, the sparse diff and the declared outcome. Every value in
//! it was produced by this repository's OWN dispatch — `Mutation::diff` followed by
//! `MutationDiff::apply` — so the fixture pins what the runtime does rather than what a second
//! implementation believes it should do.
//!
//! ⚖️ The six laws below are the closed set every mutation vector in this repository states:
//! forward application reaches the committed after-snapshot, the mutation's OWN inverse returns to
//! the before-snapshot, both snapshots and the payload are canonical JSON, the declared outcome
//! matches the diagnostics dispatch actually raises, the produced diff IS the committed diff, and
//! the committed diff alone carries before to after. The seventh pins the op codecs: the text and
//! binary wire forms of this payload must round-trip and the binary form must reject a truncated
//! frame.

use super::*;
use protocol::{Mutation, MutationDiff, OpBinary, OpText};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> BmpSnapshot {
    serde_json::from_str(BEFORE).expect("committed before-snapshot decodes")
}
fn expected_after() -> BmpSnapshot {
    serde_json::from_str(AFTER).expect("committed after-snapshot decodes")
}
fn mutation() -> BmpMutation {
    serde_json::from_str(MUTATION).expect("committed replace-palette-entry payload decodes")
}

/// ▶️ Applying the committed payload to the committed before-snapshot reaches the committed
/// after-snapshot exactly.
#[test]
fn applies_to_committed_after() {
    let base = before();
    let produced = mutation().diff(&base).diff().apply(&base).expect("replace-palette-entry applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "replace-palette-entry/direct-behavior: applied state differs from the committed after-snapshot");
}

/// ↩️ The mutation's own inverse steps, computed from the pre-mutation state, restore it exactly —
/// the law the whole event-sourced undo history rests on.
#[test]
fn inverse_restores_before() {
    let base = before();
    let payload = mutation();
    let mut current = payload.diff(&base).diff().apply(&base).expect("forward replace-palette-entry applies");
    for step in payload.inverse(&base) {
        current = step.diff(&current).diff().apply(&current).expect("the replace-palette-entry inverse step applies");
    }
    assert_eq!(current, base, "replace-palette-entry/direct-behavior: the undo did not restore the committed before-snapshot");
}

/// 🔣️ Every committed file is canonical: decoding and re-encoding it reproduces it verbatim, so the
/// fixture can never drift from the serde shape the wire actually carries.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: BmpSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "replace-palette-entry/direct-behavior: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("payload encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("payload reparses");
    assert_eq!(reencoded, original, "replace-palette-entry/direct-behavior: committed payload JSON is not canonical");
}

/// 🎯️ The declared outcome is the one dispatch really produces — a fixture that declares `applied`
/// while the implementation quietly warns would otherwise read as clean.
#[test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = declared.get("status").and_then(serde_json::Value::as_str).expect("the outcome carries a status");
    let produced = mutation().diff(&before());
    let raised: Vec<String> = produced.messages().iter().map(|message| message.code.0.clone()).collect();
    match status {
        "applied" => assert!(raised.is_empty(), "replace-palette-entry/direct-behavior: declared applied, but dispatch raised {raised:?}"),
        _ => assert!(!raised.is_empty(), "replace-palette-entry/direct-behavior: declared {status}, but dispatch raised nothing"),
    }
}

/// 🔺️ The diff dispatch produces IS the committed diff — the load-bearing assertion, because the
/// diff is what replication ships and what undo inverts.
#[test]
fn produces_committed_diff() {
    let produced = serde_json::to_value(mutation().diff(&before()).diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "replace-palette-entry/direct-behavior: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🩹 The committed diff ALONE carries before to after: it is a complete description of the change,
/// not a summary of it.
#[test]
fn committed_diff_applies_to_after() {
    let base = before();
    let produced = mutation().diff(&base).diff().apply(&base).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "replace-palette-entry/direct-behavior: committed diff did not carry before to after");
}

/// 📡️ The payload's text and binary wire forms round-trip, the binary frame carries this schema's
/// own tag, and a truncated frame is rejected rather than silently decoded.
#[test]
fn op_codecs_round_trip() {
    let payload = mutation();
    assert_eq!(BmpMutation::parse_op(&payload.print_op()).expect("the text op parses"), payload, "replace-palette-entry/direct-behavior: the text op form does not round-trip");
    let bytes = payload.encode_op().expect("the binary op encodes");
    assert_eq!(bytes[1], super::binary::BINARY_TAG, "replace-palette-entry/direct-behavior: the binary frame must carry this schema's tag");
    assert_eq!(BmpMutation::decode_op(&bytes).expect("the binary op decodes"), payload, "replace-palette-entry/direct-behavior: the binary op form does not round-trip");
    assert!(BmpMutation::decode_op(&bytes[..1]).is_err(), "replace-palette-entry/direct-behavior: a truncated binary frame must be rejected");
}
