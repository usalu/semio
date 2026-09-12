//! 🧬️ Flow editor reactive features over the owned A1 byte/message ABI.

use semio_framework::abi::{
    AbiBytes, AbiControl, AbiCursorStep, AbiError, AbiErrorCode, AbiEvent, AbiEventCode, AbiHandle, AbiHandleTable, AbiMessage, AbiMessageBytes, AbiPage, AbiPageReader, AbiPort, AbiPortPoll, AbiPortRejection, AbiReply, AbiReplyLedger,
    AbiRequest, AbiRequestId, AbiStatus, AbiStatusCode, AbiWorkBudget, ABI_MAX_BODY_BYTES, ABI_MAX_IN_FLIGHT_HANDLES, ABI_MAX_IN_FLIGHT_REQUESTS, ABI_MAX_MESSAGE_BYTES, ABI_MAX_TRANSFER_BYTES,
};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

//#region 🧬️Contract

pub const FLOW_ABI_SCHEMA: &str = include_str!("../🧬️schema/📡️abi.json");

pub const FLOW_OPERATION_FIRST: u16 = 2_500;
pub const FLOW_OPERATION_OPEN: u16 = 2_500;
pub const FLOW_OPERATION_ATTACH_SURFACE: u16 = 2_575;
pub const FLOW_OPERATION_SURFACE_STATUS: u16 = 2_576;
pub const FLOW_OPERATION_RENDER_FRAME: u16 = 2_581;
pub const FLOW_OPERATION_DISPOSE: u16 = 2_607;
pub const FLOW_OPERATION_LAST: u16 = 2_610;

pub const FLOW_EVENT_ADMITTED: u16 = 2_650;
pub const FLOW_EVENT_PROGRESS: u16 = 2_651;
pub const FLOW_EVENT_CHECKPOINT: u16 = 2_652;
pub const FLOW_EVENT_PREVIEW: u16 = 2_653;
pub const FLOW_EVENT_SURFACE_STATUS: u16 = 2_654;
pub const FLOW_EVENT_OUTPUT: u16 = 2_655;
pub const FLOW_EVENT_TERMINAL: u16 = 2_656;
pub const FLOW_EVENT_SESSION_TERMINAL: u16 = 2_657;

pub const FLOW_MAX_REQUEST_BYTES: usize = ABI_MAX_BODY_BYTES;
pub const FLOW_MAX_INLINE_REPLY_BYTES: usize = ABI_MAX_MESSAGE_BYTES;
pub const FLOW_MAX_OUTPUT_BYTES: usize = ABI_MAX_TRANSFER_BYTES;
pub const FLOW_MAX_RESOURCES: usize = ABI_MAX_IN_FLIGHT_HANDLES;
pub const FLOW_MAX_REQUESTS: usize = ABI_MAX_IN_FLIGHT_REQUESTS;
pub const FLOW_MAX_OUTBOUND: usize = 64;
pub const FLOW_MAX_EVENTS_IN_FLIGHT: usize = 64;
pub const FLOW_MAX_WORK_UNITS: usize = 64;
pub const FLOW_DEADLINE_MILLISECONDS: u64 = 8;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FlowFailure {
    pub code: AbiErrorCode,
    pub message: String,
}

