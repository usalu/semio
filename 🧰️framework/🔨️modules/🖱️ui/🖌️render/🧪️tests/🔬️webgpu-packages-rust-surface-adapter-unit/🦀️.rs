
use super::*;
use crate::abi::{AbiOperation, AbiPageBytes, AbiPortRejection};
use std::collections::VecDeque;

#[derive(Default)]
struct MockPort {
    incoming: VecDeque<AbiMessage>,
    sent: Vec<AbiMessage>,
    reject_send: bool,
    closed: bool,
}

impl AbiPort for MockPort {
    fn try_send(&mut self, message: AbiMessage, _budget: AbiWorkBudget) -> Result<(), AbiPortRejection> {
        if self.reject_send {
            self.reject_send = false;
            Err(AbiPortRejection { code: AbiErrorCode::Interrupted, message })
        } else {
            self.sent.push(message);
            Ok(())
        }
    }
    fn poll(&mut self, _budget: AbiWorkBudget) -> Result<AbiPortPoll, AbiErrorCode> {
        Ok(self.incoming.pop_front().map(AbiPortPoll::Message).unwrap_or(if self.closed { AbiPortPoll::Closed } else { AbiPortPoll::Pending }))
    }
}

fn request(operation: u16, id: u64, generation: u32, body: Vec<u8>) -> AbiMessage {
    AbiMessage::Request(AbiRequest { operation: AbiOperation::try_new(operation).unwrap(), request_id: AbiRequestId(id), generation, bytes: AbiBytes::try_new(body).unwrap() })
}

fn create_body(status: u8, surface: u32, canvas: u32, generation: u32, width: u32, height: u32) -> Vec<u8> {
    let mut body = vec![1, status];
    for value in [surface, canvas, generation, width, height, 1.0f32.to_bits()] {
        body.extend_from_slice(&value.to_le_bytes());
    }
    body
}

fn identity_body(surface: u32, generation: u32) -> Vec<u8> {
    let mut body = vec![1];
    body.extend_from_slice(&surface.to_le_bytes());
    body.extend_from_slice(&generation.to_le_bytes());
    body
}

fn resize_body(surface: u32, generation: u32, width: u32, height: u32) -> Vec<u8> {
    let mut body = identity_body(surface, generation);
    for value in [width, height, 2.0f32.to_bits()] {
        body.extend_from_slice(&value.to_le_bytes());
    }
    body
}

fn frame_body(surface: u32, generation: u32, frame: u64, payload: Vec<u8>) -> Vec<u8> {
    let mut body = identity_body(surface, generation);
    body.extend_from_slice(&frame.to_le_bytes());
    body.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    body.extend(payload);
    body
}

fn loss_body(surface: u32, generation: u32, reason: u8) -> Vec<u8> {
    let mut body = identity_body(surface, generation);
    body.push(reason);
    body
}

fn run_request(adapter: &mut WebGpuSurfaceAdapter<MockPort>) -> GpuOutcome {
    loop {
        match adapter.advance(AbiWorkBudget::credits(64)).unwrap() {
            GpuStep::Outcome(outcome) => return outcome,
            GpuStep::Progress { .. } => {}
            other => panic!("unexpected {other:?}"),
        }
    }
}

fn push(adapter: &mut WebGpuSurfaceAdapter<MockPort>, message: AbiMessage) {
    adapter.port.incoming.push_back(message);
}

fn ack_latest(adapter: &mut WebGpuSurfaceAdapter<MockPort>) {
    let (handle, index) = adapter.pages.last().map(|page| (page.handle, page.index)).unwrap();
    push(adapter, AbiMessage::Control(AbiControl::Acknowledge { handle, index }));
    assert_eq!(adapter.advance(AbiWorkBudget::credits(1)).unwrap(), GpuStep::PageSent);
    assert_eq!(adapter.advance(AbiWorkBudget::credits(1)).unwrap(), GpuStep::AwaitingHost);
}

