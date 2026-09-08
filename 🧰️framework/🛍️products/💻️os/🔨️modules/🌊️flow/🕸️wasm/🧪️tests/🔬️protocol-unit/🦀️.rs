
use super::*;
use semio_framework::abi::AbiOperation;

#[derive(Default)]
struct MockDomain {
    close_calls: usize,
    close_steps: usize,
    unproven: bool,
}

struct MockFeature {
    payload: Option<Vec<u8>>,
    phase: u8,
    cancelled: bool,
    close_failed: bool,
}

impl FlowFeature for MockFeature {
    fn step(&mut self, _: AbiWorkBudget) -> FlowFeatureStep {
        if self.cancelled {
            return FlowFeatureStep::Failed(FlowFailure::new(AbiErrorCode::Cancelled, "cancelled"));
        }
        self.phase += 1;
        if self.phase == 1 && self.payload.as_deref() == Some(&[7]) {
            return FlowFeatureStep::RetainedPage(vec![7; 129]);
        }
        match self.phase {
            1 => FlowFeatureStep::Progress { completed: 1, total: 1 },
            2 => FlowFeatureStep::Checkpoint(vec![1]),
            3 => FlowFeatureStep::Preview(vec![2]),
            _ => FlowFeatureStep::Complete(self.payload.take().unwrap_or_default()),
        }
    }

    fn cancel(&mut self, _: AbiWorkBudget) -> Result<(), FlowFailure> {
        self.cancelled = true;
        Ok(())
    }

    fn close_step(&mut self, _: AbiWorkBudget) -> Result<bool, FlowFailure> {
        if self.payload.as_deref() == Some(&[8]) && !self.close_failed {
            self.close_failed = true;
            return Err(FlowFailure::new(AbiErrorCode::Busy, "retained close retry"));
        }
        Ok(true)
    }
}

impl FlowDomain for MockDomain {
    fn start_feature(_: Rc<RefCell<Self>>, _: FlowFeatureAdmission, _: u16, payload: Vec<u8>) -> Result<Box<dyn FlowFeature>, FlowFailure> {
        Ok(Box::new(MockFeature { payload: Some(payload), phase: 0, cancelled: false, close_failed: false }))
    }

    fn begin_close(&mut self) {
        self.close_calls += 1;
    }

    fn close_step(&mut self, _: AbiWorkBudget) -> Result<bool, FlowFailure> {
        self.close_steps += 1;
        Ok(self.close_calls == 1 && self.close_steps >= 3)
    }

    fn terminal_is_empty(&self) -> bool {
        self.close_calls == 1 && self.close_steps >= 3 && !self.unproven
    }
}

fn budget() -> AbiWorkBudget {
    AbiWorkBudget::credits(64)
}

fn request(operation: u16, request_id: u64, generation: u32, bytes: Vec<u8>) -> AbiMessage {
    AbiMessage::Request(AbiRequest { operation: AbiOperation::try_new(operation).unwrap(), request_id: AbiRequestId(request_id), generation, bytes: AbiBytes::try_new(bytes).unwrap() })
}

fn poll_message(bridge: &mut FlowBridge<MockDomain>) -> AbiMessage {
    for _ in 0..512 {
        if let AbiPortPoll::Message(message) = bridge.poll(budget()).unwrap() {
            return message;
        }
    }
    panic!("Flow message did not arrive")
}

fn acknowledge_event(bridge: &mut FlowBridge<MockDomain>, event: &AbiEvent) {
    bridge.try_send(AbiMessage::Reply(success_reply(event.request_id, event.generation, Vec::new())), budget()).unwrap();
}

fn open(bridge: &mut FlowBridge<MockDomain>, request_id: u64, generation: u32) -> AbiHandle {
    bridge.try_send(request(FLOW_OPERATION_OPEN, request_id, generation, Vec::new()), budget()).unwrap();
    let AbiMessage::Reply(reply) = poll_message(bridge) else { panic!("open reply") };
    let mut reader = FlowPayloadReader::new(reply.bytes.as_slice());
    reader.handle().unwrap()
}