impl FlowFailure {
    pub fn new(code: AbiErrorCode, message: impl Into<String>) -> Self {
        Self { code, message: message.into() }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FlowFeatureStep {
    Yield,
    Progress { completed: u64, total: u64 },
    Checkpoint(Vec<u8>),
    Preview(Vec<u8>),
    SurfaceStatus(Vec<u8>),
    RetainedPage(Vec<u8>),
    Complete(Vec<u8>),
    Failed(FlowFailure),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowFeatureAdmission {
    pub session: AbiHandle,
    pub request_generation: u32,
}

pub trait FlowFeature {
    fn step(&mut self, budget: AbiWorkBudget) -> FlowFeatureStep;
    fn cancel(&mut self, budget: AbiWorkBudget) -> Result<(), FlowFailure>;

    fn preflight_acknowledge(&self, _: u32) -> Result<(), FlowFailure> {
        Err(FlowFailure::new(AbiErrorCode::UnknownHandle, "feature has no retained page"))
    }

    fn acknowledge(&mut self, _: u32, _: AbiWorkBudget) -> Result<(), FlowFailure> {
        Err(FlowFailure::new(AbiErrorCode::UnknownHandle, "feature has no retained page"))
    }

    fn close_step(&mut self, _: AbiWorkBudget) -> Result<bool, FlowFailure> {
        Ok(true)
    }
}

pub trait FlowDomain: Sized + 'static {
    fn bind_session(&mut self, _: AbiHandle) {}

    fn start_feature(domain: Rc<RefCell<Self>>, admission: FlowFeatureAdmission, operation: u16, payload: Vec<u8>) -> Result<Box<dyn FlowFeature>, FlowFailure>;
    fn begin_close(&mut self);
    fn close_step(&mut self, budget: AbiWorkBudget) -> Result<bool, FlowFailure>;
    fn terminal_is_empty(&self) -> bool;
}

//#endregion 🧬️Contract

//#region 🧱️PayloadCodec

pub struct FlowPayloadReader<'a> {
    bytes: &'a [u8],
    cursor: usize,
}

impl<'a> FlowPayloadReader<'a> {
    pub const fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, cursor: 0 }
    }

    pub fn u8(&mut self) -> Result<u8, AbiErrorCode> {
        Ok(self.take(1)?[0])
    }

    pub fn u32(&mut self) -> Result<u32, AbiErrorCode> {
        Ok(u32::from_le_bytes(self.take(4)?.try_into().map_err(|_| AbiErrorCode::MalformedLength)?))
    }

    pub fn u64(&mut self) -> Result<u64, AbiErrorCode> {
        Ok(u64::from_le_bytes(self.take(8)?.try_into().map_err(|_| AbiErrorCode::MalformedLength)?))
    }

    pub fn f64(&mut self) -> Result<f64, AbiErrorCode> {
        let value = f64::from_le_bytes(self.take(8)?.try_into().map_err(|_| AbiErrorCode::MalformedLength)?);
        value.is_finite().then_some(value).ok_or(AbiErrorCode::MalformedTag)
    }

    pub fn bool(&mut self) -> Result<bool, AbiErrorCode> {
        match self.u8()? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(AbiErrorCode::MalformedTag),
        }
    }

    pub fn bytes(&mut self) -> Result<&'a [u8], AbiErrorCode> {
        let length = self.u32()? as usize;
        self.take(length)
    }

    pub fn text(&mut self) -> Result<&'a str, AbiErrorCode> {
        std::str::from_utf8(self.bytes()?).map_err(|_| AbiErrorCode::InvalidUtf8)
    }

    pub fn optional_text(&mut self) -> Result<Option<&'a str>, AbiErrorCode> {
        match self.u8()? {
            0 => Ok(None),
            1 => self.text().map(Some),
            _ => Err(AbiErrorCode::MalformedTag),
        }
    }

    pub fn handle(&mut self) -> Result<AbiHandle, AbiErrorCode> {
        AbiHandle::try_new(self.u32()?, self.u32()?)
    }

    pub fn finish(self) -> Result<(), AbiErrorCode> {
        (self.cursor == self.bytes.len()).then_some(()).ok_or(AbiErrorCode::MalformedLength)
    }

    fn take(&mut self, length: usize) -> Result<&'a [u8], AbiErrorCode> {
        let end = self.cursor.checked_add(length).ok_or(AbiErrorCode::MalformedLength)?;
        let value = self.bytes.get(self.cursor..end).ok_or(AbiErrorCode::MalformedLength)?;
        self.cursor = end;
        Ok(value)
    }
}

#[derive(Default)]
pub struct FlowPayloadWriter {
    bytes: Vec<u8>,
}

impl FlowPayloadWriter {
    pub fn u8(&mut self, value: u8) {
        self.bytes.push(value);
    }

    pub fn u32(&mut self, value: u32) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    pub fn u64(&mut self, value: u64) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    pub fn f64(&mut self, value: f64) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    pub fn handle(&mut self, value: AbiHandle) {
        self.u32(value.slot());
        self.u32(value.generation());
    }

    pub fn bytes(&mut self, value: &[u8]) -> Result<(), AbiErrorCode> {
        self.u32(u32::try_from(value.len()).map_err(|_| AbiErrorCode::LimitExceeded)?);
        self.bytes.extend_from_slice(value);
        Ok(())
    }

    pub fn finish(self) -> Vec<u8> {
        self.bytes
    }
}

//#endregion 🧱️PayloadCodec

//#region 🔖️Runtime

