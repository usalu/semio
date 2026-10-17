//! ⏳️ Page transfer of the ABI: the work budget, the cursor step and cancel outcome and the resumable page writer and reader with the handle classifier. See [`super`].

use super::*;

//#region ⏳️RetainedTransfer
/// ⏱️ Independent transport copy and retained ownership allowances preserve caller admission.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AbiWorkBudget {
    pub byte_credit: usize,
    pub retained: RetainedCloneGrant,
    pub now_ms: u64,
    pub deadline_ms: Option<u64>,
    pub cancelled: bool,
    pub interrupted: bool,
}

impl AbiWorkBudget {
    pub const fn credits(byte_credit: usize) -> Self {
        Self { byte_credit, retained: RetainedCloneGrant { maximum_items:0, maximum_copy_bytes:0, maximum_capacity_bytes:0, maximum_release_bytes:0, maximum_depth:0 }, now_ms: 0, deadline_ms: None, cancelled: false, interrupted: false }
    }

    pub(super) fn permit(self, remaining: usize) -> Result<usize, AbiErrorCode> {
        if self.cancelled {
            return Err(AbiErrorCode::Cancelled);
        }
        if self.interrupted {
            return Err(AbiErrorCode::Interrupted);
        }
        if self.deadline_ms.is_some_and(|deadline| self.now_ms >= deadline) {
            return Err(AbiErrorCode::DeadlineExceeded);
        }
        if self.byte_credit == 0 {
            return Err(AbiErrorCode::NoCredit);
        }
        Ok(self.byte_credit.min(remaining))
    }
}

/// 🔄 Cursor step shared by retained readers, writers, and close retirement.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AbiCursorStep {
    Advanced(usize),
    PageComplete(u32),
    AwaitingAcknowledgement(u32),
    Idle,
    Complete,
}

/// ↩️ Cancellation handback plus the exact admitted and already-copied byte credits.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AbiCancelOutcome {
    pub page: Option<AbiPage>,
    pub admitted_byte_credits: usize,
    pub copied_bytes: usize,
}

struct PendingWrite {
    pub(super) page: AbiPage,
    pub(super) cursor: usize,
}

/// ✍️ Retains an admitted page and advances its copy only under explicit work credit.
pub struct AbiPageWriter {
    pub(super) handle: AbiHandle,
    pub(super) bytes: Vec<u8>,
    next_index: u32,
    pending: Option<PendingWrite>,
    sealed: bool,
    pub(super) cancelled: bool,
    closing: bool,
}

impl AbiPageWriter {
    pub fn new(handle: AbiHandle) -> Self {
        Self { handle, bytes: Vec::new(), next_index: 0, pending: None, sealed: false, cancelled: false, closing: false }
    }

    pub fn offer(&mut self, page: AbiPage) -> Result<(), AbiRejectedPage> {
        let reject = |code, page: AbiPage| AbiRejectedPage { code, handle: page.handle, index: page.index, bytes: page.bytes.into_vec() };
        if let Err(code) = classify_handle(self.handle, page.handle) {
            return Err(reject(code, page));
        }
        if self.closing {
            return Err(reject(AbiErrorCode::Closed, page));
        }
        if self.cancelled {
            return Err(reject(AbiErrorCode::Cancelled, page));
        }
        if self.sealed {
            return Err(reject(AbiErrorCode::Sealed, page));
        }
        if self.pending.is_some() {
            return Err(reject(AbiErrorCode::Busy, page));
        }
        if page.index != self.next_index {
            return Err(reject(AbiErrorCode::OutOfOrderPage, page));
        }
        if self.next_index >= ABI_MAX_PAGES_PER_TRANSFER || self.bytes.len().checked_add(page.bytes.len()).is_none_or(|len| len > ABI_MAX_TRANSFER_BYTES) {
            return Err(reject(AbiErrorCode::LimitExceeded, page));
        }
        self.pending = Some(PendingWrite { page, cursor: 0 });
        Ok(())
    }

