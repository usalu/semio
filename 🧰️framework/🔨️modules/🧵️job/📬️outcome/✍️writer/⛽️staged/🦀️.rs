//! ⛽️ The original writer quotes every inline header, staged source and committed page transfer before mutation.
use super::*;
use semio_framework_value::RetirementDemand;

impl RetainedJobPayloadWriter {
    pub fn birth_demand() -> RetirementDemand { RetirementDemand { copy_bytes: 2 * size_of::<Self>(), depth: 1, ..Default::default() } }

    pub fn new_funded(stream: JobPayloadStream, cx:&mut impl JobPayloadStep) -> Result<Option<Self>, ValueError> {
        let demand = Self::birth_demand();
        if !writer_permits(cx, demand) { return Ok(None); }
        cx.consume_retained(writer_progress(demand))?;
        Ok(Some(Self::new(stream)))
    }

    pub fn staged_page_birth_demand(&self) -> Result<RetirementDemand, ValueError> {
        let payload = self.payload.as_ref().ok_or_else(writer_sealed)?;
        if self.sealed || self.closing_ledger.is_some() || payload.page_count >= JOB_PAYLOAD_OPERATION_PAGES { return Err(writer_sealed()); }
        Ok(RetirementDemand {
            copy_bytes: size_of::<Option<(Arc<JobPayloadOperationLedger>, JobPayloadPageSource, usize)>>() + size_of::<Option<JobPayloadPageSource>>() + JobPayloadOperationLedger::reservation_copy_bytes(),
            capacity_bytes: if self.staged.is_none() && self.rejected.is_none() { JOB_PAYLOAD_PAGE_BYTES } else { 0 },
            depth: 3,
            ..Default::default()
        })
    }

    pub fn begin_staged_page_funded(&mut self, cx:&mut impl JobPayloadStep) -> Result<bool, ValueError> {
        if self.staged.is_some() { return Ok(true); }
        let demand = self.staged_page_birth_demand()?;
        if !writer_permits(cx, demand) || cx.payload_page_granted() { return Ok(false); }
        let payload = self.payload.as_ref().ok_or_else(writer_sealed)?;
        if payload.ledger.as_ref().is_some_and(|ledger| !Arc::ptr_eq(ledger, cx.payload_ledger())) { return Err(writer_foreign()); }
        let extent = self.rejected.as_ref().map_or(JOB_PAYLOAD_PAGE_BYTES, JobPayloadPageSource::allocated_capacity_bytes);
        cx.payload_ledger().reserve(payload.stream, extent).map_err(writer_fault)?;
        let source = self.rejected.take().unwrap_or_default();
        *self.staged = Some((Arc::clone(cx.payload_ledger()), source, 0));
        cx.grant_payload_page();
        cx.consume_retained(writer_progress(demand))?;
        Ok(true)
    }

    pub fn staged_source_demand(&self, source: &(impl JobPayloadStagedSource + ?Sized), cursor_copy_bytes: usize) -> Result<RetirementDemand, ValueError> {
        if self.staged.is_none() || source.byte_len() > self.staged_page_remaining() { return Err(writer_extent()); }
        let copy_bytes = source.copy_byte_demand().map_err(writer_fault)?.checked_add(source.byte_len()).and_then(|value| value.checked_add(size_of::<usize>())).and_then(|value| value.checked_add(cursor_copy_bytes)).ok_or_else(writer_extent)?;
        let depth = source.depth_demand().map_err(writer_fault)?.checked_add(1).ok_or_else(writer_extent)?;
        Ok(RetirementDemand { copy_bytes, depth, ..Default::default() })
    }

    pub fn write_staged_source_funded(&mut self, cx:&mut impl JobPayloadStep, source: &(impl JobPayloadStagedSource + ?Sized), cursor_copy_bytes: usize) -> Result<bool, ValueError> {
        let demand = self.staged_source_demand(source, cursor_copy_bytes)?;
        if !writer_permits(cx, demand) { return Ok(false); }
        let (ledger, page, length) = self.staged.as_mut().ok_or_else(writer_extent)?;
        if !Arc::ptr_eq(ledger, cx.payload_ledger()) { return Err(writer_foreign()); }
        cx.consume_retained(writer_progress(demand))?;
        let end = *length + source.byte_len();
        source.write_bytes(&mut page.storage[*length..end]).map_err(|fault| writer_fault(fault).with_retained_progress(writer_progress(demand)))?;
        *length = end;
        Ok(true)
    }

    pub fn staged_commit_demand(&self, cursor_copy_bytes: usize) -> Result<RetirementDemand, ValueError> {
        if self.staged.is_none() { return Err(writer_extent()); }
        let copy_bytes = size_of::<Option<(Arc<JobPayloadOperationLedger>, JobPayloadPageSource, usize)>>() + size_of::<Option<JobPayloadPage>>() + size_of::<Option<Arc<JobPayloadOperationLedger>>>() + 2 * size_of::<usize>();
        Ok(RetirementDemand { copy_bytes: copy_bytes.checked_add(cursor_copy_bytes).ok_or_else(writer_extent)?, depth: 3, ..Default::default() })
    }

