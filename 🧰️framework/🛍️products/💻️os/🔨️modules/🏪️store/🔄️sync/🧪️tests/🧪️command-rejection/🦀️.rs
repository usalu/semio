//! 🚫️ Language-agnostic command-rejection fixture — Rust runner. The same
//! `🏪️store/🧫️fixtures/🧫️command-rejection/🔣️.json` drives the TypeScript actor (`💻️os/🟦️.ts` region
//! `🚫️CommandRejection`), so the Rust actor and the TypeScript actor decode every hub refusal once into the same
//! rejection and emit every local refusal in the same shape (ticket 26/09/30 NON-DESTRUCTIVE-HISTORY-EDITING follow-up 3:
//! a local counter `[69]` in the old `messages: bytes` slot reached the shell's JSON decode as the text `E`). The expected
//! values are compared through `serde_json` (dev-dependency only), independent of this crate's own JSON writer.

use super::{ArtifactEvent, CommandAckOutcome, CommandRejectionCode, CommandRejectionDetail};

const FIXTURE: &str = include_str!("../../../🧫️fixtures/🧫️command-rejection/🔣️.json");
const SCHEMA: &str = include_str!("../../🧬️schema/🔣️command-rejection/🔣️.json");

fn fixture() -> serde_json::Value {
    serde_json::from_str(FIXTURE).expect("command-rejection fixture json")
}

fn wire(outcome: &CommandAckOutcome) -> serde_json::Value {
    serde_json::from_str(&semio_framework_pack_json::to_json_string(outcome)).expect("outcome json")
}

fn code(text: &str) -> CommandRejectionCode {
    semio_framework_pack_json::from_json_str(&serde_json::Value::String(text.to_string()).to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_else(|error| panic!("fixture code {text}: {error}"))
}

fn detail(value: &serde_json::Value) -> CommandRejectionDetail {
    let count = |field: &str| value.get(field).map(|count| count.as_u64().unwrap_or_else(|| panic!("fixture detail {field}")));
    CommandRejectionDetail { envelopes: count("envelopes"), bytes: count("bytes"), limit: count("limit") }
}

#[test]
fn the_code_set_is_the_schema_code_set() {
    let schema: serde_json::Value = serde_json::from_str(SCHEMA).expect("command-rejection schema json");
    let codes: Vec<String> = schema["$defs"]["Code"]["enum"].as_array().expect("code enum").iter().map(|code| code.as_str().expect("code").to_string()).collect();
    for text in &codes {
        let decoded = code(text);
        assert_eq!(serde_json::Value::String(text.clone()), serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&decoded)).expect("code json"), "{text} round-trips");
    }
    assert_eq!(codes.len(), 10, "every CommandRejectionCode variant is a schema code");
}

#[test]
fn every_hub_refusal_decodes_once_into_the_fixture_rejection() {
    let fixture = fixture();
    for row in fixture["hub"].as_array().expect("hub rows") {
        let name = row["name"].as_str().expect("row name");
        let outcome = CommandAckOutcome::hub_rejected(row["reason"].as_str().expect("reason").to_string(), row["messages"].as_str().expect("messages").as_bytes());
        assert_eq!(wire(&outcome), row["expect"], "{name}");
        let round: CommandAckOutcome = semio_framework_pack_json::from_json_str(&row["expect"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_else(|error| panic!("{name}: {error}"));
        assert_eq!(round, outcome, "{name} decodes back");
    }
}

#[test]
fn every_local_refusal_is_the_fixture_rejection() {
    let fixture = fixture();
    for row in fixture["local"].as_array().expect("local rows") {
        let text = row["code"].as_str().expect("row code");
        let counters = row.get("detail").map(detail).unwrap_or_default();
        let outcome = CommandAckOutcome::local_rejected(code(text), row["reason"].as_str().expect("reason"), counters);
        assert_eq!(wire(&outcome), row["expect"], "{text}");
    }
}

#[test]
fn the_event_wires_its_batch_id_camel_cased() {
    let event = ArtifactEvent::CommandOutcome { batch_id: u64::MAX, outcome: CommandAckOutcome::local_rejected(CommandRejectionCode::LocalBackboneMalformed, "document backbone malformed", CommandRejectionDetail::default()) };
    let value: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&event)).expect("event json");
    assert_eq!(value, serde_json::json!({ "kind": "commandOutcome", "batchId": u64::MAX, "outcome": { "kind": "rejected", "code": "local.backbone-malformed", "reason": "document backbone malformed", "messages": [] } }));
}
