//! 🔄️ A bounded operation conversion keeps the authored wire and final mutation under the same caller.

use super::*;
use semio_framework_value::retained_clone::retained_clone_grant_admits;

/// 🎛️ Supplies the typed conversion of one original operation after its authored wire is received.
pub trait OriginalConvertedOperation: OpBinaryReceiving<DomainFailure = ValueError> {
    type Mirror: DslOwnedReceivingVariants + BorrowedDslVariants + OpBinaryReceiving<
        DomainFailure = ValueError, Receiving = OriginalOperationReceiving<Self::Mirror>,
    >;
    type Conversion: Default + RetireOwned;

    fn conversion_demand(state: &Self::Conversion, input: &Option<Self::Mirror>, body: usize) -> Result<RetirementDemand, ValueError>;

    fn convert_step(
        state: &mut Self::Conversion,
        input: &mut Option<Self::Mirror>,
        output: &mut Option<Self>,
        body: usize,
        native: &mut NativeDecodeControl<'_>,
        effect: &mut RetainedAllocationEffect<'_>,
    ) -> Result<RetainedCloneProgress, ValueError>;
}

/// 📥️ Retains the original receiver and any unfinished conversion in one cursor.
pub struct ConvertedOperationReceiving<M: OriginalConvertedOperation> {
    source: (usize, usize),
    inner: <M::Mirror as OpBinaryReceiving>::Receiving,
    state: OwnedSlot<M::Conversion>,
    input: OwnedSlot<M::Mirror>,
    output: OwnedSlot<M>,
    received: bool,
    closing: bool,
    failed: bool,
}

/// 🎟️ Admits the artifact's wire cursor before allocating or reading any variable field.
pub fn admit_converted<M: OriginalConvertedOperation>(
    source: &[u8],
    range: (usize, usize),
    grant: RetainedCloneGrant,
) -> RetainedWorkOutcome<Option<(ConvertedOperationReceiving<M>, RetainedCloneProgress)>> {
    let turn = M::Mirror::admit_receiving(source, range, grant);
    RetainedWorkOutcome {
        result: turn.result.map(|admitted| admitted.map(|(inner, progress)| {
            let mut state = OwnedSlot::default();
            *state.value = Some(M::Conversion::default());
            (ConvertedOperationReceiving {
                source: (source.as_ptr() as usize, source.len()), inner, state,
                input: OwnedSlot::default(), output: OwnedSlot::default(),
                received: false, closing: false, failed: false,
            }, progress)
        })),
        work: turn.work,
    }
}

impl<M: OriginalConvertedOperation> ConvertedOperationReceiving<M> {
    fn verify_source(&self, source: &[u8]) -> Result<(), ValueError> {
        if self.source == (source.as_ptr() as usize, source.len()) {
            return Ok(());
        }
        Err(invalid("original operation conversion changed its source"))
    }

    fn convert(
        &mut self, source: &[u8], native: &mut NativeDecodeControl<'_>, body: usize,
        effect: &mut RetainedAllocationEffect<'_>,
    ) -> Result<RetainedCloneProgress, OriginalReceivingPrimary> {
        let demand = self.demands(source, body)?;
        let grant = effect.grant();
        if !retained_clone_grant_admits(grant, demand) || effect.maximum_work_units() == 0 {
            return Ok(Default::default());
        }
        if !self.received {
            if !self.inner.is_ready() {
                return self.inner.step_in(source, native, body, effect).result;
            }
            let turn = self.inner.take_into(&mut self.input.value, grant);
            perform(effect, turn.work)?;
            self.received = self.input.value.is_some();
            return Ok(turn.result?);
        }

        let state = self.state.value.as_mut().ok_or_else(|| invalid("original operation conversion state is absent"))?;
        native.checkpoint()?;
        M::convert_step(state, &mut self.input.value, &mut self.output.value, body, native, effect).map_err(Into::into)
    }
}

impl<M: OriginalConvertedOperation> OpBinaryReceivingCursor<M> for ConvertedOperationReceiving<M> {
    const STEP_WORK: u64 = <<M::Mirror as OpBinaryReceiving>::Receiving as OpBinaryReceivingCursor<M::Mirror>>::STEP_WORK;

