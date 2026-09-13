
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
}

#[test]
fn malformed_fixed_width_is_rejected_before_surface_mutation() {
    let mut domain = FlowDomainAdapter::default();
    let mut payload = surface_payload(u32::MAX, 1, 1, 1, 1.0);
    payload.push(0);
    let rejected = run(&mut domain, 2_575, payload).unwrap_err();
    assert_eq!(rejected.code, AbiErrorCode::MalformedLength);
    assert!(domain.surface.is_none());
}

#[test]
fn every_schema_feature_has_a_distinct_action_binding() {
    let expected: Vec<u16> = (2_504..=2_610).filter(|operation| !matches!(operation, 2_603 | 2_604 | 2_608)).collect();
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
    let mut expected = crate::artifact::FlowFixture::default();
    expected.schema = "flow.fixture.synchronized".into();
    let json = crate::os_pack::json::to_json_string(&expected);
    run(&mut domain, 2_610, text_payload(&json)).unwrap();
    let bytes = run(&mut domain, 2_609, Vec::new()).unwrap();
    let value = crate::os_pack::json::parse(std::str::from_utf8(&bytes).unwrap()).unwrap();
    let actual = <crate::artifact::FlowFixture as crate::os_dsl::FromValue>::from_value(crate::os_pack::json::to_dsl_value(&value)).unwrap();
    assert_eq!(actual, expected);
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
    let mut feature = FlowDomainAdapter::start_feature(domain, admission, 2_501, writer.finish()).unwrap();
    assert!(matches!(feature.step(AbiWorkBudget::credits(64)), FlowFeatureStep::Progress { completed: 0, total: 3 }));
    feature.cancel(AbiWorkBudget::credits(64)).unwrap();
    assert!(!feature.close_step(AbiWorkBudget::credits(64)).unwrap());
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
}
