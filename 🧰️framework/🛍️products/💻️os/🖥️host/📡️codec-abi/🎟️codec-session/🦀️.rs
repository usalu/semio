//! 🎟️ Codec session of the codec ABI: the cursor input union and the per-request session with its page, step, seal and close protocol. See [`super`].

use super::*;

//#region ⏳️RetainedOperation
enum OsHostCodecInput {
    Filter(FilterKindsStructuralCursor),
    Normalize(NormalizeKindStructuralCursor),
}

impl OsHostCodecInput {
    pub(super) fn new(operation: OsHostCodecOperation, declared_input_bytes: usize) -> Self {
        match operation {
            OsHostCodecOperation::MediaAcceptFilterKinds => Self::Filter(FilterKindsStructuralCursor::new()),
            OsHostCodecOperation::NormalizeStdioFormatKind => Self::Normalize(NormalizeKindStructuralCursor::new(declared_input_bytes)),
        }
    }

    pub(super) fn feed<R: OsHostFormatResolver>(&mut self, byte: u8, resolver: &mut R) {
        match self {
            Self::Filter(cursor) => cursor.feed(byte, resolver),
            Self::Normalize(cursor) => cursor.feed(byte, resolver),
        }
    }

    pub(super) fn filter(&mut self) -> Option<&mut FilterKindsStructuralCursor> {
        match self {
            Self::Filter(cursor) => Some(cursor),
            _ => None,
        }
    }

    fn normalize(&mut self) -> Option<&mut NormalizeKindStructuralCursor> {
        match self {
            Self::Normalize(cursor) => Some(cursor),
            _ => None,
        }
    }

    fn retire_one(&mut self) -> bool {
        match self {
            Self::Filter(cursor) => !cursor.close_one(),
            Self::Normalize(cursor) => !cursor.close_one(),
        }
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        match self {
            Self::Filter(cursor) => cursor.terminal_is_empty(),
            Self::Normalize(cursor) => cursor.terminal_is_empty(),
        }
    }
}

pub(super) struct OsHostCodecSession {
    pub(super) request: AbiRequest,
    pub(super) operation: OsHostCodecOperation,
    pub(super) handle: Option<AbiHandle>,
    pub(super) declared_input_bytes: usize,
    pub(super) input_bytes_received: usize,
    input: OsHostCodecInput,
    pending_input: Option<PendingInputPage>,
    next_input_index: u32,
    sealed: bool,
    decoded: bool,
    pub(super) output: Option<AbiPageReader>,
    pub(super) output_bytes: usize,
    output_copied: usize,
    reply_emitted: bool,
    terminal_failure: Option<OsHostCodecFailure>,
    sequence: u32,
    pub(super) cancelled: bool,
    closing: bool,
}

impl OsHostCodecSession {
    pub(super) fn new(request: AbiRequest, operation: OsHostCodecOperation, declared_input_bytes: usize) -> Self {
        Self {
            request,
            operation,
            handle: None,
            declared_input_bytes,
            input_bytes_received: 0,
            input: OsHostCodecInput::new(operation, declared_input_bytes),
            pending_input: None,
            next_input_index: 0,
            sealed: false,
            decoded: false,
            output: None,
            output_bytes: 0,
            output_copied: 0,
            reply_emitted: false,
            terminal_failure: None,
            sequence: 0,
            cancelled: false,
            closing: false,
        }
    }

    pub(super) fn handle(&self) -> AbiHandle {
        self.handle.expect("service assigns a handle before exposing a session")
    }

    pub(super) fn event(&mut self, phase: OsHostCodecPhase, completed: usize, total: usize) -> AbiEvent {
        self.sequence = self.sequence.saturating_add(1);
        let mut body = Vec::with_capacity(18);
        body.push(1);
        body.push(phase as u8);
        body.extend_from_slice(&(completed as u64).to_le_bytes());
        body.extend_from_slice(&(total as u64).to_le_bytes());
        AbiEvent {
            request_id: self.request.request_id,
            generation: self.request.generation,
            sequence: self.sequence,
            event: AbiEventCode::try_new(OS_HOST_CODEC_PROGRESS_EVENT).expect("schema event code is bounded"),
            status: AbiStatus::OK,
            bytes: AbiBytes::try_new(body).expect("fixed progress body is bounded"),
        }
    }