    fn demands(&self, source: &[u8], body: usize) -> Result<RetirementDemand, ValueError> {
        self.verify_source(source)?;
        if self.closing || self.failed {
            return Err(invalid("original operation conversion requires close"));
        }
        if self.is_ready() {
            return Ok(metadata::<M>());
        }
        if !self.received && self.inner.is_ready() {
            return Ok(metadata::<M::Mirror>());
        }
        if !self.received {
            return self.inner.demands(source, body);
        }
        let state = self.state.value.as_ref().ok_or_else(|| invalid("original operation conversion state is absent"))?;
        M::conversion_demand(state, &self.input.value, body)
    }

    fn step(
        &mut self, source: &[u8], native: &mut NativeDecodeControl<'_>, body: usize,
        grant: RetainedCloneGrant,
    ) -> RetainedWorkOutcome<RetainedCloneProgress, OriginalReceivingPrimary> {
        let demand = match self.demands(source, body) {
            Ok(demand) => demand,
            Err(error) => return RetainedWorkOutcome { result: Err(error.into()), work: 0 },
        };
        if !retained_clone_grant_admits(grant, demand) {
            return RetainedWorkOutcome { result: Ok(Default::default()), work: 0 };
        }
        let received = self.received;
        let step = |control: &mut NativeDecodeControl<'_>, effect: &mut RetainedAllocationEffect<'_>| self.step_in(source, control, body, effect).result;
        let turn = if received {
            native.with_allocated_effect_reported(demand, Self::STEP_WORK, step)
        } else {
            native.with_effect_reported(grant, Self::STEP_WORK, step)
        };
        match turn {
            Ok(Some((result, work))) => RetainedWorkOutcome { result, work },
            Ok(None) => RetainedWorkOutcome { result: Ok(Default::default()), work: 0 },
            Err(error) => {
                self.failed = true;
                RetainedWorkOutcome { result: Err(error.into()), work: 0 }
            }
        }
    }

    fn step_in(
        &mut self, source: &[u8], native: &mut NativeDecodeControl<'_>, body: usize,
        effect: &mut RetainedAllocationEffect<'_>,
    ) -> RetainedWorkOutcome<RetainedCloneProgress, OriginalReceivingPrimary> {
        let result = if self.is_ready() {
            self.verify_source(source).map(|()| Default::default()).map_err(Into::into)
        } else {
            self.convert(source, native, body, effect)
        };
        self.failed = result.is_err();
        RetainedWorkOutcome { result, work: effect.performed_work() }
    }

    fn domain_failure(&self) -> Option<&ValueError> { self.inner.domain_failure() }

    fn domain_failure_demand(&self) -> Result<RetirementDemand, ValueError> { self.inner.domain_failure_demand() }

    fn domain_failure_work_demand(&self) -> Result<u64, ValueError> { self.inner.domain_failure_work_demand() }

    fn take_domain_failure_into(&mut self, recipient: &mut Option<ValueError>, grant: RetainedCloneGrant) -> RetainedWorkOutcome<RetainedCloneProgress> {
        self.inner.take_domain_failure_into(recipient, grant)
    }

    fn is_ready(&self) -> bool { !self.closing && !self.failed && self.output.value.is_some() }

    fn take_into(&mut self, recipient: &mut Option<M>, grant: RetainedCloneGrant) -> RetainedWorkOutcome<RetainedCloneProgress> {
        let demand = metadata::<M>();
        if recipient.is_some() || !self.is_ready() || !retained_clone_grant_admits(grant, demand) {
            return RetainedWorkOutcome { result: Ok(Default::default()), work: 0 };
        }
        *recipient = self.output.value.take();
        self.closing = true;
        RetainedWorkOutcome { result: Ok(receipt(demand)), work: 1 }
    }

