//! 🧳️ Keeps an artifact query on the initiating render's existing ownership ladder.
use crate::jobs_instance_root::OriginalRenderTreeIdentity;
use semio_framework_job::{OriginalTurnFailure, StepContext};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind, retirement::{RetireOwned, controlled::ControlledRetirement}, retained_clone::RetainedCloneProgress};
use std::mem::ManuallyDrop;

#[path="🎟️demand/🦀️.rs"]pub mod demand;
pub struct OriginalRenderWork<W: RetireOwned> {
    owner: ManuallyDrop<Option<ControlledRetirement<W>>>,
    closing: bool,
}

impl<S: RetireOwned+Sync> OriginalRenderWork<std::sync::Arc<S>> {
    /// 🪢️ Lends only the initiating render's exact retained immutable snapshot.
    pub fn snapshot_loan<'a>(&'a mut self,identity:&mut OriginalRenderTreeIdentity,expected:&S,cx:&StepContext<'_>)->Result<&'a std::sync::Arc<S>,OriginalTurnFailure>{
        if cx.is_cancelled(){return Err(ValueError::literal(ValueRefusalKind::Canceled,"original render source loan cancelled").into());}
        let source=self.original(identity,cx)?.ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original render source loan is unavailable"))?;
        if !std::ptr::eq(source.as_ref(),expected){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original render source loan cannot substitute its snapshot").into());}
        Ok(source)
    }
}

impl<W: RetireOwned> OriginalRenderWork<W> {
    pub fn retain(work: W) -> Self { Self { owner: ManuallyDrop::new(Some(ControlledRetirement::retain(work))), closing: false } }

    pub fn original<'a>(&'a mut self, identity: &mut OriginalRenderTreeIdentity, cx: &StepContext<'_>) -> Result<Option<&'a mut W>, OriginalTurnFailure> {
        identity.validate(cx)?;
        if self.closing { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "retiring render work cannot resume a query").into()); }
        Ok(self.owner.as_mut().and_then(ControlledRetirement::original_mut))
    }

    /// 📏️ Reads only the untouched original query's next physical action.
    pub fn query_demand(&self,quote:impl FnOnce(&W)->Result<RetirementDemand,ValueError>)->Result<RetirementDemand,ValueError>{if self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"retiring render work cannot quote a query"));}match self.owner.as_ref().and_then(ControlledRetirement::original){Some(work)=>quote(work),None=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original render query owner is unavailable"))}}
    pub fn terminal_is_empty(&self) -> bool { self.owner.is_none() }

    pub fn demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        match self.owner.as_ref() {
            Some(owner) if owner.terminal_is_empty() => Ok(RetirementDemand { depth: 1, ..Default::default() }),
            Some(owner) => owner.next_demand(body),
            None => Ok(Default::default()),
        }
    }

    pub fn close(&mut self, identity: &mut OriginalRenderTreeIdentity, cx: &mut StepContext<'_>) -> Result<Option<bool>, OriginalTurnFailure> {
        identity.validate(cx)?;
        let Some(owner) = self.owner.as_mut() else { return Ok(Some(true)); };
        let demand = if owner.terminal_is_empty() { RetirementDemand { depth: 1, ..Default::default() } } else { owner.next_demand(cx.retained_grant().maximum_copy_bytes)? };
        let grant = cx.retained_grant();
        if cx.retirement_should_yield() || grant.maximum_items == 0 || demand.depth > grant.maximum_depth || demand.copy_bytes > grant.maximum_copy_bytes || demand.capacity_bytes > grant.maximum_capacity_bytes || demand.release_bytes > grant.maximum_release_bytes { return Ok(None); }
        self.closing = true;
        if owner.terminal_is_empty() {
            drop(self.owner.take());
            cx.consume_retained(RetainedCloneProgress { copied_items: 1, ..Default::default() })?;
            cx.consume_fuel(1);
            return Ok(Some(true));
        }
        semio_framework_job::close_original_retained_step(cx, demand, |grant| owner.step(grant))?;
        Ok(Some(false))
    }
}