#[test]
fn schema_and_fixtures_publish_every_operation_and_fixed_law() {
    assert!(FLOW_ABI_SCHEMA.contains("\"open\": 2500"));
    assert!(FLOW_ABI_SCHEMA.contains("\"renderDrawingScene\": 2600"));
    assert!(FLOW_ABI_SCHEMA.contains("\"documentJson\": 2609"));
    assert!(FLOW_ABI_SCHEMA.contains("\"synchronizeDocumentJson\": 2610"));
    for operation in (FLOW_OPERATION_FIRST..=FLOW_OPERATION_LAST).filter(|operation| !matches!(operation, 2_603 | 2_604 | 2_608)) {
        assert!(FLOW_ABI_SCHEMA.contains(&format!(": {operation}")));
    }
    assert!(FLOW_ABI_LIMITS.contains("request_bytes\t1048576\t1048577"));
    assert!(FLOW_ABI_TRACE.contains("rejected_controls"));
    assert!(FLOW_ABI_LEDGER.contains("selection_unknown_optional"));
    assert!(FLOW_VCS_PRODUCTION_LEDGER.contains("checkpoint\t2501\t1\t1\t1"));
    assert!(FLOW_VCS_PRODUCTION_LEDGER.contains("fault\t2502\t1\t1\t2"));
    assert!(FLOW_VCS_PRODUCTION_LEDGER.contains("cancel\t2501\t1\t1\t4"));
}

#[test]
fn every_request_is_a_cancellable_progress_checkpoint_preview_feature() {
    let mut bridge = FlowBridge::new(MockDomain::default);
    let session = open(&mut bridge, 1, 1);
    let mut body = FlowPayloadWriter::default();
    body.handle(session);
    body.bytes(b"flow").unwrap();
    bridge.try_send(request(2_501, 2, 1, body.finish()), budget()).unwrap();
    let AbiMessage::Event(admitted) = poll_message(&mut bridge) else { panic!("admitted") };
    assert_eq!(admitted.event.get(), FLOW_EVENT_ADMITTED);
    acknowledge_event(&mut bridge, &admitted);
    for expected in [FLOW_EVENT_PROGRESS, FLOW_EVENT_CHECKPOINT, FLOW_EVENT_PREVIEW] {
        let AbiMessage::Event(event) = poll_message(&mut bridge) else { panic!("feature event") };
        assert_eq!(event.event.get(), expected);
        acknowledge_event(&mut bridge, &event);
    }
    let AbiMessage::Event(terminal) = poll_message(&mut bridge) else { panic!("terminal") };
    assert_eq!(terminal.event.get(), FLOW_EVENT_TERMINAL);
    acknowledge_event(&mut bridge, &terminal);
    assert!(matches!(poll_message(&mut bridge), AbiMessage::Reply(_)));
}

#[test]
fn paged_terminal_is_lossless_and_requires_exact_ack() {
    let mut bridge = FlowBridge::new(MockDomain::default);
    let session = open(&mut bridge, 1, 1);
    let mut body = FlowPayloadWriter::default();
    body.handle(session);
    body.bytes(&vec![7; FLOW_MAX_INLINE_REPLY_BYTES + 1]).unwrap();
    bridge.try_send(request(2_503, 2, 1, body.finish()), budget()).unwrap();
    let mut operation = None;
    loop {
        let AbiMessage::Event(event) = poll_message(&mut bridge) else { panic!("feature event") };
        if event.event.get() == FLOW_EVENT_ADMITTED {
            operation = Some(FlowPayloadReader::new(event.bytes.as_slice()).handle().unwrap());
        }
        let complete = event.event.get() == FLOW_EVENT_OUTPUT;
        acknowledge_event(&mut bridge, &event);
        if complete {
            break;
        }
    }
    let operation = operation.unwrap();
    let AbiMessage::Page(page) = poll_message(&mut bridge) else { panic!("page") };
    assert_eq!(page.handle, operation);
    assert!(bridge.try_send(AbiMessage::Control(AbiControl::Acknowledge { handle: operation, index: page.index + 1 }), budget()).is_err());
    bridge.try_send(AbiMessage::Control(AbiControl::Acknowledge { handle: operation, index: page.index }), budget()).unwrap();
    assert!(bridge.try_send(AbiMessage::Control(AbiControl::Acknowledge { handle: operation, index: page.index }), budget()).is_err());
    let AbiMessage::Event(terminal) = poll_message(&mut bridge) else { panic!("terminal") };
    assert_eq!(terminal.event.get(), FLOW_EVENT_TERMINAL);
    acknowledge_event(&mut bridge, &terminal);
    assert!(matches!(poll_message(&mut bridge), AbiMessage::Reply(_)));
}