    fn close_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if !self.inner.terminal_is_empty() {
            return self.inner.close_demands(body);
        }
        if !self.output.terminal_is_empty() {
            return self.output.close_demands(body);
        }
        if !self.input.terminal_is_empty() {
            return self.input.close_demands(body);
        }
        self.state.close_demands(body)
    }

    fn close_step(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> OriginalRetirementPoll {
        self.closing = true;
        if self.terminal_is_empty() {
            return RetainedWorkPoll::Complete(RetainedWorkOutcome { result: Ok(RetainedCloneStep::Complete(Default::default())), work: 0 });
        }
        if !self.inner.terminal_is_empty() {
            return match self.inner.close_step(effect) {
                RetainedWorkPoll::Complete(turn) => RetainedWorkPoll::Complete(RetainedWorkOutcome {
                    result: turn.result.map(|step| RetainedCloneStep::Progress(step.progress())),
                    work: turn.work,
                }),
                pending => pending,
            };
        }
        if !self.output.terminal_is_empty() {
            return stepped(self.output.close_step(effect));
        }
        if !self.input.terminal_is_empty() {
            return stepped(self.input.close_step(effect));
        }
        stepped(self.state.close_step(effect))
    }

    fn terminal_is_empty(&self) -> bool {
        self.inner.terminal_is_empty() && self.output.terminal_is_empty() && self.input.terminal_is_empty() && self.state.terminal_is_empty()
    }
}

/// ♻️ Drives the same retained close before releasing the conversion cursor.
#[doc(hidden)]
pub struct ConvertedOperationRetirement<M: OriginalConvertedOperation>(ConvertedOperationReceiving<M>);

impl<M: OriginalConvertedOperation> RetireOwned for ConvertedOperationReceiving<M> {
    type RetirementCursor = ConvertedOperationRetirement<M>;

    fn retirement_seed(self) -> Self::RetirementCursor { ConvertedOperationRetirement(self) }
    fn controlled_retirement_supported() -> bool { true }
}

impl<M: OriginalConvertedOperation> ConvertedOperationRetirement<M> {
    fn close_step(&mut self, _body: usize, effect: &mut RetainedAllocationEffect<'_>) -> RetirementStep {
        erased_cursor_turn(self.0.close_step(effect))
    }
}

impl<M: OriginalConvertedOperation> RetirementCursor for ConvertedOperationRetirement<M> {
    semio_framework_value::native_retirement_reply!();

    fn next_computational_work(&self, body: usize) -> Result<u64, ValueError> {
        if !self.0.inner.terminal_is_empty() { return Ok(1); }
        if !self.0.output.terminal_is_empty() { return self.0.output.close_work_demand(body); }
        if !self.0.input.terminal_is_empty() { return self.0.input.close_work_demand(body); }
        self.0.state.close_work_demand(body)
    }

    fn terminal_is_empty(&self) -> bool { self.0.terminal_is_empty() }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.0.close_demands(usize::MAX)?.depth) }
    fn next_work_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.0.close_demands(usize::MAX)?.copy_bytes) }
    fn next_selected_work_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Ok(self.0.close_demands(body)?.copy_bytes) }
    fn next_close_byte_demand(&self) -> Option<usize> { self.0.close_demands(usize::MAX).ok().map(|demand| demand.release_bytes) }
    fn next_birth_bytes(&self, body: usize) -> Option<usize> { self.0.close_demands(body).ok().map(|demand| demand.capacity_bytes) }
    fn terminal_release_bytes(&self) -> Option<usize> { Some(size_of::<Self>()) }
}

/// 🔄️ Registers the owned mirror followed by its artifact-defined paid conversion.
#[macro_export]
macro_rules! original_converted_operation_receiving {
    ($operation:ty) => {
        impl $crate::original_receiving::OpBinaryReceiving for $operation {
            type DomainFailure = $crate::original_receiving::ValueError;
            type Receiving = $crate::original_receiving::converted::ConvertedOperationReceiving<Self>;

            fn admit_receiving(
                source: &[u8], range: (usize, usize), grant: $crate::original_receiving::RetainedCloneGrant,
            ) -> $crate::original_receiving::RetainedWorkOutcome<Option<(Self::Receiving, $crate::original_receiving::RetainedCloneProgress)>> {
                $crate::original_receiving::converted::admit_converted::<Self>(source, range, grant)
            }
        }
    };
}

