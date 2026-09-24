
use super::*;

const FLOW_VCS_PRODUCTION_LEDGER: &str = include_str!("../../🧫️fixtures/📊️.tsv");

fn bridge_budget() -> AbiWorkBudget {
    AbiWorkBudget { byte_credit: 4_096, now_ms: 0, deadline_ms: Some(8), cancelled: false, interrupted: false }
}

fn bridge_request(operation: u16, request: u64, generation: u32, bytes: Vec<u8>) -> AbiMessage {
    AbiMessage::Request(semio_framework::abi::AbiRequest {
        operation: semio_framework::abi::AbiOperation::try_new(operation).unwrap(),
        request_id: semio_framework::abi::AbiRequestId(request),
        generation,
        bytes: semio_framework::abi::AbiBytes::try_new(bytes).unwrap(),
    })
}

fn bridge_poll(bridge: &mut FlowBridge<FlowDomainAdapter>) -> AbiMessage {
    for _ in 0..512 {
        if let AbiPortPoll::Message(message) = bridge.poll(bridge_budget()).unwrap() {
            return message;
        }
    }
    panic!("production Flow VCS bridge did not progress")
}

fn acknowledge_bridge_event(bridge: &mut FlowBridge<FlowDomainAdapter>, event: &semio_framework::abi::AbiEvent) {
    bridge
        .try_send(AbiMessage::Reply(semio_framework::abi::AbiReply { request_id: event.request_id, generation: event.generation, status: semio_framework::abi::AbiStatus::OK, bytes: semio_framework::abi::AbiBytes::default() }), bridge_budget())
        .unwrap();
}

fn close_bridge(bridge: &mut FlowBridge<FlowDomainAdapter>) {
    let fixture: Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧹️session-close/🔣️.json")).unwrap();
    bridge.begin_close();
    for _ in 0..fixture["close"]["maximumTurns"].as_u64().unwrap() {
        match bridge.poll(bridge_budget()).unwrap() {
            AbiPortPoll::Message(AbiMessage::Event(event)) => acknowledge_bridge_event(bridge, &event),
            AbiPortPoll::Closed => {
                assert!(bridge.terminal_is_empty());
                return;
            }
            _ => {}
        }
    }
    panic!("retained Flow domain did not close within fixture bound");
}

/// 🧹️ Ends a domain law the way a session ends: the adapter's vcs, surface and host retire to terminal-empty within
/// the declared close bound — every Flow owner refuses a bare drop.
fn close_domain(domain: &mut FlowDomainAdapter) {
    let fixture: Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧹️session-close/🔣️.json")).unwrap();
    domain.begin_close();
    for _ in 0..fixture["close"]["maximumTurns"].as_u64().unwrap() {
        if domain.close_step(bridge_budget()).unwrap() {
            assert!(domain.terminal_is_empty());
            return;
        }
    }
    panic!("retained Flow domain did not close within fixture bound");
}

#[test]
fn compiled_session_close_retires_the_real_vcs_and_host_before_terminal_empty() {
    let fixture = crate::os_pack::json::parse(include_str!("../../🧫️fixtures/🧹️session-close/🔣️.json")).unwrap();
    let mut bridge = FlowBridge::new(FlowDomainAdapter::default);
    bridge.try_send(bridge_request(protocol::FLOW_OPERATION_OPEN, 1, 1, Vec::new()), bridge_budget()).unwrap();
    let AbiMessage::Reply(reply) = bridge_poll(&mut bridge) else { panic!("session reply") };
    let session = FlowPayloadReader::new(reply.bytes.as_slice()).handle().unwrap();
    bridge.try_send(AbiMessage::Control(semio_framework::abi::AbiControl::Close { handle: session }), bridge_budget()).unwrap();
    assert_eq!(bridge.terminal_is_empty(), fixture.get("browser").unwrap().get("terminalBeforeClose").unwrap().as_bool().unwrap());
    close_bridge(&mut bridge);
    assert_eq!(bridge.terminal_is_empty(), fixture.get("browser").unwrap().get("terminalAfterClose").unwrap().as_bool().unwrap());
    println!("[DEBUG] Flow native compiled session close: real VCS and host retired, terminal-empty=true");
}

#[test]
fn compiled_session_close_receipt_preserves_a_real_sibling_until_global_close() {
    let fixture: Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧑‍🤝‍🧑️browser-runtime/🔣️.json")).unwrap();
    let mut bridge = FlowBridge::new(FlowDomainAdapter::default);
    let mut sessions = Vec::new();
    for request in 1..=2 {
        bridge.try_send(bridge_request(protocol::FLOW_OPERATION_OPEN, request, 1, Vec::new()), bridge_budget()).unwrap();
        let AbiMessage::Reply(reply) = bridge_poll(&mut bridge) else { panic!("real session reply") };
        sessions.push(FlowPayloadReader::new(reply.bytes.as_slice()).handle().unwrap());
    }
    bridge.try_send(AbiMessage::Control(semio_framework::abi::AbiControl::Close { handle: sessions[0] }), bridge_budget()).unwrap();
    let AbiMessage::Event(receipt) = bridge_poll(&mut bridge) else { panic!("real session close receipt") };
    assert_eq!(u64::from(receipt.event.get()), fixture["receipt"]["event"].as_u64().unwrap());
    assert_eq!(receipt.request_id.0 ^ (u64::from(receipt.sequence) << 32), 1);
    assert_eq!(FlowPayloadReader::new(receipt.bytes.as_slice()).handle().unwrap(), sessions[0]);
    assert!(!bridge.terminal_is_empty());
    acknowledge_bridge_event(&mut bridge, &receipt);
    let mut payload = FlowPayloadWriter::default();
    payload.handle(sessions[1]);
    bridge.try_send(bridge_request(2_518, 3, 1, payload.finish()), bridge_budget()).unwrap();
    let mut completed = false;
    for _ in 0..64 {
        match bridge_poll(&mut bridge) {
            AbiMessage::Event(event) => acknowledge_bridge_event(&mut bridge, &event),
            AbiMessage::Reply(reply) => {
                assert_eq!(reply.request_id.0, 3);
                assert_eq!(reply.status, semio_framework::abi::AbiStatus::OK);
                let independent: Value = serde_json::from_slice(reply.bytes.as_slice()).unwrap();
                let own = crate::os_pack::json::parse(std::str::from_utf8(reply.bytes.as_slice()).unwrap()).unwrap();
                assert_eq!(crate::os_pack::json::to_string(&own), independent.to_string());
                completed = true;
                break;
            }
            _ => panic!("unexpected sibling payload"),
        }
    }
    assert!(completed);
    assert!(!bridge.terminal_is_empty());
    close_bridge(&mut bridge);
    println!("[DEBUG] Flow real adapter session A retired with its exact receipt; sibling B completed selection before global terminal close");
}