#[test]
fn stale_duplicate_controls_and_idempotent_close_do_not_leak() {
    let mut bridge = FlowBridge::new(MockDomain::default);
    let session = open(&mut bridge, 1, 1);
    assert!(bridge.try_send(AbiMessage::Control(AbiControl::Close { handle: AbiHandle::try_new(session.slot(), session.generation().saturating_add(1)).unwrap() }), budget()).is_err());
    bridge.try_send(AbiMessage::Control(AbiControl::Close { handle: session }), budget()).unwrap();
    assert!(bridge.try_send(AbiMessage::Control(AbiControl::Close { handle: session }), budget()).is_err());
    bridge.begin_close();
    bridge.begin_close();
    for _ in 0..3 {
        if let AbiPortPoll::Message(AbiMessage::Event(event)) = bridge.poll(budget()).unwrap() {
            acknowledge_event(&mut bridge, &event);
        }
    }
    assert!(bridge.terminal_is_empty());
}

#[test]
fn session_close_retains_domain_until_child_and_exact_terminal_owners_retire() {
    let text = include_str!("../../🧫️fixtures/🧹️session-close/🔣️.json");
    let fixture = crate::os_pack::json::parse(text).unwrap();
    let independent: serde_json::Value = serde_json::from_str(text).unwrap();
    let close = fixture.get("close").unwrap();
    let steps = close.get("domainSteps").unwrap().as_u64().unwrap() as usize;
    assert_eq!(steps as u64, independent["close"]["domainSteps"].as_u64().unwrap());
    for credit in close.get("byteCredits").unwrap().as_array().unwrap() {
        let credit = credit.as_u64().unwrap() as usize;
        let mut bridge = FlowBridge::new(MockDomain::default);
        let session = open(&mut bridge, 1, 1);
        let FlowResource::Session(owner) = bridge.resources.get(session).unwrap() else { panic!("session owner") };
        let domain = owner.borrow().domain.clone();
        let mut payload = FlowPayloadWriter::default();
        payload.handle(session);
        bridge.try_send(request(2_501, 2, 1, payload.finish()), budget()).unwrap();
        let AbiMessage::Event(admitted) = poll_message(&mut bridge) else { panic!("admission") };
        acknowledge_event(&mut bridge, &admitted);
        bridge.try_send(AbiMessage::Control(AbiControl::Close { handle: session }), budget()).unwrap();
        assert_eq!(bridge.active_resources, 2);
        assert_eq!(domain.borrow().close_calls, 0);
        assert_eq!(bridge.try_send(AbiMessage::Control(AbiControl::Close { handle: session }), budget()).unwrap_err().code, AbiErrorCode::Closed);
        let mut payload = FlowPayloadWriter::default();
        payload.handle(session);
        assert_eq!(bridge.try_send(request(2_501, 3, 1, payload.finish()), budget()).unwrap_err().code, AbiErrorCode::Closed);
        let mut ordering = vec!["session-closed"];
        bridge.begin_close();
        bridge.begin_close();
        let AbiMessage::Event(terminal) = poll_message(&mut bridge) else { panic!("terminal") };
        assert_eq!(terminal.event.get(), FLOW_EVENT_TERMINAL);
        assert_eq!(bridge.active_resources, 1);
        assert_eq!(domain.borrow().close_calls, 0);
        ordering.push("operation-retired");
        assert!(matches!(poll_message(&mut bridge), AbiMessage::Reply(_)));
        for kind in fixture.get("rejectedPolls").unwrap().as_array().unwrap() {
            let mut rejected = AbiWorkBudget::credits(credit);
            match kind.as_str().unwrap() {
                "zero-credit" => rejected.byte_credit = 0,
                "interrupted" => rejected.interrupted = true,
                "expired" => {
                    rejected.now_ms = 8;
                    rejected.deadline_ms = Some(8);
                }
                _ => panic!("unknown close budget"),
            }
            assert!(bridge.poll(rejected).is_err());
            assert_eq!(bridge.active_resources, 1);
            assert_eq!(domain.borrow().close_calls, 0);
        }
        let mut session_receipt = None;
        for step in 0..steps {
            match bridge.poll(AbiWorkBudget::credits(credit)).unwrap() {
                AbiPortPoll::Pending if step + 1 < steps => {}
                AbiPortPoll::Message(AbiMessage::Event(event)) if step + 1 == steps => {
                    assert_eq!(event.event.get(), FLOW_EVENT_SESSION_TERMINAL);
                    session_receipt = Some(event);
                }
                _ => panic!("session receipt must follow domain terminal"),
            }
            assert_eq!(domain.borrow().close_calls, 1);
            if step == 0 {
                ordering.push("domain-close-started");
            }
            if step + 1 < steps {
                assert_eq!(bridge.active_resources, 1);
            }
        }
        assert!(domain.borrow().terminal_is_empty());
        ordering.push("domain-retired");
        assert_eq!(bridge.active_resources, 0);
        ordering.push("session-released");
        assert_eq!(serde_json::to_value(&ordering).unwrap(), independent["ordering"]);
        assert!(!bridge.terminal_is_empty());
        acknowledge_event(&mut bridge, &terminal);
        assert!(!bridge.terminal_is_empty());
        acknowledge_event(&mut bridge, &session_receipt.unwrap());
        assert!(bridge.terminal_is_empty());
        assert!(bridge.resources.get(session).is_err());
    }
}