fn create(adapter: &mut WebGpuSurfaceAdapter<MockPort>, surface: u32, generation: u32) -> GpuOutcome {
    push(adapter, request(GPU_SURFACE_OPERATION_CREATE, surface as u64, generation, create_body(0, surface, surface, generation, 640, 480)));
    run_request(adapter)
}

#[test]
fn schema_and_language_neutral_ledgers_declare_every_operation_and_limit() {
    for operation in ["create", "resize", "frame", "deviceLoss", "recover", "drop"] {
        assert!(GPU_SURFACE_SCHEMA_JSON.contains(operation));
    }
    let mut adapter = WebGpuSurfaceAdapter::new(MockPort::default());
    for (index, line) in GPU_SURFACE_TRACE_FIXTURE.lines().filter(|line| !line.starts_with('#')).enumerate() {
        let fields: Vec<_> = line.split('\t').collect();
        let operation = fields[1].parse().unwrap();
        let generation = fields[2].parse().unwrap();
        let body = fields[3].as_bytes().chunks_exact(2).map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap()).collect();
        push(&mut adapter, request(operation, index as u64 + 1, generation, body));
        let outcome = run_request(&mut adapter);
        let actual = match outcome {
            GpuOutcome::Created { .. } => "created",
            GpuOutcome::Resized { .. } => "resized",
            GpuOutcome::FrameAccepted { .. } => "frameAccepted",
            GpuOutcome::Lost { .. } => "lost",
            GpuOutcome::Recovered { .. } => "recovered",
            GpuOutcome::Dropped { .. } => "dropped",
            GpuOutcome::Cancelled { .. } => "cancelled",
            GpuOutcome::Rejected { .. } => "rejected",
        };
        assert_eq!(actual, fields[4]);
        ack_latest(&mut adapter);
    }
    let limits: Vec<_> = GPU_SURFACE_LIMITS_FIXTURE.lines().filter(|line| !line.starts_with('#')).map(|line| line.split('\t').collect::<Vec<_>>()).collect();
    assert_eq!(limits.len(), 8);
    for row in limits {
        assert_eq!(row[2].parse::<usize>().unwrap(), row[1].parse::<usize>().unwrap() + 1);
    }
}

#[test]
fn create_resize_frame_and_drop_trace_is_ack_controlled() {
    let mut adapter = WebGpuSurfaceAdapter::new(MockPort::default());
    assert!(matches!(create(&mut adapter, 1, 1), GpuOutcome::Created { .. }));
    ack_latest(&mut adapter);
    push(&mut adapter, request(GPU_SURFACE_OPERATION_RESIZE, 2, 1, resize_body(1, 1, 800, 600)));
    assert!(matches!(run_request(&mut adapter), GpuOutcome::Resized { metrics: CanvasMetrics { width: 800, height: 600, .. }, .. }));
    ack_latest(&mut adapter);
    push(&mut adapter, request(GPU_SURFACE_OPERATION_FRAME, 3, 1, frame_body(1, 1, 7, vec![1, 2, 3])));
    assert!(matches!(run_request(&mut adapter), GpuOutcome::FrameAccepted { frame_id: 7, payload_bytes: 3, .. }));
    ack_latest(&mut adapter);
    push(&mut adapter, request(GPU_SURFACE_OPERATION_DROP, 4, 1, identity_body(1, 1)));
    assert!(matches!(run_request(&mut adapter), GpuOutcome::Dropped { .. }));
    assert_eq!(adapter.admission.sessions(), 0);
}

#[test]
fn zero_sized_surface_is_valid_and_parked() {
    let mut adapter = WebGpuSurfaceAdapter::new(MockPort::default());
    push(&mut adapter, request(GPU_SURFACE_OPERATION_CREATE, 1, 1, create_body(0, 1, 1, 1, 0, 0)));
    let GpuOutcome::Created { metrics, .. } = run_request(&mut adapter) else { panic!() };
    assert!(metrics.is_parked());
}