#[test]
fn linear_memory_close_preserves_the_exact_retained_event_until_delivery_and_ack() {
    std::thread::spawn(|| {
        let mut bytes = vec![0; 4096];
        BRIDGE.with(|bridge| bridge.borrow_mut().try_send(bridge_request(protocol::FLOW_OPERATION_OPEN, 1, 1, Vec::new()), bridge_budget()).unwrap());
        let length = unsafe { flow_bridge_poll(bytes.as_mut_ptr(), bytes.len(), 4096, 0, 8) };
        assert!(length > 0);
        let AbiMessage::Reply(reply) = decode_abi_message(&bytes[..length as usize]).unwrap() else { panic!("open reply") };
        let session = FlowPayloadReader::new(reply.bytes.as_slice()).handle().unwrap();
        let mut payload = FlowPayloadWriter::default();
        payload.handle(session);
        BRIDGE.with(|bridge| bridge.borrow_mut().try_send(bridge_request(2_518, 2, 1, payload.finish()), bridge_budget()).unwrap());
        let needed = unsafe { flow_bridge_poll(bytes.as_mut_ptr(), 1, 4096, 0, 8) };
        assert!(needed > 1);
        let expected = RETAINED.with(|retained| retained.borrow().as_ref().unwrap().bytes.clone());
        flow_bridge_begin_close();
        assert_eq!(flow_bridge_terminal_is_empty(), 0);
        RETAINED.with(|retained| assert_eq!(retained.borrow().as_ref().unwrap().bytes, expected));
        let delivered = unsafe { flow_bridge_poll(bytes.as_mut_ptr(), bytes.len(), 4096, 0, 8) };
        assert_eq!(delivered, needed);
        assert_eq!(&bytes[..delivered as usize], expected);
        let AbiMessage::Event(event) = decode_abi_message(&expected).unwrap() else { panic!("retained event") };
        BRIDGE.with(|bridge| acknowledge_bridge_event(&mut bridge.borrow_mut(), &event));
        for _ in 0..4096 {
            if flow_bridge_terminal_is_empty() == 1 {
                break;
            }
            let length = unsafe { flow_bridge_poll(bytes.as_mut_ptr(), bytes.len(), 4096, 0, 8) };
            if length > 0 {
                if let AbiMessage::Event(event) = decode_abi_message(&bytes[..length as usize]).unwrap() {
                    BRIDGE.with(|bridge| acknowledge_bridge_event(&mut bridge.borrow_mut(), &event));
                }
            }
        }
        assert_eq!(flow_bridge_terminal_is_empty(), 1);
        println!("[DEBUG] Flow linear-memory close: retained event delivered once and acknowledged before terminal-empty");
    })
    .join()
    .unwrap();
}

fn text_payload(value: &str) -> Vec<u8> {
    let mut writer = FlowPayloadWriter::default();
    writer.bytes(value.as_bytes()).unwrap();
    writer.finish()
}

fn surface_payload(surface: u32, generation: u32, width: u32, height: u32, dpr: f64) -> Vec<u8> {
    let mut writer = FlowPayloadWriter::default();
    writer.u32(surface);
    writer.u32(generation);
    writer.u32(width);
    writer.u32(height);
    writer.f64(dpr);
    writer.finish()
}

fn surface_status_payload(surface: u32, generation: u32, status: &str) -> Vec<u8> {
    let mut writer = FlowPayloadWriter::default();
    writer.u32(surface);
    writer.u32(generation);
    writer.bytes(status.as_bytes()).unwrap();
    writer.finish()
}

fn run(domain: &mut FlowDomainAdapter, operation: u16, payload: Vec<u8>) -> Result<Vec<u8>, FlowFailure> {
    let arguments = FlowArguments::preflight(operation, payload)?;
    let mut action = flow_action(operation, &arguments).ok_or_else(|| abi_failure(AbiErrorCode::UnknownOperation))?;
    loop {
        match action.advance(domain, &arguments, AbiWorkBudget::credits(1)) {
            FlowFeatureStep::Complete(output) => return Ok(output),
            FlowFeatureStep::Failed(failure) => return Err(failure),
            FlowFeatureStep::Yield | FlowFeatureStep::Progress { .. } | FlowFeatureStep::Checkpoint(_) | FlowFeatureStep::Preview(_) | FlowFeatureStep::SurfaceStatus(_) | FlowFeatureStep::RetainedPage(_) => {}
        }
    }
}

#[test]
fn malformed_omitted_and_unknown_selection_data_remain_owned() {
    let mut domain = FlowDomainAdapter::default();
    for json in ["{", "{}", r#"{"widgets":[],"futureOptional":true}"#] {
        run(&mut domain, 2_525, text_payload(json)).unwrap();
    }
    assert_eq!(domain.host.selected_widget_ids_json(), "[]");
    close_domain(&mut domain);
}