#[test]
fn session_close_never_releases_an_unproven_terminal_domain() {
    let mut bridge = FlowBridge::new(MockDomain::default);
    let session = open(&mut bridge, 1, 1);
    let FlowResource::Session(owner) = bridge.resources.get(session).unwrap() else { panic!("session owner") };
    let domain = owner.borrow().domain.clone();
    domain.borrow_mut().unproven = true;
    bridge.begin_close();
    for _ in 0..2 {
        bridge.poll(budget()).unwrap();
    }
    assert_eq!(bridge.poll(budget()).unwrap_err(), AbiErrorCode::Busy);
    assert_eq!(bridge.active_resources, 1);
    assert_eq!(domain.borrow().close_calls, 1);
    assert!(!bridge.terminal_is_empty());
    domain.borrow_mut().unproven = false;
    let AbiPortPoll::Message(AbiMessage::Event(receipt)) = bridge.poll(budget()).unwrap() else { panic!("terminal session receipt") };
    assert_eq!(receipt.event.get(), FLOW_EVENT_SESSION_TERMINAL);
    assert_eq!(bridge.active_resources, 0);
    assert!(!bridge.terminal_is_empty());
    acknowledge_event(&mut bridge, &receipt);
    assert!(bridge.terminal_is_empty());
}

#[test]
fn session_close_requeues_the_exact_operation_after_a_retained_feature_fault() {
    let mut bridge = FlowBridge::new(MockDomain::default);
    let session = open(&mut bridge, 1, 1);
    let mut payload = FlowPayloadWriter::default();
    payload.handle(session);
    payload.u8(8);
    bridge.try_send(request(2_501, 2, 1, payload.finish()), budget()).unwrap();
    let AbiMessage::Event(admitted) = poll_message(&mut bridge) else { panic!("admission") };
    acknowledge_event(&mut bridge, &admitted);
    let operation = FlowPayloadReader::new(admitted.bytes.as_slice()).handle().unwrap();
    bridge.begin_close();
    assert_eq!(bridge.poll(budget()).unwrap_err(), AbiErrorCode::Busy);
    assert!(bridge.work.contains(&operation));
    assert_eq!(bridge.active_resources, 2);
    assert!(!bridge.terminal_is_empty());
    for _ in 0..16 {
        if let AbiPortPoll::Message(AbiMessage::Event(event)) = bridge.poll(budget()).unwrap() {
            acknowledge_event(&mut bridge, &event);
        }
        if bridge.terminal_is_empty() {
            break;
        }
    }
    assert!(bridge.terminal_is_empty());
    assert_eq!(bridge.active_resources, 0);
}

#[test]
fn session_close_cancels_an_unacknowledged_page_and_retires_its_exact_bytes() {
    let mut bridge = FlowBridge::new(MockDomain::default);
    let session = open(&mut bridge, 1, 1);
    let mut payload = FlowPayloadWriter::default();
    payload.handle(session);
    payload.u8(7);
    bridge.try_send(request(2_501, 2, 1, payload.finish()), budget()).unwrap();
    for expected in [FLOW_EVENT_ADMITTED, FLOW_EVENT_OUTPUT] {
        let AbiMessage::Event(event) = poll_message(&mut bridge) else { panic!("page event") };
        assert_eq!(event.event.get(), expected);
        acknowledge_event(&mut bridge, &event);
    }
    let AbiMessage::Page(page) = poll_message(&mut bridge) else { panic!("retained page") };
    assert_eq!(page.bytes.len(), 129);
    bridge.try_send(AbiMessage::Control(AbiControl::Close { handle: session }), budget()).unwrap();
    bridge.begin_close();
    let mut turns = 0;
    while !bridge.terminal_is_empty() && turns < 4096 {
        turns += 1;
        match bridge.poll(AbiWorkBudget::credits(1)).unwrap() {
            AbiPortPoll::Message(AbiMessage::Event(event)) => acknowledge_event(&mut bridge, &event),
            AbiPortPoll::Message(AbiMessage::Page(_)) => panic!("cancelled page was republished"),
            _ => {}
        }
    }
    assert!(bridge.terminal_is_empty());
    assert!(turns >= page.bytes.len());
    assert_eq!(bridge.active_resources, 0);
}

