//! 📡️ The document-backbone effect a browser guest emits, read back by the Rust half of the JS
//! program bridge: the plugin bridge (`🐚️plugin-bridge/🟦️.ts` `wgpuBackboneMessageEffect`) projects a
//! guest `send-message` to `Backbone { uri }` onto the host `Effect` JSON, and the wasm32
//! `document_backbone_effects` parses the bridge answer with this exact `from_json_str` call. The
//! TypeScript half asserts the projection over the SAME fixture, so the two halves cannot drift.
//!
//! Fixture: `🧑‍🎨engine/🧫️fixtures/📡️wgpu-document-backbone/🔣️.json`.

use semio_framework::kernel::{Effect, InvocationResult, MessageEndpoint};

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../../../🧫️fixtures/📡️wgpu-document-backbone/🔣️.json")).expect("document backbone fixture parses")
}

fn expected(host: &serde_json::Value) -> Effect {
    let message = &host["sendMessage"];
    Effect::SendMessage {
        target: MessageEndpoint::Backbone { uri: message["target"]["backbone"]["uri"].as_str().expect("fixture uri").to_string() },
        payload: message["payload"].as_array().expect("fixture payload").iter().map(|byte| u8::try_from(byte.as_u64().expect("fixture byte")).expect("fixture byte fits u8")).collect(),
    }
}

#[test]
fn a_projected_backbone_effect_parses_into_the_host_send_message() {
    let fixture = fixture();
    let projected: Vec<&serde_json::Value> = fixture["effects"].as_array().expect("effects").iter().map(|law| &law["host"]).filter(|host| !host.is_null()).collect();
    assert_eq!(projected.len(), 1);
    for host in projected {
        let parsed = dsl::os_pack::json::from_json_str::<Effect>(&host.to_string()).expect("host effect parses");
        assert_eq!(parsed, expected(host));
    }
}

#[test]
fn the_bridge_answer_carries_the_backbone_effect_to_the_shell() {
    let fixture = fixture();
    let answer = &fixture["answer"]["json"];
    let parsed = dsl::os_pack::json::from_json_str::<InvocationResult>(&answer.to_string()).expect("bridge answer parses");
    assert_eq!(parsed.requested_effects, vec![expected(&answer["requestedEffects"][0])]);
}
