//! 🫴️ Original artifact document source custody through strict bounded base64 receiving.
use super::ArtifactDocumentPayload;
use base64_codec::{Base64Control, Base64ControlError, Base64DecodeCursor, Base64Input};
use semio_framework::action_bus::{admit_original_tool_construction_scoped, OriginalToolSourceBinding, ToolJobAdmissionDemand};
use semio_framework_job::{close_original_retained_step, InteractiveJobCloseStep, OriginalTurnFailure, StepContext};
use semio_framework_value::{ControlledRetirement, RetainedCloneProgress, RetainedCloneStep, RetirementDemand, ValueError, ValueRefusalKind};
use std::mem::{size_of, ManuallyDrop};

#[derive(semio_framework_value::RetireOwned)]
enum DocumentSourceOwner { Text(String), Bytes(Vec<u8>) }

/// 📜️ Holds the supplied Pack and SPR carriers unchanged until their decoded owners can transfer together.
pub struct OriginalArtifactDocumentReceiving {
    source: OriginalToolSourceBinding,
    cursors: ManuallyDrop<[Option<Base64DecodeCursor<'static>>; 2]>,
    decoded: ManuallyDrop<[Option<Vec<u8>>; 2]>,
    closing_text: ManuallyDrop<[Option<String>; 2]>,
    close_owner: ManuallyDrop<Option<ControlledRetirement<DocumentSourceOwner>>>,
    phase: usize,
    maximum_decoded_bytes: usize,
    fault: Option<Base64ControlError>,
    closing: bool,
}

impl OriginalArtifactDocumentReceiving {
    fn refusal_value(&self) -> ValueError {
        let kind = match self.fault.as_ref() { Some(Base64ControlError::Cancelled) => ValueRefusalKind::Canceled, Some(Base64ControlError::OutputLimit) => ValueRefusalKind::OwnershipLimit, _ => ValueRefusalKind::InvalidValue };
        ValueError::literal(kind, "artifact document retains its original base64 refusal")
    }