struct FlowSession<D> {
    domain: Rc<RefCell<D>>,
    open_request: AbiRequestId,
    open_generation: u32,
    closed: bool,
    domain_close_started: bool,
    domain_terminal: bool,
}

struct FlowOperation {
    session_handle: AbiHandle,
    request_id: AbiRequestId,
    generation: u32,
    feature: Box<dyn FlowFeature>,
    reader: Option<AbiPageReader>,
    retained_page: Option<AbiPage>,
    retained_page_emitted: bool,
    cancelled: bool,
}

enum FlowResource<D> {
    Session(Rc<RefCell<FlowSession<D>>>),
    Operation(FlowOperation),
}

#[derive(Clone, Copy)]
struct RequestEntry {
    request_id: AbiRequestId,
    generation: u32,
    operation: AbiHandle,
}

#[derive(Clone, Copy)]
struct EventEntry {
    acknowledgement: AbiRequestId,
    origin: AbiRequestId,
    generation: u32,
    code: u16,
}

pub struct FlowBridge<D: FlowDomain> {
    factory: fn() -> D,
    resources: AbiHandleTable<FlowResource<D>>,
    work: VecDeque<AbiHandle>,
    outbound: VecDeque<AbiMessage>,
    request_ledger: AbiReplyLedger,
    requests: [Option<RequestEntry>; FLOW_MAX_REQUESTS],
    events: [Option<EventEntry>; FLOW_MAX_REQUESTS],
    sessions: Vec<AbiHandle>,
    event_count: usize,
    active_resources: usize,
    next_event_sequence: u32,
    closing: bool,
}

impl<D: FlowDomain> FlowBridge<D> {
    pub fn new(factory: fn() -> D) -> Self {
        Self {
            factory,
            resources: AbiHandleTable::new(),
            work: VecDeque::new(),
            outbound: VecDeque::new(),
            request_ledger: AbiReplyLedger::new(),
            requests: [None; FLOW_MAX_REQUESTS],
            events: [None; FLOW_MAX_REQUESTS],
            sessions: Vec::new(),
            event_count: 0,
            active_resources: 0,
            next_event_sequence: 1,
            closing: false,
        }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.closing && self.active_resources == 0 && self.work.is_empty() && self.outbound.is_empty() && self.event_count == 0
    }

    pub fn begin_close(&mut self) {
        if self.closing {
            return;
        }
        self.closing = true;
        for handle in self.sessions.clone() {
            let _ = self.close_session(handle);
        }
    }

    fn accept_request(&mut self, request: AbiRequest) -> Result<(), AbiPortRejection> {
        let returned = || AbiMessage::Request(request.clone());
        if self.closing {
            return Err(AbiPortRejection { code: AbiErrorCode::Closed, message: returned() });
        }
        if request.generation == 0 || request.bytes.len() > FLOW_MAX_REQUEST_BYTES {
            return Err(AbiPortRejection { code: AbiErrorCode::LimitExceeded, message: returned() });
        }
        let code = request.operation.get();
        if !(FLOW_OPERATION_FIRST..=FLOW_OPERATION_LAST).contains(&code) || matches!(code, 2_603 | 2_604 | 2_608) {
            return Err(AbiPortRejection { code: AbiErrorCode::UnknownOperation, message: returned() });
        }
        if code == FLOW_OPERATION_OPEN {
            return self.accept_open(request);
        }
        if request.bytes.len() < 8 || self.active_resources == FLOW_MAX_RESOURCES || self.requests[request_slot(request.request_id)].is_some() {
            return Err(AbiPortRejection { code: AbiErrorCode::LimitExceeded, message: returned() });
        }
        self.preflight_outbound(2).map_err(|code| AbiPortRejection { code, message: returned() })?;
        let mut reader = FlowPayloadReader::new(request.bytes.as_slice());
        let session_handle = reader.handle().map_err(|code| AbiPortRejection { code, message: returned() })?;
        let session = match self.resources.get(session_handle) {
            Ok(FlowResource::Session(session)) if !session.borrow().closed => session.clone(),
            Ok(FlowResource::Session(_)) => return Err(AbiPortRejection { code: AbiErrorCode::Closed, message: returned() }),
            Ok(FlowResource::Operation(_)) => return Err(AbiPortRejection { code: AbiErrorCode::UnknownHandle, message: returned() }),
            Err(code) => return Err(AbiPortRejection { code, message: returned() }),
        };
        let payload = request.bytes.as_slice()[8..].to_vec();
        let admission = FlowFeatureAdmission { session: session_handle, request_generation: request.generation };
        let feature = D::start_feature(session.borrow().domain.clone(), admission, code, payload).map_err(|failure| AbiPortRejection { code: failure.code, message: returned() })?;
        self.request_ledger.admit(request.request_id, request.generation).map_err(|code| AbiPortRejection { code, message: returned() })?;
        let operation = FlowOperation { session_handle, request_id: request.request_id, generation: request.generation, feature, reader: None, retained_page: None, retained_page_emitted: false, cancelled: false };
        let handle = self.resources.open(FlowResource::Operation(operation)).map_err(|(code, _)| AbiPortRejection { code, message: returned() })?;
        self.active_resources += 1;
        self.requests[request_slot(request.request_id)] = Some(RequestEntry { request_id: request.request_id, generation: request.generation, operation: handle });
        self.work.push_back(handle);
        let mut body = FlowPayloadWriter::default();
        body.handle(handle);
        self.push_event(request.request_id, request.generation, FLOW_EVENT_ADMITTED, AbiStatus::OK, body.finish(), false).map_err(|code| AbiPortRejection { code, message: returned() })
    }