    fn step_result(&mut self, state: OsHostCodecStepState, progress: OsHostCodecProgress, input_acknowledgement: Option<AbiControl>, page: Option<AbiPage>, reply: Option<AbiReply>) -> OsHostCodecStep {
        OsHostCodecStep { state, event: self.event(progress.phase, progress.completed, progress.total), input_acknowledgement, page, reply }
    }

    pub(super) fn offer(&mut self, page: AbiPage) -> Result<(), AbiRejectedPage> {
        let reject = |code, page: AbiPage| AbiRejectedPage { code, handle: page.handle, index: page.index, bytes: page.bytes.into_vec() };
        if page.handle != self.handle() {
            let code = classify_handle(self.handle(), page.handle);
            return Err(reject(code, page));
        }
        if self.cancelled {
            return Err(reject(AbiErrorCode::Cancelled, page));
        }
        if self.closing {
            return Err(reject(AbiErrorCode::Closed, page));
        }
        if self.sealed {
            return Err(reject(AbiErrorCode::Sealed, page));
        }
        if self.pending_input.is_some() {
            return Err(reject(AbiErrorCode::Busy, page));
        }
        let total = self.input_bytes_received.checked_add(page.bytes.len());
        if page.index >= ABI_MAX_PAGES_PER_TRANSFER || page.bytes.len() > ABI_MAX_PAGE_BYTES || total.is_none_or(|total| total > self.declared_input_bytes || total > OS_HOST_CODEC_MAX_INPUT_BYTES) {
            return Err(reject(AbiErrorCode::LimitExceeded, page));
        }
        if page.index != self.next_input_index {
            return Err(reject(AbiErrorCode::OutOfOrderPage, page));
        }
        self.pending_input = Some(PendingInputPage { page, cursor: 0, retire_cursor: 0 });
        Ok(())
    }

    pub(super) fn seal(&mut self) -> Result<(), AbiErrorCode> {
        if self.cancelled {
            return Err(AbiErrorCode::Cancelled);
        }
        if self.pending_input.is_some() {
            return Err(AbiErrorCode::Busy);
        }
        if self.input_bytes_received != self.declared_input_bytes {
            return Err(AbiErrorCode::MalformedLength);
        }
        self.sealed = true;
        Ok(())
    }

    pub(super) fn cancel(&mut self) -> OsHostCodecCancelOutcome {
        self.cancelled = true;
        if let Some(output) = self.output.as_mut() {
            output.cancel();
        }
        let pending = self.pending_input.take();
        OsHostCodecCancelOutcome { admitted_byte_credits: pending.as_ref().map_or(0, |pending| pending.page.bytes.len()), copied_bytes: pending.as_ref().map_or(0, |pending| pending.cursor), page: pending.map(|pending| pending.page) }
    }

    fn failure_reply(&self, failure: &OsHostCodecFailure) -> AbiReply {
        let message = AbiMessageBytes::try_new(failure.message.clone()).unwrap_or_else(|_| AbiMessageBytes::from_text("bounded OS host codec failure").expect("fixed fallback is bounded"));
        AbiReply {
            request_id: self.request.request_id,
            generation: self.request.generation,
            status: AbiStatus { code: AbiStatusCode::Rejected, error: Some(AbiError { code: failure.abi_code(), message }) },
            bytes: AbiBytes::try_new((failure.code as u16).to_le_bytes().to_vec()).expect("error code body is bounded"),
        }
    }

    fn success_reply(&self) -> AbiReply {
        let mut summary = Vec::with_capacity(6);
        summary.push(1);
        summary.push(self.operation.reply_kind());
        summary.extend_from_slice(&(self.output_bytes as u32).to_le_bytes());
        AbiReply { request_id: self.request.request_id, generation: self.request.generation, status: AbiStatus::OK, bytes: AbiBytes::try_new(summary).expect("reply summary is bounded") }
    }