    fn validate_source(&self, cx: &StepContext<'_>) -> Result<(), OriginalTurnFailure> {
        cx.original_source_parent()?.validate_borrowed_context(cx)?;
        if !self.source.matches(cx) { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "artifact document receiving refused a foreign original source").into()); }
        Ok(())
    }

    /// 🎟️ Transfers both original text owners only inside the caller's admitted construction scope.
    pub fn admit_original(source: &mut Option<ArtifactDocumentPayload>, output: &mut Option<Self>, maximum_decoded_bytes: usize, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        cx.original_source_parent()?.validate_borrowed_context(cx)?;
        if cx.original_clock_binding().is_none() { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "artifact document receiving requires its original clock").into()); }
        if source.is_none() { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "artifact document original payload is absent").into()); }
        let demand = RetirementDemand { copy_bytes: size_of::<Self>(), depth: 1, ..Default::default() };
        admit_original_tool_construction_scoped(cx, output, ToolJobAdmissionDemand { items: 1, copy_bytes: demand.copy_bytes, capacity_bytes: 0, release_bytes: 0, depth: 1 }, |scope| {
            scope.admit_owner(demand)?;
            let ArtifactDocumentPayload { pack, spr } = source.take().expect("borrowed original document preflight");
            let candidate = Self { source: scope.identity().source_binding(), cursors: ManuallyDrop::new([Some(Base64DecodeCursor::new(Base64Input::OwnedText(pack))), Some(Base64DecodeCursor::new(Base64Input::OwnedText(spr)))]), decoded: ManuallyDrop::new([None, None]), closing_text: ManuallyDrop::new([None, None]), close_owner: ManuallyDrop::new(None), phase: 0, maximum_decoded_bytes, fault: None, closing: false };
            Ok((candidate, RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }))
        })
    }

    /// 📐️ Quotes a single existing codec transition without decoding, cloning or allocating.
    pub fn next_demands(&self) -> Result<RetirementDemand, ValueError> {
        if self.closing || self.phase >= 2 || self.fault.is_some() { return Ok(RetirementDemand::default()); }
        let cursor = self.cursors[self.phase].as_ref().expect("original document cursor remains resident");
        Ok(RetirementDemand { copy_bytes: size_of::<Base64DecodeCursor<'static>>() + cursor.next_work_demand(), capacity_bytes: cursor.next_capacity_byte_demand(), release_bytes: 0, depth: 2 })
    }

    /// 🪜️ Advances one strict validation, output birth or reconstruction chunk under the same caller and receipt.
    pub fn step(&mut self, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        self.validate_source(cx)?;
        if self.closing { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "artifact document receiving is closing").into()); }
        if cx.is_cancelled() { return Err(ValueError::literal(ValueRefusalKind::Canceled, "original artifact document receiving was cancelled").into()); }
        if self.fault.is_some() { return Err(self.refusal_value().into()); }
        if self.phase >= 2 { return Ok(self.phase == 2); }
        let total = self.cursors.iter().flatten().try_fold(0usize, |sum, cursor| sum.checked_add(cursor.expected_output_bytes())).ok_or_else(|| ValueError::literal(ValueRefusalKind::OwnershipLimit, "artifact document decoded extent overflow"))?;
        if total > self.maximum_decoded_bytes { return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "artifact document decoded pair exceeds its original contract").into()); }
        let demand = self.next_demands()?;
        let cancel = cx.borrowed_original_cancel_token()?;
        let now = cx.clock_source();
        let deadline = cx.deadline_us();
        let mut output = None;
        let mut progress = RetainedCloneProgress::default();
        admit_original_tool_construction_scoped(cx, &mut output, ToolJobAdmissionDemand { items: 1, copy_bytes: demand.copy_bytes, capacity_bytes: demand.capacity_bytes, release_bytes: 0, depth: demand.depth }, |scope| {
            scope.admit_owner(demand)?;
            let cursor = self.cursors[self.phase].as_mut().expect("original document cursor remains resident");
            let before = cursor.output_capacity();
            let mut observe = |_| !cancel.is_cancelled_now() && now().is_some_and(|instant| instant < deadline);
            let mut control = Base64Control { maximum_output_bytes: self.maximum_decoded_bytes, progress: &mut observe };
            let result = cursor.step(cursor.next_work_demand().max(1), &mut control);
            progress = RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, retained_capacity_bytes: cursor.output_capacity().saturating_sub(before), released_bytes: 0 };
            match result {
                Ok(_) if cursor.is_complete() => { self.decoded[self.phase] = cursor.take_output(); self.phase += 1; }
                Ok(_) => {}
                Err(error) => self.fault = Some(error),
            }
            Ok(((), progress))
        })?;
        if self.fault.is_some() { return Err(self.refusal_value().with_retained_progress(progress).into()); }
        Ok(self.phase == 2)
    }

    /// 🔎️ Borrows the exact codec cause while its original input and partial output remain owned.
    pub fn fault(&self) -> Option<&Base64ControlError> { self.fault.as_ref() }

    /// 📦️ Moves the actual decoded pair once; it never substitutes a current editor or drops supplied history.
    pub fn take_pair(&mut self, output: &mut Option<(Vec<u8>, Vec<u8>)>, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        self.validate_source(cx)?;
        if self.closing || self.phase != 2 || self.fault.is_some() { return Ok(false); }
        let demand = RetirementDemand { copy_bytes: size_of::<(Vec<u8>, Vec<u8>)>(), depth: 1, ..Default::default() };
        admit_original_tool_construction_scoped(cx, output, ToolJobAdmissionDemand { items: 1, copy_bytes: demand.copy_bytes, capacity_bytes: 0, release_bytes: 0, depth: 1 }, |scope| {
            scope.admit_owner(demand)?;
            let pair = (self.decoded[0].take().expect("complete Pack source"), self.decoded[1].take().expect("complete SPR source"));
            self.phase = 3;
            Ok((pair, RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }))
        })
    }

    /// ♻️ Quotes actual cursor custody, decoded bytes and each source text before any destruction.
    pub fn close_demands(&self) -> Result<RetirementDemand, ValueError> {
        if let Some(owner) = self.close_owner.as_ref() { return owner.next_demand(4096); }
        if self.terminal_is_empty() { return Ok(RetirementDemand::default()); }
        Ok(RetirementDemand { copy_bytes: size_of::<Self>(), depth: 1, ..Default::default() })
    }

    /// 🚪️ Preserves every original partial source across zero budgets, cancellation and close refusal.
    pub fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, OriginalTurnFailure> {
        self.validate_source(cx)?;
        self.closing = true;
        if self.terminal_is_empty() { return Ok(InteractiveJobCloseStep::Complete { progress: Default::default() }); }
        if let Some(owner) = self.close_owner.as_mut() {
            let step = close_original_retained_step(cx, owner.next_demand(4096)?, |grant| owner.step(grant))?;
            if owner.terminal_is_empty() { *self.close_owner = None; }
            return Ok(step);
        }
        let demand = self.close_demands()?;
        close_original_retained_step(cx, demand, |_| {
            if let Some(index) = self.cursors.iter().position(Option::is_some) {
                let parts = self.cursors[index].take().expect("original document cursor custody").into_parts();
                self.closing_text[index] = Some(match parts.input { Base64Input::OwnedText(text) => text, _ => unreachable!("original artifact document retains text") });
                if parts.output.is_some() { self.decoded[index] = parts.output; }
            } else if let Some(index) = self.closing_text.iter().position(Option::is_some) {
                *self.close_owner = Some(ControlledRetirement::retain(DocumentSourceOwner::Text(self.closing_text[index].take().expect("original encoded text"))));
            } else if let Some(index) = self.decoded.iter().position(Option::is_some) {
                *self.close_owner = Some(ControlledRetirement::retain(DocumentSourceOwner::Bytes(self.decoded[index].take().expect("original decoded bytes"))));
            }
            Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }))
        })
    }

    /// 🏁️ Both source texts, both decoded owners and every close scaffold must be gone.
    pub fn terminal_is_empty(&self) -> bool { self.closing && self.cursors.iter().all(Option::is_none) && self.decoded.iter().all(Option::is_none) && self.closing_text.iter().all(Option::is_none) && self.close_owner.is_none() }
}

impl Drop for OriginalArtifactDocumentReceiving {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "original artifact document receiving must return all source custody before drop");
        if self.terminal_is_empty() { unsafe { ManuallyDrop::drop(&mut self.cursors); ManuallyDrop::drop(&mut self.decoded); ManuallyDrop::drop(&mut self.closing_text); ManuallyDrop::drop(&mut self.close_owner); } }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
