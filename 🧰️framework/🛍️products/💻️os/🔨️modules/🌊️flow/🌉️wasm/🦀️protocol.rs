//! 🧬️ Flow editor reactive features over the owned A1 byte/message ABI.

use semio_framework::abi::{
    AbiBytes, AbiControl, AbiCursorStep, AbiError, AbiErrorCode, AbiEvent, AbiEventCode, AbiHandle, AbiHandleTable, AbiMessage, AbiMessageBytes, AbiOperation, AbiPage, AbiPageReader, AbiPort, AbiPortPoll, AbiPortRejection, AbiReply, AbiReplyLedger,
    AbiRequest, AbiRequestId, AbiStatus, AbiStatusCode, AbiWorkBudget, ABI_MAX_BODY_BYTES, ABI_MAX_IN_FLIGHT_HANDLES, ABI_MAX_IN_FLIGHT_REQUESTS, ABI_MAX_MESSAGE_BYTES, ABI_MAX_TRANSFER_BYTES,
};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

//#region 🧬️Contract

pub const FLOW_ABI_SCHEMA: &str = include_str!("🧬️schema/🔣️.json");
pub const FLOW_ABI_LEDGER: &str = include_str!("🧪️fixtures/📒️ledger.tsv");
pub const FLOW_ABI_LIMITS: &str = include_str!("🧪️fixtures/📐️limits.tsv");
pub const FLOW_ABI_TRACE: &str = include_str!("🧪️fixtures/🎞️trace.tsv");
pub const FLOW_VCS_PRODUCTION_LEDGER: &str = include_str!("🧪️fixtures/🌿️vcs-production.tsv");

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
    closed: bool,
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
        for entry in self.requests.iter().flatten() {
            if let Ok(FlowResource::Operation(operation)) = self.resources.get_mut(entry.operation) {
                if operation.feature.cancel(bridge_control_budget()).is_ok() {
                    operation.cancelled = true;
                    if let Some(reader) = operation.reader.as_mut() {
                        reader.cancel();
                    }
                }
            }
        }
        for handle in std::mem::take(&mut self.sessions) {
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
        if !(FLOW_OPERATION_FIRST..=FLOW_OPERATION_LAST).contains(&code) {
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
        let session = Rc::new(RefCell::new(FlowSession { domain: domain.clone(), closed: false }));
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
            if let Some(retained_page) = operation.retained_page.as_ref() {
                if !operation.retained_page_emitted {
                    page = Some(retained_page.clone());
                    operation.retained_page_emitted = true;
                }
            } else if operation.cancelled || session_closed {
                let feature_closed = operation.feature.close_step(budget).map_err(|failure| failure.code)?;
                if let Some(reader) = operation.reader.as_mut() {
                    let _ = reader.close_step(budget);
                    if !reader.terminal_is_empty() {
                        self.work.push_back(handle);
                        return Ok(());
                    }
                }
                if !feature_closed {
                    self.work.push_back(handle);
                    return Ok(());
                }
                event = Some(terminal_event(operation.request_id, operation.generation, handle, AbiStatusCode::Cancelled, AbiErrorCode::Cancelled));
                reply = Some(failure_reply(operation.request_id, operation.generation, AbiStatusCode::Cancelled, AbiErrorCode::Cancelled, "cancelled"));
                retain = false;
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
                }
            }
        }
        let FlowResource::Session(session) = self.resources.close(handle)? else {
            return Err(AbiErrorCode::UnknownHandle);
        };
        session.borrow_mut().closed = true;
        self.sessions.retain(|candidate| *candidate != handle);
        self.active_resources -= 1;
        Ok(())
    }

    fn push_event(&mut self, origin: AbiRequestId, generation: u32, code: u16, status: AbiStatus, bytes: Vec<u8>, replaceable: bool) -> Result<(), AbiErrorCode> {
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

fn validate_budget(budget: AbiWorkBudget) -> Result<(), AbiErrorCode> {
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
mod tests {
    use super::*;

    #[derive(Default)]
    struct MockDomain;

    struct MockFeature {
        payload: Option<Vec<u8>>,
        phase: u8,
        cancelled: bool,
    }

    impl FlowFeature for MockFeature {
        fn step(&mut self, _: AbiWorkBudget) -> FlowFeatureStep {
            if self.cancelled {
                return FlowFeatureStep::Failed(FlowFailure::new(AbiErrorCode::Cancelled, "cancelled"));
            }
            self.phase += 1;
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
    }

    impl FlowDomain for MockDomain {
        fn start_feature(_: Rc<RefCell<Self>>, _: FlowFeatureAdmission, _: u16, payload: Vec<u8>) -> Result<Box<dyn FlowFeature>, FlowFailure> {
            Ok(Box::new(MockFeature { payload: Some(payload), phase: 0, cancelled: false }))
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
        assert!(FLOW_ABI_SCHEMA.contains("\"dwgEncodeMeshJson\": 2608"));
        assert!(FLOW_ABI_SCHEMA.contains("\"documentJson\": 2609"));
        assert!(FLOW_ABI_SCHEMA.contains("\"synchronizeDocumentJson\": 2610"));
        for operation in FLOW_OPERATION_FIRST..=FLOW_OPERATION_LAST {
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
        assert!(bridge.terminal_is_empty());
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
}

//#endregion 🧪️Laws
