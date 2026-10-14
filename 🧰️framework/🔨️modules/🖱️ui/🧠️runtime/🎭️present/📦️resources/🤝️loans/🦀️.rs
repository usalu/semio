//! 🤝️ Paid local native page loans travel beside document aliases and prepared mesh uploads.

use super::*;

#[path = "../📤️transport/🦀️.rs"]
mod transport;
pub use transport::*;

pub enum UiNativeResourceClaim<'a> {
    Owner(&'a Mesh3dResourceOwner),
    Loan(&'a Mesh3dResourceLoan),
}

pub struct UiNativeResourceSource<'a> {
    pub surface: &'a ui_contract::UiText,
    pub mesh_key: &'a ui_contract::UiText,
    pub claim: UiNativeResourceClaim<'a>,
}

impl UiNativeResourceClaim<'_> {
    fn identity(&self) -> Mesh3dResourceIdentity { match self { Self::Owner(source) => source.identity(), Self::Loan(source) => source.identity() } }
    fn loan(&self, grant: RetainedCloneGrant) -> Result<Option<(Mesh3dResourceLoan, RetainedCloneProgress)>, Mesh3dFault> { match self { Self::Owner(source) => source.loan(grant), Self::Loan(source) => source.loan(grant) } }
}

pub trait UiNativeResourceLoanSource {
    fn native_resource_count(&self) -> usize;
    fn native_resource_source_identity(&self) -> Option<usize>;
    fn native_resource_source(&self, index: usize) -> Option<UiNativeResourceSource<'_>>;
}

impl UiNativeResourceLoanSource for UiNativeResources {
    fn native_resource_count(&self) -> usize { self.rows.len() }
    fn native_resource_source_identity(&self) -> Option<usize> { self.rows.get(0).map(|row| row as *const UiNativeMeshResource as usize) }
    fn native_resource_source(&self, index: usize) -> Option<UiNativeResourceSource<'_>> {
        self.rows.get(index).map(|row| UiNativeResourceSource { surface: &row.surface, mesh_key: &row.mesh_key, claim: UiNativeResourceClaim::Owner(&row.owner) })
    }
}

#[derive(Debug)]
pub struct UiNativeMeshLoan {
    pub surface: ui_contract::UiText,
    pub mesh_key: ui_contract::UiText,
    pub loan: Mesh3dResourceLoan,
}

#[derive(Debug, Default)]
pub struct UiNativeResourceLoans {
    rows: ui_contract::UiFixedList<UiNativeMeshLoan, UI_NATIVE_RESOURCE_CAPACITY>,
    source: Option<(Option<usize>, usize)>,
    next_source: usize,
    pending: Option<UiNativeMeshLoan>,
    phase: u8,
    copied: usize,
    complete: bool,
    closing: bool,
    pending_failure: Option<UiNativeResourceTurnFailure>,
}

impl UiNativeResourceLoanSource for UiNativeResourceLoans {
    fn native_resource_count(&self) -> usize { self.rows.len() }
    fn native_resource_source_identity(&self) -> Option<usize> { self.rows.get(0).map(|row| row as *const UiNativeMeshLoan as usize) }
    fn native_resource_source(&self, index: usize) -> Option<UiNativeResourceSource<'_>> {
        self.rows.get(index).map(|row| UiNativeResourceSource { surface: &row.surface, mesh_key: &row.mesh_key, claim: UiNativeResourceClaim::Loan(&row.loan) })
    }
}

impl UiNativeResourceLoans {
    pub fn len(&self) -> usize { self.rows.len() }

    pub fn row(&self, index: usize) -> Option<&UiNativeMeshLoan> { self.rows.get(index) }

    pub fn capture_complete(&self) -> bool { self.complete && !self.closing }