#[test]
fn surface_generation_cancel_loss_and_recovery_fail_closed() {
    let mut domain = FlowDomainAdapter::default();
    run(&mut domain, 2_575, surface_payload(7, 2, 800, 600, 2.0)).unwrap();
    assert_eq!(run(&mut domain, 2_575, surface_payload(7, 2, 800, 600, 2.0)).unwrap_err().code, AbiErrorCode::Busy);
    assert_eq!(run(&mut domain, 2_576, surface_status_payload(8, 2, "created")).unwrap_err().code, AbiErrorCode::UnknownHandle);
    assert_eq!(run(&mut domain, 2_576, surface_status_payload(7, 1, "created")).unwrap_err().code, AbiErrorCode::AbaHandle);
    assert_eq!(run(&mut domain, 2_576, surface_status_payload(7, 3, "created")).unwrap_err().code, AbiErrorCode::StaleGeneration);
    run(&mut domain, 2_576, surface_status_payload(7, 2, "created")).unwrap();
    run(&mut domain, 2_576, surface_status_payload(7, 2, "device-lost")).unwrap();
    run(&mut domain, 2_576, surface_status_payload(7, 2, "recovered")).unwrap();
    run(&mut domain, 2_576, surface_status_payload(7, 2, "cancelled")).unwrap();
    assert!(domain.surface.is_none());
    close_domain(&mut domain);
}

#[test]
fn malformed_fixed_width_is_rejected_before_surface_mutation() {
    let mut domain = FlowDomainAdapter::default();
    let mut payload = surface_payload(u32::MAX, 1, 1, 1, 1.0);
    payload.push(0);
    let rejected = run(&mut domain, 2_575, payload).unwrap_err();
    assert_eq!(rejected.code, AbiErrorCode::MalformedLength);
    assert!(domain.surface.is_none());
    close_domain(&mut domain);
}

#[test]
fn every_schema_feature_has_a_distinct_action_binding() {
    let expected: Vec<u16> = (2_504..=2_610).filter(|operation| !matches!(operation, 2_603 | 2_604)).collect();
    let actions: Vec<u16> = expected
        .iter()
        .copied()
        .map(|operation| flow_action(operation, &FlowArguments::preflight(operation, Vec::new()).unwrap_or_else(|_| FlowArguments { payload: Vec::new(), spans: [FlowArgumentSpan::EMPTY; 8], count: 0 })).unwrap().operation())
        .collect();
    assert_eq!(actions, expected);
}

#[test]
fn synchronized_document_json_is_the_exact_retained_document() {
    let mut domain = FlowDomainAdapter::default();
    let mut expected = crate::artifact::FlowHostSnapshot::default();
    expected.schema = "flow.host_snapshot.synchronized".into();
    for (index, widget) in ["slider", "add", "preview"].into_iter().enumerate() {
        expected.layout.insert(widget.into(), crate::artifact::WidgetLayout { x: 240.0 * index as f64, y: 40.0 });
    }
    let json = crate::os_pack::json::to_json_string(&expected);
    run(&mut domain, 2_610, text_payload(&json)).unwrap();
    let bytes = run(&mut domain, 2_609, Vec::new()).unwrap();
    let value = crate::os_pack::json::parse(std::str::from_utf8(&bytes).unwrap()).unwrap();
    let actual = <crate::artifact::FlowHostSnapshot as crate::os_dsl::FromValue>::from_value(crate::os_pack::json::to_dsl_value(&value)).unwrap();
    let equal = actual == expected;
    let report = format!("{actual:?}\n!=\n{expected:?}");
    actual.retire_cold();
    expected.retire_cold();
    close_domain(&mut domain);
    assert!(equal, "{report}");
}

/// 🧾️ Retained rows the measured document carries. Sized so the JSON is worth measuring
/// (5 140 crossings at credit 1 against 612 for the app's own starter document) while the session's
/// retirement still fits the declared close bound (`🧫️fixtures/🧹️session-close/🔣️.json`,
/// `maximumTurns` 4 096 — this document cost 2 627 turns to close before the ladder was anchored on
/// its retained items, and costs 11 now, at any row count).
const MEASURED_ROWS: u32 = 24;

fn row_document_json(rows: u32) -> String {
    let mut document = crate::artifact::FlowHostSnapshot::default();
    for index in 0..rows {
        document.widgets.push(Widget::InputNote { id: format!("measured-{index}"), text: format!("measured payload row {index}: a retained node-graph draw list carries thousands of bytes of exactly this shape") });
    }
    let json = crate::os_pack::json::to_json_string(&document);
    document.retire_cold();
    json
}

fn measured_document_json() -> String {
    row_document_json(MEASURED_ROWS)
}

/// 🔁️ Polls the bridge until it hands out one message.
fn flow_poll_one(bridge: &mut FlowBridge<FlowDomainAdapter>, budget: AbiWorkBudget) -> AbiMessage {
    for _ in 0..8_000_000 {
        if let AbiPortPoll::Message(message) = bridge.poll(budget).unwrap() {
            return message;
        }
    }
    panic!("Flow bridge did not progress")
}

/// 🧵️ Drives ONE request to its reply, acknowledging every event and page the bridge raises, and
/// answers how many `Progress` events crossed, the reply's status, and the bytes it answered.
fn flow_drain_request(bridge: &mut FlowBridge<FlowDomainAdapter>, budget: AbiWorkBudget, request_id: u64) -> (usize, semio_framework::abi::AbiStatus, Vec<u8>) {
    let mut progress = 0usize;
    let mut operation = None;
    let mut answered = Vec::new();
    loop {
        match flow_poll_one(bridge, budget) {
            AbiMessage::Event(event) => {
                if event.event.get() == protocol::FLOW_EVENT_ADMITTED {
                    operation = Some(FlowPayloadReader::new(event.bytes.as_slice()).handle().unwrap());
                }
                if event.event.get() == protocol::FLOW_EVENT_PROGRESS {
                    progress += 1;
                }
                acknowledge_bridge_event(bridge, &event);
            }
            AbiMessage::Page(page) => {
                let operation = operation.expect("admitted Flow operation");
                answered.extend_from_slice(page.bytes.as_slice());
                bridge.try_send(AbiMessage::Control(semio_framework::abi::AbiControl::Acknowledge { handle: operation, index: page.index }), budget).unwrap();
            }
            AbiMessage::Reply(reply) if reply.request_id.0 == request_id => {
                answered.extend_from_slice(reply.bytes.as_slice());
                return (progress, reply.status, answered);
            }
            _ => {}
        }
    }
}

