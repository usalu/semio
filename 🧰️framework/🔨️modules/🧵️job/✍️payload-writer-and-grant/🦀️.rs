//! ✍️ Payload writer and grant: the retained job payload writer with its drop and the payload page grant with its drop. See [`super`].

use super::*;

//#region 📄️RetainedPayload
pub struct RetainedJobPayloadWriter {
    pub(super) payload: ManuallyDrop<Option<RetainedJobPayload>>,
    pub(super) rejected: ManuallyDrop<Option<JobPayloadPageSource>>,
    pub(super) staged: ManuallyDrop<Option<(Arc<JobPayloadOperationLedger>, JobPayloadPageSource, usize)>>,
    pub(super) closing_ledger: ManuallyDrop<Option<Arc<JobPayloadOperationLedger>>>,
    pub(super) sealed: bool,
}

impl std::fmt::Debug for RetainedJobPayloadWriter {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("RetainedJobPayloadWriter").field("payload", &self.payload).field("rejected", &self.rejected).field("staged", &self.staged.as_ref().map(|(_, _, length)| length)).field("sealed", &self.sealed).finish()
    }
}

impl RetainedJobPayloadWriter {
    /// 🪪️ Observes only the committed payload ledger; staged and reservation custody remains private.
    pub fn payload_authority_identity(&self)->Option<OriginalPayloadSourceIdentity>{self.payload.as_ref().and_then(RetainedJobPayload::payload_authority_identity)}

    pub fn new(stream: JobPayloadStream) -> Self {
        Self { payload: ManuallyDrop::new(Some(RetainedJobPayload::empty(stream))), rejected: ManuallyDrop::new(None), staged: ManuallyDrop::new(None), closing_ledger: ManuallyDrop::new(None), sealed: false }
    }

    pub fn take_rejected_source(&mut self) -> Option<JobPayloadPageSource> {
        self.rejected.take()
    }

    pub fn page_count(&self) -> usize {
        self.payload.as_ref().map_or(0, RetainedJobPayload::page_count)
    }

    pub fn admit_page<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<JobPayloadPageGrant<'a>, JobPayloadAdmissionFault> {
        let source = self.rejected.take().unwrap_or_default();
        if cx.payload_page_granted {
            *self.rejected = Some(source);
            return Err(JobPayloadAdmissionFault::OpportunityExhausted);
        }
        let ledger = Arc::clone(&cx.payload_ledger);
        if let Err(fault) = self.reserve_page(&ledger,source.allocated_capacity_bytes()) {
            *self.rejected = Some(source);
            return Err(fault);
        }
        cx.payload_page_granted = true;
        Ok(self.begin_page(ledger, source))
    }

    #[expect(clippy::result_large_err, reason = "A refused finish returns the exact admitted writer and its staged pages without allocating.")]
    pub fn finish(mut self) -> Result<RetainedJobPayload, Self> {
        if self.rejected.is_some() || self.staged.is_some() || self.closing_ledger.is_some() {
            return Err(self);
        }
        self.sealed = true;
        Ok(self.payload.take().expect("retained payload writer owns payload until finish"))
    }

    pub fn begin_close(&mut self) {
        self.sealed = true;
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.sealed && self.rejected.is_none() && self.staged.is_none() && self.closing_ledger.is_none() && self.payload.is_none()
    }

    pub fn begin_staged_page(&mut self, cx: &mut StepContext<'_>) -> Result<(), JobPayloadAdmissionFault> {
        if self.staged.is_some() {
            return Ok(());
        }
        self.admit_page(cx)?.stage();
        Ok(())
    }

    pub fn staged_page_remaining(&self) -> usize {
        self.staged.as_ref().map_or(0, |(_, source, length)| source.allocated_capacity_bytes() - length)
    }

    pub fn staged_page_len(&self) -> Option<usize> {
        self.staged.as_ref().map(|(_, _, length)| *length)
    }

        /// 🪪️ Identifies the same original staged backing across admitted and refused turns.
    pub fn staged_backing_identity(&self) -> Option<*const MaybeUninit<u8>> { self.staged.as_ref().map(|(_, source, _)| source.backing_identity()) }

pub fn write_staged(&mut self, bytes: &[u8]) -> Result<(), JobPayloadAdmissionFault> {
        let (_, source, length) = self.staged.as_mut().ok_or(JobPayloadAdmissionFault::OpportunityExhausted)?;
        if bytes.len() > source.allocated_capacity_bytes() - *length {
            return Err(JobPayloadAdmissionFault::StreamBytes);
        }
        unsafe { std::ptr::copy_nonoverlapping(bytes.as_ptr(), source.storage.as_mut_ptr().cast::<u8>().add(*length), bytes.len()) };
        *length += bytes.len();
        Ok(())
    }

    pub fn commit_staged_page(&mut self) -> Result<(), JobPayloadAdmissionFault> {
        let payload = self.payload.as_ref().ok_or(JobPayloadAdmissionFault::WriterSealed)?;
        let index = payload.page_count;
        if index >= JOB_PAYLOAD_OPERATION_PAGES || payload.pages[index].is_some() { return Err(JobPayloadAdmissionFault::WriterFull); }
        let (ledger, source, length) = self.staged.take().ok_or(JobPayloadAdmissionFault::OpportunityExhausted)?;
        let payload = self.payload.as_mut().expect("staged page commit preflight retains writer payload");
        payload.pages[index] = Some(JobPayloadPage { source, length });
        payload.page_count += 1;
        payload.length += length;
        *payload.ledger = Some(ledger);
        Ok(())
    }

    pub fn write_slice_page(&mut self, cx: &mut StepContext<'_>, bytes: &[u8], cursor: &mut usize) -> Result<bool, JobPayloadAdmissionFault> {
        if *cursor > bytes.len() {
            return Err(JobPayloadAdmissionFault::StreamBytes);
        }
        if *cursor == bytes.len() {
            return Ok(true);
        }
        if cx.should_yield() {
            return Ok(false);
        }
        let mut page = self.admit_page(cx)?;
        let end = cursor.saturating_add(page.remaining()).min(bytes.len());
        page.write(&bytes[*cursor..end])?;
        page.commit();
        *cursor = end;
        Ok(*cursor == bytes.len())
    }

    pub(super) fn reserve_page(&self, ledger: &JobPayloadOperationLedger, extent: usize) -> Result<(), JobPayloadAdmissionFault> {
        if self.sealed {
            return Err(JobPayloadAdmissionFault::WriterSealed);
        }
        if self.rejected.is_some() {
            return Err(JobPayloadAdmissionFault::RejectedSourcePending);
        }
        let payload = self.payload.as_ref().ok_or(JobPayloadAdmissionFault::WriterSealed)?;
        if payload.page_count >= JOB_PAYLOAD_OPERATION_PAGES {
            return Err(JobPayloadAdmissionFault::WriterFull);
        }
        ledger.reserve(payload.stream,extent)
    }

    pub(super) fn begin_page(&mut self, ledger: Arc<JobPayloadOperationLedger>, source: JobPayloadPageSource) -> JobPayloadPageGrant<'_> {
        JobPayloadPageGrant { writer: self, ledger: Some(ledger), source: Some(source), length: 0, committed: false }
    }
}