    pub fn capture_step<S: UiNativeResourceLoanSource + ?Sized>(&mut self, source: &S, grant: RetainedCloneGrant) -> Result<Mesh3dOwnershipStep, UiNativeResourceFailure> {
        if self.closing { return Err(UiNativeResourceFailure::Mesh(Mesh3dFault::Closing)); }
        if self.complete { return Ok(Mesh3dOwnershipStep { complete: true, ..Default::default() }); }
        let identity = (source.native_resource_source_identity(), source.native_resource_count());
        if self.source.is_some_and(|original| original != identity) { return Err(UiNativeResourceFailure::Mesh(Mesh3dFault::Stale)); }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(Mesh3dOwnershipStep::default()); }
        if self.source.is_none() {
            self.source = Some(identity);
            return Ok(Mesh3dOwnershipStep { progress: RetainedCloneProgress { copied_items: 1, ..Default::default() }, ..Default::default() });
        }
        if self.next_source == identity.1 {
            self.source = None;
            self.complete = true;
            return Ok(Mesh3dOwnershipStep { complete: true, progress: RetainedCloneProgress { copied_items: 1, ..Default::default() } });
        }
        let original = source.native_resource_source(self.next_source).ok_or(UiNativeResourceFailure::Mesh(Mesh3dFault::Stale))?;
        if let Some(pending) = self.pending.as_ref() {
            if pending.loan.identity() != original.claim.identity() { return Err(UiNativeResourceFailure::Mesh(Mesh3dFault::Stale)); }
        } else {
            let Some((loan, progress)) = original.claim.loan(grant).map_err(UiNativeResourceFailure::Mesh)? else { return Ok(Mesh3dOwnershipStep::default()); };
            self.pending = Some(UiNativeMeshLoan { surface: Default::default(), mesh_key: Default::default(), loan });
            self.phase = 0;
            self.copied = 0;
            return Ok(Mesh3dOwnershipStep { progress, ..Default::default() });
        }
        if self.phase < 2 {
            let text = if self.phase == 0 { original.surface.as_str() } else { original.mesh_key.as_str() };
            let end = self.copied.checked_add(grant.maximum_copy_bytes.min(text.len().saturating_sub(self.copied))).ok_or(UiNativeResourceFailure::Mesh(Mesh3dFault::ByteCapacity))?;
            let mut end = end;
            while end > self.copied && !text.is_char_boundary(end) { end -= 1; }
            if end == self.copied && self.copied != text.len() { return Ok(Mesh3dOwnershipStep::default()); }
            let pending = self.pending.as_mut().unwrap();
            let target = if self.phase == 0 { &mut pending.surface } else { &mut pending.mesh_key };
            std::fmt::Write::write_str(target, &text[self.copied..end]).map_err(|_| UiNativeResourceFailure::Mesh(Mesh3dFault::ByteCapacity))?;
            let bytes = end - self.copied;
            self.copied = end;
            if self.copied == text.len() { self.phase += 1; self.copied = 0; }
            return Ok(Mesh3dOwnershipStep { progress: RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() }, ..Default::default() });
        }
        if !self.rows.has_reserved_slot() {
            let bytes = self.rows.next_allocation_bytes().map_err(UiNativeResourceFailure::Storage)?;
            if grant.maximum_capacity_bytes < bytes { return Ok(Mesh3dOwnershipStep::default()); }
            let step = self.rows.try_reserve_one(grant.maximum_capacity_bytes).map_err(UiNativeResourceFailure::Allocation)?;
            return Ok(Mesh3dOwnershipStep { progress: RetainedCloneProgress { copied_items: usize::from(step.progressed), retained_capacity_bytes: step.allocated_bytes, ..Default::default() }, ..Default::default() });
        }
        if let Err(original) = self.rows.try_push_reserved(self.pending.take().unwrap()) {
            self.pending = Some(original);
            return Err(UiNativeResourceFailure::Mesh(Mesh3dFault::ItemCapacity));
        }
        self.next_source += 1;
        Ok(Mesh3dOwnershipStep { progress: RetainedCloneProgress { copied_items: 1, ..Default::default() }, ..Default::default() })
    }

    pub fn capture_step_on_original<S: UiNativeResourceLoanSource + ?Sized>(&mut self, source: &S, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<bool>, semio_framework_job::OriginalTurnFailure> {
        if self.pending_failure.is_some() { return admit_native_resource_failure(&mut self.pending_failure, cx); }
        if self.capture_complete() { return Ok(Some(true)); }
        let grant = cx.retained_grant();
        if cx.should_yield() || cx.is_cancelled() || grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(None); }
        if self.pending.is_some() && self.phase >= 2 && !self.rows.has_reserved_slot() {
            let demand = match self.rows.next_allocation_bytes() {
                Ok(demand) => demand,
                Err(primary) => { self.pending_failure = Some(UiNativeResourceTurnFailure::Native { primary: UiNativeResourceFailure::Storage(primary), receipt_refusal: None }); return admit_native_resource_failure(&mut self.pending_failure, cx); }
            };
            if demand > grant.maximum_capacity_bytes { return Ok(None); }
        }
        let result = self.capture_step(source, grant);
        let progress = match &result { Ok(step) => step.progress, Err(primary) => primary.retained_progress() };
        if progress != RetainedCloneProgress::default() || result.is_err() { cx.consume_fuel(1); }
        let receipt_refusal = cx.consume_retained(progress).err();
        match (result, receipt_refusal) {
            (Ok(_), None) => Ok(Some(self.capture_complete())),
            (Err(primary), receipt_refusal) => { self.pending_failure = Some(UiNativeResourceTurnFailure::Native { primary, receipt_refusal }); Ok(None) },
            (Ok(_), Some(primary)) => { self.pending_failure = Some(UiNativeResourceTurnFailure::Receipt(primary)); Ok(None) },
        }
    }

    pub fn close_demands(&self, _work: usize) -> Result<RetirementDemand, UiNativeResourceFailure> {
        if self.pending.is_some() || self.source.is_some() { return Ok(RetirementDemand { depth: 1, ..Default::default() }); }
        if self.rows.len() != 0 { return Ok(RetirementDemand { depth: self.rows.next_pop_depth_demand().map_err(UiNativeResourceFailure::Storage)?, ..Default::default() }); }
        Ok(RetirementDemand { release_bytes: self.rows.next_release_allocation_bytes().map_err(UiNativeResourceFailure::Storage)?, depth: self.rows.next_release_depth_demand().map_err(UiNativeResourceFailure::Storage)?, ..Default::default() })
    }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<Mesh3dOwnershipStep, UiNativeResourceFailure> {
        if self.terminal_is_empty() { return Ok(Mesh3dOwnershipStep { complete: true, ..Default::default() }); }
        let demand = self.close_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_items == 0 || grant.maximum_depth < demand.depth || grant.maximum_release_bytes < demand.release_bytes { return Ok(Mesh3dOwnershipStep::default()); }
        self.closing = true;
        if let Some(pending) = self.pending.as_mut() {
            if !pending.loan.terminal_is_empty() { let mut step = pending.loan.return_step(grant).map_err(UiNativeResourceFailure::Mesh)?; step.complete = false; return Ok(step); }
            self.pending = None;
            return Ok(Mesh3dOwnershipStep { progress: RetainedCloneProgress { copied_items: 1, ..Default::default() }, ..Default::default() });
        }
        if self.source.take().is_some() { return Ok(Mesh3dOwnershipStep { progress: RetainedCloneProgress { copied_items: 1, ..Default::default() }, ..Default::default() }); }
        if let Some(row) = self.rows.last_mut() {
            if !row.loan.terminal_is_empty() { let mut step = row.loan.return_step(grant).map_err(UiNativeResourceFailure::Mesh)?; step.complete = false; return Ok(step); }
            self.rows.pop();
            return Ok(Mesh3dOwnershipStep { progress: RetainedCloneProgress { copied_items: 1, ..Default::default() }, ..Default::default() });
        }
        let step = self.rows.release_empty_page(grant.maximum_release_bytes).map_err(UiNativeResourceFailure::Storage)?;
        Ok(Mesh3dOwnershipStep { complete: self.terminal_is_empty(), progress: RetainedCloneProgress { copied_items: usize::from(step.progressed), released_bytes: step.released_allocation_bytes, ..Default::default() } })
    }

    pub fn close_step_on_original(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<bool>, semio_framework_job::OriginalTurnFailure> {
        if self.pending_failure.is_some() { return admit_native_resource_failure(&mut self.pending_failure, cx); }
        if self.terminal_is_empty() { return Ok(Some(true)); }
        let grant = cx.retained_grant();
        if cx.should_yield() || grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(None); }
        let demand = match self.close_demands(grant.maximum_copy_bytes) { Ok(demand) => demand, Err(primary) => { self.pending_failure = Some(UiNativeResourceTurnFailure::Native { primary, receipt_refusal: None }); return admit_native_resource_failure(&mut self.pending_failure, cx); } };
        if demand.copy_bytes > grant.maximum_copy_bytes || demand.capacity_bytes > grant.maximum_capacity_bytes || demand.release_bytes > grant.maximum_release_bytes || demand.depth > grant.maximum_depth { return Ok(None); }
        let result = self.close_step(grant);
        let progress = match &result { Ok(step) => step.progress, Err(primary) => primary.retained_progress() };
        if progress != RetainedCloneProgress::default() || result.is_err() { cx.consume_fuel(1); }
        let receipt_refusal = cx.consume_retained(progress).err();
        match (result, receipt_refusal) {
            (Ok(_), None) => Ok(Some(self.terminal_is_empty())),
            (Err(primary), receipt_refusal) => { self.pending_failure = Some(UiNativeResourceTurnFailure::Native { primary, receipt_refusal }); Ok(None) },
            (Ok(_), Some(primary)) => { self.pending_failure = Some(UiNativeResourceTurnFailure::Receipt(primary)); Ok(None) },
        }
    }

    pub fn terminal_is_empty(&self) -> bool { self.rows.terminal_is_empty() && self.pending.is_none() && self.source.is_none() && self.pending_failure.is_none() }
}

impl Drop for UiNativeResourceLoans {
    fn drop(&mut self) { assert!(std::thread::panicking() || self.terminal_is_empty(), "native resource loans dropped before original paid return"); }
}
