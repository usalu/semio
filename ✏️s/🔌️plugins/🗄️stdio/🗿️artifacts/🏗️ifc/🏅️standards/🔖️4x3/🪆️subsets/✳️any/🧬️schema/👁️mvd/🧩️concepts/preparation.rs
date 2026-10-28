//! 🎟️ One canonical original request/source admission pipeline serves all native MVD clauses.
use super::population::{IfcMvdPopulationLookup, IfcMvdPopulationPin, IfcMvdPopulationRequestPin, IfcMvdPopulationRequestView, IfcMvdPopulationStatus, IfcMvdPopulationView, IfcMvdPopulationWork};
use super::source_index::{IfcSourceIds, IfcSourceIndex, IdMatch};
use semio_framework_job::{OriginalSourceBinding, OriginalTurnFailure, StepContext};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneSource, retained_clone_grant_admits};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind, retirement::RetireOwned};
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcSnapshot;
use std::{mem::size_of, sync::Arc};
/// 🔐️ Metadata access stays immutable, bounded and allocation-free under the original row grant.
pub(super) trait IfcMvdRequest: IfcSourceIds + RetireOwned + Sync { fn valid_row(&self, ordinal: usize) -> bool; }
type RequestSource<R> = RetainedCloneSource<R, Arc<R>>;
fn invalid(message: &'static str) -> ValueError { ValueError::literal(ValueRefusalKind::InvalidValue, message) }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct IfcMvdRequestIndexPin { request: IfcMvdPopulationRequestPin, rows: (usize, usize), population: Option<IfcMvdPopulationPin> }
semio_framework_value::artifact_retire_leaf!(IfcMvdRequestIndexPin);
pub(super) struct IfcMvdRequestIndexView<'a, R: IfcMvdRequest> { index: &'a IfcSourceIndex, request: &'a R, pin: IfcMvdRequestIndexPin, original: OriginalSourceBinding }
impl IfcMvdRequestIndexPin {
    pub(super) fn matches<R: IfcMvdRequest>(self, view: &IfcMvdRequestIndexView<'_, R>, cx: &StepContext<'_>) -> bool { self == view.pin && view.check(cx).is_ok() }
}
impl<R: IfcMvdRequest> IfcMvdRequestIndexView<'_, R> {
    fn check(&self, cx: &StepContext<'_>) -> Result<(), ValueError> {
        if cx.is_cancelled() { return Err(ValueError::literal(ValueRefusalKind::Canceled, "io.cancelled")); }
        if !self.original.matches(cx) || self.index.identity(self.request, self.pin.request.epoch())? != self.pin.rows { return Err(invalid("mvd.preparation.request-index")); }
        Ok(())
    }
    pub(super) fn pin(&self, cx: &StepContext<'_>) -> Result<IfcMvdRequestIndexPin, ValueError> { self.check(cx)?; Ok(self.pin) }
    pub(super) fn matches_prepared(&self, view: &IfcMvdPopulationView<'_, R>, cx: &StepContext<'_>) -> bool { self.check(cx).is_ok() && self.pin.population.is_some_and(|pin| pin.matches(view, cx)) && view.request_pin(cx).is_ok_and(|pin| pin == self.pin.request) }
    pub(super) fn matches_unavailable(&self, view: &IfcMvdPopulationRequestView<'_, R>, cx: &StepContext<'_>) -> bool { self.check(cx).is_ok() && self.pin.population.is_none() && self.pin.request.matches(view, cx) }
    pub(super) fn lookup(&self, id: u64, cx: &StepContext<'_>) -> Result<IfcMvdPopulationLookup, ValueError> {
        self.check(cx)?;
        Ok(match self.index.find(id) { IdMatch::Missing => IfcMvdPopulationLookup::Missing, IdMatch::Unique(row) => IfcMvdPopulationLookup::Unique(row), IdMatch::Ambiguous => IfcMvdPopulationLookup::Ambiguous })
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) struct State {
    binding: Option<(usize, usize, u64)>,
    original: Option<semio_framework_job::OriginalSourceBinding>,
    phase: u8,
    target: usize,
    failed: bool,
}
semio_framework_value::artifact_retire_leaf!(State);
#[derive(semio_framework_value::RetireOwned)]
pub(super) struct IfcMvdPreparation<R: IfcMvdRequest> {
    request_index: IfcSourceIndex,
    state: State,
    population: IfcMvdPopulationWork<R>,
    unborn_snapshot: Option<Arc<IfcSnapshot>>,
    unborn_request: Option<Arc<R>>,
    request: Option<RequestSource<R>>,
}
impl<R: IfcMvdRequest> Default for IfcMvdPreparation<R> { fn default() -> Self { Self { request_index: Default::default(), state: Default::default(), population: Default::default(), unborn_snapshot: None, unborn_request: None, request: None } } }
impl<R: IfcMvdRequest> IfcMvdPreparation<R> {
    fn base() -> RetirementDemand { RetirementDemand { copy_bytes: 4 * size_of::<State>() + 2 * size_of::<Option<RequestSource<R>>>(), depth: 1, ..Default::default() } }
    pub(super) fn demands(&self, snapshot: &Arc<IfcSnapshot>, request: &Arc<R>) -> Result<RetirementDemand, ValueError> {
        if self.state.binding.is_some_and(|(source, input, _)| source != Arc::as_ptr(snapshot) as usize || input != Arc::as_ptr(request) as usize) { return Err(invalid("mvd.original-source")); }
        let child = match self.state.phase {
            0 => { let birth = RequestSource::<R>::borrowed_constructor_demand::<Arc<R>>(); RetirementDemand { copy_bytes: size_of::<Arc<R>>() + 2 * size_of::<Arc<IfcSnapshot>>() + RequestSource::<R>::borrowed_constructor_copy_bytes::<Arc<R>>(), capacity_bytes: birth.capacity_bytes, depth: birth.depth, ..Default::default() } },
            1 => self.request_index.demands(request.as_ref())?,
            3 => IfcMvdPopulationWork::<R>::receive_demands(),
            4 => self.population.demands(snapshot, self.state.binding.ok_or_else(|| invalid("mvd.original-source"))?.2)?,
            _ => RetirementDemand::default(),
        };
        let mut demand = Self::base();
        demand.copy_bytes = demand.copy_bytes.checked_add(child.copy_bytes).ok_or_else(|| invalid("mvd.preparation.capacity"))?;
        demand.capacity_bytes = child.capacity_bytes;
        demand.release_bytes = child.release_bytes;
        demand.depth = demand.depth.max(child.depth);
        Ok(demand)
    }
    pub(super) fn check_resume(&self, snapshot: &Arc<IfcSnapshot>, request: &Arc<R>, epoch: u64, cx: &StepContext<'_>) -> Result<(), ValueError> {
        if cx.is_cancelled() { return Err(ValueError::literal(ValueRefusalKind::Canceled, "io.cancelled")); }
        let binding = (Arc::as_ptr(snapshot) as usize, Arc::as_ptr(request) as usize, epoch);
        if self.state.binding.is_some_and(|old| old != binding) || self.state.original.is_some_and(|old| !old.matches(cx)) || self.state.failed { return Err(invalid("mvd.original-source")); }
        if (1..=3).contains(&self.state.phase) && self.unborn_snapshot.as_ref().is_none_or(|source| Arc::as_ptr(source) as usize != binding.0) { return Err(invalid("mvd.original-source")); }
        Ok(())
    }
    pub(super) fn advance(&mut self, snapshot: &Arc<IfcSnapshot>, request: &Arc<R>, epoch: u64, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        self.check_resume(snapshot, request, epoch, cx)?;
        let binding = (Arc::as_ptr(snapshot) as usize, Arc::as_ptr(request) as usize, epoch);
        let demand = self.demands(snapshot, request)?;
        if cx.should_yield() || !retained_clone_grant_admits(cx.retained_grant(), demand) { return Ok(false); }
        if self.state.phase == 5 { return Ok(true); }
        let mut posted = false;
        let result = self.advance_admitted(snapshot, request, epoch, binding, &mut posted, cx);
        match result {
            Ok(done) => Ok(done),
            Err(mut failure) => {
                self.state.failed = true;
                if self.state.original.is_none() { self.state.original = Some(OriginalSourceBinding::borrow(cx)); }
                if !posted {
                    if let Err(error) = cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: Self::base().copy_bytes, ..Default::default() }) { failure.context_refusal = Some(error); }
                    cx.consume_fuel(1);
                }
                Err(failure)
            }
        }
    }
    fn advance_admitted(&mut self, snapshot: &Arc<IfcSnapshot>, request: &Arc<R>, epoch: u64, binding: (usize, usize, u64), posted: &mut bool, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        let mut progress = RetainedCloneProgress { copied_items: 1, copied_bytes: Self::base().copy_bytes, ..Default::default() };
        match self.state.phase {
            0 => {
                if request.source_len() == 0 || request.source_len() > 1_000_000 { return Err(invalid("mvd.request").into()); }
                self.state.binding = Some(binding);
                self.state.original = Some(semio_framework_job::OriginalSourceBinding::borrow(cx));
                let alias = size_of::<Arc<R>>();
                let source_alias = 2 * size_of::<Arc<IfcSnapshot>>();
                self.unborn_snapshot = Some(Arc::clone(snapshot));
                progress.copied_bytes += source_alias;
                let grant = RetainedCloneGrant { maximum_copy_bytes: cx.retained_grant().maximum_copy_bytes - progress.copied_bytes - alias, ..cx.retained_grant() };
                let child = match RequestSource::<R>::admit_borrowed(Arc::clone(request), |value| value.as_ref(), grant) {
                    Ok((owner, child)) => { self.request = Some(owner); child },
                    Err((error, owner)) => {
                        self.unborn_request = Some(owner);
                        let child = error.retained_progress();
                        progress.copied_bytes += alias + child.copied_bytes;
                        progress.retained_capacity_bytes = child.retained_capacity_bytes;
                        progress.released_bytes = child.released_bytes;
                        let mut failure = semio_framework_job::OriginalTurnFailure::from(error);
                        *posted = true;
                        if let Err(error) = cx.consume_retained(progress) { failure.context_refusal = Some(error); }
                        cx.consume_fuel(1);
                        return Err(failure);
                    }
                };
                self.state.phase = 1;
                progress.copied_bytes += alias + child.copied_bytes;
                progress.retained_capacity_bytes = child.retained_capacity_bytes;
                progress.released_bytes = child.released_bytes;
            },
            1 => {
                progress.copied_items = 0;
                *posted = true;
                cx.consume_retained(progress)?;
                if self.request_index.advance(request.as_ref(), epoch, cx)? { self.state.phase = 2; }
                return Ok(false);
            },
            2 => {
                if self.state.target == request.source_len() { self.state.target = 0; self.state.phase = 3; }
                else {
                    if !request.valid_row(self.state.target) || !matches!(self.request_index.find(request.source_id(self.state.target)?), IdMatch::Unique(_)) { return Err(invalid("mvd.request").into()); }
                    self.state.target += 1;
                }
            },
            3 => {
                progress.copied_items = 0;
                *posted = true;
                cx.consume_retained(progress)?;
                if self.population.receive_donated(&mut self.unborn_snapshot, &mut self.request, epoch, cx)? { self.state.phase = 4; }
                return Ok(false);
            },
            4 => {
                progress.copied_items = 0;
                *posted = true;
                cx.consume_retained(progress)?;
                if self.population.advance(snapshot, epoch, cx)? { self.state.phase = 5; }
                return Ok(self.state.phase == 5);
            },
            _ => return Err(invalid("mvd.preparation.phase").into()),
        }
        *posted = true;
        cx.consume_retained(progress)?;
        cx.consume_fuel(1);
        Ok(false)
    }
    pub(super) fn ready(&self) -> bool { self.state.phase == 5 && !self.state.failed }
    pub(super) fn epoch(&self, cx: &StepContext<'_>) -> Result<u64, ValueError> {
        if cx.is_cancelled() { return Err(ValueError::literal(ValueRefusalKind::Canceled, "io.cancelled")); }
        if self.state.original.is_none_or(|original| !original.matches(cx)) || self.state.failed { return Err(invalid("mvd.original-source")); }
        Ok(self.state.binding.ok_or_else(|| invalid("mvd.original-source"))?.2)
    }
    pub(super) fn status(&self, epoch: u64, cx: &StepContext<'_>) -> Result<IfcMvdPopulationStatus, ValueError> {
        if self.epoch(cx)? != epoch { return Err(invalid("mvd.original-source")); }
        if !self.ready() { return Ok(IfcMvdPopulationStatus::Pending); }
        self.population.status(epoch, cx)
    }
    pub(super) fn with_prepared<'cx, T: Copy>(&self, epoch: u64, cx: &mut StepContext<'cx>, run: impl for<'view> FnOnce(IfcMvdPopulationView<'view, R>, IfcMvdRequestIndexView<'view, R>, &mut StepContext<'cx>) -> Result<T, OriginalTurnFailure>) -> Result<Option<T>, OriginalTurnFailure> {
        if self.status(epoch, cx)? != IfcMvdPopulationStatus::Prepared { return Ok(None); }
        self.population.with_view(epoch, cx, |view, cx| {
            let request = view.request(cx)?;
            let rows = self.request_index.identity(request, epoch)?;
            let pin = IfcMvdRequestIndexPin { request: view.request_pin(cx)?, rows, population: Some(view.pin(cx)?) };
            let requests = IfcMvdRequestIndexView { index: &self.request_index, request, pin, original: OriginalSourceBinding::borrow(cx) };
            run(view, requests, cx)
        })
    }
    pub(super) fn with_unavailable<'cx, T: Copy>(&self, epoch: u64, cx: &mut StepContext<'cx>, run: impl for<'view> FnOnce(IfcMvdPopulationRequestView<'view, R>, IfcMvdRequestIndexView<'view, R>, &mut StepContext<'cx>) -> Result<T, OriginalTurnFailure>) -> Result<Option<T>, OriginalTurnFailure> {
        if self.status(epoch, cx)? != IfcMvdPopulationStatus::Unavailable { return Ok(None); }
        self.population.with_request(epoch, cx, |view, cx| {
            let request = view.request(cx)?;
            let rows = self.request_index.identity(request, epoch)?;
            let pin = IfcMvdRequestIndexPin { request: view.pin(cx)?, rows, population: None };
            let requests = IfcMvdRequestIndexView { index: &self.request_index, request, pin, original: OriginalSourceBinding::borrow(cx) };
            run(view, requests, cx)
        })
    }
    #[cfg(test)]
    pub(super) fn metadata(&self) -> State { self.state }
    #[cfg(test)]
    pub(super) fn born_snapshot(&self) -> bool { self.unborn_snapshot.is_some() }
    #[cfg(test)]
    pub(super) fn stage(&self) -> u8 { self.state.phase }
}
#[cfg(test)]
#[path = "preparation-tests.rs"]
mod tests;