#[test]
fn bad_missing_and_unsupported_canvas_outcomes_do_not_replace_last_valid_surface() {
    let mut adapter = WebGpuSurfaceAdapter::new(MockPort::default());
    assert!(matches!(create(&mut adapter, 1, 1), GpuOutcome::Created { .. }));
    ack_latest(&mut adapter);
    for (id, status, code) in [(2, 1, GpuErrorCode::MissingCanvas), (3, 2, GpuErrorCode::BadCanvas), (4, 3, GpuErrorCode::UnsupportedAdapter)] {
        push(&mut adapter, request(GPU_SURFACE_OPERATION_CREATE, id, 1, create_body(status, id as u32, id as u32, 1, 10, 10)));
        assert!(matches!(run_request(&mut adapter), GpuOutcome::Rejected { code: actual, .. } if actual == code));
        ack_latest(&mut adapter);
    }
    assert_eq!(adapter.surface_metrics(SurfaceId::try_new(1).unwrap()).unwrap().width, 640);
    assert_eq!(adapter.admission.sessions(), 1);
}

#[test]
fn loss_stale_frame_and_deterministic_recovery_retain_metrics() {
    let mut adapter = WebGpuSurfaceAdapter::new(MockPort::default());
    create(&mut adapter, 1, 1);
    ack_latest(&mut adapter);
    push(&mut adapter, request(GPU_SURFACE_OPERATION_DEVICE_LOSS, 2, 1, loss_body(1, 1, 2)));
    assert!(matches!(run_request(&mut adapter), GpuOutcome::Lost { reason: GpuLossReason::Device, .. }));
    ack_latest(&mut adapter);
    push(&mut adapter, request(GPU_SURFACE_OPERATION_FRAME, 3, 1, frame_body(1, 1, 1, Vec::new())));
    assert!(matches!(run_request(&mut adapter), GpuOutcome::Rejected { code: GpuErrorCode::DeviceLost, .. }));
    ack_latest(&mut adapter);
    push(&mut adapter, request(GPU_SURFACE_OPERATION_RECOVER, 4, 1, identity_body(1, 1)));
    assert!(matches!(run_request(&mut adapter), GpuOutcome::Recovered { previous: SurfaceGeneration(1), generation: SurfaceGeneration(2), .. }));
    ack_latest(&mut adapter);
    push(&mut adapter, request(GPU_SURFACE_OPERATION_FRAME, 5, 1, frame_body(1, 1, 2, Vec::new())));
    assert!(matches!(run_request(&mut adapter), GpuOutcome::Rejected { code: GpuErrorCode::StaleGeneration, .. }));
    assert_eq!(adapter.surface_metrics(SurfaceId(1)).unwrap().width, 640);
}

#[test]
fn session_frame_page_and_control_admission_are_exact_at_max_and_max_plus_one() {
    let mut ledger = GpuAdmissionLedger::default();
    for _ in 0..GPU_MAX_SURFACE_SESSIONS {
        ledger.try_admit_session().unwrap();
    }
    assert_eq!(ledger.try_admit_session(), Err(AbiErrorCode::Busy));
    for _ in 0..GPU_MAX_IN_FLIGHT_FRAMES {
        ledger.try_admit_frame().unwrap();
    }
    assert_eq!(ledger.try_admit_frame(), Err(AbiErrorCode::Busy));
    for _ in 0..GPU_MAX_IN_FLIGHT_PAGES {
        ledger.try_admit_page().unwrap();
    }
    assert_eq!(ledger.try_admit_page(), Err(AbiErrorCode::Busy));
    for _ in 0..GPU_MAX_IN_FLIGHT_CONTROLS {
        ledger.try_admit_control().unwrap();
    }
    assert_eq!(ledger.try_admit_control(), Err(AbiErrorCode::Busy));
}