    fn accept_open(&mut self, request: AbiRequest) -> Result<(), AbiPortRejection> {
        let returned = || AbiMessage::Request(request.clone());
        if !request.bytes.is_empty() || self.active_resources == FLOW_MAX_RESOURCES {
            return Err(AbiPortRejection { code: AbiErrorCode::LimitExceeded, message: returned() });
        }
        self.preflight_outbound(1).map_err(|code| AbiPortRejection { code, message: returned() })?;
        self.request_ledger.admit(request.request_id, request.generation).map_err(|code| AbiPortRejection { code, message: returned() })?;
        let domain = Rc::new(RefCell::new((self.factory)()));
        let session = Rc::new(RefCell::new(FlowSession { domain: domain.clone(), open_request: request.request_id, open_generation: request.generation, closed: false, domain_close_started: false, domain_terminal: false }));
        let handle = self.resources.open(FlowResource::Session(session)).map_err(|(code, _)| AbiPortRejection { code, message: returned() })?;
        domain.borrow_mut().bind_session(handle);
        self.active_resources += 1;
        self.sessions.push(handle);
        let mut body = FlowPayloadWriter::default();
        body.handle(handle);
        self.push_outbound(AbiMessage::Reply(success_reply(request.request_id, request.generation, body.finish()))).map_err(|code| AbiPortRejection { code, message: returned() })?;
        self.request_ledger.accept(&success_reply(request.request_id, request.generation, Vec::new())).map_err(|code| AbiPortRejection { code, message: returned() })
    }

    fn accept_control(&mut self, control: AbiControl, budget: AbiWorkBudget) -> Result<(), AbiErrorCode> {
        match control {
            AbiControl::Cancel { request_id, generation } => {
                let entry = self.requests[request_slot(request_id)].ok_or(AbiErrorCode::UnknownHandle)?;
                if entry.request_id != request_id {
                    return Err(AbiErrorCode::UnknownHandle);
                }
                if generation < entry.generation {
                    return Err(AbiErrorCode::AbaHandle);
                }
                if generation > entry.generation {
                    return Err(AbiErrorCode::StaleGeneration);
                }
                let FlowResource::Operation(operation) = self.resources.get_mut(entry.operation)? else {
                    return Err(AbiErrorCode::UnknownHandle);
                };
                operation.feature.cancel(budget).map_err(|failure| failure.code)?;
                operation.cancelled = true;
                if let Some(reader) = operation.reader.as_mut() {
                    reader.cancel();
                }
                self.outbound.retain(|message| !matches!(message, AbiMessage::Page(page) if page.handle == entry.operation));
                Ok(())
            }
            AbiControl::Acknowledge { handle, index } => {
                let FlowResource::Operation(operation) = self.resources.get_mut(handle)? else {
                    return Err(AbiErrorCode::UnknownHandle);
                };
                if let Some(page) = operation.retained_page.as_ref() {
                    if page.handle != handle || page.index != index || !operation.retained_page_emitted {
                        return Err(AbiErrorCode::OutOfOrderPage);
                    }
                    operation.feature.preflight_acknowledge(index).map_err(|failure| failure.code)?;
                    operation.feature.acknowledge(index, budget).map_err(|failure| failure.code)?;
                    operation.retained_page = None;
                    operation.retained_page_emitted = false;
                    Ok(())
                } else {
                    operation.reader.as_mut().ok_or(AbiErrorCode::UnknownHandle)?.acknowledge(AbiControl::Acknowledge { handle, index })
                }
            }
            AbiControl::Close { handle } => match self.resources.get(handle)? {
                FlowResource::Session(_) => self.close_session(handle),
                FlowResource::Operation(_) => {
                    let FlowResource::Operation(operation) = self.resources.get_mut(handle)? else {
                        return Err(AbiErrorCode::UnknownHandle);
                    };
                    operation.feature.cancel(budget).map_err(|failure| failure.code)?;
                    operation.cancelled = true;
                    Ok(())
                }
            },
        }
    }