    fn install_output(&mut self, bytes: &[u8]) -> Result<(), OsHostCodecFailure> {
        let payload_len = bytes.len().checked_add(6).ok_or_else(|| OsHostCodecFailure::fixed(OsHostCodecErrorCode::OutputLimit, "OS host codec reply exceeds output limit"))?;
        if payload_len > OS_HOST_CODEC_MAX_OUTPUT_BYTES {
            return Err(OsHostCodecFailure::fixed(OsHostCodecErrorCode::OutputLimit, "OS host codec reply exceeds output limit"));
        }
        let mut payload = Vec::with_capacity(payload_len);
        payload.push(1);
        payload.push(self.operation.reply_kind());
        payload.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
        payload.extend_from_slice(bytes);
        self.install_payload(payload)
    }

    fn install_payload(&mut self, payload: Vec<u8>) -> Result<(), OsHostCodecFailure> {
        if payload.len() > OS_HOST_CODEC_MAX_OUTPUT_BYTES {
            return Err(OsHostCodecFailure::fixed(OsHostCodecErrorCode::OutputLimit, "OS host codec reply exceeds output limit"));
        }
        self.output_bytes = payload.len();
        self.output = Some(AbiPageReader::try_new(self.handle(), payload).map_err(|_| OsHostCodecFailure::fixed(OsHostCodecErrorCode::OutputLimit, "OS host codec reply exceeds transfer limit"))?);
        self.decoded = true;
        Ok(())
    }

    fn execute(&mut self) -> Result<(), OsHostCodecFailure> {
        match self.operation {
            OsHostCodecOperation::NormalizeStdioFormatKind => {
                let output = self.input.normalize().expect("normalize operation owns a structural cursor").finish()?;
                self.install_output(&output)
            }
            OsHostCodecOperation::MediaAcceptFilterKinds => {
                let output = self.input.filter().expect("filter operation owns a structural cursor").finish()?;
                self.install_output(&output)
            }
        }
    }