    pub fn write_step(&mut self, budget: AbiWorkBudget) -> Result<AbiCursorStep, AbiErrorCode> {
        if self.cancelled {
            return Err(AbiErrorCode::Cancelled);
        }
        let Some(pending) = self.pending.as_mut() else {
            return Ok(if self.sealed { AbiCursorStep::Complete } else { AbiCursorStep::Idle });
        };
        let remaining = pending.page.bytes.len() - pending.cursor;
        if remaining == 0 {
            let index = pending.page.index;
            self.pending = None;
            self.next_index += 1;
            return Ok(AbiCursorStep::PageComplete(index));
        }
        let permitted = budget.permit(remaining)?;
        let end = pending.cursor + permitted;
        self.bytes.extend_from_slice(&pending.page.bytes.as_slice()[pending.cursor..end]);
        pending.cursor = end;
        if end == pending.page.bytes.len() {
            let index = pending.page.index;
            self.pending = None;
            self.next_index += 1;
            Ok(AbiCursorStep::PageComplete(index))
        } else {
            Ok(AbiCursorStep::Advanced(permitted))
        }
    }

    pub fn seal(&mut self) -> Result<(), AbiErrorCode> {
        if self.cancelled {
            return Err(AbiErrorCode::Cancelled);
        }
        if self.pending.is_some() {
            return Err(AbiErrorCode::Busy);
        }
        self.sealed = true;
        Ok(())
    }

    pub fn cancel(&mut self) -> AbiCancelOutcome {
        self.cancelled = true;
        let page = self.pending.take().map(|pending| pending.page);
        AbiCancelOutcome { admitted_byte_credits: page.as_ref().map_or(0, |page| page.bytes.len()), copied_bytes: self.bytes.len(), page }
    }

    pub const fn is_sealed(&self) -> bool {
        self.sealed
    }

    pub const fn is_cancelled(&self) -> bool {
        self.cancelled
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn close_step(&mut self, budget: AbiWorkBudget) -> Result<AbiCursorStep, AbiErrorCode> {
        self.closing = true;
        let pending_len = self.pending.as_ref().map_or(0, |pending| pending.page.bytes.len());
        let remaining = self.bytes.len() + pending_len;
        if remaining == 0 {
            self.pending = None;
            return Ok(AbiCursorStep::Complete);
        }
        let permitted = budget.permit(remaining)?;
        let from_pending = permitted.min(pending_len);
        if let Some(pending) = self.pending.as_mut() {
            let new_len = pending.page.bytes.0.len() - from_pending;
            pending.page.bytes.0.truncate(new_len);
            pending.cursor = pending.cursor.min(new_len);
            if pending.page.bytes.is_empty() {
                self.pending = None;
            }
        }
        let from_bytes = permitted - from_pending;
        self.bytes.truncate(self.bytes.len() - from_bytes.min(self.bytes.len()));
        Ok(if self.terminal_is_empty() { AbiCursorStep::Complete } else { AbiCursorStep::Advanced(permitted) })
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.closing && self.bytes.is_empty() && self.pending.is_none()
    }
}

/// 📖 Retains source, staging cursor, and one exact outstanding page until its ACK arrives.
pub struct AbiPageReader {
    pub(super) handle: AbiHandle,
    source: Vec<u8>,
    pub(super) source_cursor: usize,
    next_index: u32,
    pub(super) staging: Vec<u8>,
    pub(super) target_len: usize,
    outstanding: Option<AbiPage>,
    last_acked: Option<u32>,
    pub(super) cancelled: bool,
    closing: bool,
}

impl AbiPageReader {
    pub fn try_new(handle: AbiHandle, source: Vec<u8>) -> Result<Self, AbiRejectedBytes> {
        if source.len() > ABI_MAX_TRANSFER_BYTES {
            return Err(AbiRejectedBytes { code: AbiErrorCode::LimitExceeded, bytes: source });
        }
        Ok(Self { handle, source, source_cursor: 0, next_index: 0, staging: Vec::new(), target_len: 0, outstanding: None, last_acked: None, cancelled: false, closing: false })
    }