/// 🚪️ Opens one real session on a fresh bridge.
fn flow_open_session(bridge: &mut FlowBridge<FlowDomainAdapter>, budget: AbiWorkBudget) -> semio_framework::abi::AbiHandle {
    bridge.try_send(bridge_request(protocol::FLOW_OPERATION_OPEN, 1, 1, Vec::new()), budget).unwrap();
    let AbiMessage::Reply(opened) = flow_poll_one(bridge, budget) else { panic!("Flow session reply") };
    let mut reader = FlowPayloadReader::new(opened.bytes.as_slice());
    let session = reader.handle().unwrap();
    reader.finish().unwrap();
    session
}

/// ✍️ A request payload: the session handle the bridge routes by, then the operation's own text.
fn flow_session_text(session: semio_framework::abi::AbiHandle, text: &str) -> Vec<u8> {
    let mut writer = FlowPayloadWriter::default();
    writer.handle(session);
    writer.bytes(text.as_bytes()).unwrap();
    writer.finish()
}

/// ⚡️ Drives one whole `documentJson` through the REAL bridge at a given poll credit and reports
/// what it cost: how many `FLOW_EVENT_PROGRESS` events crossed, and the bytes the operation answered.
///
/// The document is seeded through the bridge's own `synchronizeDocument` first, so the measured
/// crossing carries a real payload rather than an empty default.
///
/// Counted at the bridge rather than at the feature because the event is the unit that matters: every
/// `Progress` is its own ABI message the host has to poll, decode and reply to.
fn document_json_transfer(byte_credit: usize) -> (usize, Vec<u8>) {
    let budget = AbiWorkBudget { byte_credit, now_ms: 0, deadline_ms: None, cancelled: false, interrupted: false };
    let mut bridge = FlowBridge::new(FlowDomainAdapter::default);
    let session = flow_open_session(&mut bridge, budget);

    bridge.try_send(bridge_request(2_610, 2, 1, flow_session_text(session, &measured_document_json())), budget).unwrap();
    flow_drain_request(&mut bridge, budget, 2);

    let mut payload = FlowPayloadWriter::default();
    payload.handle(session);
    bridge.try_send(bridge_request(2_609, 3, 1, payload.finish()), budget).unwrap();
    let (progress, _, answered) = flow_drain_request(&mut bridge, budget, 3);
    close_bridge(&mut bridge);
    (progress, answered)
}

/// ⚡️ One poll spends the byte credit it was GRANTED, not one byte.
///
/// Every incremental phase of a flow operation — argument decode, the retained-DAG cursors, output
/// encode — advances by a single byte and answers `Progress`, and the bridge makes each `Progress`
/// its own ABI message. A poll granted 4 096 bytes therefore moved one byte, so an operation's
/// payload crossed at roughly 68 KB/s: the node-graph board's own draw list cost ~20 000 round trips
/// per frame, one present took 200–450 ms idle and over 2.5 s under a scroll gesture, and the Flow
/// window painted ONCE for a thirty-tick scroll (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
/// `📓️flow-scroll-render-perf-2026-09-15.md` §3).
///
/// The law is the ratio, not an absolute: at N times the credit an operation must cost at most about
/// 1/N of the crossings, so a future phase that forgets to honour its budget fails here.
#[test]
fn one_poll_spends_its_whole_byte_credit_instead_of_one_byte() {
    let (single, _) = document_json_transfer(1);
    let (granted, _) = document_json_transfer(4_096);
    println!("flow poll credit: {MEASURED_ROWS} retained rows crossed in {single} progress events at credit 1 and {granted} at credit 4096");
    assert!(single > 1_000, "the document payload must be large enough to measure: {single} crossings at credit 1");
    assert!(granted * 100 < single, "credit 4096 cost {granted} crossings against {single} at credit 1 — the budget is not being spent");
    assert!(granted >= 1, "an operation still reports progress at least once before it completes");
}

/// 🛍️ Drives one `setCatalogueJson` payload through a fresh session and answers what the guest then
/// holds, read back through its own `catalogueJson`. Fresh bridge per call, because the point of the
/// law is what a SECOND session inherits from the process the first one registered into.
fn catalogue_round_trip(payload: &str) -> (semio_framework::abi::AbiStatus, String) {
    let budget = bridge_budget();
    let mut bridge = FlowBridge::new(FlowDomainAdapter::default);
    let session = flow_open_session(&mut bridge, budget);
    bridge.try_send(bridge_request(2_505, 2, 1, flow_session_text(session, payload)), budget).unwrap();
    let (_, status, _) = flow_drain_request(&mut bridge, budget, 2);
    let mut read = FlowPayloadWriter::default();
    read.handle(session);
    bridge.try_send(bridge_request(2_504, 3, 1, read.finish()), budget).unwrap();
    let (_, _, answered) = flow_drain_request(&mut bridge, budget, 3);
    close_bridge(&mut bridge);
    (status, String::from_utf8(answered).expect("catalogue json is utf-8"))
}

