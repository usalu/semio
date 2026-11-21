//! 🏷️ Authored tagged Pack operations move their payload into original typed receiving.

use super::*;
use semio_framework_dsl_record::{FieldValue, receiving::owned::{NativeSourceOrigin, NativeSourceReceiving, SourceReceived}};

type TaggedValue = (String, DslValue);

struct TaggedOrigin;

impl NativeSourceOrigin<TaggedValue> for TaggedOrigin {
    const RECORD_FIELD: Option<u16> = None;

    fn check(input: &Option<FieldValue>) -> Result<(), ValueError> {
        match input {
            Some(FieldValue::Value(DslValue::Object(fields))) if fields.len() == 1 => Ok(()),
            _ => Err(invalid("original tagged operation requires one variant")),
        }
    }

    fn extract(input: &mut Option<FieldValue>) -> Result<TaggedValue, ValueError> {
        Self::check(input)?;
        match input {
            Some(FieldValue::Value(DslValue::Object(fields))) => fields.pop().ok_or_else(|| invalid("original tagged operation variant is absent")),
            _ => Err(ValueError::invariant("original tagged operation source changed")),
        }
    }
}

type Binding<M> = NativeSourceReceiving<TaggedValue, M, TaggedOrigin>;

/// 🧳️ Receives the exact wire body before publishing the original typed operation over bounded loans.
pub struct OriginalTaggedPackValueReceiving<M: SourceReceived<TaggedValue>> {
    wire: OriginalPackValueReceiving<DslValue>,
    binding: Option<Binding<M>>,
    output: OwnedSlot<M>,
    took: bool,
    closing: bool,
    closed: bool,
    failed: bool,
}

type Turn = RetainedWorkOutcome<RetainedCloneProgress, OriginalReceivingPrimary>;

fn denied() -> Turn {
    RetainedWorkOutcome { result: Ok(Default::default()), work: 0 }
}

fn refused(error: ValueError) -> Turn {
    RetainedWorkOutcome { result: Err(error.into()), work: 0 }
}

impl<M: SourceReceived<TaggedValue>> OriginalTaggedPackValueReceiving<M> {
    /// 🎟️ Admits the authored protocol and the exact range of the stored operation.
    pub fn admit(
        source: &[u8], range: (usize, usize), protocol: &'static str, grant: RetainedCloneGrant,
    ) -> RetainedWorkOutcome<Option<(Self, RetainedCloneProgress)>> {
        let turn = OriginalPackValueReceiving::admit_tagged_key(source, range, protocol, grant);
        RetainedWorkOutcome {
            result: turn.result.map(|ready| ready.map(|(wire, progress)| (
                Self { wire, binding: None, output: Default::default(), took: false, closing: false, closed: false, failed: false },
                progress,
            ))),
            work: turn.work,
        }
    }

