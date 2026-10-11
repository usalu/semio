
use super::*;

#[cfg(not(target_arch = "wasm32"))]
use crate::observed_allocator::measured;

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

#[cfg(not(target_arch="wasm32"))]
#[test]
fn native_clipboard_original_string_close_requires_whole_capacity_and_depth(){
 use semio_framework_job::{InteractiveJob,InteractiveJobCloseStep as Close};
 use semio_framework_value::{RetainedCloneGrant as Grant,RetainedCloneProgress as Progress};
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/📋️clipboard-close/🔣️.json")).unwrap();
 for text in fixture["texts"].as_array().unwrap(){
  let mut original=String::with_capacity(fixture["reserve"].as_u64().unwrap()as usize);original.push_str(text.as_str().unwrap());let pointer=original.as_ptr();let capacity=original.capacity();
  let mut job=super::NativeClipboardJob::write(original);job.begin_close();
  let exact=Grant{maximum_items:fixture["closeItems"].as_u64().unwrap()as usize,maximum_copy_bytes:fixture["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:fixture["capacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:capacity,maximum_depth:fixture["closeDepth"].as_u64().unwrap()as usize};
  assert_eq!(job.next_close_copy_byte_demand().unwrap(),0);assert_eq!(job.next_close_capacity_byte_demand(0).unwrap(),0);assert_eq!(job.next_close_release_byte_demand().unwrap(),capacity);assert_eq!(job.next_close_depth_demand().unwrap(),1);
  for short in [Grant{maximum_items:0,..exact},Grant{maximum_release_bytes:capacity-1,..exact},Grant{maximum_depth:0,..exact}]{
   assert_eq!(job.close_step(short),Close::Pending{progress:Progress::default()});
   let Some(super::NativeClipboardOperation::Write(text))=job.operation.as_ref() else{panic!("original clipboard owner survived refusal")};assert_eq!(text.as_ptr(),pointer);assert_eq!(text.capacity(),capacity);
  }
  let progress=Progress{copied_items:1,released_bytes:capacity,..Progress::default()};assert_eq!(job.close_step(exact),Close::Complete{progress});assert!(progress.fits(exact));assert!(job.terminal_is_empty());assert_eq!(job.close_step(exact),Close::Complete{progress:Progress::default()});
  eprintln!("[DEBUG] original clipboard close sameString=true empty={} wholeCapacity={} exactRelease={} depth=1",text.as_str().unwrap().is_empty(),capacity,progress.released_bytes);
 }
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn cancelled_clipboard_outcome_preserves_original_input_until_descriptor_ack_and_paid_close() {
    use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep as Step, RetainedCloneGrant, RetainedCloneProgress, StepContextOwner, StepBudget, OperationId, Generation, JobOutcomeKind, JobOutcomeView};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/♻️physical-job-close/🧫️fixtures/🔣️.json")).unwrap();
    let law = &fixture["clipboardOutcome"];
    let original: serde_json::Value = serde_json::from_str(include_str!("../../../../🧵️job/📬️outcome/🤝️loan/🧫️fixtures/🔣️.json")).unwrap();
    let policy = &original["policy"];
    let supplied = RetainedCloneGrant { maximum_items: policy["items"].as_u64().unwrap() as usize, maximum_copy_bytes: policy["copy"].as_u64().unwrap() as usize, maximum_capacity_bytes: policy["capacity"].as_u64().unwrap() as usize, maximum_release_bytes: policy["release"].as_u64().unwrap() as usize, maximum_depth: policy["depth"].as_u64().unwrap() as usize };
    let (mut owner, _) = StepContextOwner::new(OperationId(1), Generation(1), supplied).unwrap();
    let mut source = String::with_capacity(fixture["capacities"][1].as_u64().unwrap() as usize);
    source.push_str(fixture["texts"][1].as_str().unwrap());
    let pointer = source.as_ptr();
    let capacity = source.capacity();
    let mut job = NativeClipboardJob::write(source);
    let cancel = semio_framework_job::root_cancel_token();
    cancel.cancel_now();
    let mut sequence = 0;
    let mut denied_receipt = RetainedCloneProgress::default();
    let mut denied = owner.context(StepBudget::new(100, 10, RetainedCloneGrant { maximum_items: law["deniedItems"].as_u64().unwrap() as usize, ..supplied }), cancel.clone(), || Some(0), &mut sequence, &mut denied_receipt).unwrap();
    let (outcome, born, freed) = measured(|| job.step(&mut denied).unwrap().map(|outcome| outcome.into_descriptor()));
    assert!(outcome.is_none());
    assert_eq!((born, freed), (0, 0));
    assert_eq!(denied.retained_progress(), RetainedCloneProgress::default());
    drop(denied);
    let mut receipt = RetainedCloneProgress::default();
    let mut cx = owner.context(StepBudget::new(100, 10, supplied), cancel, || Some(0), &mut sequence, &mut receipt).unwrap();
    let (outcome, born, freed) = measured(|| job.step(&mut cx).unwrap().map(|outcome| outcome.into_descriptor()));
    let mut descriptor = outcome.expect("original cancelled outcome is admitted");
    assert_eq!((born, freed), (0, 0));
    assert_eq!(descriptor.kind(), JobOutcomeKind::Cancelled);
    assert_eq!(cx.retained_progress(), RetainedCloneProgress { copied_items: 1, ..Default::default() });
    drop(cx);
    let (view, born, freed) = measured(|| job.borrow_outcome(&descriptor).unwrap());
    let JobOutcomeView::Cancelled { admission } = view else { panic!("original cancelled descriptor") };
    assert!(std::ptr::eq(admission, descriptor.admission()));
    assert_eq!((born, freed), (law["borrowHeapBytes"].as_u64().unwrap() as usize, 0));
    let Some(NativeClipboardOperation::Write(text)) = job.operation.as_ref() else { panic!("original write input retained through cancellation") };
    assert_eq!(text.as_ptr(), pointer);
    assert_eq!(text.capacity(), capacity);
    assert_eq!(serde_json::to_value(text).unwrap(), fixture["texts"][1]);
    assert!(!job.terminal_is_empty());
    assert_eq!(descriptor.acknowledge(supplied).progress().copied_items, law["acknowledgementItems"].as_u64().unwrap() as usize);
    drop(descriptor);
    job.begin_close();
    let (closed, born, freed) = measured(|| job.close_step(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 0, maximum_capacity_bytes: 0, maximum_release_bytes: capacity, maximum_depth: 1 }));
    assert!(matches!(closed, Step::Complete { .. }));
    assert_eq!((born, freed), (0, capacity));
    assert_eq!(closed.progress(), RetainedCloneProgress { copied_items: 1, released_bytes: capacity, ..Default::default() });
    assert!(job.terminal_is_empty());
    while !owner.terminal_is_empty() { assert!(owner.close_step(supplied).progress().fits(supplied)); }
    eprintln!("[DEBUG] clipboard cancelled descriptor original input pointer/capacity preserved; denied outcome0heap, borrowed admission0heap, original ACK and exact native release funded separately");
}

