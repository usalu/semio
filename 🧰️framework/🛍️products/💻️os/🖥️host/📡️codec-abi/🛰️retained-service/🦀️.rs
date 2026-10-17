//! 🛰️ Retained service of the codec ABI: budget and handle validation and the retained codec service over a format resolver. See [`super`].

use super::*;

//#region ⏳️RetainedOperation
pub(super) fn validate_budget(budget: AbiWorkBudget) -> Result<(), AbiErrorCode> {
    if budget.cancelled {
        Err(AbiErrorCode::Cancelled)
    } else if budget.interrupted {
        Err(AbiErrorCode::Interrupted)
    } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
        Err(AbiErrorCode::DeadlineExceeded)
    } else if budget.byte_credit == 0 {
        Err(AbiErrorCode::NoCredit)
    } else {
        Ok(())
    }
}

pub(super) fn classify_handle(expected: AbiHandle, actual: AbiHandle) -> AbiErrorCode {
    if expected.slot() != actual.slot() {
        AbiErrorCode::UnknownHandle
    } else if actual.generation() < expected.generation() {
        AbiErrorCode::AbaHandle
    } else if actual.generation() > expected.generation() {
        AbiErrorCode::StaleGeneration
    } else {
        AbiErrorCode::UnknownHandle
    }
}

fn request_slot(request_id: AbiRequestId) -> usize {
    (request_id.0 % 256) as usize
}

pub(super) struct RetainedOsHostCodecService<R> {
    pub(super) resolver: R,
    pub(super) handles: AbiHandleTable<OsHostCodecSession>,
    requests: [Option<AbiHandle>; 256],
}

impl<R: OsHostFormatResolver> RetainedOsHostCodecService<R> {
    pub fn new(resolver: R) -> Self {
        Self { resolver, handles: AbiHandleTable::new(), requests: [None; 256] }
    }

    pub fn begin(&mut self, request: AbiRequest) -> Result<AbiHandle, (AbiErrorCode, AbiRequest)> {
        let operation = match OsHostCodecOperation::from_abi(request.operation) {
            Ok(operation) => operation,
            Err(code) => return Err((code, request)),
        };
        if request.generation == 0 {
            return Err((AbiErrorCode::StaleGeneration, request));
        }
        let declared_input_bytes = match decode_request_metadata(request.bytes.as_slice()) {
            Ok(value) => value,
            Err(code) => return Err((code, request)),
        };
        if operation == OsHostCodecOperation::NormalizeStdioFormatKind && declared_input_bytes > OS_HOST_CODEC_MAX_KIND_BYTES {
            return Err((AbiErrorCode::LimitExceeded, request));
        }
        let slot = request_slot(request.request_id);
        if self.requests[slot].is_some() {
            return Err((AbiErrorCode::Busy, request));
        }
        let session = OsHostCodecSession::new(request, operation, declared_input_bytes);
        let handle = match self.handles.open(session) {
            Ok(handle) => handle,
            Err((code, session)) => return Err((code, session.request)),
        };
        self.handles.get_mut(handle).expect("new handle resolves").handle = Some(handle);
        self.requests[slot] = Some(handle);
        Ok(handle)
    }

    pub fn offer(&mut self, handle: AbiHandle, page: AbiPage) -> Result<(), AbiRejectedPage> {
        match self.handles.get_mut(handle) {
            Ok(session) => session.offer(page),
            Err(code) => Err(AbiRejectedPage { code, handle: page.handle, index: page.index, bytes: page.bytes.into_vec() }),
        }
    }

    pub fn seal(&mut self, handle: AbiHandle) -> Result<(), AbiErrorCode> {
        self.handles.get_mut(handle)?.seal()
    }

    pub fn step(&mut self, handle: AbiHandle, budget: AbiWorkBudget) -> Result<OsHostCodecStep, AbiErrorCode> {
        self.handles.get_mut(handle)?.step(&mut self.resolver, budget)
    }

    pub fn control(&mut self, control: AbiControl) -> Result<Option<OsHostCodecCancelOutcome>, AbiErrorCode> {
        match control {
            AbiControl::Cancel { request_id, generation } => {
                let handle = self.requests[request_slot(request_id)].ok_or(AbiErrorCode::UnknownHandle)?;
                let session = self.handles.get_mut(handle)?;
                if session.request.request_id != request_id {
                    return Err(AbiErrorCode::UnknownHandle);
                }
                if generation < session.request.generation {
                    return Err(AbiErrorCode::AbaHandle);
                }
                if generation > session.request.generation {
                    return Err(AbiErrorCode::StaleGeneration);
                }
                Ok(Some(session.cancel()))
            }
            AbiControl::Acknowledge { handle, .. } => {
                self.handles.get_mut(handle)?.acknowledge(control)?;
                Ok(None)
            }
            AbiControl::Close { .. } => Err(AbiErrorCode::MalformedTag),
        }
    }

    pub fn close_step(&mut self, control: AbiControl, budget: AbiWorkBudget) -> Result<bool, AbiErrorCode> {
        let AbiControl::Close { handle } = control else { return Err(AbiErrorCode::MalformedTag) };
        let complete = self.handles.get_mut(handle)?.close_step(budget)?;
        if complete {
            let session = self.handles.close(handle)?;
            let slot = request_slot(session.request.request_id);
            if self.requests[slot] == Some(handle) {
                self.requests[slot] = None;
            }
        }
        Ok(complete)
    }

    pub fn lose(&mut self, handle: AbiHandle) -> Result<(), AbiErrorCode> {
        let session = self.handles.lose(handle)?;
        let slot = request_slot(session.request.request_id);
        if self.requests[slot] == Some(handle) {
            self.requests[slot] = None;
        }
        Ok(())
    }
}
//#endregion ⏳️RetainedOperation
