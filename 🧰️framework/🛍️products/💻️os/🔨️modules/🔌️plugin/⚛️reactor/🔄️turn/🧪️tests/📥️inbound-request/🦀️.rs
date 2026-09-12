//! 📥️ The inbound half of the ABI's ONE call-into-this-actor seam: `Event::Request` in,
//! `Effect::Respond` out, on the SAME turn.
//!
//! Until this landed the turn loop matched `Event::Request { .. } => {}` — every caller's request was
//! dropped on the floor and its parked completion never settled. In the React renderer that showed up
//! as `extension.invoke-unavailable` on every `evaluate` and `meshes: 0` for eight boots
//! (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
//!
//! Driven by `⚛️reactor/🧫️fixtures/📥️inbound-request/🔣️.json`, whose TypeScript twin
//! (`🧑‍🎨engine/🧪️tests/📥️inbound-request/🟦️.ts`) drives the HOST half of the same rows.
use super::*;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct InboundRequestSeam {
    event_kind: String,
    effect_tag: String,
    outcome_arms: Vec<String>,
    origin_tag: String,
    answered_on_turn: usize,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct InboundRequestRow {
    name: String,
    bundle: bool,
    capability: String,
    request_text: String,
    outcome: String,
    #[serde(default)]
    answer_text: Option<String>,
    #[serde(default)]
    fault_code: Option<String>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct InboundRequestFixture {
    seam: InboundRequestSeam,
    capability: String,
    rows: Vec<InboundRequestRow>,
}

fn inbound_request_fixture() -> InboundRequestFixture {
    serde_json::from_str(include_str!("../../../🧫️fixtures/📥️inbound-request/🔣️.json")).expect("the inbound-request fixture must parse")
}

fn inbound_request_budget() -> semio_framework::kernel::Budget {
    semio_framework::kernel::Budget { fuel: 64, deadline_ms: 1_000, max_effects: 16, max_patch_bytes: 65_536, max_frames: 16 }
}

/// 🧩️ A bundle whose one handler mirrors what every real flow extension installs: a pure
/// `Fn(&[u8]) -> Result<Vec<u8>, Fault>` that answers in the capability's own encoding and refuses an
/// undecodable request with the SAME `extension.evaluate.bad-request` code the shipped crates use.
fn inbound_request_bundle(capability: &str) -> crate::plugin_runtime::ExtensionBundle {
    crate::plugin_runtime::ExtensionBundle::new("inbound-request", "Inbound Request", "0.0.1").extends("test").handler(capability, |request| {
        let text = std::str::from_utf8(request).map_err(|error| semio_framework::Fault::new(semio_framework::FaultOrigin::Plugin, semio_framework::FaultCode::new("extension.evaluate.bad-request"), error.to_string()))?;
        let value: serde_json::Value = serde_json::from_str(text).map_err(|error| semio_framework::Fault::new(semio_framework::FaultOrigin::Plugin, semio_framework::FaultCode::new("extension.evaluate.bad-request"), error.to_string()))?;
        Ok(serde_json::to_vec(&serde_json::json!({ "echo": value })).expect("the echo answer serializes"))
    })
}

async fn drive_inbound_request(runtime: &crate::plugin_runtime::PluginRuntime<crate::app::NoPluginApp>, req: u64, capability: &str, request: &str) -> Vec<Effect> {
    let event = Event::Request {
        req: semio_framework::kernel::RequestId(req),
        from: semio_framework::kernel::MessageEndpoint::Shell { instance: semio_framework::kernel::PluginInstanceId("0".into()) },
        capability: capability.to_string(),
        payload: request.as_bytes().to_vec(),
    };
    crate::reactor::poll_kernel(runtime, vec![event], None, None, inbound_request_budget()).await.expect("one inbound-request turn").effects
}

/// ⚖️ LAW: every fixture row is answered by exactly one `Effect::Respond` carrying the SAME `req`, on
/// the FIRST turn — an actor with no active bundle refuses with a typed fault instead of going silent,
/// an unknown capability refuses by name, and a handler's own refusal crosses as the `fault` arm
/// (pack-encoded, the shape `decodePackValue`/`decodeFaultFromWire` read on the host).
#[test]
fn every_inbound_request_row_is_answered_on_the_turn_it_arrives() {
    let fixture = inbound_request_fixture();
    assert_eq!(fixture.seam.event_kind, "request");
    assert_eq!(fixture.seam.effect_tag, "respond");
    assert_eq!(fixture.seam.outcome_arms, vec!["ok".to_string(), "fault".to_string()]);
    assert_eq!(fixture.seam.origin_tag, "shell");
    assert_eq!(fixture.seam.answered_on_turn, 1);
    semio_framework::io::resolve_ready(async {
        let runtime = crate::plugin_runtime::PluginRuntime::<crate::app::NoPluginApp>::new();
        crate::plugin_runtime::extension_deactivate().await;
        let mut installed = false;
        for (index, row) in fixture.rows.iter().enumerate() {
            if row.bundle && !installed {
                crate::plugin_runtime::install_extension_bundle(inbound_request_bundle(&fixture.capability)).await;
                crate::plugin_runtime::extension_activate().await.expect("the fixture bundle activates");
                installed = true;
            }
            let req = (index as u64) + 1;
            let effects = drive_inbound_request(&runtime, req, &row.capability, &row.request_text).await;
            let answers: Vec<&semio_framework::kernel::RequestOutcome> = effects
                .iter()
                .filter_map(|effect| if let Effect::Respond { req: answered, result } = effect { (answered.0 == req).then_some(result) } else { None })
                .collect();
            assert_eq!(answers.len(), 1, "{} must be answered exactly once", row.name);
            match (&row.outcome[..], answers[0]) {
                ("ok", semio_framework::kernel::RequestOutcome::Ok(bytes)) => {
                    let expected: serde_json::Value = serde_json::from_str(row.answer_text.as_deref().expect("an ok row declares its answer")).expect("the declared answer parses");
                    let produced: serde_json::Value = serde_json::from_slice(bytes).expect("the guest answers the capability's own encoding");
                    assert_eq!(produced, expected, "{}", row.name);
                }
                ("fault", semio_framework::kernel::RequestOutcome::Err(bytes)) => {
                    let decoded = store::pack_rt::decode_wire_value(bytes).expect("the fault arm is a pack");
                    let fault: semio_framework::Fault = dsl::from_dsl_value(decoded).expect("the fault arm decodes as a Fault");
                    assert_eq!(fault.code.0.as_str(), row.fault_code.as_deref().expect("a fault row declares its code"), "{}", row.name);
                }
                (expected, produced) => panic!("{} expected {expected}, got {produced:?}", row.name),
            }
        }
    });
}
