//! 🎟️ A local clause delegates original request/source preparation to the canonical MVD owner.
use super::*;
use super::clause::IfcAlignmentDirectClause;
use super::super::preparation::IfcMvdPreparation;
use super::super::population::IfcMvdPopulationStatus;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) struct State { done: bool, failed: bool, donated: bool }
semio_framework_value::artifact_retire_leaf!(State);
#[derive(Default, semio_framework_value::RetireOwned)]
pub struct IfcAlignmentDirectWork { clause: IfcAlignmentDirectClause, state: State, preparation: IfcMvdPreparation<IfcAlignmentDirectRequest> }
impl IfcAlignmentDirectWork {
    fn base() -> RetirementDemand { RetirementDemand { copy_bytes: 4 * size_of::<State>(), depth: 1, ..Default::default() } }
    pub fn demands(&self, snapshot: &Arc<IfcSnapshot>, request: &Arc<IfcAlignmentDirectRequest>) -> Result<RetirementDemand, ValueError> {
        let preparation = self.preparation.demands(snapshot, request)?;
        let child = if self.preparation.ready() { self.clause.demands(request.as_ref())? } else { preparation };
        let mut demand = Self::base();
        demand.copy_bytes = demand.copy_bytes.checked_add(child.copy_bytes).ok_or_else(|| invalid("mvd.direct.capacity"))?;
        demand.capacity_bytes = child.capacity_bytes; demand.release_bytes = child.release_bytes; demand.depth = demand.depth.max(child.depth);
        Ok(demand)
    }
    pub fn advance(&mut self, snapshot: &Arc<IfcSnapshot>, request: &Arc<IfcAlignmentDirectRequest>, epoch: u64, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        self.preparation.check_resume(snapshot, request, epoch, cx)?;
        if self.state.failed || self.state.donated { return Err(invalid("mvd.original-source").into()); }
        let demand = self.demands(snapshot, request)?;
        if cx.should_yield() || !retained_clone_grant_admits(cx.retained_grant(), demand) { return Ok(false); }
        if self.state.done { return Ok(true); }
        cx.consume_retained(RetainedCloneProgress { copied_items: 0, copied_bytes: Self::base().copy_bytes, ..Default::default() })?;
        let result = if !self.preparation.ready() { self.preparation.advance(snapshot, request, epoch, cx).map(|_| false) } else {
            let clause = &mut self.clause;
            match self.preparation.status(epoch, cx)? {
                IfcMvdPopulationStatus::Prepared => self.preparation.with_prepared(epoch, cx, |view, _requests, cx| clause.advance_prepared(view, epoch, cx)).map(|done| done.unwrap_or(false)),
                IfcMvdPopulationStatus::Unavailable => self.preparation.with_unavailable(epoch, cx, |view, _requests, cx| clause.advance_unavailable(view, epoch, cx)).map(|done| done.unwrap_or(false)),
                IfcMvdPopulationStatus::Pending => Err(invalid("mvd.direct.population-pending").into()),
            }
        };
        match &result { Ok(true) => self.state.done = true, Err(_) => self.state.failed = true, _ => {} }
        result
    }
    pub fn take(&mut self, output: &mut Option<IfcAlignmentDirectReport>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        let epoch = self.preparation.epoch(cx)?;
        if self.state.failed { return Err(invalid("mvd.original-source").into()); }
        let child = IfcAlignmentDirectClause::take_demands();
        let bytes = Self::base().copy_bytes.checked_add(child.copy_bytes).ok_or_else(|| invalid("mvd.direct.capacity"))?;
        if cx.should_yield() || !retained_clone_grant_admits(cx.retained_grant(), RetirementDemand { copy_bytes: bytes, ..child }) { return Ok(false); }
        if !self.state.done || self.state.donated || output.is_some() { return Err(invalid("mvd.direct.output").into()); }
        cx.consume_retained(RetainedCloneProgress { copied_items: 0, copied_bytes: Self::base().copy_bytes, ..Default::default() })?;
        let clause = &mut self.clause;
        let donated = match self.preparation.status(epoch, cx)? {
            IfcMvdPopulationStatus::Prepared => self.preparation.with_prepared(epoch, cx, |view, _requests, cx| clause.take_prepared(view, output, cx))?,
            IfcMvdPopulationStatus::Unavailable => self.preparation.with_unavailable(epoch, cx, |view, _requests, cx| clause.take_unavailable(view, output, cx))?,
            IfcMvdPopulationStatus::Pending => return Err(invalid("mvd.direct.population-pending").into()),
        }.unwrap_or(false);
        self.state.donated = donated;
        Ok(donated)
    }
    #[cfg(test)]
    pub(super) fn metadata(&self) -> (State, super::super::preparation::State, super::clause::ClauseMetadata) { (self.state, self.preparation.metadata(), self.clause.metadata()) }
}
#[cfg(test)]
#[path = "direct-population-tests.rs"]
mod tests;