    fn accept_event_ack(&mut self, reply: AbiReply) -> Result<(), AbiErrorCode> {
        let slot = request_slot(reply.request_id);
        let entry = self.events[slot].ok_or(AbiErrorCode::LateReply)?;
        if entry.acknowledgement != reply.request_id || entry.generation != reply.generation {
            return Err(AbiErrorCode::LateReply);
        }
        self.events[slot] = None;
        self.event_count -= 1;
        Ok(())
    }

    fn advance(&mut self, budget: AbiWorkBudget) -> Result<(), AbiErrorCode> {
        validate_budget(budget)?;
        let Some(handle) = self.work.pop_front() else {
            return Ok(());
        };
        if matches!(self.resources.get(handle)?, FlowResource::Session(_)) {
            let retired = self.advance_session_close(handle, budget);
            if !matches!(retired, Ok(true)) { self.work.push_back(handle); }
            return retired.map(|_| ());
        }
        let result = self.advance_operation(handle, budget);
        if result.is_err() && self.resources.get(handle).is_ok() && !self.work.contains(&handle) { self.work.push_back(handle); }
        result
    }

    fn advance_operation(&mut self, handle: AbiHandle, budget: AbiWorkBudget) -> Result<(), AbiErrorCode> {
        self.preflight_outbound(2)?;
        let mut retain = true;
        let mut event: Option<(AbiRequestId, u32, u16, AbiStatus, Vec<u8>, bool)> = None;
        let mut page = None;
        let mut reply = None;
        let session_handle = match self.resources.get(handle)? {
            FlowResource::Operation(operation) => operation.session_handle,
            FlowResource::Session(_) => return Err(AbiErrorCode::UnknownHandle),
        };
        let session_closed = matches!(self.resources.get(session_handle), Ok(FlowResource::Session(session)) if session.borrow().closed);
        {
            let FlowResource::Operation(operation) = self.resources.get_mut(handle)? else {
                return Err(AbiErrorCode::UnknownHandle);
            };
            if session_closed && !operation.cancelled {
                operation.feature.cancel(budget).map_err(|failure| failure.code)?;
                operation.cancelled = true;
            }
            if operation.cancelled {
                if let Some(page) = operation.retained_page.take() {
                    if operation.reader.is_some() { operation.retained_page = Some(page); return Err(AbiErrorCode::Busy); }
                    operation.reader = Some(AbiPageReader::try_new(handle, page.bytes.into_vec()).map_err(|rejected| rejected.code)?);
                    operation.retained_page_emitted = false;
                }
                if let Some(reader) = operation.reader.as_mut() {
                    reader.cancel();
                    if !reader.terminal_is_empty() {
                        reader.close_step(budget)?;
                        self.work.push_back(handle);
                        return Ok(());
                    }
                }
                let feature_closed = operation.feature.close_step(budget).map_err(|failure| failure.code)?;
                if !feature_closed {
                    self.work.push_back(handle);
                    return Ok(());
                }
                event = Some(terminal_event(operation.request_id, operation.generation, handle, AbiStatusCode::Cancelled, AbiErrorCode::Cancelled));
                reply = Some(failure_reply(operation.request_id, operation.generation, AbiStatusCode::Cancelled, AbiErrorCode::Cancelled, "cancelled"));
                retain = false;
            } else if let Some(retained_page) = operation.retained_page.as_ref() {
                if !operation.retained_page_emitted {
                    page = Some(retained_page.clone());
                    operation.retained_page_emitted = true;
                }
            } else if let Some(reader) = operation.reader.as_mut() {
                match reader.read_step(budget)? {
                    AbiCursorStep::PageComplete(_) => page = reader.page().cloned(),
                    AbiCursorStep::Complete => {
                        event = Some(terminal_event(operation.request_id, operation.generation, handle, AbiStatusCode::Ok, AbiErrorCode::Sealed));
                        reply = Some(success_reply(operation.request_id, operation.generation, Vec::new()));
                        retain = false;
                    }
                    _ => {}
                }
            } else {
                match operation.feature.step(budget) {
                    FlowFeatureStep::Yield => {}
                    FlowFeatureStep::Progress { completed, total } => {
                        let mut body = FlowPayloadWriter::default();
                        body.handle(handle);
                        body.u64(completed);
                        body.u64(total);
                        event = Some((operation.request_id, operation.generation, FLOW_EVENT_PROGRESS, AbiStatus::OK, body.finish(), true));
                    }
                    FlowFeatureStep::Checkpoint(bytes) => event = Some((operation.request_id, operation.generation, FLOW_EVENT_CHECKPOINT, AbiStatus::OK, bytes, true)),
                    FlowFeatureStep::Preview(bytes) => event = Some((operation.request_id, operation.generation, FLOW_EVENT_PREVIEW, AbiStatus::OK, bytes, true)),
                    FlowFeatureStep::SurfaceStatus(bytes) => event = Some((operation.request_id, operation.generation, FLOW_EVENT_SURFACE_STATUS, AbiStatus::OK, bytes, true)),
                    FlowFeatureStep::RetainedPage(output) => {
                        let total = output.len();
                        operation.retained_page = Some(AbiPage::try_new(handle, 0, output).map_err(|rejected| rejected.code)?);
                        operation.retained_page_emitted = false;
                        let mut body = FlowPayloadWriter::default();
                        body.handle(handle);
                        body.u64(total as u64);
                        event = Some((operation.request_id, operation.generation, FLOW_EVENT_OUTPUT, AbiStatus::OK, body.finish(), false));
                    }
                    FlowFeatureStep::Failed(failure) => {
                        event = Some(terminal_event(operation.request_id, operation.generation, handle, AbiStatusCode::Failed, failure.code));
                        reply = Some(failure_reply(operation.request_id, operation.generation, AbiStatusCode::Failed, failure.code, &failure.message));
                        retain = false;
                    }
                    FlowFeatureStep::Complete(output) if output.len() > FLOW_MAX_OUTPUT_BYTES => {
                        event = Some(terminal_event(operation.request_id, operation.generation, handle, AbiStatusCode::Rejected, AbiErrorCode::LimitExceeded));
                        reply = Some(failure_reply(operation.request_id, operation.generation, AbiStatusCode::Rejected, AbiErrorCode::LimitExceeded, "output limit"));
                        retain = false;
                    }
                    FlowFeatureStep::Complete(output) if output.len() <= FLOW_MAX_INLINE_REPLY_BYTES => {
                        event = Some(terminal_event(operation.request_id, operation.generation, handle, AbiStatusCode::Ok, AbiErrorCode::Sealed));
                        reply = Some(success_reply(operation.request_id, operation.generation, output));
                        retain = false;
                    }
                    FlowFeatureStep::Complete(output) => {
                        let total = output.len();
                        operation.reader = Some(AbiPageReader::try_new(handle, output).map_err(|rejected| rejected.code)?);
                        let mut body = FlowPayloadWriter::default();
                        body.handle(handle);
                        body.u64(total as u64);
                        event = Some((operation.request_id, operation.generation, FLOW_EVENT_OUTPUT, AbiStatus::OK, body.finish(), false));
                    }
                }
            }
        }
        if let Some((request_id, generation, code, status, bytes, replaceable)) = event {
            self.push_event(request_id, generation, code, status, bytes, replaceable)?;
        }
        if let Some(page) = page {
            self.push_outbound(AbiMessage::Page(page))?;
        }
        if let Some(reply) = reply {
            self.push_outbound(AbiMessage::Reply(reply))?;
        }
        if retain {
            self.work.push_back(handle);
        } else {
            self.retire_operation(handle)?;
        }
        Ok(())
    }