/// 📦️ An app-static payload crosses the ABI once per CONTENT GENERATION, not once per surface attach.
///
/// Every flow session in a page lives in one wasm module, and the operator catalogue is the same
/// bytes for all of them — 88 438 B of `setNeuronKindInfosJson` plus 22 849 B of `setCatalogueJson`,
/// re-crossed by every board that attached (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
/// `📓️flow-scroll-render-perf-2026-09-15.md` §9). A second session names what the first delivered.
#[test]
fn a_shared_payload_crosses_once_and_a_second_session_names_it() {
    retire_flow_shared_payloads();
    let sections = r#"[{"id":"shared-payload-law","title":"Shared Payload Law","items":[{"kind":"neuron","neuronKind":"brep.box","name":"Box","abbreviation":"Box","icon":"cube","summary":"a measured catalogue entry"}]}]"#;
    let digest = "24.abc.def";
    let (carried_status, carried) = catalogue_round_trip(&format!("@{digest}\n{sections}"));
    assert_eq!(carried_status, semio_framework::abi::AbiStatus::OK);
    let (named_status, named) = catalogue_round_trip(&format!("@{digest}"));
    assert_eq!(named_status, semio_framework::abi::AbiStatus::OK);
    assert!(carried.contains("Shared Payload Law"), "the carried body must reach the guest catalogue: {carried}");
    assert_eq!(named, carried, "a named payload must install exactly what the carried one did");
    println!("shared flow payload: carried {} B, named {} B for the same catalogue", sections.len() + digest.len() + 2, digest.len() + 1);
}

/// 📦️ A reference this guest process does not hold FAILS, so the sender carries the body again
/// instead of installing an empty catalogue behind a stale note of what the guest holds.
#[test]
fn a_shared_payload_reference_the_guest_lost_fails_closed() {
    retire_flow_shared_payloads();
    let (status, answered) = catalogue_round_trip("@a-digest-this-process-never-registered");
    assert_ne!(status, semio_framework::abi::AbiStatus::OK, "an unheld shared payload reference must not answer OK");
    assert!(!answered.contains("Shared Payload Law"), "a failed reference must install nothing: {answered}");
}

/// 📦️ The envelope itself: a carried body registers and answers itself, a reference answers the
/// registered body, an unheld reference is an error, and a bare body is its own payload.
#[test]
fn shared_payload_envelope_carries_names_and_fails_closed() {
    retire_flow_shared_payloads();
    assert!(resolve_flow_shared_payload("@missing").is_err());
    assert_eq!(resolve_flow_shared_payload("[]").expect("a bare body is its own payload"), "[]");
    assert_eq!(resolve_flow_shared_payload("@k1\n[1,2,3]").expect("a carried body answers itself"), "[1,2,3]");
    assert_eq!(resolve_flow_shared_payload("@k1").expect("a registered digest answers its body"), "[1,2,3]");
    retire_flow_shared_payloads();
    assert!(resolve_flow_shared_payload("@k1").is_err(), "a retired registry holds nothing");
}

/// 🧩️ A payload composed of parts names the parts the guest holds and carries only the new one — the
/// app-static operator table must not re-cross to append the scene's own records.
#[test]
fn a_composed_payload_carries_only_the_part_the_guest_has_never_seen() {
    retire_flow_shared_payloads();
    let separator = FLOW_SHARED_PAYLOAD_PART_SEPARATOR;
    let app = "[\"app-operator\"]";
    let scene = "[\"scene-operator\"]";
    let carried = format!("@app\n{app}");
    assert_eq!(resolve_flow_shared_payload_parts(&carried).unwrap(), vec![app.to_string()]);
    let appended = format!("@app{separator}@scene\n{scene}");
    assert_eq!(resolve_flow_shared_payload_parts(&appended).unwrap(), vec![app.to_string(), scene.to_string()]);
    assert!(appended.len() * 4 < carried.len() + scene.len() + 32 || appended.len() < carried.len() + scene.len(), "appending must not re-carry the first part");
    let both_named = format!("@app{separator}@scene");
    assert_eq!(resolve_flow_shared_payload_parts(&both_named).unwrap(), vec![app.to_string(), scene.to_string()]);
    retire_flow_shared_payloads();
    assert!(resolve_flow_shared_payload_parts(&both_named).is_err(), "a retired registry holds no part");
}

/// 🧩️ A two-part operator table installs exactly the ids of both parts, in the order the host wrote
/// them — the composition must mean what one concatenated body meant.
#[test]
fn a_composed_operator_table_installs_every_part() {
    retire_flow_shared_payloads();
    let app = r#"[{"id":"app.only","extension":"app","name":"App Only","abbreviation":"AO","icon":"box","summary":"","inputs":[],"outputs":[]}]"#;
    let scene = r#"[{"id":"scene.only","extension":"scene","name":"Scene Only","abbreviation":"SO","icon":"box","summary":"","inputs":[],"outputs":[]}]"#;
    let separator = FLOW_SHARED_PAYLOAD_PART_SEPARATOR;
    let mut host = FlowHost::default();
    host.set_neuron_kind_infos_payload(&format!("@app\n{app}{separator}@scene\n{scene}")).expect("a carried composition installs");
    assert_eq!(host.neuron_kind_ids(), vec!["app.only".to_string(), "scene.only".to_string()], "both parts must install");
    host.set_neuron_kind_infos_payload(&format!("@app{separator}@scene")).expect("a fully named composition installs");
    assert_eq!(host.neuron_kind_ids(), vec!["app.only".to_string(), "scene.only".to_string()], "naming both parts must install the same table");
    host.retire_cold();
}

/// ⚡️ Spending the credit may only change how many polls an operation took, never what it answered.
#[test]
fn spending_the_credit_does_not_change_the_operation_result() {
    let (_, single) = document_json_transfer(1);
    let (_, granted) = document_json_transfer(4_096);
    assert!(!single.is_empty(), "documentJson answered nothing");
    assert_eq!(single, granted);
}

