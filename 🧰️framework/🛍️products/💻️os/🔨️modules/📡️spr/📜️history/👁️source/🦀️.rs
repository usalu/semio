//! 👁️ One immutable original history source owns the backing until every controlled fold alias closes.
use super::{HistoryLog, RetainedHistoryFold};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}, retirement::arc_custody::{SharedControlledRetirement, arc_bytes}};
use semio_framework_value_native_owned::{native_decoding::NativeDecodeControl, native_decoding::NativeConstructionStepOutcome, native_decoding::NativeConstructionStepResult, native_decoding::NativeDecodeCloseControl, native_decoding::OriginalReceivingPrimary};
use std::{mem::{ManuallyDrop, size_of}, sync::Arc};

pub struct OriginalHistorySource {
    source: ManuallyDrop<Option<Arc<HistoryLog>>>,
    closing: ManuallyDrop<Option<SharedControlledRetirement<HistoryLog>>>,
    binding: (u64, Option<u64>),
}

fn invalid(message: &'static str) -> ValueError { ValueError::literal(ValueRefusalKind::InvariantViolated, message) }
fn refused<T>(error: ValueError) -> NativeConstructionStepOutcome<T, OriginalReceivingPrimary> { NativeConstructionStepOutcome { result: Some(Err(error.into())), receipt_refusal: None, progress: Default::default() } }

impl OriginalHistorySource {
    /// 📐️ The defining Arc layout and fixed owner moves are quoted before consuming the original history.
    pub fn birth_demand() -> RetirementDemand { RetirementDemand { copy_bytes: 2 * size_of::<Self>() + size_of::<HistoryLog>(), capacity_bytes: arc_bytes::<HistoryLog>(), depth: 1, ..Default::default() } }

    /// 🫴️ Refusal preserves the input; a born source is installed before any accounting or receipt failure.
    pub fn admit(history: &mut Option<HistoryLog>, output: &mut Option<Self>, native: &mut NativeDecodeControl<'_>) -> NativeConstructionStepOutcome<bool, OriginalReceivingPrimary> {
        if history.is_none() || output.is_some() { return refused(invalid("original history source requires an occupied input and empty destination")); }
        let demand = Self::birth_demand();
        native.receive_construction_step_typed(demand, |native, _| {
            if native.owned_bytes().checked_add(demand.capacity_bytes).is_none_or(|bytes| bytes > native.maximum_bytes()) { return NativeConstructionStepResult { result: Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "history shared source exceeds its original decoder ceiling").into()), progress: Default::default() }; }
            *output = Some(Self { source: ManuallyDrop::new(Some(Arc::new(history.take().unwrap()))), closing: ManuallyDrop::new(None), binding: (native.ledger_identity(), native.retirement_recipient_identity()) });
            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, retained_capacity_bytes: demand.capacity_bytes, ..Default::default() };
            let allocation_refusal = native.record_constructed_allocation(progress.retained_capacity_bytes).err();
            NativeConstructionStepResult { result: if allocation_refusal.is_none() { Ok(true) } else { Err(OriginalReceivingPrimary { primary: None, quote_refusal: None, allocation_refusal, progress }) }, progress }
        })
    }

    fn validate(&self, native: &dyn NativeDecodeCloseControl) -> Result<(), ValueError> {
        if self.binding != (native.ledger_identity(), native.retirement_recipient_identity()) { return Err(invalid("history source changed its original receiving authority")); }
        Ok(())
    }

    /// 📖️ Readers borrow the retained immutable root without acquiring an unpriced alias.
    pub fn history(&self) -> Option<&HistoryLog> { self.source.as_deref() }

    /// 🎞️ The sole fold alias is born inside the actual funded coroutine constructor.
    pub fn admit_fold(&self, shape: protocol::HistoryShape, output: &mut Option<protocol::HistoryFoldJob<'static, RetainedHistoryFold>>, native: &mut NativeDecodeControl<'_>) -> NativeConstructionStepOutcome<bool, OriginalReceivingPrimary> {
        if let Err(error) = self.validate(native) { return refused(error); }
        let Some(source) = self.source.as_ref() else { return refused(invalid("history source is already closing")); };
        let mut runner = Some(|control| { let source = Arc::clone(source); async move { source.fold_controlled(shape, &control).await } });
        protocol::HistoryFoldJob::admit_original(&mut runner, output, native)
    }

    /// 📬️ Only the final unique owner can transfer the same history after every fold alias has closed.
    pub fn take_history(&mut self, output: &mut Option<HistoryLog>, native: &mut NativeDecodeControl<'_>) -> NativeConstructionStepOutcome<bool, OriginalReceivingPrimary> {
        if let Err(error) = self.validate(native) { return refused(error); }
        if output.is_some() || self.closing.is_some() || self.source.is_none() { return refused(invalid("original history handoff requires the resident root and an empty destination")); }
        let source = self.source.as_ref().unwrap();
        if Arc::strong_count(source) != 1 || Arc::weak_count(source) != 0 { return NativeConstructionStepOutcome { result: None, receipt_refusal: None, progress: Default::default() }; }
        let demand = RetirementDemand { copy_bytes: 2 * size_of::<HistoryLog>() + size_of::<Option<Arc<HistoryLog>>>(), release_bytes: arc_bytes::<HistoryLog>(), depth: 1, ..Default::default() };
        native.receive_construction_step_typed(demand, |_, _| {
            match Arc::try_unwrap(self.source.take().unwrap()) {
                Ok(history) => { *output = Some(history); NativeConstructionStepResult { result: Ok(true), progress: RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, released_bytes: demand.release_bytes, ..Default::default() } } }
                Err(source) => { *self.source = Some(source); NativeConstructionStepResult { result: Ok(false), progress: Default::default() } }
            }
        })
    }

    /// 📏️ Shared-source retirement starts with a heapless, separately quoted fixed owner move.
    pub fn next_close_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if self.source.is_some() { return Ok(RetirementDemand { copy_bytes: size_of::<Arc<HistoryLog>>() + size_of::<SharedControlledRetirement<HistoryLog>>(), depth: 1, ..Default::default() }); }
        self.closing.as_ref().map_or(Ok(Default::default()), |owner| owner.next_demand(body))
    }

    /// ♻️ Cancellation borrows the original close authority and waits for live fold aliases before releasing history.
    pub fn close_original(&mut self, native: &mut dyn NativeDecodeCloseControl, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        self.validate(native)?;
        let demand = self.next_close_demand(grant.maximum_copy_bytes)?;
        if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant, demand) { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if let Some(source) = self.source.take() {
            *self.closing = Some(SharedControlledRetirement::new(source));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }));
        }
        if let Some(owner) = self.closing.as_mut() {
            native.charge(demand.capacity_bytes)?;
            let step = owner.step(grant)?;
            if owner.terminal_is_empty() { *self.closing = None; }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        Ok(RetainedCloneStep::Complete(Default::default()))
    }

    pub fn terminal_is_empty(&self) -> bool { self.source.is_none() && self.closing.is_none() }
}

impl Drop for OriginalHistorySource {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "original history source abandoned its shared allocation");
        if self.terminal_is_empty() { unsafe { ManuallyDrop::drop(&mut self.source); ManuallyDrop::drop(&mut self.closing); } }
    }
}
