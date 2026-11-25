//! 🖼️ A canvas attached through the real bridge answers its scene sync, and a refused DAG input retires its facts instead of trapping the guest.

use semio_framework_abi::{AbiBytes, AbiControl, AbiHandle, AbiMessage, AbiOperation, AbiPort, AbiPortPoll, AbiReply, AbiRequest, AbiRequestId, AbiStatus, AbiStatusCode, AbiWorkBudget};
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_os_flow_wasm::FlowBridge;
use semio_os_flow_wasm_domain::protocol::{FlowPayloadReader, FlowPayloadWriter, FLOW_EVENT_ADMITTED, FLOW_OPERATION_OPEN};
use semio_os_flow_wasm_domain::FlowDomainAdapter;

fn budget_with_copy(copy: usize) -> AbiWorkBudget {
    AbiWorkBudget { byte_credit: 4_096, retained: RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: u32::MAX as usize, maximum_release_bytes: u32::MAX as usize, maximum_depth: 4096 }, now_ms: 0, deadline_ms: None, cancelled: false, interrupted: false }
}

fn budget() -> AbiWorkBudget {
    budget_with_copy(semio_os_flow_wasm_domain::protocol::FLOW_MAX_REQUEST_BYTES)
}

fn request(operation: u16, id: u64, bytes: Vec<u8>) -> AbiMessage {
    AbiMessage::Request(AbiRequest { operation: AbiOperation::try_new(operation).unwrap(), request_id: AbiRequestId(id), generation: 1, bytes: AbiBytes::try_new(bytes).unwrap() })
}

fn poll_one(bridge: &mut FlowBridge<FlowDomainAdapter>, budget: AbiWorkBudget) -> AbiMessage {
    for turn in 0..8_000_000 {
        match bridge.poll(budget) {
            Ok(AbiPortPoll::Message(message)) => return message,
            _ if turn < 64 => {},
            Err(code) => panic!("Flow bridge refused its poll with {code:?}"),
            Ok(_) => {}
        }
    }
    panic!("Flow bridge did not progress")
}

fn acknowledge(bridge: &mut FlowBridge<FlowDomainAdapter>, event: &semio_framework_abi::AbiEvent, budget: AbiWorkBudget) {
    bridge.try_send(AbiMessage::Reply(AbiReply { request_id: event.request_id, generation: event.generation, status: AbiStatus::OK, bytes: AbiBytes::default() }), budget).unwrap();
}

fn drain(bridge: &mut FlowBridge<FlowDomainAdapter>, id: u64, budget: AbiWorkBudget) -> AbiStatus {
    let mut operation = None;
    loop {
        match poll_one(bridge, budget) {
            AbiMessage::Event(event) => {
                if event.event.get() == FLOW_EVENT_ADMITTED {
                    operation = Some(FlowPayloadReader::new(event.bytes.as_slice()).handle().unwrap());
                }
                acknowledge(bridge, &event, budget);
            }
            AbiMessage::Page(page) => bridge.try_send(AbiMessage::Control(AbiControl::Acknowledge { handle: operation.expect("admitted operation"), index: page.index }), budget).unwrap(),
            AbiMessage::Reply(reply) if reply.request_id.0 == id => return reply.status,
            _ => {}
        }
    }
}

fn close(bridge: &mut FlowBridge<FlowDomainAdapter>, budget: AbiWorkBudget) {
    bridge.begin_close();
    for _ in 0..4_096 {
        match bridge.poll(budget).unwrap() {
            AbiPortPoll::Message(AbiMessage::Event(event)) => acknowledge(bridge, &event, budget),
            AbiPortPoll::Closed => {
                assert!(bridge.terminal_is_empty());
                return;
            }
            _ => {}
        }
    }
    panic!("Flow bridge did not close within its bound");
}

struct Host {
    bridge: FlowBridge<FlowDomainAdapter>,
    session: AbiHandle,
    next: u64,
    budget: AbiWorkBudget,
}

impl Host {
    fn open() -> Self {
        Self::open_with(budget())
    }

    fn open_with(budget: AbiWorkBudget) -> Self {
        let mut bridge = FlowBridge::new(FlowDomainAdapter::default);
        bridge.try_send(request(FLOW_OPERATION_OPEN, 1, Vec::new()), budget).unwrap();
        let AbiMessage::Reply(opened) = poll_one(&mut bridge, budget) else { panic!("Flow session reply") };
        let session = FlowPayloadReader::new(opened.bytes.as_slice()).handle().unwrap();
        Self { bridge, session, next: 1, budget }
    }

    fn send(&mut self, operation: u16, fields: impl FnOnce(&mut FlowPayloadWriter)) -> AbiStatus {
        let mut writer = FlowPayloadWriter::default();
        writer.handle(self.session);
        fields(&mut writer);
        self.next += 1;
        self.bridge.try_send(request(operation, self.next, writer.finish()), self.budget).unwrap();
        drain(&mut self.bridge, self.next, self.budget)
    }

    fn json(&mut self, operation: u16, json: &str) -> AbiStatus {
        self.send(operation, |writer| writer.bytes(json.as_bytes()).unwrap())
    }

    fn attach_canvas(&mut self) {
        let attached = self.send(2_575, |writer| {
            writer.u32(1);
            writer.u32(1);
            writer.u32(800);
            writer.u32(600);
            writer.f64(2.0);
        });
        assert_eq!(attached, AbiStatus::OK);
        let created = self.send(2_576, |writer| {
            writer.u32(1);
            writer.u32(1);
            writer.bytes(b"created").unwrap();
        });
        assert_eq!(created, AbiStatus::OK);
    }
}

