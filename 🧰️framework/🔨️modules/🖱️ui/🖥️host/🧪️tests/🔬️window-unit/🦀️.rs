
use super::*;

//#region 📐️Metrics tests

#[test]
fn scale_factor_change_produces_the_right_logical_metrics() {
    let metrics = WindowMetrics { physical: PhysicalSize::new(1600, 1200), scale_factor: 2.0 };
    assert_eq!(metrics.logical_size(), (800.0, 600.0));
}

#[test]
fn a_zero_scale_factor_parks_instead_of_dividing_by_zero() {
    let metrics = WindowMetrics { physical: PhysicalSize::new(100, 100), scale_factor: 0.0 };
    assert_eq!(metrics.logical_size(), (0.0, 0.0));
}

//#endregion 📐️Metrics tests

//#region 🎯️Redraw gating tests

#[test]
fn a_clean_window_never_requests_a_redraw() {
    let mut scheduler = FrameScheduler::new();
    assert_eq!(should_request_redraw(&mut scheduler, 0.0), None);
}

#[test]
fn a_dirty_window_requests_exactly_one_redraw() {
    let mut scheduler = FrameScheduler::new();
    scheduler.invalidate(InvalidationReason::PAINT);
    assert!(should_request_redraw(&mut scheduler, 0.0).is_some());
    assert_eq!(should_request_redraw(&mut scheduler, 0.0), None, "must not double-fire");
}

#[test]
fn a_due_deadline_requests_exactly_one_redraw() {
    let mut scheduler = FrameScheduler::new();
    scheduler.request_deadline(5.0, InvalidationReason::ANIMATION);
    assert_eq!(should_request_redraw(&mut scheduler, 4.0), None, "not due yet");
    assert!(should_request_redraw(&mut scheduler, 5.0).is_some());
    assert_eq!(should_request_redraw(&mut scheduler, 5.0), None);
}

//#endregion 🎯️Redraw gating tests