#[test]
fn cancellation_prevents_the_bound_domain_action() {
    let domain = Rc::new(RefCell::new(FlowDomainAdapter::default()));
    let session = semio_framework::abi::AbiHandle::try_new(1, 1).unwrap();
    domain.borrow_mut().bind_session(session);
    let mut writer = FlowPayloadWriter::default();
    writer.u32(1);
    writer.u64(0);
    writer.u64(0);
    let admission = FlowFeatureAdmission { session, request_generation: 1 };
    let mut feature = FlowDomainAdapter::start_feature(Rc::clone(&domain), admission, 2_501, writer.finish()).unwrap();
    assert!(matches!(feature.step(AbiWorkBudget::credits(64)), FlowFeatureStep::Progress { completed: 0, total: 3 }));
    feature.cancel(AbiWorkBudget::credits(64)).unwrap();
    assert!(!feature.close_step(AbiWorkBudget::credits(64)).unwrap());
    while !feature.close_step(AbiWorkBudget::credits(64)).unwrap() {}
    drop(feature);
    close_domain(&mut domain.borrow_mut());
}

#[test]
fn production_bridge_installs_vcs_authority_page_ack_retry_and_incremental_close() {
    let mut bridge = FlowBridge::new(FlowDomainAdapter::default);
    bridge.try_send(bridge_request(protocol::FLOW_OPERATION_OPEN, 1, 1, Vec::new()), bridge_budget()).unwrap();
    let AbiMessage::Reply(opened) = bridge_poll(&mut bridge) else { panic!("Flow session reply") };
    let mut opened_reader = FlowPayloadReader::new(opened.bytes.as_slice());
    let session = opened_reader.handle().unwrap();
    opened_reader.finish().unwrap();

    let mut payload = FlowPayloadWriter::default();
    payload.handle(session);
    payload.u32(session.generation());
    payload.u64(0);
    payload.u64(0);
    bridge.try_send(bridge_request(2_501, 2, 1, payload.finish()), bridge_budget()).unwrap();

    let mut operation = None;
    let mut page_seen = false;
    let mut events = [false; 7];
    loop {
        match bridge_poll(&mut bridge) {
            AbiMessage::Event(event) => {
                if event.event.get() == protocol::FLOW_EVENT_ADMITTED {
                    operation = Some(FlowPayloadReader::new(event.bytes.as_slice()).handle().unwrap());
                }
                events[usize::from(event.event.get() - protocol::FLOW_EVENT_ADMITTED)] = true;
                acknowledge_bridge_event(&mut bridge, &event);
            }
            AbiMessage::Page(page) => {
                let operation = operation.expect("admitted Flow VCS operation");
                assert_eq!(page.handle, operation);
                assert!(!page.bytes.is_empty());
                assert!(bridge.try_send(AbiMessage::Control(semio_framework::abi::AbiControl::Acknowledge { handle: operation, index: page.index + 1 }), bridge_budget()).is_err());
                bridge.try_send(AbiMessage::Control(semio_framework::abi::AbiControl::Acknowledge { handle: operation, index: page.index }), bridge_budget()).unwrap();
                page_seen = true;
            }
            AbiMessage::Reply(reply) if reply.request_id.0 == 2 => {
                assert_eq!(reply.status, semio_framework::abi::AbiStatus::OK);
                assert!(reply.bytes.is_empty());
                break;
            }
            _ => {}
        }
    }
    assert!(page_seen);
    for event in [protocol::FLOW_EVENT_ADMITTED, protocol::FLOW_EVENT_PROGRESS, protocol::FLOW_EVENT_CHECKPOINT, protocol::FLOW_EVENT_PREVIEW, protocol::FLOW_EVENT_OUTPUT, protocol::FLOW_EVENT_TERMINAL] {
        assert!(events[usize::from(event - protocol::FLOW_EVENT_ADMITTED)]);
    }

    let mut fault_payload = FlowPayloadWriter::default();
    fault_payload.handle(session);
    fault_payload.u32(session.generation());
    fault_payload.u64(1);
    fault_payload.u64(0);
    bridge.try_send(bridge_request(2_502, 3, 1, fault_payload.finish()), bridge_budget()).unwrap();
    loop {
        match bridge_poll(&mut bridge) {
            AbiMessage::Event(event) => acknowledge_bridge_event(&mut bridge, &event),
            AbiMessage::Reply(reply) if reply.request_id.0 == 3 => {
                assert_ne!(reply.status, semio_framework::abi::AbiStatus::OK);
                break;
            }
            _ => {}
        }
    }

    let mut cancel_payload = FlowPayloadWriter::default();
    cancel_payload.handle(session);
    cancel_payload.u32(session.generation());
    cancel_payload.u64(1);
    cancel_payload.u64(0);
    bridge.try_send(bridge_request(2_501, 4, 1, cancel_payload.finish()), bridge_budget()).unwrap();
    let AbiMessage::Event(admitted) = bridge_poll(&mut bridge) else { panic!("cancel admission") };
    assert_eq!(admitted.event.get(), protocol::FLOW_EVENT_ADMITTED);
    acknowledge_bridge_event(&mut bridge, &admitted);
    let AbiMessage::Event(progress) = bridge_poll(&mut bridge) else { panic!("cancel progress") };
    assert_eq!(progress.event.get(), protocol::FLOW_EVENT_PROGRESS);
    acknowledge_bridge_event(&mut bridge, &progress);
    bridge.try_send(AbiMessage::Control(semio_framework::abi::AbiControl::Cancel { request_id: semio_framework::abi::AbiRequestId(4), generation: 1 }), bridge_budget()).unwrap();
    loop {
        match bridge_poll(&mut bridge) {
            AbiMessage::Event(event) => acknowledge_bridge_event(&mut bridge, &event),
            AbiMessage::Reply(reply) if reply.request_id.0 == 4 => {
                assert_ne!(reply.status, semio_framework::abi::AbiStatus::OK);
                break;
            }
            _ => {}
        }
    }
    bridge.try_send(AbiMessage::Control(semio_framework::abi::AbiControl::Close { handle: session }), bridge_budget()).unwrap();
    assert!(!bridge.terminal_is_empty());
    close_bridge(&mut bridge);
    assert!(bridge.terminal_is_empty());
}