    fn retire_operation(&mut self, handle: AbiHandle) -> Result<(), AbiErrorCode> {
        let FlowResource::Operation(operation) = self.resources.close(handle)? else {
            return Err(AbiErrorCode::UnknownHandle);
        };
        let slot = request_slot(operation.request_id);
        if self.requests[slot].is_some_and(|entry| entry.operation == handle) {
            self.requests[slot] = None;
        }
        self.request_ledger.accept(&success_reply(operation.request_id, operation.generation, Vec::new()))?;
        self.active_resources -= 1;
        Ok(())
    }

    fn close_session(&mut self, handle: AbiHandle) -> Result<(), AbiErrorCode> {
        let FlowResource::Session(session) = self.resources.get(handle)? else {
            return Err(AbiErrorCode::UnknownHandle);
        };
        if session.borrow().closed { return Err(AbiErrorCode::Closed); }
        session.borrow_mut().closed = true;
        self.work.push_back(handle);
        for slot in 0..self.requests.len() {
            let Some(entry) = self.requests[slot] else {
                continue;
            };
            let belongs_to_session = matches!(self.resources.get(entry.operation), Ok(FlowResource::Operation(operation)) if operation.session_handle == handle);
            if belongs_to_session {
                let Ok(FlowResource::Operation(operation)) = self.resources.get_mut(entry.operation) else {
                    continue;
                };
                if operation.feature.cancel(bridge_control_budget()).is_ok() {
                    operation.cancelled = true;
                    if let Some(reader) = operation.reader.as_mut() { reader.cancel(); }
                }
            }
        }
        Ok(())
    }