const SCENE_SYNC_INPUTS: [(u16, &str); 4] = [
    (2_525, r#"{"nodes":["profile"],"edges":[],"handles":[]}"#),
    (2_514, r#"{"active":"extrusion-axis","stale":["profile","extrude"]}"#),
    (2_515, r#"{"profile":{"status":"queued"},"extrusion-axis":{"status":"computing"},"extrude":{"status":"blocked","ports":["profile"]}}"#),
    (2_528, r#"[{"widgetId":"profile","port":"out","direction":"out"}]"#),
];

/// 🖼️ Every DAG input the host sends right after attaching a canvas reaches its answer.
#[test]
fn an_attached_canvas_answers_the_scene_sync_inputs_through_the_bridge() {
    let mut host = Host::open();
    host.attach_canvas();
    for (operation, json) in SCENE_SYNC_INPUTS {
        assert_eq!(host.json(operation, json), AbiStatus::OK, "operation {operation}");
    }
    close(&mut host.bridge, host.budget);
}

/// 🧯️ A DAG input refused after its facts exist answers its own refusal, retires the facts and leaves the session usable.
#[test]
fn a_refused_dag_input_retires_its_facts_and_reports_its_refusal() {
    let refused: [(u16, &str); 4] = [
        (2_525, r#"{"nodes":["profile"],"edges":[7],"handles":[]}"#),
        (2_514, r#"{"active":null,"stale":["profile",7]}"#),
        (2_515, r#"{"profile":{"status":"queued"},"extrude":{"status":"bogus"}}"#),
        (2_528, r#"[{"widgetId":"profile","port":"out","direction":"out"},{"widgetId":"extrude","port":"in","direction":"sideways"}]"#),
    ];
    for (operation, json) in refused {
        let mut host = Host::open();
        let status = host.json(operation, json);
        assert_eq!(status.code, AbiStatusCode::Failed, "operation {operation}");
        assert!(status.error.is_some(), "operation {operation} must report its refusal");
        let valid = SCENE_SYNC_INPUTS.iter().find(|input| input.0 == operation).unwrap().1;
        assert_eq!(host.json(operation, valid), AbiStatus::OK, "operation {operation} after its refusal");
        close(&mut host.bridge, host.budget);
    }
}

/// 🧯️ A grant no poll can ever meet for the displaced custody ends in a named refusal after the turn limit, retires its facts, and never traps or spins.
#[test]
fn an_unmeetable_displaced_custody_quote_is_refused_by_name_and_retires_its_facts() {
    for (operation, json) in SCENE_SYNC_INPUTS.into_iter().filter(|input| input.0 == 2_525) {
        let mut host = Host::open_with(budget_with_copy(8));
        let mut payload = FlowPayloadWriter::default();
        payload.handle(host.session);
        payload.bytes(json.as_bytes()).unwrap();
        let request_id = host.next + 1;
        host.bridge.try_send(request(operation, request_id, payload.finish()), host.budget).unwrap();
        let mut refused = false;
        for _ in 0..20_000 {
            match host.bridge.poll(host.budget) {
                Err(semio_framework_abi::AbiErrorCode::NoCredit) => { refused = true; break; }
                Ok(AbiPortPoll::Message(AbiMessage::Event(event))) => acknowledge(&mut host.bridge, &event, host.budget),
                Ok(AbiPortPoll::Pending) => {},
                result => panic!("unexpected unfunded poll {result:?}"),
            }
        }
        assert!(refused, "retiring the operation must report its insufficient poll grant");
        host.budget = budget();
        let status = drain(&mut host.bridge, request_id, host.budget);
        assert_eq!(status.code, AbiStatusCode::Failed, "operation {operation}");
        let message = status.error.as_ref().map(|error| format!("{:?}", error.message)).unwrap_or_default();
        assert!(status.error.is_some_and(|error| error.code == semio_framework_abi::AbiErrorCode::NoCredit), "operation {operation} must name its refusal: {message}");
        close(&mut host.bridge, host.budget);
    }
}

/// 📏️ Every original DAG input owner is born into its retirement frame within the live copy grant, so no unit is inadmissible.
#[test]
fn every_dag_input_owner_birth_fits_the_live_copy_grant() {
    use semio_framework_value::retirement::controlled::ControlledRetirement;
    use semio_os_infinite::board::io::text::dag_input::{output::DagOutputPreparation, retained::DagInputCursor};
    use semio_os_infinite::board::ports::directed_dag::input_application::{DagInputApplication, DagInputDisplaced};
    fn birth<T: semio_framework_value::retirement::RetireOwned>() -> usize {
        size_of::<Option<T>>() + size_of::<ControlledRetirement<T>>()
    }
    let births = [
        ("application", birth::<DagInputApplication>()),
        ("input", birth::<DagInputCursor>()),
        ("displaced", birth::<DagInputDisplaced>()),
        ("output", birth::<semio_framework_pack_json::JsonBorrowedWriteCursor>()),
        ("preparation", birth::<DagOutputPreparation>()),
        ("payload", birth::<Vec<u8>>()),
    ];
    for (owner, bytes) in births {
        let limit = budget().retained.maximum_copy_bytes;
        assert!(bytes <= limit, "{owner} birth copies {bytes} bytes into its retirement frame but the live grant is {limit}: {births:?}");
    }
}