#[test]
fn production_reachability_fixture_and_hostile_source_census_reject_the_old_route() {
    let component = include_str!("../../🦀️.rs");
    let production = component.split_once("//#region 🧪️DomainLaws").expect("Flow production/test boundary").0;
    let bridge_protocol = include_str!("../../📡️protocol/🦀️.rs");
    let protocol_production = bridge_protocol.split_once("//#region 🧪️Laws").expect("Flow protocol production/test boundary").0;
    let schema = protocol::FLOW_ABI_SCHEMA;
    let host = include_str!("../../🖥️host/🏃️runtime/🟨️.js");
    let browser = include_str!("../../🌐️browser/🏃️runtime/🟨️.js");
    let packaged_host = include_str!("../../../🫀️core/🕸️bindings/🖥️host/🟨️.js");
    let packaged_browser = include_str!("../../../🫀️core/🕸️bindings/🌐️browser/🟨️.js");
    let package_manifest = include_str!("../../../🫀️core/🕸️bindings/package.json");
    let package_build = include_str!("../../../🫀️core/📦️packages/🦀️rust/📜️script.ts");
    let production_loader = include_str!("../../../../📺️renderer/🧑‍🎨engine/🧱️elements/🪪️WasmSessionLoader/🟦️.tsx");
    for symbol in ["vcsCheckpoint", "vcsFault", "vcsRetryCheckpoint"] {
        assert!(schema.contains(symbol));
        assert!(host.contains(symbol));
        assert!(packaged_host.contains(symbol));
    }
    for legacy in ["loadFixtureJson", "resyncFixtureJson", "fixtureJson"] {
        assert!(!schema.contains(legacy));
        assert!(!host.contains(legacy));
        assert!(!packaged_host.contains(legacy));
    }
    let callers = [
        ["FlowRetained", "Vcs::new"].concat(),
        [".begin_check", "point(self.authority)"].concat(),
        ["vcs.poll(", "handle, grant)"].concat(),
        ["vcs.take_", "page(handle)"].concat(),
        ["vcs.resume_", "page(handle, sequence)"].concat(),
        ["vcs.retry_", "page(handle, sequence)"].concat(),
        ["vcs.acknowledge_", "page(handle, sequence)"].concat(),
        ["vcs.cancel(", "handle, grant)"].concat(),
        ["vcs.fault(", "handle, grant)"].concat(),
        ["vcs.close_operation_", "step(handle, grant)"].concat(),
        ["vcs.close_retired_", "step(grant)"].concat(),
    ];
    for caller in callers {
        assert!(production.contains(&caller), "missing production Flow VCS caller {caller}");
    }
    for legacy_mapping in [["2_501 => Some(Box::new(Flow", "Action2501"].concat(), ["2_502 => Some(Box::new(Flow", "Action2502"].concat(), ["2_503 => Some(Box::new(Flow", "Action2503"].concat()] {
        assert!(!production.contains(&legacy_mapping));
    }
    assert!(production.contains("FlowBridge::new(FlowDomainAdapter::default)"));
    assert!(production.contains("matches!(operation, 2_501..=2_503)"));
    assert!(protocol_production.contains("D::start_feature(session.borrow().domain.clone(), admission, code, payload)"));
    assert!(browser.contains("createFlowFeatures(host)"));
    assert!(browser.contains("Object.keys(FlowOperation).slice(1)"));
    for export in ["flow_bridge_send", "flow_bridge_poll", "flow_bridge_begin_close", "flow_bridge_terminal_is_empty"] {
        assert!(production.contains(export));
        assert!(host.contains(export));
    }
    for control in ["encodeCancel(", "encodeClose(", "encodeAcknowledge("] {
        assert!(host.contains(control));
    }
    assert_eq!(host, packaged_host);
    assert_eq!(browser, packaged_browser);
    assert!(package_manifest.contains("\"./🌐️flow-browser.js\""));
    assert!(package_build.contains("manifest.exports"));
    assert!(package_build.contains("publishFlowBrowserPackage(join(FAMILY_RS_DIR"));
    assert!(production_loader.contains("import(\"@semio-tech/flow-core/🌐️flow-browser.js\")"));
    let rows: Vec<&str> = FLOW_VCS_PRODUCTION_LEDGER.lines().skip(1).collect();
    assert_eq!(rows.len(), 6);
    for unit in ["begin_checkpoint", "poll", "take_page", "resume_page", "retry_page", "acknowledge_page", "cancel", "fault", "close_operation_step", "close_retired_step"] {
        assert!(FLOW_VCS_PRODUCTION_LEDGER.contains(unit));
    }
}

#[test]
fn selected_widget_query_uses_census_and_multiple_cancellable_grants() {
    let mut domain = FlowDomainAdapter::default();
    let arguments = FlowArguments::preflight(2_518, Vec::new()).unwrap();
    let mut action = flow_action(2_518, &arguments).unwrap();
    let mut grants = 0usize;
    let output = loop {
        grants += 1;
        match action.advance(&mut domain, &arguments, AbiWorkBudget::credits(1)) {
            FlowFeatureStep::Complete(output) => break output,
            FlowFeatureStep::Failed(failure) => panic!("selected widget cursor failed: {failure:?}"),
            _ => {}
        }
    };
    assert!(grants > output.len() + 4);
    assert_eq!(output, domain.host.selected_widget_ids_json().into_bytes());

    let mut action = flow_action(2_518, &arguments).unwrap();
    assert!(matches!(action.advance(&mut domain, &arguments, AbiWorkBudget::credits(1)), FlowFeatureStep::Progress { .. }));
    let mut cancelled = AbiWorkBudget::credits(1);
    cancelled.cancelled = true;
    assert!(matches!(action.advance(&mut domain, &arguments, cancelled), FlowFeatureStep::Failed(FlowFailure { code: AbiErrorCode::Cancelled, .. })));
    close_domain(&mut domain);
}

//#region 🪜️RetirementLadder