    fn advance_session_close(&mut self, handle: AbiHandle, budget: AbiWorkBudget) -> Result<bool, AbiErrorCode> {
        if self.requests.iter().flatten().any(|entry| matches!(self.resources.get(entry.operation), Ok(FlowResource::Operation(operation)) if operation.session_handle == handle)) {
            return Ok(false);
        }
        let FlowResource::Session(session) = self.resources.get(handle)? else {
            return Err(AbiErrorCode::UnknownHandle);
        };
        let (origin, generation) = {
            let mut session = session.borrow_mut();
            if !session.closed { return Err(AbiErrorCode::Busy); }
            if !session.domain_close_started {
                session.domain.borrow_mut().begin_close();
                session.domain_close_started = true;
            }
            if !session.domain_terminal {
                {
                    let mut domain = session.domain.borrow_mut();
                    if !domain.close_step(budget).map_err(|failure| failure.code)? { return Ok(false); }
                    if !domain.terminal_is_empty() { return Err(AbiErrorCode::Busy); }
                }
                session.domain_terminal = true;
            }
            (session.open_request, session.open_generation)
        };
        let mut body = FlowPayloadWriter::default(); body.handle(handle);
        match self.push_event(origin, generation, FLOW_EVENT_SESSION_TERMINAL, AbiStatus::OK, body.finish(), false) {
            Ok(()) => {},
            Err(AbiErrorCode::Busy | AbiErrorCode::LimitExceeded) => return Ok(false),
            Err(code) => return Err(code),
        }
        self.resources.close(handle)?;
        self.sessions.retain(|candidate| *candidate != handle);
        self.active_resources -= 1;
        Ok(true)
    }

    fn push_event(&mut self, origin: AbiRequestId, generation: u32, code: u16, status: AbiStatus, bytes: Vec<u8>, replaceable: bool) -> Result<(), AbiErrorCode> {
        self.preflight_outbound(1)?;
        if replaceable {
            if let Some((slot, old)) = self.events.iter().enumerate().find_map(|(slot, entry)| entry.filter(|entry| entry.origin == origin && entry.generation == generation && entry.code == code).map(|entry| (slot, entry))) {
                self.outbound.retain(|message| !matches!(message, AbiMessage::Event(event) if event.request_id == old.acknowledgement));
                self.events[slot] = None;
                self.event_count -= 1;
            }
        }
        if self.event_count == FLOW_MAX_EVENTS_IN_FLIGHT {
            return Err(AbiErrorCode::LimitExceeded);
        }
        let acknowledgement = AbiRequestId(origin.0 ^ ((self.next_event_sequence as u64) << 32));
        let slot = request_slot(acknowledgement);
        if self.events[slot].is_some() {
            return Err(AbiErrorCode::Busy);
        }
        let event = AbiEvent { request_id: acknowledgement, generation, sequence: self.next_event_sequence, event: AbiEventCode::try_new(code)?, status, bytes: AbiBytes::try_new(bytes).map_err(|rejected| rejected.code)? };
        self.next_event_sequence = self.next_event_sequence.checked_add(1).ok_or(AbiErrorCode::GenerationExhausted)?;
        self.events[slot] = Some(EventEntry { acknowledgement, origin, generation, code });
        self.event_count += 1;
        self.push_outbound(AbiMessage::Event(event))
    }