    pub fn commit_staged_page_funded(&mut self, cx:&mut impl JobPayloadStep, cursor_copy_bytes: usize) -> Result<bool, ValueError> {
        let demand = self.staged_commit_demand(cursor_copy_bytes)?;
        if !writer_permits(cx, demand) { return Ok(false); }
        let (ledger, _, _) = self.staged.as_ref().ok_or_else(writer_extent)?;
        if !Arc::ptr_eq(ledger, cx.payload_ledger()) { return Err(writer_foreign()); }
        let payload = self.payload.as_ref().ok_or_else(writer_sealed)?;
        if self.sealed || payload.page_count >= JOB_PAYLOAD_OPERATION_PAGES || payload.pages[payload.page_count].is_some() { return Err(writer_sealed()); }
        self.commit_staged_page().map_err(writer_fault)?;
        cx.consume_retained(writer_progress(demand))?;
        Ok(true)
    }

    pub fn finish_demand() -> RetirementDemand { RetirementDemand { copy_bytes: 2 * size_of::<Option<Self>>() + 2 * size_of::<Option<RetainedJobPayload>>(), depth: 2, ..Default::default() } }

    pub fn take_finished_demand(cursor_copy_bytes: usize) -> Result<RetirementDemand, ValueError> {
        let copy_bytes = (2 * size_of::<Option<RetainedJobPayload>>() + size_of::<bool>()).checked_add(cursor_copy_bytes).ok_or_else(writer_extent)?;
        Ok(RetirementDemand { copy_bytes, depth: 2, ..Default::default() })
    }

    pub fn take_finished_funded(&mut self, output: &mut Option<RetainedJobPayload>, cx:&mut impl JobPayloadStep, cursor_copy_bytes: usize) -> Result<bool, ValueError> {
        if output.is_some() { return Err(writer_sealed()); }
        let demand = Self::take_finished_demand(cursor_copy_bytes)?;
        if !writer_permits(cx, demand) { return Ok(false); }
        if self.payload.is_none() || self.rejected.is_some() || self.staged.is_some() || self.closing_ledger.is_some() { return Err(writer_sealed()); }
        self.sealed = true;
        *output = self.payload.take();
        cx.consume_retained(writer_progress(demand))?;
        Ok(true)
    }

    pub fn finish_funded(writer: &mut Option<Self>, output: &mut Option<RetainedJobPayload>, cx:&mut impl JobPayloadStep) -> Result<bool, ValueError> {
        if output.is_some() { return Err(writer_sealed()); }
        let demand = Self::finish_demand();
        if !writer_permits(cx, demand) { return Ok(false); }
        let source = writer.as_ref().ok_or_else(writer_sealed)?;
        if source.rejected.is_some() || source.staged.is_some() || source.closing_ledger.is_some() { return Err(writer_sealed()); }
        let source = writer.take().ok_or_else(writer_sealed)?;
        match source.finish() {
            Ok(payload) => { *output = Some(payload); }
            Err(source) => { *writer = Some(source); return Err(writer_sealed()); }
        }
        cx.consume_retained(writer_progress(demand))?;
        Ok(true)
    }
}

impl JobPayloadStagedSource for [u8] {
    fn byte_len(&self) -> usize { self.len() }
    fn copy_byte_demand(&self) -> Result<usize, JobPayloadAdmissionFault> { Ok(self.len()) }
    fn depth_demand(&self) -> Result<usize, JobPayloadAdmissionFault> { Ok(1) }
    fn write_bytes(&self, target: &mut [MaybeUninit<u8>]) -> Result<(), JobPayloadAdmissionFault> {
        if target.len() != self.len() { return Err(JobPayloadAdmissionFault::StreamBytes); }
        for (target, source) in target.iter_mut().zip(self) { target.write(*source); }
        Ok(())
    }
}

fn writer_permits(cx:&impl JobPayloadStep, demand: RetirementDemand) -> bool {
    let grant = cx.retained_grant();
    !cx.is_cancelled() && !cx.should_yield() && grant.maximum_items != 0 && grant.maximum_copy_bytes >= demand.copy_bytes && grant.maximum_capacity_bytes >= demand.capacity_bytes && grant.maximum_release_bytes >= demand.release_bytes && grant.maximum_depth >= demand.depth
}
fn writer_progress(demand: RetirementDemand) -> RetainedCloneProgress { RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, retained_capacity_bytes: demand.capacity_bytes, released_bytes: demand.release_bytes } }
fn writer_sealed() -> ValueError { ValueError::literal(ValueRefusalKind::InvariantViolated, "original staged writer has no writable payload slot") }
fn writer_foreign() -> ValueError { ValueError::literal(ValueRefusalKind::InvariantViolated, "original staged writer requires its original source ledger") }
fn writer_extent() -> ValueError { ValueError::literal(ValueRefusalKind::OwnershipLimit, "original staged source exceeds its quoted extent") }
fn writer_fault(fault: JobPayloadAdmissionFault) -> ValueError {
    ValueError::literal(ValueRefusalKind::OwnershipLimit, match fault {
        JobPayloadAdmissionFault::OperationItems => "original operation page count refused",
        JobPayloadAdmissionFault::OperationBytes => "original operation page bytes refused",
        JobPayloadAdmissionFault::StreamItems => "original stream page count refused",
        JobPayloadAdmissionFault::StreamBytes => "original stream page bytes refused",
        JobPayloadAdmissionFault::ProcessBytes => "original process page bytes refused",
        JobPayloadAdmissionFault::WriterSealed => "original writer is sealed",
        JobPayloadAdmissionFault::WriterFull => "original writer has no page slots",
        JobPayloadAdmissionFault::OpportunityExhausted => "original writer opportunity is exhausted",
        JobPayloadAdmissionFault::RejectedSourcePending => "original writer retains a refused source",
    })
}