/// 📏️ A retained document whose STRUCTURE is fixed (eight note rows) and whose PAYLOAD is scaled to
/// `target_bytes`, so a close-turn census that changes with the size is reading bytes and one that
/// does not is reading surfaces (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
fn scaled_document_json(target_bytes: usize) -> String {
    const ROWS: usize = 8;
    let text = "measured retained payload ".repeat(1 + target_bytes / (ROWS * 26));
    let mut document = crate::artifact::FlowHostSnapshot::default();
    for index in 0..ROWS {
        document.widgets.push(Widget::InputNote { id: format!("scaled-{index}"), text: text.clone() });
    }
    let json = crate::os_pack::json::to_json_string(&document);
    document.retire_cold();
    json
}

/// 🧮️ Seeds one real session with `document_json` through the real bridge, closes it, and answers
/// how many close TURNS the retirement ladder needed — one turn is one `poll`, which is one ABI
/// round trip the host has to make.
fn close_ladder_turns(document_json: &str) -> usize {
    let budget = bridge_budget();
    let mut bridge = FlowBridge::new(FlowDomainAdapter::default);
    let session = flow_open_session(&mut bridge, budget);
    bridge.try_send(bridge_request(2_610, 2, 1, flow_session_text(session, document_json)), budget).unwrap();
    flow_drain_request(&mut bridge, budget, 2);
    let mut read = FlowPayloadWriter::default();
    read.handle(session);
    bridge.try_send(bridge_request(2_609, 3, 1, read.finish()), budget).unwrap();
    flow_drain_request(&mut bridge, budget, 3);
    bridge.begin_close();
    let mut turns = 0usize;
    loop {
        turns += 1;
        assert!(turns < 8_000_000, "retained Flow domain never closed");
        match bridge.poll(budget).unwrap() {
            AbiPortPoll::Message(AbiMessage::Event(event)) => acknowledge_bridge_event(&mut bridge, &event),
            AbiPortPoll::Closed => {
                assert!(bridge.terminal_is_empty());
                return turns;
            }
            _ => {}
        }
    }
}

/// 🪜️ A retained document retires in turns proportional to the SURFACES it holds, never to the bytes
/// under them.
///
/// The close ladder had the same defect `FlowProgramFeature::step` had one layer down: a turn granted
/// 4 096 bytes of credit advanced the retirement by one rung, so the turn count was linear in the
/// payload — a 24-row document cost 2 626 turns and a 64-row one 5 718, against a declared bound of
/// 4 096 (`🧫️fixtures/🧹️session-close/🔣️.json` `close.maximumTurns`). Any document over ~20 KB could
/// not be closed inside its own contract (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
/// `📓️flow-surface-followup-2026-09-15.md` §2.4).
///
/// Nothing crosses the ABI while a session retires, so the bytes under a retained item are not a
/// turn's currency: its structure is. The three documents here carry the SAME eight rows at 1 KB,
/// 32 KB and 256 KB, so a ladder that moves per surface answers the same count for all three.
#[test]
fn a_retained_document_retires_in_turns_that_do_not_count_its_bytes() {
    let sizes = [1_024usize, 32_768, 262_144];
    let mut census = Vec::new();
    for size in sizes {
        let json = scaled_document_json(size);
        let turns = close_ladder_turns(&json);
        println!("[DEBUG] flow close ladder: document {} B ({} B of json) retired in {turns} turns", size, json.len());
        census.push((json.len(), turns));
    }
    let bound = serde_json::from_str::<Value>(include_str!("../../🧫️fixtures/🧹️session-close/🔣️.json")).unwrap()["close"]["maximumTurns"].as_u64().unwrap() as usize;
    let smallest = census.first().expect("a measured document").1;
    let largest = census.last().expect("a measured document").1;
    assert!(census.last().unwrap().0 > census.first().unwrap().0 * 64, "the payloads must differ by two orders of magnitude to measure a per-byte ladder: {census:?}");
    assert_eq!(smallest, largest, "the close turn count must not read the payload: {census:?}");
    assert!(largest * 32 < bound, "a retained document must retire with margin inside its declared bound of {bound}: {census:?}");
}


/// 🧾️ Drives ONE retained domain's close ladder to terminal and answers, per close turn, the phase
/// the turn anchored on — the ladder printed rung by rung.
fn close_ladder_census(document_json: &str) -> Vec<(String, usize)> {
    let mut domain = FlowDomainAdapter::default();
    domain.bind_session(semio_framework::abi::AbiHandle::try_new(1, 1).unwrap());
    run(&mut domain, 2_610, text_payload(document_json)).unwrap();
    domain.begin_close();
    let mut census: Vec<(String, usize)> = Vec::new();
    for turn in 0..8_000_000usize {
        let phase = format!("{:?}", domain.close_phase());
        match census.last_mut() {
            Some((held, count)) if *held == phase => *count += 1,
            _ => census.push((phase, 1)),
        }
        if domain.close_step(bridge_budget()).unwrap() {
            assert!(domain.terminal_is_empty(), "a complete close must be terminal at turn {turn}");
            return census;
        }
    }
    panic!("retained Flow domain never closed")
}

/// 🪜️ Prints the retirement ladder of a 32 KB retained document, turn by turn.
#[test]
fn the_retirement_ladder_names_every_turn_it_spends() {
    let census = close_ladder_census(&scaled_document_json(32_768));
    let turns: usize = census.iter().map(|(_, count)| count).sum();
    println!("[DEBUG] flow close ladder census (32 KB document): {turns} turns");
    for (phase, count) in &census {
        println!("[DEBUG]   {phase} x{count}");
    }
    for rows in [24u32, 64] {
        println!("[DEBUG] flow close ladder: the {rows}-row measured document retired in {} turns", close_ladder_turns(&row_document_json(rows)));
    }
    assert!(turns < 64, "a 32 KB document must retire in turns worth naming, not thousands: {census:?}");
    assert!(!census.iter().any(|(phase, _)| phase.contains("Backing") || phase.contains("Domain") || phase.contains("Neural")), "a payload frontier must never anchor a close turn: {census:?}");
}

//#endregion 🪜️RetirementLadder