    fn push_outbound(&mut self, message: AbiMessage) -> Result<(), AbiErrorCode> {
        self.preflight_outbound(1)?;
        self.outbound.push_back(message);
        Ok(())
    }

    fn preflight_outbound(&self, additional: usize) -> Result<(), AbiErrorCode> {
        self.outbound.len().checked_add(additional).filter(|total| *total <= FLOW_MAX_OUTBOUND).map(|_| ()).ok_or(AbiErrorCode::LimitExceeded)
    }
}

impl<D: FlowDomain> AbiPort for FlowBridge<D> {
    fn try_send(&mut self, message: AbiMessage, budget: AbiWorkBudget) -> Result<(), AbiPortRejection> {
        if let Err(code) = validate_budget(budget) {
            return Err(AbiPortRejection { code, message });
        }
        match message {
            AbiMessage::Request(request) => self.accept_request(request),
            AbiMessage::Control(control) => self.accept_control(control, budget).map_err(|code| AbiPortRejection { code, message: AbiMessage::Control(control) }),
            AbiMessage::Reply(reply) => {
                let returned = reply.clone();
                self.accept_event_ack(reply).map_err(|code| AbiPortRejection { code, message: AbiMessage::Reply(returned) })
            }
            other => Err(AbiPortRejection { code: AbiErrorCode::MalformedTag, message: other }),
        }
    }

    fn poll(&mut self, budget: AbiWorkBudget) -> Result<AbiPortPoll, AbiErrorCode> {
        validate_budget(budget)?;
        if let Some(message) = self.outbound.pop_front() {
            return Ok(AbiPortPoll::Message(message));
        }
        self.advance(budget)?;
        if let Some(message) = self.outbound.pop_front() {
            Ok(AbiPortPoll::Message(message))
        } else if self.terminal_is_empty() {
            Ok(AbiPortPoll::Closed)
        } else {
            Ok(AbiPortPoll::Pending)
        }
    }
}

fn terminal_event(request_id: AbiRequestId, generation: u32, handle: AbiHandle, status: AbiStatusCode, code: AbiErrorCode) -> (AbiRequestId, u32, u16, AbiStatus, Vec<u8>, bool) {
    let mut body = FlowPayloadWriter::default();
    body.handle(handle);
    body.u8(status as u8);
    body.u32(code as u32);
    (request_id, generation, FLOW_EVENT_TERMINAL, AbiStatus::OK, body.finish(), false)
}

fn success_reply(request_id: AbiRequestId, generation: u32, bytes: Vec<u8>) -> AbiReply {
    AbiReply { request_id, generation, status: AbiStatus::OK, bytes: AbiBytes::try_new(bytes).expect("bounded Flow reply") }
}

fn failure_reply(request_id: AbiRequestId, generation: u32, status: AbiStatusCode, code: AbiErrorCode, message: &str) -> AbiReply {
    let message = AbiMessageBytes::from_text(message).unwrap_or_default();
    AbiReply { request_id, generation, status: AbiStatus { code: status, error: Some(AbiError { code, message }) }, bytes: AbiBytes::default() }
}

fn request_slot(request_id: AbiRequestId) -> usize {
    (request_id.0 % FLOW_MAX_REQUESTS as u64) as usize
}

pub(super) fn validate_budget(budget: AbiWorkBudget) -> Result<(), AbiErrorCode> {
    if budget.cancelled {
        Err(AbiErrorCode::Cancelled)
    } else if budget.interrupted {
        Err(AbiErrorCode::Interrupted)
    } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
        Err(AbiErrorCode::DeadlineExceeded)
    } else if budget.byte_credit == 0 || budget.byte_credit > FLOW_MAX_WORK_UNITS * FLOW_MAX_REQUEST_BYTES {
        Err(AbiErrorCode::NoCredit)
    } else {
        Ok(())
    }
}

fn bridge_control_budget() -> AbiWorkBudget {
    AbiWorkBudget { byte_credit: 1, now_ms: 0, deadline_ms: Some(FLOW_DEADLINE_MILLISECONDS), cancelled: false, interrupted: false }
}

//#endregion 🔖️Runtime

//#region 🧪️Laws

#[cfg(test)]
#[path = "../🧪️tests/🔬️protocol-unit/🦀️.rs"]
mod tests;

//#endregion 🧪️Laws
