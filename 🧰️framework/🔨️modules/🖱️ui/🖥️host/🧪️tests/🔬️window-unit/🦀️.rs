
use super::*;

#[cfg(not(target_arch = "wasm32"))]
use crate::observed_allocator::measured;


#[cfg(not(target_arch="wasm32"))]
const CLIPBOARD_FIXTURE_FUEL:u64=20000;
#[cfg(not(target_arch="wasm32"))]
fn clipboard_fixture_now_us()->Option<u64>{Some(0)}
#[cfg(not(target_arch="wasm32"))]
struct ClipboardCaller{caller:semio_framework_job::OriginalJobFixtureCaller,cancel:semio_framework_job::CancelToken,stock:semio_framework_job::RetainedCloneGrant}
#[cfg(not(target_arch="wasm32"))]
impl ClipboardCaller {
 fn new(operation:semio_framework_job::OperationId,generation:semio_framework_job::Generation,stock:semio_framework_job::RetainedCloneGrant)->Self{
  let caller=semio_framework_job::OriginalJobFixtureCaller::admit(operation,generation,stock,clipboard_fixture_now_us,u64::MAX,CLIPBOARD_FIXTURE_FUEL);let cancel=caller.cancel().clone();
  Self{caller,cancel,stock}
 }
 fn turn<R>(&mut self,grant:semio_framework_job::RetainedCloneGrant,advance:impl FnOnce(&mut semio_framework_job::StepContext<'_>)->Result<R,semio_framework_job::OriginalTurnFailure>)->R{self.caller.bare_turn(grant,advance).expect("original clipboard fixture turn")}
 fn alias(&mut self)->semio_framework_job::CancelToken{self.caller.admit_alias(self.stock)}
 fn finish(self){self.caller.retire(self.stock);}
}

#[cfg(not(target_arch="wasm32"))]
impl ClipboardCaller {
 fn outcome(&mut self,origin:&mut semio_framework_job::OriginalJobReturnOrigin,job:&mut NativeClipboardJob,business:semio_framework_job::RetainedCloneGrant,site:&'static str,heap:&mut (usize,usize))->bool{
  let mut received=false;
  self.caller.refreshed_turn(business,|cx|cx.with_original_work_loan(1,cx.original_absolute_deadline_us(),|cx|{
   if origin.admit(cx)?{
    let receiver=origin.receiver()?;let mut verdict=None;
    let(returned,born,freed)=measured(||semio_framework_job::drive_step(job,cx,receiver,site,semio_framework_job::InteractiveStage::InteractiveStep,&mut verdict));heap.0+=born;heap.1+=freed;
    if let semio_framework_job::OriginalJobReturn::Outcome(outcome)=returned?{let mut input=Some(outcome.into_descriptor());received=origin.receive_outcome(cx,&mut input)?;}
   }
   Ok(())
  })).expect("original clipboard return turn");
  received
 }
 fn acknowledge(&mut self,origin:&mut semio_framework_job::OriginalJobReturnOrigin,business:semio_framework_job::RetainedCloneGrant)->semio_framework_job::RetainedCloneProgress{
  let mut receipt=Default::default();
  self.caller.refreshed_turn(business,|cx|{assert!(origin.acknowledge_outcome(cx)?);receipt=cx.retained_progress();Ok(())}).expect("original clipboard acknowledgement turn");
  receipt
 }
 fn close_origin(&mut self,origin:&mut semio_framework_job::OriginalJobReturnOrigin){for _ in 0..64{if origin.terminal_is_empty(){break}let stock=self.stock;self.caller.refreshed_turn(stock,|cx|{origin.close_step(cx)?;Ok(())}).expect("original clipboard return close turn");}assert!(origin.terminal_is_empty());}
}

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
    fn try_send(&mut self, message: AbiMessage, _budget: AbiWorkBudget) -> Result<(), semio_framework_abi::AbiPortRejection> {
        let mut state = self.0.borrow_mut();
        if state.reject_next {
            state.reject_next = false;
            return Err(semio_framework_abi::AbiPortRejection { code: AbiErrorCode::Interrupted, message });
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
    port.0.borrow_mut().incoming.push_back(AbiMessage::Reply(AbiReply { request_id: AbiRequestId(1), generation: 1, status: semio_framework_abi::AbiStatus::OK, bytes: body }));
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
    AbiMessage::Event(AbiEvent { request_id: AbiRequestId(100 + sequence as u64), generation, sequence, event: semio_framework_abi::AbiEventCode::try_new(code).unwrap(), status: semio_framework_abi::AbiStatus::OK, bytes: AbiBytes::try_new(bytes).unwrap() })
}

fn raw_browser_event(body_bytes: usize) -> AbiMessage {
    AbiMessage::Event(AbiEvent {
        request_id: AbiRequestId(900),
        generation: 1,
        sequence: 1,
        event: semio_framework_abi::AbiEventCode::try_new(crate::event::BROWSER_EVENT_TEXT).unwrap(),
        status: semio_framework_abi::AbiStatus::OK,
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
    port.0.borrow_mut().incoming.push_back(AbiMessage::Reply(AbiReply { request_id: AbiRequestId(1), generation: 1, status: semio_framework_abi::AbiStatus { code: AbiStatusCode::Failed, error: None }, bytes: AbiBytes::try_new(vec![2]).unwrap() }));
    host.step(AbiWorkBudget::credits(1)).unwrap();
    assert_eq!(host.step(AbiWorkBudget::credits(1)).unwrap(), BrowserHostStep::Unavailable(BrowserHostUnavailable::Document));
    for (code, expected) in [(1, BrowserHostUnavailable::Window), (3, BrowserHostUnavailable::Canvas), (4, BrowserHostUnavailable::Clipboard)] {
        let (mut host, port) = browser_fixture();
        while port.0.borrow().sent.is_empty() {
            host.step(AbiWorkBudget::credits(1)).unwrap();
        }
        port.0.borrow_mut().incoming.push_back(AbiMessage::Reply(AbiReply { request_id: AbiRequestId(1), generation: 1, status: semio_framework_abi::AbiStatus { code: AbiStatusCode::Failed, error: None }, bytes: AbiBytes::try_new(vec![code]).unwrap() }));
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
    after_port.0.borrow_mut().incoming.push_back(AbiMessage::Reply(AbiReply { request_id, generation: 1, status: semio_framework_abi::AbiStatus::OK, bytes: AbiBytes::try_new(b"x".to_vec()).unwrap() }));
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
                during_port.0.borrow_mut().incoming.push_back(AbiMessage::Reply(AbiReply { request_id, generation, status: semio_framework_abi::AbiStatus::OK, bytes: AbiBytes::default() }));
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
    port.0.borrow_mut().incoming.push_back(AbiMessage::Control(AbiControl::Close { handle: semio_framework_abi::AbiHandle::try_new(1, 2).unwrap() }));
    assert_eq!(host.step(AbiWorkBudget::credits(1)), Err(BrowserHostError::Abi(AbiErrorCode::StaleGeneration)));
    port.0.borrow_mut().incoming.push_back(AbiMessage::Control(AbiControl::Close { handle: semio_framework_abi::AbiHandle::try_new(2, 1).unwrap() }));
    assert_eq!(host.step(AbiWorkBudget::credits(1)), Err(BrowserHostError::Abi(AbiErrorCode::UnknownHandle)));
}

#[test]
fn browser_linear_memory_exact_envelope_retry_and_preflight_laws() {
    use super::browser::linear_memory_test_import::{self as seam, AfterProbe};

    seam::reset();
    let empty = semio_framework_abi::encode_abi_message(&raw_browser_event(0));
    assert_eq!(empty.len(), BROWSER_HOST_EVENT_ENVELOPE_BYTES);
    seam::enqueue(empty);
    let mut port = LinearMemoryBrowserHostPort;
    assert!(matches!(port.poll(AbiWorkBudget::credits(1)).unwrap(), AbiPortPoll::Message(AbiMessage::Event(event)) if event.bytes.is_empty()));
    assert_eq!(seam::census(), (vec![BROWSER_HOST_INITIAL_POLL_BYTES], 1, 0, false));

    seam::reset();
    let maximum = semio_framework_abi::encode_abi_message(&raw_browser_event(BROWSER_HOST_MAX_EVENT_BODY_BYTES));
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
    let oversized = semio_framework_abi::encode_abi_message(&raw_browser_event(BROWSER_HOST_MAX_EVENT_BODY_BYTES + 1));
    assert_eq!(oversized.len(), BROWSER_HOST_MAX_ENCODED_EVENT_BYTES + 1);
    seam::enqueue(oversized);
    assert_eq!(port.poll(AbiWorkBudget::credits(1)), Err(AbiErrorCode::LimitExceeded));
    assert_eq!(seam::census(), (vec![BROWSER_HOST_INITIAL_POLL_BYTES], 0, 1, false));

    seam::reset();
    let page = AbiPage::try_new(semio_framework_abi::AbiHandle::try_new(1, 1).unwrap(), 0, vec![7; BROWSER_HOST_MAX_PAGE_BODY_BYTES]).unwrap();
    let encoded_page = semio_framework_abi::encode_abi_message(&AbiMessage::Page(page));
    assert_eq!(encoded_page.len(), BROWSER_HOST_MAX_ENCODED_EVENT_BYTES);
    seam::enqueue(encoded_page);
    assert!(matches!(port.poll(AbiWorkBudget::credits(1)).unwrap(), AbiPortPoll::Message(AbiMessage::Page(page)) if page.bytes.len() == BROWSER_HOST_MAX_PAGE_BODY_BYTES));
    assert_eq!(seam::census(), (vec![BROWSER_HOST_INITIAL_POLL_BYTES, BROWSER_HOST_MAX_ENCODED_EVENT_BYTES], 1, 0, false));

    seam::reset();
    let oversized_page = AbiPage::try_new(semio_framework_abi::AbiHandle::try_new(1, 1).unwrap(), 0, vec![7; BROWSER_HOST_MAX_PAGE_BODY_BYTES + 1]).unwrap();
    let encoded_page = semio_framework_abi::encode_abi_message(&AbiMessage::Page(oversized_page));
    assert_eq!(encoded_page.len(), BROWSER_HOST_MAX_ENCODED_EVENT_BYTES + 1);
    seam::enqueue(encoded_page);
    assert_eq!(port.poll(AbiWorkBudget::credits(1)), Err(AbiErrorCode::LimitExceeded));
    assert_eq!(seam::census(), (vec![BROWSER_HOST_INITIAL_POLL_BYTES], 0, 1, false));

    for (action, expected, closed) in [(AfterProbe::Cancel, AbiPortPoll::Pending, false), (AfterProbe::Close, AbiPortPoll::Closed, true)] {
        seam::reset();
        seam::enqueue(semio_framework_abi::encode_abi_message(&raw_browser_event(BROWSER_HOST_MAX_EVENT_BODY_BYTES)));
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

    let handle = semio_framework_abi::AbiHandle::try_new(9, 1).unwrap();
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
 use semio_framework_job::{InteractiveJob,InteractiveJobCloseStep as Close,OperationId,Generation};
 use semio_framework_value::{RetainedCloneGrant as Grant,RetainedCloneProgress as Progress};
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/📋️clipboard-close/🔣️.json")).unwrap();
 for text in fixture["texts"].as_array().unwrap(){
  let caller=&fixture["callerGrant"];let declared=Grant{maximum_items:caller["maximumItems"].as_u64().unwrap()as usize,maximum_copy_bytes:caller["maximumCopyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:caller["maximumCapacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:caller["maximumReleaseBytes"].as_u64().unwrap()as usize,maximum_depth:caller["maximumDepth"].as_u64().unwrap()as usize};assert_eq!(declared,ui_contract::UI_WORKER_RETIREMENT_POLICY);
  let mut owner=ClipboardCaller::new(OperationId(1),Generation(1),declared);
  let mut original=String::with_capacity(fixture["reserve"].as_u64().unwrap()as usize);original.push_str(text.as_str().unwrap());let pointer=original.as_ptr();let capacity=original.capacity();
  let mut job=super::NativeClipboardJob::write(original);job.begin_close();
  let exact=Grant{maximum_items:fixture["closeItems"].as_u64().unwrap()as usize,maximum_copy_bytes:fixture["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:fixture["capacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:capacity,maximum_depth:fixture["closeDepth"].as_u64().unwrap()as usize};
  assert_eq!(job.next_close_copy_byte_demand().unwrap(),0);assert_eq!(job.next_close_capacity_byte_demand(0).unwrap(),0);assert_eq!(job.next_close_release_byte_demand().unwrap(),capacity);assert_eq!(job.next_close_depth_demand().unwrap(),1);
  for short in [Grant{maximum_items:0,..exact},Grant{maximum_release_bytes:capacity-1,..exact},Grant{maximum_depth:0,..exact}]{
   owner.turn(short,|cx|{let(step,born,freed)=measured(||job.close_step(cx).unwrap());assert_eq!(step,Close::Pending{progress:Progress::default()});assert_eq!(cx.retained_progress(),Progress::default());assert_eq!((born,freed),(0,0));Ok(())});
   let Some(super::NativeClipboardOperation::Write(text))=job.operation.as_ref() else{panic!("original clipboard owner survived refusal")};assert_eq!(text.as_ptr(),pointer);assert_eq!(text.capacity(),capacity);
  }
  let progress=Progress{copied_items:1,released_bytes:capacity,..Progress::default()};
  owner.turn(exact,|cx|{let(step,born,freed)=measured(||job.close_step(cx).unwrap());assert_eq!(step,Close::Complete{progress});assert_eq!(cx.retained_progress(),progress);assert_eq!((born,freed),(0,capacity));assert!(progress.fits(exact));assert!(job.terminal_is_empty());assert_eq!(job.close_step(cx).unwrap(),Close::Complete{progress:Progress::default()});Ok(())});
  owner.finish();
 }
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn cancelled_clipboard_outcome_preserves_original_input_until_descriptor_ack_and_paid_close() {
    use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep as Step, RetainedCloneGrant, RetainedCloneProgress, OperationId, Generation, JobOutcomeKind, JobOutcomeView, OriginalJobReturnOrigin};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/♻️physical-job-close/🧫️fixtures/🔣️.json")).unwrap();
    let law = &fixture["clipboardOutcome"];
    let original: serde_json::Value = serde_json::from_str(include_str!("../../../../🧵️job/📬️outcome/🤝️loan/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let policy = &original["policy"];
    let supplied = RetainedCloneGrant { maximum_items: policy["items"].as_u64().unwrap() as usize, maximum_copy_bytes: policy["copy"].as_u64().unwrap() as usize, maximum_capacity_bytes: policy["capacity"].as_u64().unwrap() as usize, maximum_release_bytes: policy["release"].as_u64().unwrap() as usize, maximum_depth: policy["depth"].as_u64().unwrap() as usize };
    let mut owner = ClipboardCaller::new(OperationId(1), Generation(1), supplied);
    let mut source = String::with_capacity(fixture["capacities"][1].as_u64().unwrap() as usize);
    source.push_str(fixture["texts"][1].as_str().unwrap());
    let pointer = source.as_ptr();
    let capacity = source.capacity();
    let mut job = NativeClipboardJob::write(source);
    owner.cancel.cancel_now();
    owner.caller.admit_return_table();
    let mut origin = OriginalJobReturnOrigin::empty();
    let mut heap = (0, 0);
    let denied = RetainedCloneGrant { maximum_items: law["deniedItems"].as_u64().unwrap() as usize, ..supplied };
    assert!(!owner.outcome(&mut origin, &mut job, denied, "original_clipboard_cancelled_outcome", &mut heap));
    assert_eq!(heap, (0, 0));
    let mut received = false;
    for _ in 0..64 { received = owner.outcome(&mut origin, &mut job, supplied, "original_clipboard_cancelled_outcome", &mut heap); if received { break; } }
    assert!(received, "original cancelled outcome is admitted");
    assert_eq!(heap, (0, 0));
    {
        let descriptor = origin.outcome().expect("original cancelled outcome is held by its receiver");
        assert_eq!(descriptor.kind(), JobOutcomeKind::Cancelled);
        let (view, born, freed) = measured(|| job.borrow_outcome(descriptor).unwrap());
        let JobOutcomeView::Cancelled { admission } = view else { panic!("original cancelled descriptor") };
        assert!(std::ptr::eq(admission, descriptor.admission()));
        assert_eq!((born, freed), (law["borrowHeapBytes"].as_u64().unwrap() as usize, 0));
    }
    let Some(NativeClipboardOperation::Write(text)) = job.operation.as_ref() else { panic!("original write input retained through cancellation") };
    assert_eq!(text.as_ptr(), pointer);
    assert_eq!(text.capacity(), capacity);
    assert_eq!(serde_json::to_value(text).unwrap(), fixture["texts"][1]);
    assert!(!job.terminal_is_empty());
    assert_eq!(owner.acknowledge(&mut origin, supplied).copied_items, law["acknowledgementItems"].as_u64().unwrap() as usize);
    assert!(origin.outcome().is_none());
    job.begin_close();
    let close_grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 0, maximum_capacity_bytes: 0, maximum_release_bytes: capacity, maximum_depth: 1 };
    owner.turn(close_grant, |cx| {
        let (closed, born, freed) = measured(|| job.close_step(cx).unwrap());
        assert_eq!(closed.progress(), cx.retained_progress());
        assert!(matches!(closed, Step::Complete { .. }));
        assert_eq!((born, freed), (0, capacity));
        assert_eq!(closed.progress(), RetainedCloneProgress { copied_items: 1, released_bytes: capacity, ..Default::default() });
        Ok(())
    });
    assert!(job.terminal_is_empty());
    owner.close_origin(&mut origin);
    drop(origin);
    owner.finish();
}

#[cfg(not(target_arch="wasm32"))]
#[test]
fn cancelled_clipboard_driver_binds_the_original_producer_witness(){
    use semio_framework_job::{InteractiveJob,OperationId,Generation,JobOutcomeView,RetainedCloneGrant,RetainedCloneProgress,OriginalJobReturnOrigin};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🧪️tests/♻️physical-job-close/🧫️fixtures/🔣️.json")).unwrap();
    let original:serde_json::Value=serde_json::from_str(include_str!("../../../../🧵️job/📬️outcome/🤝️loan/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let policy=&original["policy"];
    let grant=RetainedCloneGrant{maximum_items:policy["items"].as_u64().unwrap()as usize,maximum_copy_bytes:policy["copy"].as_u64().unwrap()as usize,maximum_capacity_bytes:policy["capacity"].as_u64().unwrap()as usize,maximum_release_bytes:policy["release"].as_u64().unwrap()as usize,maximum_depth:policy["depth"].as_u64().unwrap()as usize};
    let mut owner=ClipboardCaller::new(OperationId(1),Generation(1),grant);
    let mut text=String::with_capacity(fixture["capacities"][1].as_u64().unwrap()as usize);text.push_str(fixture["texts"][1].as_str().unwrap());let capacity=text.capacity();
    let mut job=super::NativeClipboardJob::write(text);
    owner.cancel.cancel_now();owner.caller.admit_return_table();
    let mut origin=OriginalJobReturnOrigin::empty();let mut heap=(0,0);let mut received=false;
    for _ in 0..64{received=owner.outcome(&mut origin,&mut job,grant,"original_clipboard_cancelled_bridge",&mut heap);if received{break}}
    assert!(received,"original cancelled bridge outcome is admitted");
    {let(view,born,freed)=measured(||job.borrow_outcome(origin.outcome().expect("original cancelled bridge outcome is held")).unwrap());assert!(matches!(view,JobOutcomeView::Cancelled{..}));assert_eq!((born,freed),(0,0));}
    assert_eq!(owner.acknowledge(&mut origin,grant).copied_items,1);
    job.begin_close();let close_grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:capacity,maximum_depth:1};
    owner.turn(close_grant,|cx|{let closed=job.close_step(cx).unwrap();assert_eq!(closed.progress(),RetainedCloneProgress{copied_items:1,released_bytes:capacity,..Default::default()});assert_eq!(closed.progress(),cx.retained_progress());Ok(())});
    assert!(job.terminal_is_empty());owner.close_origin(&mut origin);drop(origin);
    owner.finish();
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn original_clipboard_session_admission_is_paid_before_birth_and_preserves_denied_owners() {
    use semio_framework_job::{Generation,InteractiveJob,InteractiveStage,OperationId,RetainedCloneGrant,RetainedCloneProgress};
    use semio_framework_job_worker::{BatchJobSession,WorkerJobSessionAdmission};use semio_framework_job_worker_authority::{BatchDriveConfig,BatchJobParams};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🧪️tests/♻️physical-job-close/🧫️fixtures/🔣️.json")).unwrap();
    let original:serde_json::Value=serde_json::from_str(include_str!("../../../../🧵️job/📬️outcome/🤝️loan/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let policy=&original["policy"];
    let supplied=RetainedCloneGrant{maximum_items:policy["items"].as_u64().unwrap()as usize,maximum_copy_bytes:policy["copy"].as_u64().unwrap()as usize,maximum_capacity_bytes:policy["capacity"].as_u64().unwrap()as usize,maximum_release_bytes:policy["release"].as_u64().unwrap()as usize,maximum_depth:policy["depth"].as_u64().unwrap()as usize};
    let mut owner=ClipboardCaller::new(OperationId(1),Generation(1),supplied);
    let mut text=String::with_capacity(fixture["capacities"][1].as_u64().unwrap()as usize);text.push_str(fixture["texts"][1].as_str().unwrap());
    let pointer=text.as_ptr();let capacity=text.capacity();let oracle:String=serde_json::from_value(fixture["texts"][1].clone()).unwrap();
    let mut job=Some(NativeClipboardJob::write(text));
    let params_for=|cancel:semio_framework_job::CancelToken,operation:OperationId,generation:Generation,site:&'static str|BatchJobParams{operation,generation,cancel,config:BatchDriveConfig{retained:RetainedCloneGrant::default(),site,stage:InteractiveStage::InteractiveStep,fuel_per_step:100,step_budget_us:10},now_us:clipboard_fixture_now_us};
    let mut params=Some(params_for(owner.alias(),OperationId(1),Generation(1),"clipboard_original_owned_admission"));
    let source_address=job.as_ref().unwrap()as*const NativeClipboardJob;let params_address=params.as_ref().unwrap()as*const BatchJobParams;
    let mut admission=WorkerJobSessionAdmission::<NativeClipboardJob,BatchJobSession<NativeClipboardJob>>::new();let mut output:Option<BatchJobSession<NativeClipboardJob>>=None;
    let retained=|job:&Option<NativeClipboardJob>|{let Some(NativeClipboardOperation::Write(text))=job.as_ref().unwrap().operation.as_ref()else{panic!("refusal retains original source")};assert_eq!(text.as_ptr(),pointer);assert_eq!(text.capacity(),capacity);assert_eq!(text,&oracle);};
    let advance=|owner:&mut ClipboardCaller,admission:&mut WorkerJobSessionAdmission<NativeClipboardJob,BatchJobSession<NativeClipboardJob>>,job:&mut Option<NativeClipboardJob>,params:&mut Option<BatchJobParams>,output:&mut Option<BatchJobSession<NativeClipboardJob>>,grant:RetainedCloneGrant|owner.turn(grant,|cx|{let(result,born,freed)=measured(||admission.advance(job,params,output,cx));Ok((result,born,freed,cx.retained_progress()))});
    let(first,born,freed,receipt)=advance(&mut owner,&mut admission,&mut job,&mut params,&mut output,supplied);assert!(!first.expect("source binding turn"));assert_eq!((born,freed),(receipt.retained_capacity_bytes,receipt.released_bytes));
    for (operation,generation) in [(OperationId(2),Generation(1)),(OperationId(1),Generation(2))] {
        let original_params=params.as_mut().unwrap();original_params.operation=operation;original_params.generation=generation;
        let(result,born,freed,receipt)=advance(&mut owner,&mut admission,&mut job,&mut params,&mut output,supplied);
        assert!(matches!(result,Err(_)));assert_eq!((born,freed),(0,0));assert_eq!(receipt,RetainedCloneProgress::default());
        assert_eq!(job.as_ref().unwrap()as*const NativeClipboardJob,source_address);assert_eq!(params.as_ref().unwrap()as*const BatchJobParams,params_address);retained(&job);
        let original_params=params.as_mut().unwrap();original_params.operation=OperationId(1);original_params.generation=Generation(1);
    }
    let stock=owner.stock;let(unrelated,admitted)=owner.turn(stock,|cx|{let(root,born,freed)=measured(||semio_framework_async::CancelToken::admit_root(supplied));let(root,admitted)=root.result.unwrap().unwrap();assert!(admitted.fits(supplied));assert_eq!((born,freed),(admitted.retained_capacity_bytes,admitted.released_bytes));let _=cx;Ok((root,admitted))});let _=admitted;
    let original_alias=std::mem::replace(&mut params.as_mut().unwrap().cancel,unrelated.clone());let mut original_alias=semio_framework_async::CancelTokenRetirement::from_token(original_alias);let witness=owner.cancel.clone();
    owner.turn(supplied,|cx|{let(returned,born,freed)=measured(||cx.with_original_effect(supplied,|_,effect|original_alias.return_alias_step(&witness,effect).result.unwrap()).unwrap().expect("original effect funded").0);assert!(returned.progress().fits(supplied));assert_eq!((born,freed),(returned.progress().retained_capacity_bytes,returned.progress().released_bytes));Ok(())});assert!(original_alias.terminal_is_empty());
    let(result,born,freed,receipt)=advance(&mut owner,&mut admission,&mut job,&mut params,&mut output,supplied);assert!(matches!(result,Err(_)));assert_eq!((born,freed),(0,0));assert_eq!(receipt,RetainedCloneProgress::default());
    assert_eq!(job.as_ref().unwrap()as*const NativeClipboardJob,source_address);assert_eq!(params.as_ref().unwrap()as*const BatchJobParams,params_address);retained(&job);
    let unrelated_alias=std::mem::replace(&mut params.as_mut().unwrap().cancel,witness.clone());let mut unrelated_alias=semio_framework_async::CancelTokenRetirement::from_token(unrelated_alias);
    owner.turn(supplied,|cx|{let(returned,born,freed)=measured(||cx.with_original_effect(supplied,|_,effect|unrelated_alias.return_alias_step(&unrelated,effect).result.unwrap()).unwrap().expect("original effect funded").0);assert!(returned.progress().fits(supplied));assert_eq!((born,freed),(returned.progress().retained_capacity_bytes,returned.progress().released_bytes));Ok(())});assert!(unrelated_alias.terminal_is_empty());
    let mut unrelated=semio_framework_async::CancelTokenRetirement::from_token(unrelated);
    while !unrelated.terminal_is_empty(){owner.turn(supplied,|cx|{let(closed,born,freed)=measured(||cx.with_original_effect(supplied,|_,effect|unrelated.close_step(effect).result.unwrap()).unwrap().expect("original effect funded").0);assert!(closed.progress().fits(supplied));assert_eq!((born,freed),(closed.progress().retained_capacity_bytes,closed.progress().released_bytes));Ok(())});}
    for denied in [RetainedCloneGrant{maximum_items:0,..supplied},RetainedCloneGrant{maximum_depth:0,..supplied}] {
        let(result,born,freed,receipt)=advance(&mut owner,&mut admission,&mut job,&mut params,&mut output,denied);
        assert!(matches!(result,Ok(false)));assert_eq!((born,freed),(0,0));assert_eq!(receipt,RetainedCloneProgress::default());
        assert_eq!(job.as_ref().unwrap()as*const NativeClipboardJob,source_address);assert_eq!(params.as_ref().unwrap()as*const BatchJobParams,params_address);retained(&job);
    }
    let mut turns=0;while output.is_none(){let(result,born,freed,receipt)=advance(&mut owner,&mut admission,&mut job,&mut params,&mut output,supplied);result.expect("original clipboard session admission turn");assert!(receipt.fits(supplied));assert_eq!((born,freed),(receipt.retained_capacity_bytes,receipt.released_bytes));turns+=1;assert!(turns<4096);}
    assert!(admission.terminal_is_empty()&&job.is_none()&&params.is_none());let mut session=output.take().unwrap();
    let cancelled_alias=owner.alias();owner.cancel.cancel_now();let _=session.begin_close();
    for _ in 0..4096 {if session.terminal_is_empty(){break}owner.turn(supplied,|cx|{let(step,born,freed)=measured(||{if session.has_original_cancel_alias_witness(cx.original_cancel_token()).unwrap(){let step=cx.with_original_effect(cx.retained_grant(),|cx,effect|session.return_original_cancel_alias_step(cx.original_cancel_token(),effect))?.map_or(semio_framework_value::RetainedWorkOutcome{result:Ok(None),work:0},|(turn,_)|turn).result.unwrap().unwrap();cx.consume_retained(step.progress()).unwrap();cx.consume_fuel(1);Ok::<_,semio_framework_job::OriginalTurnFailure>(semio_framework_job_worker_authority::WorkerJobCloseStep::Pending{progress:step.progress()})}else{session.close_step(cx)}});let step=step?;assert!(step.progress().fits(supplied));assert_eq!(step.progress().retained_capacity_bytes,born);assert_eq!(step.progress().released_bytes,freed);Ok(())});}
    assert!(session.terminal_is_empty());let(_,born,freed)=measured(||drop(session));assert_eq!((born,freed),(0,0));
    let mut cancelled_text=String::with_capacity(capacity);cancelled_text.push_str(&oracle);let cancelled_pointer=cancelled_text.as_ptr();
    let mut cancelled_job=Some(NativeClipboardJob::write(cancelled_text));
    let mut cancelled_params=Some(params_for(cancelled_alias,OperationId(1),Generation(1),"clipboard_original_cancelled_admission"));
    let mut cancelled_admission=WorkerJobSessionAdmission::<NativeClipboardJob,BatchJobSession<NativeClipboardJob>>::new();let mut cancelled_output=None;
    let(result,born,freed,receipt)=advance(&mut owner,&mut cancelled_admission,&mut cancelled_job,&mut cancelled_params,&mut cancelled_output,supplied);assert!(matches!(result,Ok(false)));assert_eq!((born,freed),(receipt.retained_capacity_bytes,receipt.released_bytes));
    let(result,born,freed,receipt)=advance(&mut owner,&mut cancelled_admission,&mut cancelled_job,&mut cancelled_params,&mut cancelled_output,supplied);assert!(matches!(result,Ok(false)));assert_eq!((born,freed),(0,0));assert_eq!(receipt,RetainedCloneProgress::default());assert!(cancelled_output.is_none());
    let Some(NativeClipboardOperation::Write(text))=cancelled_job.as_ref().unwrap().operation.as_ref()else{panic!("original precancelled input")};assert_eq!(text.as_ptr(),cancelled_pointer);assert_eq!(text.capacity(),capacity);assert_eq!(text,&oracle);assert!(cancelled_params.is_some());
    for _ in 0..4096{if cancelled_admission.terminal_is_empty(){break}owner.turn(supplied,|cx|{cancelled_admission.close_step(cx)?;Ok(())});}assert!(cancelled_admission.terminal_is_empty());
    let mut cancelled_job=cancelled_job.take().unwrap();cancelled_job.begin_close();owner.turn(RetainedCloneGrant{maximum_items:1,..supplied},|cx|{let step=cancelled_job.close_step(cx).unwrap();assert_eq!(step.progress(),RetainedCloneProgress{copied_items:1,released_bytes:capacity,..Default::default()});assert_eq!(step.progress(),cx.retained_progress());Ok(())});assert!(cancelled_job.terminal_is_empty());
    let params=cancelled_params.take().unwrap();let mut alias=semio_framework_async::CancelTokenRetirement::from_token(params.cancel);let witness=owner.cancel.clone();while !alias.terminal_is_empty(){owner.turn(supplied,|cx|{assert!(cx.with_original_effect(supplied,|_,effect|alias.return_alias_step(&witness,effect).result.unwrap()).unwrap().expect("original effect funded").0.progress().fits(supplied));Ok(())});}
    owner.finish();
}