    pub fn read_step(&mut self, budget: AbiWorkBudget) -> Result<AbiCursorStep, AbiErrorCode> {
        if self.closing {
            return Err(AbiErrorCode::Closed);
        }
        if self.cancelled {
            return Err(AbiErrorCode::Cancelled);
        }
        if let Some(page) = &self.outstanding {
            return Ok(AbiCursorStep::AwaitingAcknowledgement(page.index));
        }
        if self.source_cursor == self.source.len() {
            return Ok(AbiCursorStep::Complete);
        }
        let target_len = if self.target_len == 0 { ABI_MAX_PAGE_BYTES.min(self.source.len() - self.source_cursor) } else { self.target_len };
        let remaining = target_len - self.staging.len();
        let permitted = budget.permit(remaining)?;
        self.staging.try_reserve_exact(permitted).map_err(|_| AbiErrorCode::LimitExceeded)?;
        self.target_len = target_len;
        let end = self.source_cursor + permitted;
        self.staging.extend_from_slice(&self.source[self.source_cursor..end]);
        self.source_cursor = end;
        if self.staging.len() == self.target_len {
            let index = self.next_index;
            let bytes = std::mem::take(&mut self.staging);
            self.target_len = 0;
            self.outstanding = Some(AbiPage { handle: self.handle, index, bytes: AbiPageBytes(bytes) });
            Ok(AbiCursorStep::PageComplete(index))
        } else {
            Ok(AbiCursorStep::Advanced(permitted))
        }
    }

    pub fn page(&self) -> Option<&AbiPage> {
        self.outstanding.as_ref()
    }

    pub fn acknowledge(&mut self, control: AbiControl) -> Result<(), AbiErrorCode> {
        let AbiControl::Acknowledge { handle, index } = control else {
            return Err(AbiErrorCode::MalformedTag);
        };
        classify_handle(self.handle, handle)?;
        if self.last_acked == Some(index) {
            return Err(AbiErrorCode::DuplicateAcknowledgement);
        }
        let Some(page) = self.outstanding.as_ref() else {
            return Err(AbiErrorCode::UnknownHandle);
        };
        if page.index != index {
            return Err(AbiErrorCode::OutOfOrderPage);
        }
        self.outstanding = None;
        self.last_acked = Some(index);
        self.next_index += 1;
        Ok(())
    }

    pub fn cancel(&mut self) {
        self.cancelled = true;
    }

    pub fn close_step(&mut self, budget: AbiWorkBudget) -> Result<AbiCursorStep, AbiErrorCode> {
        self.closing = true;
        let outstanding_len = self.outstanding.as_ref().map_or(0, |page| page.bytes.len());
        let remaining = self.source.len() + self.staging.len() + outstanding_len;
        if remaining == 0 {
            self.outstanding = None;
            return Ok(AbiCursorStep::Complete);
        }
        let mut permitted = budget.permit(remaining)?;
        if let Some(page) = self.outstanding.as_mut() {
            let remove = permitted.min(page.bytes.len());
            page.bytes.0.truncate(page.bytes.len() - remove);
            permitted -= remove;
            if page.bytes.is_empty() {
                self.outstanding = None;
            }
        }
        let remove = permitted.min(self.staging.len());
        self.staging.truncate(self.staging.len() - remove);
        permitted -= remove;
        self.source.truncate(self.source.len() - permitted.min(self.source.len()));
        self.source_cursor = self.source_cursor.min(self.source.len());
        Ok(if self.terminal_is_empty() { AbiCursorStep::Complete } else { AbiCursorStep::Advanced(budget.byte_credit.min(remaining)) })
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.closing && self.source.is_empty() && self.staging.is_empty() && self.outstanding.is_none()
    }
}

fn classify_handle(expected: AbiHandle, actual: AbiHandle) -> Result<(), AbiErrorCode> {
    if actual.slot != expected.slot {
        Err(AbiErrorCode::UnknownHandle)
    } else if actual.generation < expected.generation {
        Err(AbiErrorCode::AbaHandle)
    } else if actual.generation > expected.generation {
        Err(AbiErrorCode::StaleGeneration)
    } else {
        Ok(())
    }
}
//#endregion ⏳️RetainedTransfer