    fn kind(&self) -> Result<&'static str, ValueError> {
        self.wire.tagged.as_ref().and_then(TaggedPackOperation::selected)
            .ok_or_else(|| ValueError::invariant("tagged Pack operation has no selected variant"))
    }

    fn demands_inner(&self, source: &[u8], body: usize) -> Result<RetirementDemand, ValueError> {
        self.wire.validate(source)?;
        if self.closing {
            return self.close_demand(body);
        }
        if self.failed || self.took || self.output.value.is_some() {
            return Ok(Default::default());
        }
        if self.wire.phase < 3 {
            return self.wire.demands_inner(source, body);
        }
        if let Some(binding) = &self.binding {
            return if binding.is_ready() { Ok(metadata::<()>()) } else { binding.demands(&None, body) };
        }
        let length = self.kind()?.len();
        Ok(RetirementDemand { copy_bytes: length, capacity_bytes: length, depth: 1, ..Default::default() })
    }

    fn advance_local(&mut self, source: &[u8], native: &mut NativeDecodeControl<'_>, body: usize, effect: &mut RetainedAllocationEffect<'_>) -> Turn {
        let result = (|| {
            let demand = self.demands_inner(source, body)?;
            if self.failed || self.closing || self.took || self.output.value.is_some() || !funded(effect.grant(), demand) {
                return Ok(Default::default());
            }
            if self.wire.phase < 3 {
                return self.wire.step_action(source, native, body, effect);
            }
            if let Some(binding) = &mut self.binding {
                if binding.is_ready() {
                    return unpoll(binding.take_into(&mut self.output.value, effect)).result.map_err(Into::into);
                }
                return unpoll(binding.advance_local(&mut None, body, effect)).result;
            }
            native.checkpoint()?;
            perform(effect, 1)?;
            let kind = self.kind()?;
            if self.wire.value.value.is_none() {
                return Err(ValueError::invariant("original Pack payload is absent").into());
            }
            let mut name = String::new();
            name.try_reserve_exact(kind.len()).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "tagged Pack variant allocation failed"))?;
            tagged_value_binary::write_case(kind, true, &mut name);
            let bytes = name.capacity();
            *effect.actual() = Some(bytes);
            let value = self.wire.value.value.take().expect("original Pack payload checked before allocation");
            self.binding = Some(Binding::from_original((name, value)));
            Ok(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, retained_capacity_bytes: bytes, ..Default::default() })
        })();
        self.failed |= result.is_err();
        RetainedWorkOutcome { result, work: effect.performed_work() }
    }

    fn nested_work(&self, body: usize) -> Result<Option<u64>, ValueError> {
        if self.failed || self.closing || self.took || self.output.value.is_some() {
            return Ok(None);
        }
        match &self.binding {
            Some(binding) if !binding.is_ready() => match binding.select_advance(&None, body, u64::MAX)? {
                OwnedAdvanceSelection::Local { .. } => Ok(None),
                OwnedAdvanceSelection::Nested => binding.selected_work_demand(&None, body).map(Some),
            },
            _ => Ok(None),
        }
    }

    fn advance_nested(&mut self, source: &[u8], native: &mut NativeDecodeControl<'_>, body: usize, work: u64, grant: RetainedCloneGrant) -> Turn {
        let demand = match self.demands_inner(source, body) {
            Ok(demand) => demand,
            Err(error) => return refused(error),
        };
        if !funded(grant, demand) {
            return denied();
        }
        let witness = match native.source_witness() {
            Ok(witness) => witness,
            Err(error) => return refused(error),
        };
        let Some(binding) = &mut self.binding else {
            return refused(ValueError::invariant("original tagged binding is absent"));
        };
        let turn = unpoll(binding.advance_nested(&mut None, native, witness, body, work.max(1), grant));
        self.failed |= turn.result.is_err();
        turn
    }

    fn close_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if self.closed { return Ok(Default::default()); }
        if !self.closing { return Ok(metadata::<()>()); }
        if !self.output.terminal_is_empty() { return self.output.close_demands(body); }
        if let Some(binding) = &self.binding {
            return if binding.terminal_is_empty() { Ok(metadata::<()>()) } else { binding.close_demands(body) };
        }
        if !self.wire.closed { return self.wire.close_demand(body); }
        Ok(metadata::<()>())
    }

    fn close_work_demand(&self, body: usize) -> Result<u64, ValueError> {
        if self.closed { return Ok(0); }
        if !self.closing { return Ok(1); }
        if !self.output.terminal_is_empty() { return self.output.close_work_demand(body); }
        if let Some(binding) = &self.binding {
            return if binding.terminal_is_empty() { Ok(1) } else { binding.close_work_demand(body) };
        }
        if !self.wire.closed { return self.wire.close_work(body); }
        Ok(1)
    }

    fn close(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> OriginalRetirementPoll {
        let complete = |effect: &RetainedAllocationEffect<'_>, result: Result<RetainedCloneStep, ValueError>| {
            RetainedWorkPoll::Complete(RetainedWorkOutcome { result: result.map_err(Into::into), work: effect.performed_work() })
        };
        if self.closed { return complete(effect, Ok(RetainedCloneStep::Complete(Default::default()))); }
        let grant = effect.grant();
        let demand = match self.close_demand(grant.maximum_copy_bytes) {
            Ok(demand) => demand,
            Err(error) => return complete(effect, Err(error)),
        };
        if !funded(grant, demand) {
            return complete(effect, Ok(RetainedCloneStep::Progress(Default::default())));
        }
        if self.closing {
            if !self.output.terminal_is_empty() { return stepped(self.output.close_step(effect)); }
            if let Some(binding) = &mut self.binding {
                if !binding.terminal_is_empty() { return stepped(binding.close_step(effect)); }
            } else if !self.wire.closed {
                return self.wire.close(effect).map(|step| RetainedCloneStep::Progress(step.progress()));
            }
        }
        if let Err(error) = perform(effect, 1) { return complete(effect, Err(error)); }
        let step = if !self.closing {
            self.closing = true;
            RetainedCloneStep::Progress(receipt(demand))
        } else if self.binding.is_some() {
            self.binding = None;
            RetainedCloneStep::Progress(receipt(demand))
        } else {
            self.closed = true;
            RetainedCloneStep::Complete(receipt(demand))
        };
        complete(effect, Ok(step))
    }
}

