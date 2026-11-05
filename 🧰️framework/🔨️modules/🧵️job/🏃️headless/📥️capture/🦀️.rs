//! 📥️ Terminal bytes retain their exact source and paid backing through acknowledgement and teardown.
use super::*;
use semio_framework_job::OriginalSourceBinding;
use semio_framework_value::RetirementDemand;

type PayloadWire = (usize, usize, Option<(u64, u64)>);

#[derive(Clone, Copy, PartialEq, Eq)]
struct CaptureSource {
    original: OriginalSourceBinding,
    wire: Option<PayloadWire>,
    pages: usize,
}

impl CaptureSource {
    fn borrow(payload: Option<&RetainedJobPayload>, cx: &StepContext<'_>) -> Result<Self, ValueError> {
        cx.original_source_parent()?.validate_borrowed_context(cx)?;
        let wire = payload.map(RetainedJobPayload::original_wire_identity);
        if wire.is_some_and(|(_, length, operation)| length != 0 && operation != Some((cx.operation().0, cx.generation().0))) {
            return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "headless capture requires its original operation payload"));
        }
        Ok(Self { original: OriginalSourceBinding::borrow(cx), wire, pages: payload.map_or(0, RetainedJobPayload::page_count) })
    }

    fn length(self) -> usize { self.wire.map_or(0, |(_, length, _)| length) }
}

/// 🧺️ Keeps one admitted contiguous buffer and its immutable original page frontier.
pub(super) struct PayloadCapture {
    pub(super) bytes: Option<Vec<u8>>,
    source: Option<CaptureSource>,
    page: usize,
    offset: usize,
    complete: bool,
    closing: bool,
}

impl PayloadCapture {
    pub(super) fn new() -> Self {
        Self { bytes: None, source: None, page: 0, offset: 0, complete: false, closing: false }
    }

    pub(super) fn terminal_is_empty(&self) -> bool { self.bytes.is_none() && self.source.is_none() }

    pub(super) fn retirement_demands(&self) -> RetirementDemand {
        RetirementDemand { release_bytes: self.bytes.as_ref().map_or(0, Vec::capacity), depth: usize::from(!self.terminal_is_empty()), ..Default::default() }
    }

    pub(super) fn advance(&mut self, payload: Option<&RetainedJobPayload>, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        if self.closing { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "closed headless capture cannot resume").into()); }
        let source = CaptureSource::borrow(payload, cx)?;
        if self.source.is_some_and(|original| original != source) {
            return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "headless capture changed its original source or payload").into());
        }
        if self.complete { return Ok(true); }
        let grant = cx.retained_grant();
        let demand = RetirementDemand { capacity_bytes: if self.bytes.is_none() { source.length() } else { 0 }, depth: 1, ..Default::default() };
        if cx.is_cancelled() || !cx.retained_work_can_enter(demand) { return Ok(false); }
        if self.source.is_none() {
            self.source = Some(source);
            Self::receive(cx, RetainedCloneProgress { copied_items: 1, ..Default::default() })?;
            return Ok(false);
        }
        if self.bytes.is_none() {
            let mut bytes = Vec::new();
            if bytes.try_reserve_exact(source.length()).is_err() {
                semio_framework_job::receive_original_retained_work(cx, RetainedWorkOutcome::<RetainedCloneStep, ValueError> { result: Err(ValueError::literal(ValueRefusalKind::AllocationFailed, "headless capture allocation refused")), work: 1 })?;
                return Ok(false);
            }
            let capacity = bytes.capacity();
            self.bytes = Some(bytes);
            self.complete = source.length() == 0;
            Self::receive(cx, RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: capacity, ..Default::default() })?;
            return Ok(self.complete);
        }
        let payload = payload.ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "headless capture lost its original payload"))?;
        let page = payload.page(self.page).ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "headless capture page frontier is unavailable"))?;
        let remaining = page.len().checked_sub(self.offset).ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "headless capture page cursor exceeded its source"))?;
        let count = remaining.min(grant.maximum_copy_bytes);
        if count == 0 && remaining != 0 { return Ok(false); }
        let bytes = self.bytes.as_mut().expect("admitted capture backing");
        let extent_left = source.length().checked_sub(bytes.len()).ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "headless capture cursor exceeded its original extent"))?;
        if count > bytes.capacity() - bytes.len() || count > extent_left {
            return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "headless capture exceeds its original extent or backing").into());
        }
        let completes = self.offset + count == page.len() && self.page + 1 == source.pages;
        if completes && bytes.len() + count != source.length() {
            return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "headless capture pages do not cover the original extent").into());
        }
        bytes.extend_from_slice(&page[self.offset..self.offset + count]);
        self.offset += count;
        if self.offset == page.len() { self.page += 1; self.offset = 0; }
        self.complete = completes;
        Self::receive(cx, RetainedCloneProgress { copied_items: 1, copied_bytes: count, ..Default::default() })?;
        Ok(self.complete)
    }

    fn receive(cx: &mut StepContext<'_>, progress: RetainedCloneProgress) -> Result<(), OriginalTurnFailure> {
        semio_framework_job::receive_original_retained_work(cx, RetainedWorkOutcome::<_, ValueError> { result: Ok(RetainedCloneStep::Progress(progress)), work: 1 })?;
        Ok(())
    }

    pub(super) fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, OriginalTurnFailure> {
        if self.terminal_is_empty() { return Ok(InteractiveJobCloseStep::Complete { progress: Default::default() }); }
        if self.source.is_none_or(|source| !source.original.matches(cx)) {
            return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "headless capture retirement requires its original caller").into());
        }
        semio_framework_job::close_original_granted_step(cx, |grant| {
            let demand = self.retirement_demands();
            if grant.maximum_items == 0 || grant.maximum_depth < demand.depth || grant.maximum_release_bytes < demand.release_bytes {
                return RetainedWorkOutcome { result: Ok(InteractiveJobCloseStep::Pending { progress: Default::default() }), work: 0 };
            }
            self.closing = true;
            self.complete = false;
            if let Some(bytes) = self.bytes.take() {
                let released_bytes = bytes.capacity();
                drop(bytes);
                return RetainedWorkOutcome { result: Ok(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 1, released_bytes, ..Default::default() } }), work: 1 };
            }
            self.source = None;
            self.page = 0;
            self.offset = 0;
            RetainedWorkOutcome { result: Ok(InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress { copied_items: 1, ..Default::default() } }), work: 1 }
        })
    }
}

impl Drop for PayloadCapture {
    fn drop(&mut self) { assert!(std::thread::panicking() || self.terminal_is_empty(), "headless capture requires original paid teardown"); }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
