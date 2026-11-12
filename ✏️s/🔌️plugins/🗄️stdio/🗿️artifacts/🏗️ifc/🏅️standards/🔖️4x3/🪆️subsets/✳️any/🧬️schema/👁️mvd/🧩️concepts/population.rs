//! 🪢️ One original population owner lends a completed source and stable index for a single clause turn.
use super::native::ShapeCursor;
use semio_framework_job::{OriginalConstructionDemand, OriginalConstructionAdmission, OriginalSourceBinding, OriginalTurnFailure, StepContext, admit_original_construction_settled};
use semio_framework_value::retained_clone::{RetainedCloneProgress, RetainedCloneSource, StagedSourceAdmission, retained_clone_grant_admits};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind, retirement::RetireOwned};
use super::lease::{Lease, funded_turn};
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::{IfcSnapshot, IfcValue};
use std::{mem::size_of, sync::Arc};
use super::source_index::{IdMatch, IfcSourceIndex};
#[cfg(test)]
#[path = "population-tests.rs"]
mod tests;
type SnapshotSource = RetainedCloneSource<IfcSnapshot, Lease<IfcSnapshot>>;
type SnapshotAdmission = StagedSourceAdmission<IfcSnapshot, Lease<IfcSnapshot>>;
pub(super) type IfcMvdPopulationRequestSource<R> = RetainedCloneSource<R, Lease<R>>;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum IfcMvdPopulationStatus { Pending, Unavailable, Prepared }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum IfcMvdPopulationLookup { Missing, Unique(usize), Ambiguous }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct IfcMvdPopulationRequestPin {
    source: usize,
    request: usize,
    request_type: std::any::TypeId,
    epoch: u64,
    original: OriginalSourceBinding,
}
impl IfcMvdPopulationRequestPin {
    pub(super) fn matches<R: RetireOwned + Sync>(self, view: &IfcMvdPopulationRequestView<'_, R>, cx: &StepContext<'_>) -> bool { self == view.pin && self.original.matches(cx) && !cx.is_cancelled() }
    pub(super) fn epoch(self) -> u64 { self.epoch }
}
pub(super) struct IfcMvdPopulationRequestView<'a, R: RetireOwned + Sync> { request: &'a R, pin: IfcMvdPopulationRequestPin, status: IfcMvdPopulationStatus }
impl<'a, R: RetireOwned + Sync> IfcMvdPopulationRequestView<'a, R> {
    pub(super) fn request(&self, cx: &StepContext<'_>) -> Result<&'a R, ValueError> {
        if cx.is_cancelled() { return Err(cancelled()); }
        if !self.pin.original.matches(cx) { return Err(invalid("mvd.population.original")); }
        Ok(self.request)
    }
    pub(super) fn pin(&self, cx: &StepContext<'_>) -> Result<IfcMvdPopulationRequestPin, ValueError> { self.request(cx)?; Ok(self.pin) }
    pub(super) fn status(&self, cx: &StepContext<'_>) -> Result<IfcMvdPopulationStatus, ValueError> { self.request(cx)?; Ok(self.status) }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct IfcMvdPopulationPin {
    source: usize,
    request: usize,
    request_type: std::any::TypeId,
    epoch: u64,
    index: usize,
    rows: (usize, usize),
    original: OriginalSourceBinding,
}
impl IfcMvdPopulationPin {
    pub(super) fn matches<R: RetireOwned + Sync>(self, view: &IfcMvdPopulationView<'_, R>, cx: &StepContext<'_>) -> bool { self == view.pin && self.original.matches(cx) && !cx.is_cancelled() }
    pub(super) fn epoch(self) -> u64 { self.epoch }
}
pub(super) struct IfcMvdPopulationView<'a, R: RetireOwned + Sync> {
    source: &'a IfcSnapshot,
    request: &'a R,
    index: &'a IfcSourceIndex,
    pin: IfcMvdPopulationPin,
}
impl<'a, R: RetireOwned + Sync> IfcMvdPopulationView<'a, R> {
    fn check(&self, cx: &StepContext<'_>) -> Result<(), ValueError> {
        if cx.is_cancelled() { return Err(cancelled()); }
        if !self.pin.original.matches(cx) { return Err(invalid("mvd.population.original")); }
        Ok(())
    }
    pub(super) fn snapshot(&self, cx: &StepContext<'_>) -> Result<&'a IfcSnapshot, ValueError> { self.check(cx)?; Ok(self.source) }
    pub(super) fn request(&self, cx: &StepContext<'_>) -> Result<&'a R, ValueError> { self.check(cx)?; Ok(self.request) }
    pub(super) fn pin(&self, cx: &StepContext<'_>) -> Result<IfcMvdPopulationPin, ValueError> { self.check(cx)?; Ok(self.pin) }
    pub(super) fn request_pin(&self, cx: &StepContext<'_>) -> Result<IfcMvdPopulationRequestPin, ValueError> {
        self.check(cx)?;
        Ok(IfcMvdPopulationRequestPin { source: self.pin.source, request: self.pin.request, request_type: self.pin.request_type, epoch: self.pin.epoch, original: self.pin.original })
    }
    pub(super) fn lookup(&self, id: u64, cx: &StepContext<'_>) -> Result<IfcMvdPopulationLookup, ValueError> {
        self.check(cx)?;
        Ok(match self.index.find(id) { IdMatch::Missing => IfcMvdPopulationLookup::Missing, IdMatch::Unique(row) => IfcMvdPopulationLookup::Unique(row), IdMatch::Ambiguous => IfcMvdPopulationLookup::Ambiguous })
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct State {
    binding: Option<(usize, usize, u64)>,
    original: Option<OriginalSourceBinding>,
    phase: u8,
    unavailable: bool,
    failed: bool,
}
semio_framework_value::artifact_retire_leaf!(State, IfcMvdPopulationPin, IfcMvdPopulationRequestPin);
#[derive(semio_framework_value::RetireOwned)]
pub(super) struct IfcMvdPopulationWork<R: RetireOwned + Sync> {
    index: Option<Box<IfcSourceIndex>>,
    construction: OriginalConstructionAdmission,
    shape: ShapeCursor,
    state: State,
    source: Option<SnapshotSource>,
    unborn: Option<Lease<IfcSnapshot>>,
    admission: Option<SnapshotAdmission>,
    request: Option<IfcMvdPopulationRequestSource<R>>,
}
impl<R: RetireOwned + Sync> Default for IfcMvdPopulationWork<R> {
    fn default() -> Self { Self { index: None, construction: Default::default(), shape: Default::default(), state: Default::default(), source: None, unborn: None, admission: None, request: None } }
}
fn invalid(message: &'static str) -> ValueError { ValueError::literal(ValueRefusalKind::InvalidValue, message) }
fn cancelled() -> ValueError { ValueError::literal(ValueRefusalKind::Canceled, "io.cancelled") }
fn plus(mut parent: RetirementDemand, child: RetirementDemand) -> Result<RetirementDemand, ValueError> {
    parent.copy_bytes = parent.copy_bytes.checked_add(child.copy_bytes).ok_or_else(|| invalid("mvd.population.capacity"))?;
    parent.capacity_bytes = child.capacity_bytes;
    parent.release_bytes = child.release_bytes;
    parent.depth = parent.depth.max(child.depth);
    Ok(parent)
}
impl<R: RetireOwned + Sync> IfcMvdPopulationWork<R> {
    fn base() -> RetirementDemand { RetirementDemand { copy_bytes: 4 * size_of::<State>() + 2 * size_of::<usize>(), depth: 1, ..Default::default() } }
    pub(super) fn receive_demands() -> RetirementDemand { RetirementDemand { copy_bytes: Self::base().copy_bytes + 2 * size_of::<IfcMvdPopulationRequestSource<R>>() + 2 * size_of::<Lease<IfcSnapshot>>(), depth: 1, ..Default::default() } }
    fn source(&self) -> Result<&IfcSnapshot, ValueError> { self.source.as_ref().ok_or_else(|| invalid("mvd.population.source"))?.try_authority()?.get() }
    fn original(&self, epoch: u64, cx: &StepContext<'_>) -> Result<(), ValueError> {
        if cx.is_cancelled() { return Err(cancelled()); }
        if self.state.original.is_none_or(|original| !original.matches(cx)) || self.state.binding.is_none_or(|(_, _, old)| old != epoch) || self.state.failed { return Err(invalid("mvd.population.original")); }
        Ok(())
    }
    fn source_matches(&self, snapshot: &Arc<IfcSnapshot>, epoch: u64) -> Result<(), ValueError> {
        if self.state.binding.is_none_or(|(source, _, old)| source != Arc::as_ptr(snapshot) as usize || old != epoch) { return Err(invalid("mvd.population.source")); }
        Ok(())
    }
    pub(super) fn receive(&mut self, snapshot: &Arc<IfcSnapshot>, donor: &mut Option<IfcMvdPopulationRequestSource<R>>, epoch: u64, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        if cx.is_cancelled() { return Err(cancelled().into()); }
        if self.state != State::default() || self.request.is_some() || self.source.is_some() || self.unborn.is_some() || self.admission.is_some() || self.index.is_some() { return Err(invalid("mvd.population.receive").into()); }
        let request = donor.as_ref().ok_or_else(|| invalid("mvd.population.request"))?.try_authority()?;
        let binding = (Arc::as_ptr(snapshot) as usize, request.addr(), epoch);
        let demand = Self::receive_demands();
        if cx.should_yield() || !retained_clone_grant_admits(cx.retained_grant(), demand) { return Ok(false); }
        self.unborn = Some(Lease::new(Arc::clone(snapshot)));
        self.request = donor.take();
        self.state.binding = Some(binding);
        self.state.original = Some(OriginalSourceBinding::borrow(cx));
        self.state.phase = 1;
        cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })?;
        cx.consume_fuel(1);
        Ok(true)
    }
    pub(super) fn receive_donated(&mut self, snapshot: &mut Option<Lease<IfcSnapshot>>, donor: &mut Option<IfcMvdPopulationRequestSource<R>>, epoch: u64, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        if cx.is_cancelled() { return Err(cancelled().into()); }
        if self.state != State::default() || self.request.is_some() || self.source.is_some() || self.unborn.is_some() || self.admission.is_some() || self.index.is_some() { return Err(invalid("mvd.population.receive").into()); }
        let source = snapshot.as_ref().ok_or_else(|| invalid("mvd.population.source"))?;
        let request = donor.as_ref().ok_or_else(|| invalid("mvd.population.request"))?.try_authority()?;
        let binding = (source.addr(), request.addr(), epoch);
        let demand = Self::receive_demands();
        if cx.should_yield() || !retained_clone_grant_admits(cx.retained_grant(), demand) { return Ok(false); }
        self.unborn = snapshot.take();
        self.request = donor.take();
        self.state.binding = Some(binding);
        self.state.original = Some(OriginalSourceBinding::borrow(cx));
        self.state.phase = 1;
        cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })?;
        cx.consume_fuel(1);
        Ok(true)
    }
    pub(super) fn demands(&self, snapshot: &Arc<IfcSnapshot>, epoch: u64) -> Result<RetirementDemand, ValueError> {
        self.source_matches(snapshot, epoch)?;
        let child = match self.state.phase {
            1 => match self.admission.as_ref() { Some(admission) => admission.next_demand()?, None => RetirementDemand { copy_bytes: 2 * size_of::<Lease<IfcSnapshot>>(), depth: 1, ..Default::default() } },
            2 => RetirementDemand { copy_bytes: self.shape.copy_bound(), depth: 1, ..Default::default() },
            3 => RetirementDemand { copy_bytes: size_of::<Box<IfcSourceIndex>>() + size_of::<IfcSourceIndex>(), capacity_bytes: size_of::<IfcSourceIndex>(), depth: 1, ..Default::default() },
            4 => self.index.as_ref().ok_or_else(|| invalid("mvd.population.index"))?.demands(self.source()?)?,
            _ => RetirementDemand::default(),
        };
        plus(Self::base(), child)
    }
    pub(super) fn advance(&mut self, snapshot: &Arc<IfcSnapshot>, epoch: u64, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        self.original(epoch, cx)?;
        self.source_matches(snapshot, epoch)?;
        let demand = self.demands(snapshot, epoch)?;
        if cx.should_yield() || !retained_clone_grant_admits(cx.retained_grant(), demand) { return Ok(false); }
        let result = self.advance_admitted(epoch, cx);
        if result.is_err() { self.state.failed = true; }
        result
    }
    fn advance_admitted(&mut self, epoch: u64, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        let mut progress = RetainedCloneProgress { copied_items: 1, copied_bytes: Self::base().copy_bytes, ..Default::default() };
        match self.state.phase {
            1 => {
                if self.admission.is_none() {
                    let owner = self.unborn.take().ok_or_else(|| invalid("mvd.population.source"))?;
                    self.admission = Some(SnapshotAdmission::borrowed(owner));
                    progress.copied_bytes += 2 * size_of::<Lease<IfcSnapshot>>();
                } else {
                    let admission = self.admission.as_mut().ok_or_else(|| invalid("mvd.population.source"))?;
                    if admission.terminal_is_empty() {
                        if self.source.is_none() { return Err(invalid("mvd.population.source").into()); }
                        self.admission = None;
                        self.state.phase = 2;
                    } else {
                        let source = &mut self.source;
                        match funded_turn(cx, |effect| admission.advance(source, effect)) {
                            Ok(Some(_)) => {}
                            Ok(None) => return Ok(false),
                            Err(mut failure) => {
                                if let Err(error) = cx.consume_retained(progress) { failure.context_refusal = Some(error); }
                                cx.consume_fuel(1);
                                return Err(failure);
                            }
                        }
                    }
                }
            },
            2 => {
                let source = self.source.as_ref().ok_or_else(|| invalid("mvd.population.source"))?.try_authority()?.get()?;
                let header = source.schema == "stdio.ifc" && matches!(source.header.file_schema.as_slice(), [IfcValue::Aggregate(items)] if matches!(items.as_slice(), [IfcValue::String(name)] if name == "IFC4X3_ADD2"));
                if !header || source.entities.len() > 1_000_000 { self.state.unavailable = true; self.state.phase = 5; }
                else if self.shape.advance(source) { self.state.unavailable = !self.shape.valid(); self.state.phase = if self.state.unavailable { 5 } else { 3 }; }
                progress.copied_bytes += self.shape.copy_bound();
            },
            3 => {
                progress.copied_items = 0;
                cx.consume_retained(progress)?;
                let bytes = size_of::<IfcSourceIndex>();
                let demand = OriginalConstructionDemand { items: 1, copy_bytes: bytes + size_of::<Box<IfcSourceIndex>>(), capacity_bytes: bytes, release_bytes: 0, depth: 1 };
                if admit_original_construction_settled(cx, &mut self.construction, &mut self.index, demand, 1, |_, _| semio_framework_value::RetainedWorkOutcome { result: Ok((Box::new(IfcSourceIndex::default()), RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, retained_capacity_bytes: bytes, ..Default::default() })), work: 1 })? { self.state.phase = 4; }
                return Ok(false);
            },
            4 => {
                progress.copied_items = 0;
                cx.consume_retained(progress)?;
                let source = self.source.as_ref().ok_or_else(|| invalid("mvd.population.source"))?.try_authority()?.get()?;
                if self.index.as_mut().ok_or_else(|| invalid("mvd.population.index"))?.advance(source, epoch, cx)? { self.state.phase = 5; }
                return Ok(self.state.phase == 5);
            },
            5 => return Ok(true),
            _ => return Err(invalid("mvd.population.phase").into()),
        }
        cx.consume_retained(progress)?;
        cx.consume_fuel(1);
        Ok(self.state.phase == 5)
    }
    pub(super) fn status(&self, epoch: u64, cx: &StepContext<'_>) -> Result<IfcMvdPopulationStatus, ValueError> {
        self.original(epoch, cx)?;
        if self.state.phase != 5 { return Ok(IfcMvdPopulationStatus::Pending); }
        Ok(if self.state.unavailable { IfcMvdPopulationStatus::Unavailable } else { IfcMvdPopulationStatus::Prepared })
    }
    pub(super) fn with_request<'cx, T: Copy>(&self, epoch: u64, cx: &mut StepContext<'cx>, run: impl for<'view> FnOnce(IfcMvdPopulationRequestView<'view, R>, &mut StepContext<'cx>) -> Result<T, OriginalTurnFailure>) -> Result<Option<T>, OriginalTurnFailure> {
        self.original(epoch, cx)?;
        if cx.should_yield() { return Ok(None); }
        let request = self.request.as_ref().ok_or_else(|| invalid("mvd.population.request"))?.try_authority()?.get()?;
        let source = if let Some(source) = &self.source { source.try_authority()?.addr() } else { self.state.binding.ok_or_else(|| invalid("mvd.population.source"))?.0 };
        let (source_pin, request_pin, _) = self.state.binding.unwrap();
        if source != source_pin || request as *const R as usize != request_pin { return Err(invalid("mvd.population.source").into()); }
        let pin = IfcMvdPopulationRequestPin { source, request: request_pin, request_type: std::any::TypeId::of::<R>(), epoch, original: self.state.original.unwrap() };
        run(IfcMvdPopulationRequestView { request, pin, status: self.status(epoch, cx)? }, cx).map(Some)
    }
    pub(super) fn with_view<'cx, T: Copy>(&self, epoch: u64, cx: &mut StepContext<'cx>, run: impl for<'view> FnOnce(IfcMvdPopulationView<'view, R>, &mut StepContext<'cx>) -> Result<T, OriginalTurnFailure>) -> Result<Option<T>, OriginalTurnFailure> {
        if self.status(epoch, cx)? != IfcMvdPopulationStatus::Prepared { return Ok(None); }
        if cx.should_yield() { return Ok(None); }
        if !self.shape.complete() || !self.shape.valid() { return Err(invalid("mvd.population.shape").into()); }
        let source = self.source()?;
        let request = self.request.as_ref().ok_or_else(|| invalid("mvd.population.request"))?.try_authority()?.get()?;
        let (source_pin, request_pin, _) = self.state.binding.unwrap();
        if source_pin != source as *const IfcSnapshot as usize || request_pin != request as *const R as usize { return Err(invalid("mvd.population.source").into()); }
        let index = self.index.as_deref().ok_or_else(|| invalid("mvd.population.index"))?;
        let rows = index.identity(source, epoch)?;
        let pin = IfcMvdPopulationPin { source: source_pin, request: request_pin, request_type: std::any::TypeId::of::<R>(), epoch, index: index as *const IfcSourceIndex as usize, rows, original: self.state.original.unwrap() };
        run(IfcMvdPopulationView { source, request, index, pin }, cx).map(Some)
    }
    #[cfg(test)]
    fn metadata(&self) -> State { self.state }
}
