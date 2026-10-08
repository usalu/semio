//! 🧪️ `insert-page` fixture — `🔄️round`.
//!
//! Source of truth is the committed JSON quintet beside this file. Every value in it was produced
//! by this repository's OWN dispatch — `Mutation::diff` followed by `protocol::apply_diff` — so the
//! fixture pins what the runtime does rather than what a second implementation believes it should.
//!
//! ⚖️ The six laws below are the closed set every mutation vector in this repository states, and the
//! seventh pins the op codecs. The concrete inverse is held to the ROUND-TRIP law rather than to a
//! committed step list: the step list is an implementation choice, while "the mutation's own
//! inverse returns the document to where it started" is the property undo actually depends on.

use super::*;
use crate::standards::v1_4::subsets::base::io::binary::mutations as binary;
use protocol::{Mutation, MutationDiff, OpBinary, OpText};

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📥️insert-page/🔄️round/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📥️insert-page/🔄️round/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📥️insert-page/🔄️round/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📥️insert-page/🔄️round/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📥️insert-page/🔄️round/🎯️outcome/🔣️.json");

fn before() -> PdfSnapshot {
    semio_framework_pack_json::from_json_str(BEFORE,semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed before-snapshot decodes")
}
fn expected_after() -> PdfSnapshot {
    semio_framework_pack_json::from_json_str(AFTER,semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed after-snapshot decodes")
}
fn mutation() -> PdfMutation {
    semio_framework_pack_json::from_json_str(MUTATION,semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed insert-page payload decodes")
}

/// ▶️ Applying the committed payload to the committed before-snapshot reaches the committed
/// after-snapshot exactly.
#[test]
fn applies_to_committed_after() {
    let base = before();
    let mut state = base.clone();
    let outcome = crate::standards::v1_4::subsets::base::io::mutation_bridge::apply_outcome(mutation().diff(&state), &mut state);
    assert!(outcome.messages().is_empty(), "insert-page/round-trips-the-concrete-inverse: the committed vector is a clean applied vector");
    assert_eq!(state, expected_after(), "insert-page/round-trips-the-concrete-inverse: applied state differs from the committed after-snapshot");
}

/// ↩️ The mutation's own inverse steps, computed from the pre-mutation state, restore it exactly.
#[test]
fn inverse_restores_before() {
    let base = before();
    let payload = mutation();
    let mut state = base.clone();
    crate::standards::v1_4::subsets::base::io::mutation_bridge::apply_outcome(payload.diff(&state), &mut state);
    let inverse = payload.inverse(&base).expect("valid retained mutation inverse fixture");
    assert!(!inverse.is_empty(), "insert-page/round-trips-the-concrete-inverse: a mutation that really moved the document must offer an undo");
    for step in &inverse {
        assert!(crate::standards::v1_4::subsets::base::io::mutation_bridge::apply_outcome(step.diff(&state), &mut state).messages().is_empty(), "insert-page/round-trips-the-concrete-inverse: an inverse step was refused");
    }
    assert_eq!(state, base, "insert-page/round-trips-the-concrete-inverse: the undo did not restore the committed before-snapshot");
}

/// 🔣️ Every committed file is canonical, and the numeric shape survives the round trip — a page box
/// written as `612` must not come back as `612.0`.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: PdfSnapshot = semio_framework_pack_json::from_json_str(text,semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&decoded)).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "insert-page/round-trips-the-concrete-inverse: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&(mutation()))).expect("payload encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("payload reparses");
    assert_eq!(reencoded, original, "insert-page/round-trips-the-concrete-inverse: committed payload JSON is not canonical");
}

/// 🎯️ The declared outcome is the one dispatch really produces.
#[test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = declared.get("status").and_then(serde_json::Value::as_str).expect("the outcome carries a status");
    let raised: Vec<String> = mutation().diff(&before()).messages().iter().map(|message| message.code.0.clone()).collect();
    match status {
        "applied" => assert!(raised.is_empty(), "insert-page/round-trips-the-concrete-inverse: declared applied, but dispatch raised {raised:?}"),
        _ => assert!(!raised.is_empty(), "insert-page/round-trips-the-concrete-inverse: declared {status}, but dispatch raised nothing"),
    }
}

/// 🔺️ The diff dispatch produces IS the committed diff — the load-bearing assertion, because the
/// diff is what replication ships and what undo inverts.
#[test]
fn produces_committed_diff() {
    let produced = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(mutation().diff(&before()).diff())).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "insert-page/round-trips-the-concrete-inverse: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🩹 The committed diff ALONE carries before to after.
#[test]
fn committed_diff_applies_to_after() {
    let base = before();
    let decoded: PdfDiff = semio_framework_pack_json::from_json_str(DIFF,semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    let produced = protocol::apply_diff(&decoded, &base).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "insert-page/round-trips-the-concrete-inverse: committed diff did not carry before to after");
}

/// 📡️ The payload's text, binary and JSON wire forms all round-trip — for the forward step and for
/// every inverse step it computes.
#[test]
fn op_codecs_round_trip() {
    let payload = mutation();
    for step in std::iter::once(payload.clone()).chain(payload.inverse(&before()).expect("valid retained mutation inverse fixture")) {
        assert_eq!(PdfMutation::parse_op(&step.print_op()).expect("the text op parses"), step, "insert-page/round-trips-the-concrete-inverse: the text op form does not round-trip");
        assert_eq!(PdfMutation::decode_op(&step.encode_op().expect("the binary op encodes")).expect("the binary op decodes"), step, "insert-page/round-trips-the-concrete-inverse: the binary op form does not round-trip");
        assert_eq!(
            <PdfMutation as semio_framework_value::FromValue>::from_value((serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&step)).expect("the payload encodes")).into()).expect("the payload decodes"),
            step,
            "insert-page/round-trips-the-concrete-inverse: the JSON form does not round-trip"
        );
    }
}