//#region 🚦️Native control-flow tests

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn no_deadline_waits_indefinitely() {
    let scheduler = FrameScheduler::new();
    let clock = MonotonicClock::new();
    assert!(matches!(control_flow_for(&scheduler, &clock), winit::event_loop::ControlFlow::Wait));
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_pending_deadline_waits_until_a_specific_instant() {
    let mut scheduler = FrameScheduler::new();
    scheduler.request_deadline(1.0, InvalidationReason::ANIMATION);
    let clock = MonotonicClock::new();
    assert!(matches!(control_flow_for(&scheduler, &clock), winit::event_loop::ControlFlow::WaitUntil(_)));
}

//#endregion 🚦️Native control-flow tests

//#region 🖱️Cursor tests

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn every_cursor_request_maps_to_a_distinct_icon() {
    use std::collections::HashSet;
    let icons: HashSet<_> = [CursorRequest::Default, CursorRequest::Pointer, CursorRequest::Text, CursorRequest::Grab, CursorRequest::Grabbing].into_iter().map(|cursor| format!("{:?}", cursor_icon_for(cursor))).collect();
    assert_eq!(icons.len(), 5);
}

#[test]
fn cursor_css_names_match_the_standard_css_keywords() {
    assert_eq!(cursor_css_for(CursorRequest::Grabbing), "grabbing");
    assert_eq!(cursor_css_for(CursorRequest::Default), "default");
}

//#endregion 🖱️Cursor tests

//#region 🌐️Browser host laws

#[derive(Default)]
struct BrowserFixtureState {
    incoming: std::collections::VecDeque<AbiMessage>,
    sent: Vec<AbiMessage>,
    reject_next: bool,
    closed: bool,
}

#[derive(Clone, Default)]
struct BrowserFixturePort(std::rc::Rc<std::cell::RefCell<BrowserFixtureState>>);

impl AbiPort for BrowserFixturePort {
    fn try_send(&mut self, message: AbiMessage, _budget: AbiWorkBudget) -> Result<(), crate::abi::AbiPortRejection> {
        let mut state = self.0.borrow_mut();
        if state.reject_next {
            state.reject_next = false;
            return Err(crate::abi::AbiPortRejection { code: AbiErrorCode::Interrupted, message });
        }
        state.sent.push(message);
        Ok(())
    }

    fn poll(&mut self, _budget: AbiWorkBudget) -> Result<AbiPortPoll, AbiErrorCode> {
        let mut state = self.0.borrow_mut();
        Ok(state.incoming.pop_front().map(AbiPortPoll::Message).unwrap_or(if state.closed { AbiPortPoll::Closed } else { AbiPortPoll::Pending }))
    }
}

struct BrowserFixtureDelegate {
    scheduler: FrameScheduler,
    events: Vec<DispatchEvent>,
    metrics: Vec<WindowMetrics>,
    frames: usize,
}

impl Default for BrowserFixtureDelegate {
    fn default() -> Self {
        Self { scheduler: FrameScheduler::new(), events: Vec::new(), metrics: Vec::new(), frames: 0 }
    }
}

impl WindowDelegate for BrowserFixtureDelegate {
    fn scheduler_mut(&mut self) -> &mut FrameScheduler {
        &mut self.scheduler
    }

    fn handle_event(&mut self, event: DispatchEvent) {
        self.events.push(event);
    }

    fn handle_metrics(&mut self, metrics: WindowMetrics) {
        self.metrics.push(metrics);
    }

    fn redraw(&mut self, _reason: InvalidationReason) -> RedrawOutcome {
        RedrawOutcome { cursor: CursorRequest::Pointer, ime: None }
    }
}

impl BrowserWindowDelegate for BrowserFixtureDelegate {
    fn enqueue_browser_frame(&mut self, _reason: InvalidationReason) {
        self.frames += 1;
    }

    fn present_browser_frame(&mut self) -> RedrawOutcome {
        self.redraw(InvalidationReason::PAINT)
    }
}

fn browser_fixture() -> (CanvasHost<BrowserFixturePort, BrowserFixtureDelegate>, BrowserFixturePort) {
    let port = BrowserFixturePort::default();
    let host = CanvasHost::new(crate::event::CanvasId::try_new(1).unwrap(), port.clone(), BrowserFixtureDelegate::default()).unwrap();
    (host, port)
}

fn attach_browser(host: &mut CanvasHost<BrowserFixturePort, BrowserFixtureDelegate>, port: &BrowserFixturePort) {
    while port.0.borrow().sent.is_empty() {
        host.step(AbiWorkBudget::credits(1)).unwrap();
    }
    let body = AbiBytes::try_new([vec![1], 1_u32.to_le_bytes().to_vec(), 1_u32.to_le_bytes().to_vec()].concat()).unwrap();
    port.0.borrow_mut().incoming.push_back(AbiMessage::Reply(AbiReply { request_id: AbiRequestId(1), generation: 1, status: crate::abi::AbiStatus::OK, bytes: body }));
    host.step(AbiWorkBudget::credits(1)).unwrap();
    for _ in 0..8 {
        assert!(matches!(host.step(AbiWorkBudget::credits(1)).unwrap(), BrowserHostStep::Progress(_)));
    }
    assert_eq!(host.step(AbiWorkBudget::credits(1)).unwrap(), BrowserHostStep::Event);
}

fn browser_event(code: u16, sequence: u32, tail: Vec<u8>, generation: u32) -> AbiMessage {
    let mut bytes = vec![1];
    bytes.extend_from_slice(&1_u32.to_le_bytes());
    bytes.extend_from_slice(&1_u32.to_le_bytes());
    bytes.extend_from_slice(&1_u32.to_le_bytes());
    bytes.extend_from_slice(&tail);
    AbiMessage::Event(AbiEvent { request_id: AbiRequestId(100 + sequence as u64), generation, sequence, event: crate::abi::AbiEventCode::try_new(code).unwrap(), status: crate::abi::AbiStatus::OK, bytes: AbiBytes::try_new(bytes).unwrap() })
}

fn raw_browser_event(body_bytes: usize) -> AbiMessage {
    AbiMessage::Event(AbiEvent {
        request_id: AbiRequestId(900),
        generation: 1,
        sequence: 1,
        event: crate::abi::AbiEventCode::try_new(crate::event::BROWSER_EVENT_TEXT).unwrap(),
        status: crate::abi::AbiStatus::OK,
        bytes: AbiBytes::try_new(vec![0; body_bytes]).unwrap(),
    })
}

fn inspect_and_classify(host: &mut CanvasHost<BrowserFixturePort, BrowserFixtureDelegate>, bytes: usize) {
    assert!(matches!(host.step(AbiWorkBudget::credits(1)).unwrap(), BrowserHostStep::Progress(_)));
    for _ in 0..bytes {
        assert!(matches!(host.step(AbiWorkBudget::credits(1)).unwrap(), BrowserHostStep::Progress(_)));
    }
}

#[test]
fn browser_missing_objects_and_interrupted_send_are_owned_failures() {
    let (mut host, port) = browser_fixture();
    port.0.borrow_mut().reject_next = true;
    for _ in 0..4 {
        assert!(matches!(host.step(AbiWorkBudget::credits(1)).unwrap(), BrowserHostStep::Progress(_)));
    }
    assert_eq!(host.step(AbiWorkBudget::credits(1)), Err(BrowserHostError::Abi(AbiErrorCode::Interrupted)));
    host.step(AbiWorkBudget::credits(1)).unwrap();
    port.0.borrow_mut().incoming.push_back(AbiMessage::Reply(AbiReply { request_id: AbiRequestId(1), generation: 1, status: crate::abi::AbiStatus { code: AbiStatusCode::Failed, error: None }, bytes: AbiBytes::try_new(vec![2]).unwrap() }));
    host.step(AbiWorkBudget::credits(1)).unwrap();
    assert_eq!(host.step(AbiWorkBudget::credits(1)).unwrap(), BrowserHostStep::Unavailable(BrowserHostUnavailable::Document));
    for (code, expected) in [(1, BrowserHostUnavailable::Window), (3, BrowserHostUnavailable::Canvas), (4, BrowserHostUnavailable::Clipboard)] {
        let (mut host, port) = browser_fixture();
        while port.0.borrow().sent.is_empty() {
            host.step(AbiWorkBudget::credits(1)).unwrap();
        }
        port.0.borrow_mut().incoming.push_back(AbiMessage::Reply(AbiReply { request_id: AbiRequestId(1), generation: 1, status: crate::abi::AbiStatus { code: AbiStatusCode::Failed, error: None }, bytes: AbiBytes::try_new(vec![code]).unwrap() }));
        host.step(AbiWorkBudget::credits(1)).unwrap();
        assert_eq!(host.step(AbiWorkBudget::credits(1)).unwrap(), BrowserHostStep::Unavailable(expected));
    }
}

#[test]
fn browser_resize_storm_is_latest_wins_and_one_byte_is_inspected_per_grant() {
    let (mut host, port) = browser_fixture();
    attach_browser(&mut host, &port);
    for (sequence, width) in [(1, 10_u32), (2, 20_u32)] {
        let mut tail = width.to_le_bytes().to_vec();
        tail.extend_from_slice(&30_u32.to_le_bytes());
        tail.extend_from_slice(&1_f32.to_le_bytes());
        let message = browser_event(crate::event::BROWSER_EVENT_METRICS, sequence, tail, 1);
        let length = match &message {
            AbiMessage::Event(event) => event.bytes.len(),
            _ => 0,
        };
        port.0.borrow_mut().incoming.push_back(message);
        inspect_and_classify(&mut host, length);
    }
    assert!(matches!(host.step(AbiWorkBudget::credits(1)).unwrap(), BrowserHostStep::Progress(_)));
    assert_eq!(host.step(AbiWorkBudget::credits(1)).unwrap(), BrowserHostStep::Event);
    assert_eq!(host.delegate().metrics, vec![WindowMetrics { physical: PhysicalSize::new(20, 30), scale_factor: 1.0 }]);
}

#[test]
fn browser_frame_is_bounded_and_stale_generation_is_rejected() {
    let (mut host, port) = browser_fixture();
    attach_browser(&mut host, &port);
    assert_eq!(host.request_wake().unwrap(), true);
    assert_eq!(host.request_wake().unwrap(), false);
    let sent = port.0.borrow().sent.len();
    while port.0.borrow().sent.len() == sent {
        host.step(AbiWorkBudget::credits(1)).unwrap();
    }
    port.0.borrow_mut().incoming.push_back(browser_event(crate::event::BROWSER_EVENT_FRAME, 1, 16_f64.to_le_bytes().to_vec(), 2));
    assert_eq!(host.step(AbiWorkBudget::credits(1)), Err(BrowserHostError::Abi(AbiErrorCode::StaleGeneration)));
}

#[test]
fn browser_frame_ack_and_cursor_are_separate_bounded_steps() {
    let (mut host, port) = browser_fixture();
    attach_browser(&mut host, &port);
    host.delegate_mut().scheduler.invalidate(InvalidationReason::PAINT);
    assert!(host.request_wake().unwrap());
    let sent = port.0.borrow().sent.len();
    while port.0.borrow().sent.len() == sent {
        host.step(AbiWorkBudget::credits(1)).unwrap();
    }
    let message = browser_event(crate::event::BROWSER_EVENT_FRAME, 1, 16_f64.to_le_bytes().to_vec(), 1);
    let length = match &message {
        AbiMessage::Event(event) => event.bytes.len(),
        _ => unreachable!(),
    };
    port.0.borrow_mut().incoming.push_back(message);
    inspect_and_classify(&mut host, length);
    assert!(matches!(host.step(AbiWorkBudget::credits(1)).unwrap(), BrowserHostStep::Frame(_)));
    host.step(AbiWorkBudget::credits(1)).unwrap();
    host.step(AbiWorkBudget::credits(1)).unwrap();
    for _ in 0..6 {
        host.step(AbiWorkBudget::credits(1)).unwrap();
    }
    assert_eq!(host.delegate().frames, 1);
    assert!(port.0.borrow().sent.iter().any(|message| matches!(message, AbiMessage::Request(request) if request.operation.get() == BROWSER_HOST_OPERATION_CURSOR)));
}

#[test]
fn browser_clipboard_cancel_before_during_after_and_close_are_deterministic() {
    let (mut before, before_port) = browser_fixture();
    attach_browser(&mut before, &before_port);
    before.request_clipboard_read().unwrap();
    assert_eq!(before.cancel_clipboard().unwrap(), true);
    assert_eq!(before.cancel_clipboard().unwrap(), false);

    let (mut during, during_port) = browser_fixture();
    attach_browser(&mut during, &during_port);
    during.request_clipboard_read().unwrap();
    let sent = during_port.0.borrow().sent.len();
    while during_port.0.borrow().sent.len() == sent {
        during.step(AbiWorkBudget::credits(1)).unwrap();
    }
    assert_eq!(during.cancel_clipboard().unwrap(), true);

    let (mut after, after_port) = browser_fixture();
    attach_browser(&mut after, &after_port);
    let request_id = after.request_clipboard_read().unwrap();
    let sent = after_port.0.borrow().sent.len();
    while after_port.0.borrow().sent.len() == sent {
        after.step(AbiWorkBudget::credits(1)).unwrap();
    }
    after_port.0.borrow_mut().incoming.push_back(AbiMessage::Reply(AbiReply { request_id, generation: 1, status: crate::abi::AbiStatus::OK, bytes: AbiBytes::try_new(b"x".to_vec()).unwrap() }));
    after.step(AbiWorkBudget::credits(1)).unwrap();
    assert!(matches!(after.step(AbiWorkBudget::credits(1)).unwrap(), BrowserHostStep::Clipboard { text: Some(text), .. } if text == "x"));
    assert_eq!(after.cancel_clipboard().unwrap(), false);

    during.begin_close();
    assert_eq!(during.step(AbiWorkBudget { interrupted: true, ..AbiWorkBudget::credits(1) }), Err(BrowserHostError::Abi(AbiErrorCode::Interrupted)));
    let mut detach_replied = false;
    for _ in 0..64 {
        during.step(AbiWorkBudget::credits(1)).unwrap();
        if !detach_replied {
            let detach = during_port.0.borrow().sent.iter().find_map(|message| match message {
                AbiMessage::Request(request) if request.operation.get() == BROWSER_HOST_OPERATION_DETACH => Some((request.request_id, request.generation)),
                _ => None,
            });
            if let Some((request_id, generation)) = detach {
                assert!(!during.terminal_is_empty());
                during_port.0.borrow_mut().incoming.push_back(AbiMessage::Reply(AbiReply { request_id, generation, status: crate::abi::AbiStatus::OK, bytes: AbiBytes::default() }));
                detach_replied = true;
            }
        }
        if during.terminal_is_empty() {
            break;
        }
    }
    assert!(detach_replied);
    assert!(during.terminal_is_empty());
}

#[test]
fn browser_listener_aba_and_event_decoder_terminal_laws() {
    let listener = crate::event::ListenerId::try_new(1, 2).unwrap();
    let event = match browser_event(crate::event::BROWSER_EVENT_CLOSE, 1, Vec::new(), 1) {
        AbiMessage::Event(event) => event,
        _ => unreachable!(),
    };
    assert_eq!(crate::event::decode_browser_host_event(&event, crate::event::CanvasId::try_new(1).unwrap(), listener), Err(AbiErrorCode::AbaHandle));
    assert_eq!(crate::event::CanvasId::try_new(0), Err(AbiErrorCode::UnknownHandle));
    let (mut host, port) = browser_fixture();
    attach_browser(&mut host, &port);
    port.0.borrow_mut().incoming.push_back(AbiMessage::Control(AbiControl::Close { handle: crate::abi::AbiHandle::try_new(1, 2).unwrap() }));
    assert_eq!(host.step(AbiWorkBudget::credits(1)), Err(BrowserHostError::Abi(AbiErrorCode::StaleGeneration)));
    port.0.borrow_mut().incoming.push_back(AbiMessage::Control(AbiControl::Close { handle: crate::abi::AbiHandle::try_new(2, 1).unwrap() }));
    assert_eq!(host.step(AbiWorkBudget::credits(1)), Err(BrowserHostError::Abi(AbiErrorCode::UnknownHandle)));
}

#[test]
fn browser_linear_memory_exact_envelope_retry_and_preflight_laws() {
    use super::browser::linear_memory_test_import::{self as seam, AfterProbe};

    seam::reset();
    let empty = crate::abi::encode_abi_message(&raw_browser_event(0));
    assert_eq!(empty.len(), BROWSER_HOST_EVENT_ENVELOPE_BYTES);
    seam::enqueue(empty);
    let mut port = LinearMemoryBrowserHostPort;
    assert!(matches!(port.poll(AbiWorkBudget::credits(1)).unwrap(), AbiPortPoll::Message(AbiMessage::Event(event)) if event.bytes.is_empty()));
    assert_eq!(seam::census(), (vec![BROWSER_HOST_INITIAL_POLL_BYTES], 1, 0, false));

    seam::reset();
    let maximum = crate::abi::encode_abi_message(&raw_browser_event(BROWSER_HOST_MAX_EVENT_BODY_BYTES));
    assert_eq!(maximum.len(), BROWSER_HOST_MAX_ENCODED_EVENT_BYTES);
    seam::enqueue(maximum);
    assert!(matches!(port.poll(AbiWorkBudget::credits(1)).unwrap(), AbiPortPoll::Message(AbiMessage::Event(event)) if event.bytes.len() == BROWSER_HOST_MAX_EVENT_BODY_BYTES));
    assert_eq!(seam::census(), (vec![BROWSER_HOST_INITIAL_POLL_BYTES, BROWSER_HOST_MAX_ENCODED_EVENT_BYTES], 1, 0, false));
    assert_eq!(port.poll(AbiWorkBudget::credits(1)).unwrap(), AbiPortPoll::Pending);

    seam::reset();
    let maximum_send = raw_browser_event(BROWSER_HOST_MAX_EVENT_BODY_BYTES);
    port.try_send(maximum_send, AbiWorkBudget::credits(1)).unwrap();
    assert_eq!(seam::sent_lengths(), vec![BROWSER_HOST_MAX_ENCODED_EVENT_BYTES]);
    let oversized_send = raw_browser_event(BROWSER_HOST_MAX_EVENT_BODY_BYTES + 1);
    let rejection = port.try_send(oversized_send.clone(), AbiWorkBudget::credits(1)).unwrap_err();
    assert_eq!(rejection.code, AbiErrorCode::LimitExceeded);
    assert_eq!(rejection.message, oversized_send);
    assert_eq!(seam::sent_lengths(), vec![BROWSER_HOST_MAX_ENCODED_EVENT_BYTES]);

    seam::reset();
    let oversized = crate::abi::encode_abi_message(&raw_browser_event(BROWSER_HOST_MAX_EVENT_BODY_BYTES + 1));
    assert_eq!(oversized.len(), BROWSER_HOST_MAX_ENCODED_EVENT_BYTES + 1);
    seam::enqueue(oversized);
    assert_eq!(port.poll(AbiWorkBudget::credits(1)), Err(AbiErrorCode::LimitExceeded));
    assert_eq!(seam::census(), (vec![BROWSER_HOST_INITIAL_POLL_BYTES], 0, 1, false));

    seam::reset();
    let page = AbiPage::try_new(crate::abi::AbiHandle::try_new(1, 1).unwrap(), 0, vec![7; BROWSER_HOST_MAX_PAGE_BODY_BYTES]).unwrap();
    let encoded_page = crate::abi::encode_abi_message(&AbiMessage::Page(page));
    assert_eq!(encoded_page.len(), BROWSER_HOST_MAX_ENCODED_EVENT_BYTES);
    seam::enqueue(encoded_page);
    assert!(matches!(port.poll(AbiWorkBudget::credits(1)).unwrap(), AbiPortPoll::Message(AbiMessage::Page(page)) if page.bytes.len() == BROWSER_HOST_MAX_PAGE_BODY_BYTES));
    assert_eq!(seam::census(), (vec![BROWSER_HOST_INITIAL_POLL_BYTES, BROWSER_HOST_MAX_ENCODED_EVENT_BYTES], 1, 0, false));

    seam::reset();
    let oversized_page = AbiPage::try_new(crate::abi::AbiHandle::try_new(1, 1).unwrap(), 0, vec![7; BROWSER_HOST_MAX_PAGE_BODY_BYTES + 1]).unwrap();
    let encoded_page = crate::abi::encode_abi_message(&AbiMessage::Page(oversized_page));
    assert_eq!(encoded_page.len(), BROWSER_HOST_MAX_ENCODED_EVENT_BYTES + 1);
    seam::enqueue(encoded_page);
    assert_eq!(port.poll(AbiWorkBudget::credits(1)), Err(AbiErrorCode::LimitExceeded));
    assert_eq!(seam::census(), (vec![BROWSER_HOST_INITIAL_POLL_BYTES], 0, 1, false));

    for (action, expected, closed) in [(AfterProbe::Cancel, AbiPortPoll::Pending, false), (AfterProbe::Close, AbiPortPoll::Closed, true)] {
        seam::reset();
        seam::enqueue(crate::abi::encode_abi_message(&raw_browser_event(BROWSER_HOST_MAX_EVENT_BODY_BYTES)));
        seam::after_probe(action);
        assert_eq!(port.poll(AbiWorkBudget::credits(1)).unwrap(), expected);
        assert_eq!(seam::census(), (vec![BROWSER_HOST_INITIAL_POLL_BYTES, BROWSER_HOST_MAX_ENCODED_EVENT_BYTES], 0, 0, closed));
    }
}

#[test]
fn browser_exact_event_and_page_are_credited_and_acknowledged_once() {
    let (mut host, port) = browser_fixture();
    attach_browser(&mut host, &port);
    let text_length = BROWSER_HOST_MAX_EVENT_BODY_BYTES - 15;
    let mut text = (text_length as u16).to_le_bytes().to_vec();
    text.extend(std::iter::repeat(b'x').take(text_length));
    let event = browser_event(crate::event::BROWSER_EVENT_TEXT, 1, text, 1);
    let event_bytes = match &event {
        AbiMessage::Event(event) => event.bytes.len(),
        _ => unreachable!(),
    };
    assert_eq!(event_bytes, BROWSER_HOST_MAX_EVENT_BODY_BYTES);
    port.0.borrow_mut().incoming.push_back(event);
    inspect_and_classify(&mut host, event_bytes);
    assert_eq!(host.step(AbiWorkBudget::credits(1)).unwrap(), BrowserHostStep::Event);
    host.step(AbiWorkBudget::credits(1)).unwrap();
    let event_acks = port.0.borrow().sent.iter().filter(|message| matches!(message, AbiMessage::Reply(reply) if reply.request_id == AbiRequestId(101))).count();
    assert_eq!(event_acks, 1);

    let handle = crate::abi::AbiHandle::try_new(9, 1).unwrap();
    let page = AbiPage::try_new(handle, 0, vec![5; BROWSER_HOST_MAX_PAGE_BODY_BYTES]).unwrap();
    port.0.borrow_mut().incoming.push_back(AbiMessage::Page(page));
    assert!(matches!(host.step(AbiWorkBudget::credits(1)).unwrap(), BrowserHostStep::Progress(_)));
    for _ in 0..BROWSER_HOST_MAX_PAGE_BODY_BYTES {
        assert!(matches!(host.step(AbiWorkBudget::credits(1)).unwrap(), BrowserHostStep::Progress(_)));
    }
    host.step(AbiWorkBudget::credits(1)).unwrap();
    let page_acks = port.0.borrow().sent.iter().filter(|message| matches!(message, AbiMessage::Control(AbiControl::Acknowledge { handle: actual, index: 0 }) if *actual == handle)).count();
    assert_eq!(page_acks, 1);
}

//#endregion 🌐️Browser host laws