    pub(super) fn step<R: OsHostFormatResolver>(&mut self, resolver: &mut R, budget: AbiWorkBudget) -> Result<OsHostCodecStep, AbiErrorCode> {
        if self.cancelled {
            return Err(AbiErrorCode::Cancelled);
        }
        if self.closing {
            return Err(AbiErrorCode::Closed);
        }
        validate_budget(budget)?;
        if self.pending_input.is_some() {
            let (byte, index, complete) = {
                let pending = self.pending_input.as_mut().expect("checked pending input");
                let byte = if pending.cursor < pending.page.bytes.len() {
                    let byte = pending.page.bytes.as_slice()[pending.cursor];
                    pending.cursor += 1;
                    Some(byte)
                } else {
                    None
                };
                (byte, pending.page.index, pending.cursor == pending.page.bytes.len())
            };
            if let Some(byte) = byte {
                self.input_bytes_received += 1;
                self.input.feed(byte, resolver);
            }
            if complete {
                self.pending_input = None;
                self.next_input_index += 1;
                let acknowledgement = AbiControl::Acknowledge { handle: self.handle(), index };
                return Ok(self.step_result(OsHostCodecStepState::InputAcknowledged, OsHostCodecProgress { phase: OsHostCodecPhase::Input, completed: self.input_bytes_received, total: self.declared_input_bytes }, Some(acknowledgement), None, None));
            }
            return Ok(self.step_result(OsHostCodecStepState::Progress, OsHostCodecProgress { phase: OsHostCodecPhase::Decode, completed: self.input_bytes_received, total: self.declared_input_bytes }, None, None, None));
        }
        if !self.sealed {
            return Ok(self.step_result(OsHostCodecStepState::Idle, OsHostCodecProgress { phase: OsHostCodecPhase::Input, completed: self.input_bytes_received, total: self.declared_input_bytes }, None, None, None));
        }
        if let Some(failure) = self.terminal_failure.clone() {
            if self.reply_emitted {
                return Ok(self.step_result(OsHostCodecStepState::Idle, OsHostCodecProgress { phase: OsHostCodecPhase::Reply, completed: 1, total: 1 }, None, None, None));
            }
            self.reply_emitted = true;
            let reply = self.failure_reply(&failure);
            return Ok(self.step_result(OsHostCodecStepState::Reply, OsHostCodecProgress { phase: OsHostCodecPhase::Reply, completed: 1, total: 1 }, None, None, Some(reply)));
        }
        if !self.decoded {
            match self.execute() {
                Ok(()) => return Ok(self.step_result(OsHostCodecStepState::Progress, OsHostCodecProgress { phase: OsHostCodecPhase::Decode, completed: 1, total: 1 }, None, None, None)),
                Err(failure) => {
                    self.terminal_failure = Some(failure.clone());
                    self.reply_emitted = true;
                    let reply = self.failure_reply(&failure);
                    return Ok(self.step_result(OsHostCodecStepState::Reply, OsHostCodecProgress { phase: OsHostCodecPhase::Reply, completed: 1, total: 1 }, None, None, Some(reply)));
                }
            }
        }
        let output_step = self.output.as_mut().expect("decoded operation owns a paged output").read_step(AbiWorkBudget { byte_credit: 1, ..budget })?;
        match output_step {
            AbiCursorStep::Advanced(copied) => {
                self.output_copied = self.output_copied.saturating_add(copied).min(self.output_bytes);
                Ok(self.step_result(OsHostCodecStepState::Progress, OsHostCodecProgress { phase: OsHostCodecPhase::Output, completed: self.output_copied, total: self.output_bytes }, None, None, None))
            }
            AbiCursorStep::PageComplete(index) => {
                let page = self.output.as_ref().and_then(AbiPageReader::page).cloned().expect("page-complete retains the exact page until ACK");
                self.output_copied = (index as usize * ABI_MAX_PAGE_BYTES + page.bytes.len()).min(self.output_bytes);
                Ok(self.step_result(OsHostCodecStepState::OutputPage, OsHostCodecProgress { phase: OsHostCodecPhase::Output, completed: self.output_copied, total: self.output_bytes }, None, Some(page), None))
            }
            AbiCursorStep::AwaitingAcknowledgement(_) => Ok(self.step_result(OsHostCodecStepState::AwaitingAcknowledgement, OsHostCodecProgress { phase: OsHostCodecPhase::AwaitingAcknowledgement, completed: self.output_copied, total: self.output_bytes }, None, None, None)),
            AbiCursorStep::Complete => {
                if self.reply_emitted {
                    Ok(self.step_result(OsHostCodecStepState::Idle, OsHostCodecProgress { phase: OsHostCodecPhase::Reply, completed: self.output_bytes, total: self.output_bytes }, None, None, None))
                } else {
                    self.reply_emitted = true;
                    let reply = self.success_reply();
                    Ok(self.step_result(OsHostCodecStepState::Reply, OsHostCodecProgress { phase: OsHostCodecPhase::Reply, completed: self.output_bytes, total: self.output_bytes }, None, None, Some(reply)))
                }
            }
            AbiCursorStep::Idle => Ok(self.step_result(OsHostCodecStepState::Idle, OsHostCodecProgress { phase: OsHostCodecPhase::Output, completed: 0, total: self.output_bytes }, None, None, None)),
        }
    }

    pub(super) fn acknowledge(&mut self, control: AbiControl) -> Result<(), AbiErrorCode> {
        self.output.as_mut().ok_or(AbiErrorCode::UnknownHandle)?.acknowledge(control)
    }

    pub(super) fn close_step(&mut self, budget: AbiWorkBudget) -> Result<bool, AbiErrorCode> {
        self.closing = true;
        let input_empty = self.input.terminal_is_empty();
        let failure_empty = self.terminal_failure.as_ref().is_none_or(|failure| failure.message.is_empty());
        if self.pending_input.is_none() && input_empty && failure_empty && self.output.as_ref().is_none_or(AbiPageReader::terminal_is_empty) {
            return Ok(true);
        }
        validate_budget(budget)?;
        if let Some(pending) = self.pending_input.as_mut() {
            if pending.page.bytes.is_empty() {
                self.pending_input = None;
                return Ok(false);
            }
            pending.retire_cursor += 1;
            if pending.retire_cursor == pending.page.bytes.len() {
                self.pending_input = None;
            }
            return Ok(false);
        }
        if self.input.retire_one() {
            return Ok(false);
        }
        if let Some(failure) = self.terminal_failure.as_mut() {
            if failure.message.pop().is_some() {
                return Ok(false);
            }
        }
        if let Some(output) = self.output.as_mut() {
            return output.close_step(AbiWorkBudget { byte_credit: 1, ..budget }).map(|step| step == AbiCursorStep::Complete);
        }
        Ok(true)
    }
}
//#endregion ⏳️RetainedOperation
