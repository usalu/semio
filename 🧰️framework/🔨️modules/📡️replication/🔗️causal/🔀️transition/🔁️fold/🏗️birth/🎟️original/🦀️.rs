//! 🎟️ History coroutines retain original runners and decoder lineage through construction and closure.
use super::super::*;
use semio_framework_value_native_owned::{native_decoding::NativeDecodeControl, native_decoding::NativeDecodeCloseControl, native_decoding::OriginalReceivingPrimary, native_decoding::NativeConstructionStepOutcome, native_decoding::NativeConstructionStepResult};

fn invalid(message: &'static str) -> ValueError { ValueError::literal(ValueRefusalKind::InvariantViolated, message) }

fn shared_bytes<T>() -> usize { std::alloc::Layout::new::<[usize; 2]>().extend(std::alloc::Layout::new::<T>()).unwrap().0.pad_to_align().size() }

impl<'a, T: RetireOwned> HistoryFoldJob<'a, T> {
    /// 📐️ Quotes the concrete future, fixed retirement directory and actual shared control headers.
    pub fn original_birth_demand<R, F>(_: &R) -> Result<RetirementDemand, ValueError>
    where R: FnOnce(HistoryFoldControl) -> F, F: Future<Output = Result<T, semio_framework_replication_error::ProtocolError>> + Send + 'a {
        let capacity_bytes = shared_bytes::<FoldControlState>().checked_add(128 * size_of::<Box<dyn ErasedSnapshotRetirement>>()).and_then(|bytes| bytes.checked_add(size_of::<F>())).and_then(|bytes| bytes.checked_add(shared_bytes::<FoldWake>())).ok_or_else(|| invalid("history fold original construction size overflow"))?;
        let copy_bytes = size_of::<Self>().checked_mul(2).and_then(|bytes| bytes.checked_add(size_of::<R>())).ok_or_else(|| invalid("history fold original construction copy overflow"))?;
        Ok(RetirementDemand { copy_bytes, capacity_bytes, depth: 1, ..Default::default() })
    }

    /// 🫴️ The runner remains original on refusal; the admitted job precedes every receipt response.
    pub fn admit_original<R, F>(runner: &mut Option<R>, output: &mut Option<Self>, native: &mut NativeDecodeControl<'_>) -> NativeConstructionStepOutcome<bool, OriginalReceivingPrimary>
    where R: FnOnce(HistoryFoldControl) -> F, F: Future<Output = Result<T, semio_framework_replication_error::ProtocolError>> + Send + 'a {
        let demand = match runner.as_ref().ok_or_else(|| invalid("history fold requires its original runner")).and_then(Self::original_birth_demand) {
            Ok(demand) if output.is_none() => demand,
            Ok(_) => return NativeConstructionStepOutcome { result: Some(Err(invalid("history fold destination is occupied").into())), receipt_refusal: None, progress: Default::default() },
            Err(error) => return NativeConstructionStepOutcome { result: Some(Err(error.into())), receipt_refusal: None, progress: Default::default() },
        };
        native.receive_construction_step_typed(demand, |native, _| {
            if native.owned_bytes().checked_add(demand.capacity_bytes).is_none_or(|bytes| bytes > native.maximum_bytes()) { return NativeConstructionStepResult { result: Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "history fold birth exceeds its original decoder ceiling").into()), progress: Default::default() }; }
            let mut job = Self::new(runner.take().unwrap());
            job.native_binding = Some((native.ledger_identity(), native.retirement_recipient_identity()));
            *output = Some(job);
            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, retained_capacity_bytes: demand.capacity_bytes, released_bytes: 0 };
            let allocation_refusal = native.record_constructed_allocation(progress.retained_capacity_bytes).err();
            let result = if allocation_refusal.is_none() { Ok(true) } else { Err(OriginalReceivingPrimary { primary: None, quote_refusal: None, allocation_refusal, progress }) };
            NativeConstructionStepResult { result, progress }
        })
    }

    fn validate_original(&self, native: &dyn NativeDecodeCloseControl) -> Result<(), ValueError> {
        if self.native_binding != Some((native.ledger_identity(), native.retirement_recipient_identity())) { return Err(invalid("history fold changed its original receiving source")); }
        Ok(())
    }

    /// 🪜️ Runs one original transition; completed output remains resident through every refused receipt.
    pub fn step_original(&mut self, native: &mut NativeDecodeControl<'_>, body: usize) -> NativeConstructionStepOutcome<HistoryFoldJobStep<()>, OriginalReceivingPrimary> {
        if let Err(error) = self.validate_original(native) { return NativeConstructionStepOutcome { result: Some(Err(error.into())), receipt_refusal: None, progress: Default::default() }; }
        if self.original_output.is_some() { return NativeConstructionStepOutcome { result: Some(Ok(HistoryFoldJobStep::Ready { value: (), progress: Default::default() })), receipt_refusal: None, progress: Default::default() }; }
        let ready_copy = if !self.control_closed && self.future.is_none() && self.waker.is_none() && self.control.empty() && self.control.0.retirements.lock().capacity() == 0 && matches!(self.result, Some(Ok(_))) { 2 * size_of::<ControlledRetirement<T>>() } else { 0 };
        let demand = match self.original_step_demand(body) {
            Ok(demand) => demand,
            Err(error) => return NativeConstructionStepOutcome { result: Some(Err(error.into())), receipt_refusal: None, progress: Default::default() },
        };
        native.receive_construction_step_typed(demand, |native, grant| {
            let result = self.step_granted(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes - ready_copy, maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth }, body, &mut || false);
            let mut progress = match &result {
                Ok(HistoryFoldJobStep::Pending { progress, .. } | HistoryFoldJobStep::Ready { progress, .. } | HistoryFoldJobStep::Rejected { progress }) => *progress,
                Err(error) => error.retained_progress(),
            };
            let result = result.map(|step| match step {
                HistoryFoldJobStep::Ready { value, .. } => {
                    self.original_output = Some(ControlledRetirement::retain(value));
                    self.terminal = false;
                    progress.copied_bytes += ready_copy;
                    HistoryFoldJobStep::Ready { value: (), progress }
                }
                HistoryFoldJobStep::Pending { completed, released_bytes, progress } => HistoryFoldJobStep::Pending { completed, released_bytes, progress },
                HistoryFoldJobStep::Rejected { progress } => HistoryFoldJobStep::Rejected { progress },
            });
            let quote_refusal = if progress.fits(grant) && progress.copied_bytes <= demand.copy_bytes && progress.retained_capacity_bytes <= demand.capacity_bytes && progress.released_bytes <= demand.release_bytes { None } else { Some(invalid("history fold transition exceeded its declared physical effects").with_retained_progress(progress)) };
            let allocation_refusal = if progress.retained_capacity_bytes == 0 { None } else { native.record_constructed_allocation(progress.retained_capacity_bytes).err() };
            let result = match result {
                Ok(step) if quote_refusal.is_none() && allocation_refusal.is_none() => Ok(step),
                result => Err(OriginalReceivingPrimary { primary: result.err(), quote_refusal, allocation_refusal, progress }),
            };
            NativeConstructionStepResult { result, progress }
        })
    }

    /// 📏️ Borrows the next original coroutine action including its final retained output handoff.
    pub fn original_step_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
        let ready_copy=if !self.control_closed&&self.future.is_none()&&self.waker.is_none()&&self.control.empty()&&self.control.0.retirements.lock().capacity()==0&&matches!(self.result,Some(Ok(_))){2*size_of::<ControlledRetirement<T>>()}else{0};
        self.next_step_grant(1,body).and_then(|grant|Ok(RetirementDemand{copy_bytes:grant.maximum_copy_bytes.checked_add(ready_copy).ok_or_else(||invalid("original history output copy overflow"))?,capacity_bytes:grant.maximum_capacity_bytes,release_bytes:grant.maximum_release_bytes,depth:grant.maximum_depth}))
    }

    /// 👁️ Observes whether the same original native output is ready for its funded handoff.
    pub fn original_output_ready(&self)->bool{self.original_output.is_some()}
    /// 🧯️ Observes an original refusal only after all producer cleanup has completed.
    pub fn original_rejection_ready(&self)->bool{self.control_closed&&matches!(self.result,Some(Err(_)))}

    /// 📬️ A separate funded header handoff installs the actual output before receipt acceptance.
    pub fn take_original_output(&mut self, output: &mut Option<T>, native: &mut NativeDecodeControl<'_>) -> NativeConstructionStepOutcome<bool, OriginalReceivingPrimary> {
        let valid = self.validate_original(native).and_then(|()| if output.is_none() && self.original_output.as_ref().is_some_and(ControlledRetirement::original_is_untouched) { Ok(()) } else { Err(invalid("original history fold output is absent, retiring or its destination is occupied")) });
        if let Err(error) = valid { return NativeConstructionStepOutcome { result: Some(Err(error.into())), receipt_refusal: None, progress: Default::default() }; }
        let demand = Self::original_output_demand();
        native.receive_construction_step_typed(demand, |_, _| {
            *output = self.original_output.as_mut().unwrap().take_original();
            self.original_output = None;
            self.terminal = self.control_closed && self.future.is_none() && self.result.is_none() && self.waker.is_none();
            NativeConstructionStepResult { result: Ok(true), progress: RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() } }
        })
    }

    /// 📦️ Quotes the unchanged output header before the original decoder is borrowed for its transfer.
    pub fn original_output_demand()->RetirementDemand{RetirementDemand{copy_bytes:2*size_of::<T>()+2*size_of::<Option<T>>(),depth:1,..Default::default()}}

    /// 📏️ Completion and post-effect refusal keep actual output inside the original job until handoff or closure.
    pub fn next_original_close_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if self.terminal_is_empty(){return Ok(Default::default())}
        if let Some(owner)=&self.original_output{if owner.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<ControlledRetirement<T>>>()+size_of::<bool>(),depth:1,..Default::default()})}let mut demand=owner.next_demand(body)?;demand.depth=demand.depth.max(1);return Ok(demand)}
        if self.control_closed&&self.future.is_none()&&self.result.is_none()&&self.waker.is_none(){return Ok(RetirementDemand{copy_bytes:size_of::<bool>(),depth:1,..Default::default()})}
        let mut demand=self.next_demand(body)?;demand.depth=demand.depth.max(1);Ok(demand)
    }

    /// ♻️ Closing borrows the original decoder even after cancellation, without creating fresh authority.
    pub fn close_original(&mut self, native: &mut dyn NativeDecodeCloseControl, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        self.validate_original(native)?;
        let demand = self.next_original_close_demand(grant.maximum_copy_bytes)?;
        if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant, demand) { return Ok(RetainedCloneStep::Progress(Default::default())); }
        native.charge(demand.capacity_bytes)?;
        if let Some(owner) = self.original_output.as_mut() {
            if owner.terminal_is_empty(){self.original_output=None;self.terminal=self.control_closed&&self.future.is_none()&&self.result.is_none()&&self.waker.is_none();return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))}
            return owner.step(grant).map(|step|RetainedCloneStep::Progress(step.progress()));
        }
        if self.control_closed&&self.future.is_none()&&self.result.is_none()&&self.waker.is_none(){self.terminal=true;return Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))}
        self.close_step(grant)
    }
}
