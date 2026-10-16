//! 🌉️ The Flow bridge runtime: sessions, operations, request and event tables and the owned ABI port.

use super::*;

struct FlowSession<D> {
    pub domain: Rc<RefCell<D>>,
    open_request: AbiRequestId,
    open_generation: u32,
    closed: bool,
    domain_close_started: bool,
    domain_terminal: bool,
}

pub(super) struct FlowOperation {
    session_handle: AbiHandle,
    pub(super) request_id: AbiRequestId,
    pub generation: u32,
    feature: Box<dyn FlowFeature>,
    reader: Option<AbiPageReader>,
    retained_page: Option<AbiPage>,
    retained_page_emitted: bool,
    pub cancelled: bool,
}

pub(super) enum FlowResource<D> {
    Session(Rc<RefCell<FlowSession<D>>>),
    Operation(FlowOperation),
}

#[derive(Clone, Copy)]
struct RequestEntry {
    pub(super) request_id: AbiRequestId,
    pub generation: u32,
    pub operation: AbiHandle,
}

#[derive(Clone, Copy)]
struct EventEntry {
    acknowledgement: AbiRequestId,
    origin: AbiRequestId,
    pub generation: u32,
    pub code: u16,
}

pub struct FlowBridge<D: FlowDomain> {
    factory: fn() -> D,
    pub(super) resources: AbiHandleTable<FlowResource<D>>,
    pub work: VecDeque<AbiHandle>,
    outbound: VecDeque<AbiMessage>,
    request_ledger: AbiReplyLedger,
    requests: [Option<RequestEntry>; FLOW_MAX_REQUESTS],
    events: [Option<EventEntry>; FLOW_MAX_REQUESTS],
    sessions: Vec<AbiHandle>,
    event_count: usize,
    pub(super) active_resources: usize,
    next_event_sequence: u32,
    closing: bool,
    pub retained_progress:RetainedCloneProgress,
}

impl<D: FlowDomain> FlowBridge<D> {
    /// 🫴️ Transfers one actual feature result before protocol encoding under the initiating context.
    pub fn poll_on_original_caller(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<(AbiHandle,FlowFeatureStep)>,FlowFailure>{
        self.retained_progress=Default::default();if cx.should_yield(){return Ok(None)}
        let Some(handle)=self.work.front().copied()else{return Ok(None)};
        let resource=self.resources.get_mut(handle).map_err(|code|FlowFailure::from_value(code,semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original Flow operation handle is absent")))?;
        let FlowResource::Operation(operation)=resource else{return Err(FlowFailure::from_value(AbiErrorCode::NoCredit,semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original Flow session closure requires its defining producer")))};
        if operation.cancelled||operation.reader.is_some()||operation.retained_page.is_some(){return Err(FlowFailure::from_value(AbiErrorCode::Busy,semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original Flow operation retains another receiver or close phase")))}
        let before=cx.retained_progress();let step=operation.feature.step_on_original_caller(cx);let after=cx.retained_progress();
        self.retained_progress=RetainedCloneProgress{copied_items:after.copied_items-before.copied_items,copied_bytes:after.copied_bytes-before.copied_bytes,retained_capacity_bytes:after.retained_capacity_bytes-before.retained_capacity_bytes,released_bytes:after.released_bytes-before.released_bytes};
        Ok(Some((handle,step)))
    }
    /// 🧾️ Actual retained effects survive the original success and refusal protocol paths.
    pub fn step_progress(&self)->RetainedCloneProgress{self.retained_progress}

    pub(super) fn clear_step_progress(&mut self){self.retained_progress=Default::default();}

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
            retained_progress:Default::default(),
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
        if !(FLOW_OPERATION_FIRST..=FLOW_OPERATION_LAST).contains(&code) || matches!(code, 2_603 | 2_604) {
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

    pub fn advance(&mut self, budget: AbiWorkBudget) -> Result<(), AbiErrorCode> {
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
                let close=operation.feature.close_step(budget);
                self.retained_progress=match &close{Ok(step)=>step.progress(),Err(failure)=>failure.retained_progress};
                let feature_closed=close.map_err(|failure|failure.code)?;
                if !matches!(feature_closed,RetainedCloneStep::Complete(_)) {
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
                        self.retained_progress = failure.retained_progress;
                        event = Some(terminal_event(operation.request_id, operation.generation, handle, AbiStatusCode::Failed, failure.code));
                        reply = Some(failure_reply(operation.request_id, operation.generation, AbiStatusCode::Failed, failure.code, failure.message()));
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
                    let close=domain.close_step(budget);
                    self.retained_progress=match &close{Ok(step)=>step.progress(),Err(failure)=>failure.retained_progress};
                    if !matches!(close.map_err(|failure|failure.code)?,RetainedCloneStep::Complete(_)){return Ok(false)}
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

    pub(super) fn push_event(&mut self, origin: AbiRequestId, generation: u32, code: u16, status: AbiStatus, bytes: Vec<u8>, replaceable: bool) -> Result<(), AbiErrorCode> {
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
        self.retained_progress=Default::default();
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

pub(super) fn success_reply(request_id: AbiRequestId, generation: u32, bytes: Vec<u8>) -> AbiReply {
    AbiReply { request_id, generation, status: AbiStatus::OK, bytes: AbiBytes::try_new(bytes).expect("bounded Flow reply") }
}

fn failure_reply(request_id: AbiRequestId, generation: u32, status: AbiStatusCode, code: AbiErrorCode, message: &str) -> AbiReply {
    let message = AbiMessageBytes::from_text(message).unwrap_or_default();
    AbiReply { request_id, generation, status: AbiStatus { code: status, error: Some(AbiError { code, message }) }, bytes: AbiBytes::default() }
}

fn request_slot(request_id: AbiRequestId) -> usize {
    (request_id.0 % FLOW_MAX_REQUESTS as u64) as usize
}

pub fn validate_budget(budget: AbiWorkBudget) -> Result<(), AbiErrorCode> {
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
    AbiWorkBudget { byte_credit: 1, retained:AbiWorkBudget::credits(0).retained, now_ms: 0, deadline_ms: Some(FLOW_DEADLINE_MILLISECONDS), cancelled: false, interrupted: false }
}