#[test]
fn frame_payload_max_is_accepted_and_max_plus_one_rejected_before_frame_admission() {
    let mut adapter = WebGpuSurfaceAdapter::new(MockPort::default());
    create(&mut adapter, 1, 1);
    ack_latest(&mut adapter);
    push(&mut adapter, request(GPU_SURFACE_OPERATION_FRAME, 2, 1, frame_body(1, 1, 1, vec![0; GPU_MAX_FRAME_BYTES])));
    assert!(matches!(run_request(&mut adapter), GpuOutcome::FrameAccepted { payload_bytes, .. } if payload_bytes as usize == GPU_MAX_FRAME_BYTES));
    ack_latest(&mut adapter);
    push(&mut adapter, request(GPU_SURFACE_OPERATION_FRAME, 3, 1, frame_body(1, 1, 2, vec![0; GPU_MAX_FRAME_BYTES + 1])));
    assert!(matches!(run_request(&mut adapter), GpuOutcome::Rejected { code: GpuErrorCode::LimitExceeded, .. }));
    assert_eq!(adapter.admission.frames(), 0);
}

#[test]
fn actual_page_and_frame_ledgers_stop_at_max_without_mutating_plus_one_request() {
    let mut adapter = WebGpuSurfaceAdapter::new(MockPort::default());
    create(&mut adapter, 1, 1);
    ack_latest(&mut adapter);
    for request_id in 10..10 + GPU_MAX_IN_FLIGHT_PAGES as u64 {
        push(&mut adapter, request(GPU_SURFACE_OPERATION_RESIZE, request_id, 1, resize_body(1, 1, request_id as u32, 1)));
        assert!(matches!(run_request(&mut adapter), GpuOutcome::Resized { .. }));
        assert_eq!(adapter.advance(AbiWorkBudget::credits(1)).unwrap(), GpuStep::PageSent);
    }
    let before = adapter.surface_metrics(SurfaceId(1)).unwrap();
    push(&mut adapter, request(GPU_SURFACE_OPERATION_RESIZE, 99, 1, resize_body(1, 1, 999, 1)));
    assert_eq!(adapter.advance(AbiWorkBudget::credits(64)).unwrap(), GpuStep::AwaitingAcknowledgement);
    assert_eq!(adapter.surface_metrics(SurfaceId(1)).unwrap(), before);
    assert_eq!(adapter.admission.pages(), GPU_MAX_IN_FLIGHT_PAGES);
}

#[test]
fn cancelled_frame_releases_owner_and_returns_paged_cancel_outcome() {
    let mut adapter = WebGpuSurfaceAdapter::new(MockPort::default());
    create(&mut adapter, 1, 1);
    ack_latest(&mut adapter);
    push(&mut adapter, request(GPU_SURFACE_OPERATION_FRAME, 2, 1, frame_body(1, 1, 7, vec![9; 100])));
    assert!(matches!(run_request(&mut adapter), GpuOutcome::FrameAccepted { .. }));
    push(&mut adapter, AbiMessage::Control(AbiControl::Cancel { request_id: AbiRequestId(2), generation: 2 }));
    assert_eq!(adapter.advance(AbiWorkBudget::credits(1)).unwrap(), GpuStep::PageSent);
    assert_eq!(adapter.advance(AbiWorkBudget::credits(1)), Err(AbiErrorCode::StaleGeneration));
    assert_eq!(adapter.admission.frames(), 1);
    push(&mut adapter, AbiMessage::Control(AbiControl::Cancel { request_id: AbiRequestId(2), generation: 1 }));
    assert!(matches!(adapter.advance(AbiWorkBudget::credits(1)).unwrap(), GpuStep::Outcome(GpuOutcome::Cancelled { .. })));
    assert_eq!(adapter.admission.frames(), 0);
}

#[test]
fn interrupted_callback_and_rejected_send_retain_exact_state_for_retry() {
    let mut adapter = WebGpuSurfaceAdapter::new(MockPort::default());
    push(&mut adapter, request(GPU_SURFACE_OPERATION_CREATE, 1, 1, create_body(0, 1, 1, 1, 640, 480)));
    assert_eq!(adapter.advance(AbiWorkBudget { byte_credit: 64, now_ms: 0, deadline_ms: None, cancelled: false, interrupted: true }), Err(AbiErrorCode::Interrupted));
    assert!(matches!(run_request(&mut adapter), GpuOutcome::Created { .. }));
    adapter.port.reject_send = true;
    assert_eq!(adapter.advance(AbiWorkBudget::credits(1)), Err(AbiErrorCode::Interrupted));
    assert!(adapter.outbound.is_some());
    assert_eq!(adapter.advance(AbiWorkBudget::credits(1)).unwrap(), GpuStep::PageSent);
}