#[test]
fn session_close_receipt_retries_a_colliding_event_slot_and_preserves_sibling() {
    let text = include_str!("../../🧫️fixtures/🧑‍🤝‍🧑️browser-runtime/🔣️.json");
    let fixture = crate::os_pack::json::parse(text).unwrap();
    let independent: serde_json::Value = serde_json::from_str(text).unwrap();
    let code = fixture.get("receipt").unwrap().get("event").unwrap().as_u64().unwrap() as u16;
    assert_eq!(u64::from(code), independent["receipt"]["event"].as_u64().unwrap());
    let mut bridge = FlowBridge::new(MockDomain::default);
    let session = open(&mut bridge, 1, 1);
    let sibling = open(&mut bridge, 2, 1);
    let FlowResource::Session(owner) = bridge.resources.get(session).unwrap() else { panic!("session") };
    let domain = owner.borrow().domain.clone();
    bridge.push_event(AbiRequestId(FLOW_MAX_REQUESTS as u64 + 1), 1, FLOW_EVENT_PROGRESS, AbiStatus::OK, Vec::new(), false).unwrap();
    let AbiMessage::Event(collision) = poll_message(&mut bridge) else { panic!("collision") };
    bridge.try_send(AbiMessage::Control(AbiControl::Close { handle: session }), budget()).unwrap();
    for _ in 0..4 {
        assert!(matches!(bridge.poll(budget()).unwrap(), AbiPortPoll::Pending));
    }
    assert!(domain.borrow().terminal_is_empty());
    assert_eq!(domain.borrow().close_calls, 1);
    assert_eq!(bridge.active_resources, 2);
    acknowledge_event(&mut bridge, &collision);
    let AbiMessage::Event(receipt) = poll_message(&mut bridge) else { panic!("session receipt") };
    assert_eq!(receipt.event.get(), code);
    assert_eq!(receipt.request_id.0 ^ (u64::from(receipt.sequence) << 32), 1);
    assert_eq!(FlowPayloadReader::new(receipt.bytes.as_slice()).handle().unwrap(), session);
    assert_eq!(bridge.active_resources, 1);
    assert!(bridge.resources.get(session).is_err());
    assert!(bridge.resources.get(sibling).is_ok());
    let mut wrong = success_reply(receipt.request_id, receipt.generation + 1, Vec::new());
    assert!(bridge.try_send(AbiMessage::Reply(wrong.clone()), budget()).is_err());
    wrong.generation = receipt.generation;
    bridge.try_send(AbiMessage::Reply(wrong), budget()).unwrap();
    let mut payload = FlowPayloadWriter::default();
    payload.handle(sibling);
    bridge.try_send(request(2_501, 3, 1, payload.finish()), budget()).unwrap();
    bridge.begin_close();
    for _ in 0..64 {
        if let AbiPortPoll::Message(AbiMessage::Event(event)) = bridge.poll(budget()).unwrap() {
            acknowledge_event(&mut bridge, &event);
        }
        if bridge.terminal_is_empty() {
            break;
        }
    }
    assert!(bridge.terminal_is_empty());
    println!("[DEBUG] Flow session receipt retained a terminal domain across event-slot backpressure and preserved its live sibling");
}

#[test]
fn zero_credit_deadline_interruption_and_max_plus_one_reject_before_admission() {
    let mut bridge = FlowBridge::new(MockDomain::default);
    let zero = AbiWorkBudget::credits(0);
    assert!(bridge.try_send(request(FLOW_OPERATION_OPEN, 1, 1, Vec::new()), zero).is_err());
    let mut deadline = budget();
    deadline.now_ms = 8;
    deadline.deadline_ms = Some(8);
    assert!(bridge.try_send(request(FLOW_OPERATION_OPEN, 1, 1, Vec::new()), deadline).is_err());
    let mut interrupted = budget();
    interrupted.interrupted = true;
    assert!(bridge.try_send(request(FLOW_OPERATION_OPEN, 1, 1, Vec::new()), interrupted).is_err());
    assert!(AbiBytes::try_new(vec![0; FLOW_MAX_REQUEST_BYTES + 1]).is_err());
}