impl<M> OpBinaryReceivingCursor<M> for OriginalTaggedPackValueReceiving<M>
where M: SourceReceived<TaggedValue> + OpBinaryReceiving<DomainFailure = ValueError, Receiving = Self>,
{
    fn demands(&self, source: &[u8], body: usize) -> Result<RetirementDemand, ValueError> { self.demands_inner(source, body) }

    fn step_work_demand(&self, source: &[u8], body: usize) -> Result<u64, ValueError> {
        self.wire.validate(source)?;
        if let Some(work) = self.nested_work(body)? { return Ok(work.max(1)); }
        match &self.binding {
            Some(binding) if !binding.is_ready() && !self.closing && !self.failed && !self.took =>
                binding.selected_work_demand(&None, body).map(|work| work.max(1)),
            _ => Ok(1),
        }
    }

    fn step_in(&mut self, source: &[u8], native: &mut NativeDecodeControl<'_>, body: usize, effect: &mut RetainedAllocationEffect<'_>) -> Turn {
        match self.nested_work(body) {
            Ok(Some(work)) if effect.maximum_work_units() >= work => self.advance_nested(source, native, body, work, effect.grant()),
            Ok(Some(_)) => denied(),
            Ok(None) => self.advance_local(source, native, body, effect),
            Err(error) => refused(error),
        }
    }

    fn step(&mut self, source: &[u8], native: &mut NativeDecodeControl<'_>, body: usize, grant: RetainedCloneGrant) -> Turn {
        match self.nested_work(body) {
            Ok(Some(work)) => return self.advance_nested(source, native, body, work, grant),
            Ok(None) => {}
            Err(error) => return refused(error),
        }
        let work = match self.step_work_demand(source, body) {
            Ok(work) => work,
            Err(error) => return refused(error),
        };
        match native.with_effect(grant, work, |control, effect| self.advance_local(source, control, body, effect)) {
            Ok(Some(turn)) => turn,
            Ok(None) => denied(),
            Err(error) => refused(error),
        }
    }

    fn domain_failure(&self) -> Option<&ValueError> { None }
    fn domain_failure_demand(&self) -> Result<RetirementDemand, ValueError> { Ok(Default::default()) }
    fn domain_failure_work_demand(&self) -> Result<u64, ValueError> { Ok(0) }
    fn take_domain_failure_into(&mut self, recipient: &mut Option<ValueError>, _: RetainedCloneGrant) -> RetainedWorkOutcome<RetainedCloneProgress> {
        RetainedWorkOutcome {
            result: if recipient.is_none() { Ok(Default::default()) } else { Err(invalid("tagged Pack failure recipient must be empty")) },
            work: 0,
        }
    }
    fn is_ready(&self) -> bool { !self.failed && !self.closing && self.output.value.is_some() }
    fn take_into(&mut self, recipient: &mut Option<M>, grant: RetainedCloneGrant) -> RetainedWorkOutcome<RetainedCloneProgress> {
        let demand = metadata::<M>();
        if !self.is_ready() || recipient.is_some() || !funded(grant, demand) {
            return RetainedWorkOutcome { result: Ok(Default::default()), work: 0 };
        }
        *recipient = self.output.value.take();
        self.took = true;
        RetainedWorkOutcome { result: Ok(receipt(demand)), work: 1 }
    }
    fn close_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> { self.close_demand(body) }
    fn close_work_demand(&self, body: usize) -> Result<u64, ValueError> { OriginalTaggedPackValueReceiving::close_work_demand(self, body) }
    fn close_step(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> OriginalRetirementPoll { self.close(effect) }
    fn terminal_is_empty(&self) -> bool { self.closed }
}

/// ♻️ Retires the typed binding, original wire and untaken native output.
#[doc(hidden)]
pub struct TaggedPackValueRetirement<M: SourceReceived<TaggedValue>>(ManuallyDrop<OriginalTaggedPackValueReceiving<M>>);

impl<M: SourceReceived<TaggedValue>> RetireOwned for OriginalTaggedPackValueReceiving<M> {
    type RetirementCursor = TaggedPackValueRetirement<M>;
    fn retirement_seed(self) -> Self::RetirementCursor { TaggedPackValueRetirement(ManuallyDrop::new(self)) }
    fn controlled_retirement_supported() -> bool { true }
}

impl<M: SourceReceived<TaggedValue>> TaggedPackValueRetirement<M> {
    fn close_step(&mut self, _: usize, effect: &mut RetainedAllocationEffect<'_>) -> RetirementStep {
        erased_cursor_turn(self.0.close(effect))
    }
}

impl<M: SourceReceived<TaggedValue>> RetirementCursor for TaggedPackValueRetirement<M> {
    semio_framework_value::native_retirement_reply!();
    fn next_computational_work(&self, body: usize) -> Result<u64, ValueError> { self.0.close_work_demand(body) }
    fn terminal_is_empty(&self) -> bool { self.0.closed }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.0.close_demand(usize::MAX)?.depth) }
    fn next_work_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.0.close_demand(usize::MAX)?.copy_bytes) }
    fn next_selected_work_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Ok(self.0.close_demand(body)?.copy_bytes) }
    fn next_close_byte_demand(&self) -> Option<usize> { self.0.close_demand(usize::MAX).ok().map(|demand| demand.release_bytes) }
    fn next_birth_bytes(&self, body: usize) -> Option<usize> { self.0.close_demand(body).ok().map(|demand| demand.capacity_bytes) }
    fn terminal_release_bytes(&self) -> Option<usize> { Some(size_of::<Self>()) }
}

impl<M: SourceReceived<TaggedValue>> Drop for TaggedPackValueRetirement<M> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.0.closed, "tagged Pack operation requires funded retirement");
        if self.0.closed { unsafe { ManuallyDrop::drop(&mut self.0) } }
    }
}