#[test]
fn lost_stale_and_duplicate_page_controls_are_explicit() {
    let mut adapter = WebGpuSurfaceAdapter::new(MockPort::default());
    create(&mut adapter, 1, 2);
    let (handle, index) = adapter.pages.last().map(|page| (page.handle, page.index)).unwrap();
    push(&mut adapter, AbiMessage::Control(AbiControl::Acknowledge { handle: AbiHandle::try_new(99, 2).unwrap(), index }));
    assert_eq!(adapter.advance(AbiWorkBudget::credits(1)).unwrap(), GpuStep::PageSent);
    assert_eq!(adapter.advance(AbiWorkBudget::credits(1)), Err(AbiErrorCode::UnknownHandle));
    push(&mut adapter, AbiMessage::Control(AbiControl::Acknowledge { handle: AbiHandle::try_new(1, 1).unwrap(), index }));
    assert_eq!(adapter.advance(AbiWorkBudget::credits(1)), Err(AbiErrorCode::AbaHandle));
    push(&mut adapter, AbiMessage::Control(AbiControl::Acknowledge { handle, index }));
    assert_eq!(adapter.advance(AbiWorkBudget::credits(1)).unwrap(), GpuStep::AwaitingHost);
    push(&mut adapter, AbiMessage::Control(AbiControl::Acknowledge { handle, index }));
    assert_eq!(adapter.advance(AbiWorkBudget::credits(1)), Err(AbiErrorCode::DuplicateAcknowledgement));
}

#[test]
fn callback_work_is_capped_and_close_reaches_terminal_empty_incrementally() {
    let mut adapter = WebGpuSurfaceAdapter::new(MockPort::default());
    push(&mut adapter, request(GPU_SURFACE_OPERATION_CREATE, 1, 1, create_body(0, 1, 1, 1, 640, 480)));
    assert!(matches!(adapter.advance(AbiWorkBudget::credits(usize::MAX)).unwrap(), GpuStep::Outcome(_)));
    adapter.begin_close();
    let mut steps = 0;
    while !adapter.terminal_is_empty() {
        assert!(matches!(adapter.advance(AbiWorkBudget::credits(1)).unwrap(), GpuStep::Closing { .. } | GpuStep::TerminalEmpty));
        steps += 1;
        assert!(steps < 16);
    }
    assert_eq!(adapter.advance(AbiWorkBudget::credits(1)).unwrap(), GpuStep::TerminalEmpty);
    assert_eq!(adapter.admission, GpuAdmissionLedger::default());
}

#[test]
fn page_close_is_exact_and_releases_frame_owner() {
    let mut adapter = WebGpuSurfaceAdapter::new(MockPort::default());
    create(&mut adapter, 1, 1);
    ack_latest(&mut adapter);
    push(&mut adapter, request(GPU_SURFACE_OPERATION_FRAME, 2, 1, frame_body(1, 1, 1, vec![1])));
    run_request(&mut adapter);
    let handle = adapter.pages.last().unwrap().handle;
    assert_eq!(adapter.pages.last().unwrap().frame_owner.as_ref().unwrap().len(), 22);
    push(&mut adapter, AbiMessage::Control(AbiControl::Close { handle }));
    assert_eq!(adapter.advance(AbiWorkBudget::credits(1)).unwrap(), GpuStep::PageSent);
    assert_eq!(adapter.advance(AbiWorkBudget::credits(1)).unwrap(), GpuStep::AwaitingHost);
    assert_eq!(adapter.admission.frames(), 0);
    assert_eq!(adapter.admission.pages(), 0);
}

#[test]
fn malformed_page_body_fixture_type_remains_dependency_free() {
    let page = AbiPage { handle: AbiHandle::try_new(1, 1).unwrap(), index: 0, bytes: AbiPageBytes::try_new(Vec::new()).unwrap() };
    assert_eq!(page.bytes.len(), 0);
}
