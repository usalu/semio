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
use semio_framework::kernel::{Event, MessageEndpoint};
#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
use semio_framework::kernel::RequestOutcome;
#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
use semio_framework_plugin_reactor_wit_bridge::wit_bridge::poll;

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
    pub name: String,
    pub bundle: bool,
    pub capability: String,
    pub request_text: String,
    pub outcome: String,
    #[serde(default)]
    answer_text: Option<String>,
    #[serde(default)]
    fault_code: Option<String>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct InboundRequestFixture {
    seam: InboundRequestSeam,
    pub capability: String,
    pub rows: Vec<InboundRequestRow>,
}

fn inbound_request_fixture() -> InboundRequestFixture {
    serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/📥️inbound-request/🔣️.json")).expect("the inbound-request fixture must parse")
}

fn inbound_request_budget() -> semio_framework::kernel::Budget {
    semio_framework::kernel::Budget { retained: semio_framework_plugin_testing_laws::artifact_app_laws::fixture_retained_turn(), fuel: 64, deadline_ms: 1_000, max_effects: 16, max_patch_bytes: 65_536, max_frames: 16 }
}

/// 🧩️ A bundle whose one handler mirrors what every real flow extension installs: a pure
/// `Fn(&[u8]) -> Result<Vec<u8>, Fault>` that answers in the capability's own encoding and refuses an
/// undecodable request with the SAME `extension.evaluate.bad-request` code the shipped crates use.
fn inbound_request_bundle(capability: &str) -> semio_framework_plugin_extension::extension_runtime_lift::ExtensionBundle {
    semio_framework_plugin_extension::extension_runtime_lift::ExtensionBundle::new("inbound-request", "Inbound Request", "0.0.1").extends("test").handler(capability, |request, cx| {
        let text = std::str::from_utf8(request).map_err(|error| semio_framework::Fault::new(semio_framework::FaultOrigin::Plugin, semio_framework::FaultCode::new("extension.evaluate.bad-request"), error.to_string()))?;
        let value: serde_json::Value = serde_json::from_str(text).map_err(|error| semio_framework::Fault::new(semio_framework::FaultOrigin::Plugin, semio_framework::FaultCode::new("extension.evaluate.bad-request"), error.to_string()))?;
        let answer = serde_json::to_vec(&serde_json::json!({ "echo": value })).expect("the echo answer serializes");
        let progress = semio_framework_value::RetainedCloneProgress { copied_items: 1, copied_bytes: answer.len(), retained_capacity_bytes: answer.capacity(), ..Default::default() };
        cx.consume_retained(progress)?;
        Ok(semio_framework_plugin_extension::extension_runtime_lift::extension_retirement::ExtensionInvokeStep { payload: Some(answer), retained_progress: progress, refusal: None })
    })
}

async fn drive_inbound_request(runtime: &semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime<semio_framework_plugin_reactor_app::NoPluginApp>, req: u64, capability: &str, request: &str,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>) -> Vec<Effect> {
    let event = Event::Request {
        req: semio_framework::kernel::RequestId(req),
        from: semio_framework::kernel::MessageEndpoint::Shell { instance: semio_framework::kernel::PluginInstanceId("0".into()) },
        capability: capability.to_string(),
        payload: request.as_bytes().to_vec(),
    };
    semio_framework_plugin_reactor_turn::poll_kernel(runtime, vec![event], None, None, inbound_request_budget(), identity,cx).await.expect("one inbound-request turn").effects
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
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().unwrap();
    let mut cx=original_caller.context();let mut identity=semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity();
    ::semio_framework_async::poll::resolve_ready(async {
        let runtime = semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime::<semio_framework_plugin_reactor_app::NoPluginApp>::new({ let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }; semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } }).expect("explicit test mounted owner policy");
        semio_framework_plugin_extension::extension_runtime_lift::extension_deactivate().await;
        let mut installed = false;
        for (index, row) in fixture.rows.iter().enumerate() {
            if row.bundle && !installed {
                assert!(semio_framework_plugin_extension::extension_runtime_lift::install_extension_bundle(&mut Some(inbound_request_bundle(&fixture.capability))).await.unwrap());
                semio_framework_plugin_extension::extension_runtime_lift::extension_activate().await.expect("the fixture bundle activates");
                installed = true;
            }
            let req = (index as u64) + 1;
            let effects = drive_inbound_request(&runtime,req,&row.capability,&row.request_text,&mut identity,&mut cx).await;
            let answers: Vec<&semio_framework::kernel::RequestOutcome> = effects.iter().filter_map(|effect| if let Effect::Respond { req: answered, result } = effect { (answered.0 == req).then_some(result) } else { None }).collect();
            assert_eq!(answers.len(), 1, "{} must be answered exactly once", row.name);
            match (&row.outcome[..], answers[0]) {
                ("ok", semio_framework::kernel::RequestOutcome::Ok(bytes)) => {
                    let expected: serde_json::Value = serde_json::from_str(row.answer_text.as_deref().expect("an ok row declares its answer")).expect("the declared answer parses");
                    let produced: serde_json::Value = serde_json::from_slice(bytes).expect("the guest answers the capability's own encoding");
                    assert_eq!(produced, expected, "{}", row.name);
                }
                ("fault", semio_framework::kernel::RequestOutcome::Err(bytes)) => {
                    let decoded = store::pack_rt::decode_wire_value(bytes).expect("the fault arm is a pack");
                    let fault: semio_framework::Fault = semio_framework_value::FromValue::from_value(decoded).expect("the fault arm decodes as a Fault");
                    assert_eq!(fault.code.0.as_str(), row.fault_code.as_deref().expect("a fault row declares its code"), "{}", row.name);
                }
                (expected, produced) => panic!("{} expected {expected}, got {produced:?}", row.name),
            }
        }
        semio_framework_plugin_extension::extension_runtime_lift::extension_dispose_cold(&mut cx).unwrap();
    });
    let fuel=cx.fuel_remaining();drop(cx);original_caller.retire(fuel).unwrap();
}
