//! 🧪️ `resize-page` fixture — `round-trips-the-concrete-inverse`.
//!
//! Source of truth is the committed JSON quintet beside this file. Every value in it was produced
//! by this repository's OWN dispatch — `Mutation::diff` followed by `MutationDiff::apply` — so the
//! fixture pins what the runtime does rather than what a second implementation believes it should.
//!
//! ⚖️ The six laws below are the closed set every mutation vector in this repository states, and the
//! seventh pins the op codecs. The concrete inverse is held to the ROUND-TRIP law rather than to a
//! committed step list: the step list is an implementation choice, while "the mutation's own
//! inverse returns the document to where it started" is the property undo actually depends on.

use super::*;
use protocol::{Mutation, MutationDiff, OpBinary, OpText};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> PdfSnapshot {
    serde_json::from_str(BEFORE).expect("committed before-snapshot decodes")
}
fn expected_after() -> PdfSnapshot {
    serde_json::from_str(AFTER).expect("committed after-snapshot decodes")
}
fn mutation() -> PdfMutation {
    serde_json::from_str(MUTATION).expect("committed resize-page payload decodes")
}

/// ▶️ Applying the committed payload to the committed before-snapshot reaches the committed
/// after-snapshot exactly.
#[test]
fn applies_to_committed_after() {
    let base = before();
    let mut state = base.clone();
    let outcome = mutation().diff(&state).apply_to(&mut state);
    assert!(outcome.messages().is_empty(), "resize-page/round-trips-the-concrete-inverse: the committed vector is a clean applied vector");
    assert_eq!(state, expected_after(), "resize-page/round-trips-the-concrete-inverse: applied state differs from the committed after-snapshot");
}

/// ↩️ The mutation's own inverse steps, computed from the pre-mutation state, restore it exactly.
#[test]
fn inverse_restores_before() {
    let base = before();
    let payload = mutation();
    let mut state = base.clone();
    payload.diff(&state).apply_to(&mut state);
    let inverse = payload.inverse(&base);
    assert!(!inverse.is_empty(), "resize-page/round-trips-the-concrete-inverse: a mutation that really moved the document must offer an undo");
    for step in &inverse {
        assert!(step.diff(&state).apply_to(&mut state).messages().is_empty(), "resize-page/round-trips-the-concrete-inverse: an inverse step was refused");
    }
    assert_eq!(state, base, "resize-page/round-trips-the-concrete-inverse: the undo did not restore the committed before-snapshot");
}

/// 🔣️ Every committed file is canonical, and the numeric shape survives the round trip — a page box
/// written as `612` must not come back as `612.0`.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: PdfSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "resize-page/round-trips-the-concrete-inverse: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("payload encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("payload reparses");
    assert_eq!(reencoded, original, "resize-page/round-trips-the-concrete-inverse: committed payload JSON is not canonical");
}

/// 🎯️ The declared outcome is the one dispatch really produces.
#[test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = declared.get("status").and_then(serde_json::Value::as_str).expect("the outcome carries a status");
    let raised: Vec<String> = mutation().diff(&before()).messages().iter().map(|message| message.code.0.clone()).collect();
    match status {
        "applied" => assert!(raised.is_empty(), "resize-page/round-trips-the-concrete-inverse: declared applied, but dispatch raised {raised:?}"),
        _ => assert!(!raised.is_empty(), "resize-page/round-trips-the-concrete-inverse: declared {status}, but dispatch raised nothing"),
    }
}

/// 🔺️ The diff dispatch produces IS the committed diff — the load-bearing assertion, because the
/// diff is what replication ships and what undo inverts.
#[test]
fn produces_committed_diff() {
    let produced = serde_json::to_value(mutation().diff(&before()).diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "resize-page/round-trips-the-concrete-inverse: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🩹 The committed diff ALONE carries before to after.
#[test]
fn committed_diff_applies_to_after() {
    let base = before();
    let decoded: PdfDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = decoded.apply(&base).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "resize-page/round-trips-the-concrete-inverse: committed diff did not carry before to after");
}

/// 📡️ The payload's text, binary and JSON wire forms all round-trip — for the forward step and for
/// every inverse step it computes.
#[test]
fn op_codecs_round_trip() {
    let payload = mutation();
    for step in std::iter::once(payload.clone()).chain(payload.inverse(&before())) {
        assert_eq!(PdfMutation::parse_op(&step.print_op()).expect("the text op parses"), step, "resize-page/round-trips-the-concrete-inverse: the text op form does not round-trip");
        assert_eq!(PdfMutation::decode_op(&step.encode_op().expect("the binary op encodes")).expect("the binary op decodes"), step, "resize-page/round-trips-the-concrete-inverse: the binary op form does not round-trip");
        assert_eq!(serde_json::from_value::<PdfMutation>(serde_json::to_value(&step).expect("the payload encodes")).expect("the payload decodes"), step, "resize-page/round-trips-the-concrete-inverse: the JSON form does not round-trip");
    }
}