#[test]
fn cancelled_clipboard_driver_binds_the_original_producer_witness(){
    use semio_framework_job::{InteractiveJob,StepContextOwner,StepBudget,OperationId,Generation,JobOutcomeView,RetainedCloneGrant,RetainedCloneProgress};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🧪️tests/♻️physical-job-close/🧫️fixtures/🔣️.json")).unwrap();
    let original:serde_json::Value=serde_json::from_str(include_str!("../../../../🧵️job/📬️outcome/🤝️loan/🧫️fixtures/🔣️.json")).unwrap();
    let policy=&original["policy"];
    let grant=RetainedCloneGrant{maximum_items:policy["items"].as_u64().unwrap()as usize,maximum_copy_bytes:policy["copy"].as_u64().unwrap()as usize,maximum_capacity_bytes:policy["capacity"].as_u64().unwrap()as usize,maximum_release_bytes:policy["release"].as_u64().unwrap()as usize,maximum_depth:policy["depth"].as_u64().unwrap()as usize};
    let(mut owner,_)=StepContextOwner::new(OperationId(1),Generation(1),grant).unwrap();
    let mut text=String::with_capacity(fixture["capacities"][1].as_u64().unwrap()as usize);text.push_str(fixture["texts"][1].as_str().unwrap());let capacity=text.capacity();
    let mut job=super::NativeClipboardJob::write(text);
    let cancel=semio_framework_job::root_cancel_token();cancel.cancel_now();
    let mut sequence=0;let mut receipt=RetainedCloneProgress::default();let mut verdict=None;
    let mut cx=owner.context(StepBudget::new(100,10,grant),cancel,||Some(0),&mut sequence,&mut receipt).unwrap();
    let mut descriptor=semio_framework_job::drive_step(&mut job,&mut cx,"original_clipboard_cancelled_bridge",semio_framework_job::InteractiveStage::InteractiveStep,&mut verdict).unwrap().unwrap().into_descriptor();
    assert_eq!(cx.retained_progress(),RetainedCloneProgress{copied_items:1,..Default::default()});drop(cx);
    let(view,born,freed)=measured(||job.borrow_outcome(&descriptor).unwrap());assert!(matches!(view,JobOutcomeView::Cancelled{..}));assert_eq!((born,freed),(0,0));
    assert_eq!(descriptor.acknowledge(grant).progress().copied_items,1);drop(descriptor);
    job.begin_close();let closed=job.close_step(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:capacity,maximum_depth:1});assert_eq!(closed.progress(),RetainedCloneProgress{copied_items:1,released_bytes:capacity,..Default::default()});assert!(job.terminal_is_empty());
    while !owner.terminal_is_empty(){assert!(owner.close_step(grant).progress().fits(grant))}
    eprintln!("[DEBUG] actual clipboard drive_step original cancelled producer witness borrowed until funded ACK and physical input close");
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn original_clipboard_session_admission_is_paid_before_birth_and_preserves_denied_owners() {
    use semio_framework_job::{BatchDriveConfig,BatchJobParams,Generation,InteractiveJob,InteractiveStage,OperationId,RetainedCloneGrant,RetainedCloneProgress,StepBudget,StepContextOwner,WorkerJobSession};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🧪️tests/♻️physical-job-close/🧫️fixtures/🔣️.json")).unwrap();
    let original:serde_json::Value=serde_json::from_str(include_str!("../../../../🧵️job/📬️outcome/🤝️loan/🧫️fixtures/🔣️.json")).unwrap();
    let policy=&original["policy"];
    let supplied=RetainedCloneGrant{maximum_items:policy["items"].as_u64().unwrap()as usize,maximum_copy_bytes:policy["copy"].as_u64().unwrap()as usize,maximum_capacity_bytes:policy["capacity"].as_u64().unwrap()as usize,maximum_release_bytes:policy["release"].as_u64().unwrap()as usize,maximum_depth:policy["depth"].as_u64().unwrap()as usize};
    let demand=WorkerJobSession::<NativeClipboardJob>::admission_demand();
    assert!(demand.capacity_bytes>0&&demand.capacity_bytes<=supplied.maximum_capacity_bytes);
    assert!(demand.copy_bytes<=supplied.maximum_copy_bytes&&demand.depth<=supplied.maximum_depth);
    let(mut owner,_)=StepContextOwner::new(OperationId(1),Generation(1),supplied).unwrap();
    let cancel=semio_framework_job::root_cancel_token();
    let mut text=String::with_capacity(fixture["capacities"][1].as_u64().unwrap()as usize);text.push_str(fixture["texts"][1].as_str().unwrap());
    let pointer=text.as_ptr();let capacity=text.capacity();let oracle:String=serde_json::from_value(fixture["texts"][1].clone()).unwrap();
    let mut job=Some(NativeClipboardJob::write(text));
    let mut params=Some(BatchJobParams{operation:OperationId(1),generation:Generation(1),cancel:cancel.clone(),config:BatchDriveConfig{retained:RetainedCloneGrant::default(),site:"clipboard_original_owned_admission",stage:InteractiveStage::InteractiveStep,fuel_per_step:100,step_budget_us:10},now_us:||Some(0)});
    let source_address=job.as_ref().unwrap()as*const NativeClipboardJob;let params_address=params.as_ref().unwrap()as*const BatchJobParams;
    let mut sequence=0;
    for (operation,generation) in [(OperationId(2),Generation(1)),(OperationId(1),Generation(2))] {
        let original_params=params.as_mut().unwrap();original_params.operation=operation;original_params.generation=generation;
        let mut receipt=RetainedCloneProgress::default();let mut cx=owner.context(StepBudget::new(100,10,supplied),cancel.clone(),||Some(0),&mut sequence,&mut receipt).unwrap();
        let(result,born,freed)=measured(||WorkerJobSession::try_admit_owned(&mut job,&mut params,&mut cx));
        assert!(matches!(result,Err(ref error) if error.kind==semio_framework_value::ValueRefusalKind::InvariantViolated));assert_eq!((born,freed),(0,0));assert_eq!(cx.retained_progress(),RetainedCloneProgress::default());drop(cx);
        assert_eq!(job.as_ref().unwrap()as*const NativeClipboardJob,source_address);assert_eq!(params.as_ref().unwrap()as*const BatchJobParams,params_address);
        let Some(NativeClipboardOperation::Write(text))=job.as_ref().unwrap().operation.as_ref()else{panic!("identity refusal retains original source")};assert_eq!(text.as_ptr(),pointer);assert_eq!(text.capacity(),capacity);assert_eq!(text,&oracle);
        let original_params=params.as_mut().unwrap();original_params.operation=OperationId(1);original_params.generation=Generation(1);
    }
    let(unrelated,born,freed)=measured(||semio_framework_async::CancelToken::admit_root(supplied));let(unrelated,admitted)=unrelated.unwrap().unwrap();assert!(admitted.fits(supplied));assert_eq!((born,freed),(admitted.retained_capacity_bytes,admitted.released_bytes));
    let original_alias=std::mem::replace(&mut params.as_mut().unwrap().cancel,unrelated.clone());let mut original_alias=semio_framework_async::CancelTokenRetirement::from_token(original_alias);
    let(returned,born,freed)=measured(||original_alias.return_alias_step(&cancel,supplied).unwrap());assert!(returned.progress().fits(supplied));assert_eq!((born,freed),(returned.progress().retained_capacity_bytes,returned.progress().released_bytes));assert!(original_alias.terminal_is_empty());
    let mut receipt=RetainedCloneProgress::default();let mut cx=owner.context(StepBudget::new(100,10,supplied),cancel.clone(),||Some(0),&mut sequence,&mut receipt).unwrap();
    let(result,born,freed)=measured(||WorkerJobSession::try_admit_owned(&mut job,&mut params,&mut cx));assert!(matches!(result,Err(ref error) if error.kind==semio_framework_value::ValueRefusalKind::InvariantViolated));assert_eq!((born,freed),(0,0));assert_eq!(cx.retained_progress(),RetainedCloneProgress::default());drop(cx);
    assert_eq!(job.as_ref().unwrap()as*const NativeClipboardJob,source_address);assert_eq!(params.as_ref().unwrap()as*const BatchJobParams,params_address);
    let Some(NativeClipboardOperation::Write(text))=job.as_ref().unwrap().operation.as_ref()else{panic!("original cancellation refusal retains source")};assert_eq!(text.as_ptr(),pointer);assert_eq!(text.capacity(),capacity);assert_eq!(text,&oracle);
    let unrelated_alias=std::mem::replace(&mut params.as_mut().unwrap().cancel,cancel.clone());let mut unrelated_alias=semio_framework_async::CancelTokenRetirement::from_token(unrelated_alias);
    let(returned,born,freed)=measured(||unrelated_alias.return_alias_step(&unrelated,supplied).unwrap());assert!(returned.progress().fits(supplied));assert_eq!((born,freed),(returned.progress().retained_capacity_bytes,returned.progress().released_bytes));assert!(unrelated_alias.terminal_is_empty());
    let mut unrelated=semio_framework_async::CancelTokenRetirement::from_token(unrelated);while !unrelated.terminal_is_empty(){let(closed,born,freed)=measured(||unrelated.close_step(supplied).unwrap());assert!(closed.progress().fits(supplied));assert_eq!((born,freed),(closed.progress().retained_capacity_bytes,closed.progress().released_bytes));}
    for grant in [RetainedCloneGrant{maximum_items:0,..supplied},RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes-1,..supplied},RetainedCloneGrant{maximum_depth:0,..supplied}] {
        let mut receipt=RetainedCloneProgress::default();let mut cx=owner.context(StepBudget::new(100,10,grant),cancel.clone(),||Some(0),&mut sequence,&mut receipt).unwrap();
        let(result,born,freed)=measured(||WorkerJobSession::try_admit_owned(&mut job,&mut params,&mut cx));
        assert!(matches!(result,Ok(None)));assert_eq!((born,freed),(0,0));assert_eq!(cx.retained_progress(),RetainedCloneProgress::default());drop(cx);
        assert_eq!(job.as_ref().unwrap()as*const NativeClipboardJob,source_address);assert_eq!(params.as_ref().unwrap()as*const BatchJobParams,params_address);
        let Some(NativeClipboardOperation::Write(text))=job.as_ref().unwrap().operation.as_ref()else{panic!("denial retained original source")};assert_eq!(text.as_ptr(),pointer);assert_eq!(text.capacity(),capacity);assert_eq!(text,&oracle);
    }
    let mut receipt=RetainedCloneProgress::default();let mut cx=owner.context(StepBudget::new(100,10,supplied),cancel.clone(),||Some(0),&mut sequence,&mut receipt).unwrap();
    let(result,born,freed)=measured(||WorkerJobSession::try_admit_owned(&mut job,&mut params,&mut cx));
    let(mut session,admission)=result.unwrap().expect("original caller independently funds session admission");
    assert!(job.is_none()&&params.is_none());assert_eq!(admission,cx.retained_progress());assert!(admission.fits(supplied));assert_eq!(admission.retained_capacity_bytes,born);assert_eq!(admission.released_bytes,freed);assert_eq!(born,demand.capacity_bytes);assert_eq!(freed,0);drop(cx);
    cancel.cancel_now();let _=session.begin_close();
    for _ in 0..4096 {if session.terminal_is_empty(){break}let(step,born,freed)=measured(||session.close_step(supplied));assert!(step.progress().fits(supplied));assert_eq!(step.progress().retained_capacity_bytes,born);assert_eq!(step.progress().released_bytes,freed);}
    assert!(session.terminal_is_empty());let(_,born,freed)=measured(||drop(session));assert_eq!((born,freed),(0,0));
    let mut cancelled_text=String::with_capacity(capacity);cancelled_text.push_str(&oracle);let cancelled_pointer=cancelled_text.as_ptr();
    let mut cancelled_job=Some(NativeClipboardJob::write(cancelled_text));
    let mut cancelled_params=Some(BatchJobParams{operation:OperationId(1),generation:Generation(1),cancel:cancel.clone(),config:BatchDriveConfig{retained:RetainedCloneGrant::default(),site:"clipboard_original_cancelled_admission",stage:InteractiveStage::InteractiveStep,fuel_per_step:100,step_budget_us:10},now_us:||Some(0)});
    let mut cancelled_receipt=RetainedCloneProgress::default();let mut cx=owner.context(StepBudget::new(100,10,supplied),cancel.clone(),||Some(0),&mut sequence,&mut cancelled_receipt).unwrap();
    let(result,born,freed)=measured(||WorkerJobSession::try_admit_owned(&mut cancelled_job,&mut cancelled_params,&mut cx));
    assert!(matches!(result,Ok(None)));assert_eq!((born,freed),(0,0));assert_eq!(cx.retained_progress(),RetainedCloneProgress::default());drop(cx);
    let Some(NativeClipboardOperation::Write(text))=cancelled_job.as_ref().unwrap().operation.as_ref()else{panic!("original precancelled input")};assert_eq!(text.as_ptr(),cancelled_pointer);assert_eq!(text.capacity(),capacity);assert_eq!(text,&oracle);assert!(cancelled_params.is_some());
    let mut cancelled_job=cancelled_job.take().unwrap();cancelled_job.begin_close();assert_eq!(cancelled_job.close_step(supplied).progress(),RetainedCloneProgress{copied_items:1,released_bytes:capacity,..Default::default()});assert!(cancelled_job.terminal_is_empty());
    while !owner.terminal_is_empty(){assert!(owner.close_step(supplied).progress().fits(supplied))}
    eprintln!("[DEBUG] original clipboard session source/params pointer preserved by prebirth denial; admission uses caller context, ignores config policy, returns exact native heap receipt before exposing session");
}