#[path="📬️outcome/✍️writer/⛽️staged/🦀️.rs"]
mod original_staged_writer;
impl Drop for RetainedJobPayloadWriter {
    fn drop(&mut self) {
        if self.payload.is_none() && self.rejected.is_none() && self.staged.is_none() && self.closing_ledger.is_none() {
            unsafe {
                ManuallyDrop::drop(&mut self.payload);
                ManuallyDrop::drop(&mut self.rejected);
                ManuallyDrop::drop(&mut self.staged);
            }
        } else {
            debug_assert!(false, "retained job payload writer requires exact finish or incremental close");
        }
    }
}

pub struct JobPayloadPageGrant<'a> {
    pub(super) writer: &'a mut RetainedJobPayloadWriter,
    pub(super) ledger: Option<Arc<JobPayloadOperationLedger>>,
    pub(super) source: Option<JobPayloadPageSource>,
    pub(super) length: usize,
    committed: bool,
}

impl JobPayloadPageGrant<'_> {
    pub fn remaining(&self) -> usize {
        self.source.as_ref().expect("original page grant retains backing").allocated_capacity_bytes() - self.length
    }

    pub fn write(&mut self, bytes: &[u8]) -> Result<(), JobPayloadAdmissionFault> {
        if bytes.len() > self.remaining() {
            return Err(JobPayloadAdmissionFault::StreamBytes);
        }
        let source = self.source.as_mut().expect("uncommitted job payload grant owns page source");
        for (target, byte) in source.storage[self.length..self.length + bytes.len()].iter_mut().zip(bytes.iter().copied()) {
            target.write(byte);
        }
        self.length += bytes.len();
        Ok(())
    }

    pub fn initialized_remaining_mut(&mut self) -> &mut [u8] {
        let source = self.source.as_mut().expect("uncommitted job payload grant owns page source");
        for byte in &mut source.storage[self.length..] {
            byte.write(0);
        }
        unsafe { std::slice::from_raw_parts_mut(source.storage.as_mut_ptr().cast::<u8>().add(self.length), source.allocated_capacity_bytes() - self.length) }
    }

    pub fn advance_written(&mut self, bytes: usize) -> Result<(), JobPayloadAdmissionFault> {
        if bytes > self.remaining() {
            return Err(JobPayloadAdmissionFault::StreamBytes);
        }
        self.length += bytes;
        Ok(())
    }

    pub fn commit(mut self) {
        let payload = self.writer.payload.as_mut().expect("retained payload writer owns payload while page is granted");
        let index=payload.page_count;assert!(index<JOB_PAYLOAD_OPERATION_PAGES&&payload.pages[index].is_none(),"preflighted original page prefix slot remains vacant");
        let source = self.source.take().expect("committed job payload grant owns page source");
        payload.pages[index] = Some(JobPayloadPage { source, length: self.length });
        payload.page_count += 1;
        payload.length += self.length;
        *payload.ledger = self.ledger.take();
        self.committed = true;
    }

    pub fn stage(mut self) {
        let ledger = self.ledger.take().expect("admitted staged page owns ledger credit");
        let source = self.source.take().expect("admitted staged page owns backing");
        *self.writer.staged = Some((ledger, source, self.length));
        self.committed = true;
    }
}

impl Drop for JobPayloadPageGrant<'_> {
    fn drop(&mut self) {
        if self.committed {
            return;
        }
        if let Some(ledger) = self.ledger.take() {
            let stream = self.writer.payload.as_ref().expect("retained payload writer owns payload while grant is live").stream;
            ledger.release(stream,self.source.as_ref().expect("original page grant retains backing").allocated_capacity_bytes());
        }
        *self.writer.rejected = self.source.take();
    }
}
//#endregion 📄️RetainedPayload

pub trait JobPayloadStagedSource {
    fn byte_len(&self) -> usize;
    fn copy_byte_demand(&self) -> Result<usize, JobPayloadAdmissionFault>;
    fn depth_demand(&self) -> Result<usize, JobPayloadAdmissionFault>;
    fn write_bytes(&self, target: &mut [MaybeUninit<u8>]) -> Result<(), JobPayloadAdmissionFault>;
}
