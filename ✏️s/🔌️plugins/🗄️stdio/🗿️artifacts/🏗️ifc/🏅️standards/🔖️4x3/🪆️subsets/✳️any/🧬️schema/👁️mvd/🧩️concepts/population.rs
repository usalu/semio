//! 🪢️ One original population owner lends a completed source and stable index for a single clause turn.
use super::native::ShapeCursor;
use semio_framework_job::{OriginalConstructionDemand, OriginalSourceBinding, OriginalTurnFailure, StepContext, admit_original_construction};
use semio_framework_value::retained_clone::{RetainedCloneClose, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneSource, retained_clone_grant_admits};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind, retirement::{RetireOwned, RetirementCursor, RetirementStep}};
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::{IfcSnapshot, IfcValue};
use std::{mem::size_of, sync::Arc};
use super::source_index::{IdMatch, IfcSourceIndex};
#[cfg(test)]
#[path = "population-tests.rs"]
mod tests;
type SnapshotSource = RetainedCloneSource<IfcSnapshot, Arc<IfcSnapshot>>;
pub(super) type IfcMvdPopulationRequestSource<R> = RetainedCloneSource<R, Arc<R>>;
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
    close_phase: u8,
    unavailable: bool,
    failed: bool,
    closing: bool,
    closed: bool,
}
semio_framework_value::artifact_retire_leaf!(State, IfcMvdPopulationPin, IfcMvdPopulationRequestPin);
#[derive(Default)]
struct PopulationClose(RetainedCloneClose);
impl std::ops::Deref for PopulationClose { type Target = RetainedCloneClose; fn deref(&self) -> &Self::Target { &self.0 } }
impl std::ops::DerefMut for PopulationClose { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 } }
impl RetireOwned for PopulationClose {
    fn retirement(self) -> Box<dyn RetirementCursor> { Box::new(self) }
    fn retirement_birth_bytes(&self) -> Option<usize> { Some(size_of::<Self>()) }
    fn controlled_retirement_supported() -> bool { true }
}
impl RetirementCursor for PopulationClose {
    fn next_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> { self.0.next_owner_demands::<State>(false, &None, body) }
    fn close_step(&mut self, grant: RetainedCloneGrant) -> RetirementStep { match self.0.step_granted(grant) { Ok(step) => RetirementStep::Progress(step.progress()), Err(error) => RetirementStep::Failure(error) } }
    fn terminal_is_empty(&self) -> bool { self.0.is_empty() }
    fn terminal_release_bytes(&self) -> Option<usize> { Some(size_of::<Self>()) }
}
#[derive(semio_framework_value::RetireOwned)]
pub(super) struct IfcMvdPopulationWork<R: RetireOwned + Sync> {
    index: Option<Box<IfcSourceIndex>>,
    retiring: PopulationClose,
    shape: ShapeCursor,
    state: State,
    source: Option<SnapshotSource>,
    unborn: Option<Arc<IfcSnapshot>>,
    request: Option<IfcMvdPopulationRequestSource<R>>,
}
impl<R: RetireOwned + Sync> Default for IfcMvdPopulationWork<R> {
    fn default() -> Self { Self { index: None, retiring: Default::default(), shape: Default::default(), state: Default::default(), source: None, unborn: None, request: None } }
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
    pub(super) fn receive_demands() -> RetirementDemand { RetirementDemand { copy_bytes: Self::base().copy_bytes + 2 * size_of::<IfcMvdPopulationRequestSource<R>>() + 2 * size_of::<Arc<IfcSnapshot>>(), depth: 1, ..Default::default() } }
    fn source(&self) -> Result<&IfcSnapshot, ValueError> { self.source.as_ref().ok_or_else(|| invalid("mvd.population.source"))?.try_authority().map(Arc::as_ref) }
    fn original(&self, epoch: u64, cx: &StepContext<'_>) -> Result<(), ValueError> {
        if cx.is_cancelled() { return Err(cancelled()); }
        if self.state.original.is_none_or(|original| !original.matches(cx)) || self.state.binding.is_none_or(|(_, _, old)| old != epoch) || self.state.closing || self.state.failed { return Err(invalid("mvd.population.original")); }
        Ok(())
    }
    fn source_matches(&self, snapshot: &Arc<IfcSnapshot>, epoch: u64) -> Result<(), ValueError> {
        if self.state.binding.is_none_or(|(source, _, old)| source != Arc::as_ptr(snapshot) as usize || old != epoch) { return Err(invalid("mvd.population.source")); }
        Ok(())
    }
    pub(super) fn receive(&mut self, snapshot: &Arc<IfcSnapshot>, donor: &mut Option<IfcMvdPopulationRequestSource<R>>, epoch: u64, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        if cx.is_cancelled() { return Err(cancelled().into()); }
        if self.state != State::default() || self.request.is_some() || self.source.is_some() || self.unborn.is_some() || self.index.is_some() || !self.retiring.is_empty() { return Err(invalid("mvd.population.receive").into()); }
        let request = donor.as_ref().ok_or_else(|| invalid("mvd.population.request"))?.try_authority()?;
        let binding = (Arc::as_ptr(snapshot) as usize, Arc::as_ptr(request) as usize, epoch);
        let demand = Self::receive_demands();
        if cx.should_yield() || !retained_clone_grant_admits(cx.retained_grant(), demand) { return Ok(false); }
        self.unborn = Some(Arc::clone(snapshot));
        self.request = donor.take();
        self.state.binding = Some(binding);
        self.state.original = Some(OriginalSourceBinding::borrow(cx));
        self.state.phase = 1;
        cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })?;
        cx.consume_fuel(1);
        Ok(true)
    }
    pub(super) fn receive_donated(&mut self, snapshot: &mut Option<Arc<IfcSnapshot>>, donor: &mut Option<IfcMvdPopulationRequestSource<R>>, epoch: u64, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        if cx.is_cancelled() { return Err(cancelled().into()); }
        if self.state != State::default() || self.request.is_some() || self.source.is_some() || self.unborn.is_some() || self.index.is_some() || !self.retiring.is_empty() { return Err(invalid("mvd.population.receive").into()); }
        let source = snapshot.as_ref().ok_or_else(|| invalid("mvd.population.source"))?;
        let request = donor.as_ref().ok_or_else(|| invalid("mvd.population.request"))?.try_authority()?;
        let binding = (Arc::as_ptr(source) as usize, Arc::as_ptr(request) as usize, epoch);
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
            1 => { let birth = SnapshotSource::borrowed_constructor_demand::<Arc<IfcSnapshot>>(); RetirementDemand { copy_bytes: SnapshotSource::borrowed_constructor_copy_bytes::<Arc<IfcSnapshot>>() + 2 * size_of::<Arc<IfcSnapshot>>(), capacity_bytes: birth.capacity_bytes, depth: birth.depth, ..Default::default() } },
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
                let grant = RetainedCloneGrant { maximum_copy_bytes: cx.retained_grant().maximum_copy_bytes - progress.copied_bytes - 2 * size_of::<Arc<IfcSnapshot>>(), ..cx.retained_grant() };
                let result = SnapshotSource::admit_borrowed(self.unborn.take().ok_or_else(|| invalid("mvd.population.source"))?, |value| value.as_ref(), grant);
                match result {
                    Ok((owner, child)) => { self.source = Some(owner); self.state.phase = 2; progress.copied_bytes += child.copied_bytes + 2 * size_of::<Arc<IfcSnapshot>>(); progress.retained_capacity_bytes = child.retained_capacity_bytes; progress.released_bytes = child.released_bytes; },
                    Err((error, owner)) => {
                        self.unborn = Some(owner);
                        let child = error.retained_progress();
                        progress.copied_bytes += child.copied_bytes + 2 * size_of::<Arc<IfcSnapshot>>();
                        progress.retained_capacity_bytes = child.retained_capacity_bytes;
                        progress.released_bytes = child.released_bytes;
                        let mut failure = OriginalTurnFailure::from(error);
                        if let Err(error) = cx.consume_retained(progress) { failure.context_refusal = Some(error); }
                        cx.consume_fuel(1);
                        return Err(failure);
                    }
                }
            },
            2 => {
                let source = self.source.as_ref().ok_or_else(|| invalid("mvd.population.source"))?.try_authority()?.as_ref();
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
                if admit_original_construction(cx, &mut self.index, demand, |_| Ok((Box::new(IfcSourceIndex::default()), RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, retained_capacity_bytes: bytes, ..Default::default() })))? { self.state.phase = 4; }
                return Ok(false);
            },
            4 => {
                progress.copied_items = 0;
                cx.consume_retained(progress)?;
                let source = self.source.as_ref().ok_or_else(|| invalid("mvd.population.source"))?.try_authority()?.as_ref();
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
        let request = self.request.as_ref().ok_or_else(|| invalid("mvd.population.request"))?.try_authority()?.as_ref();
        let source = if let Some(source) = &self.source { Arc::as_ptr(source.try_authority()?) as usize } else { Arc::as_ptr(self.unborn.as_ref().ok_or_else(|| invalid("mvd.population.source"))?) as usize };
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
        let request = self.request.as_ref().ok_or_else(|| invalid("mvd.population.request"))?.try_authority()?.as_ref();
        let (source_pin, request_pin, _) = self.state.binding.unwrap();
        if source_pin != source as *const IfcSnapshot as usize || request_pin != request as *const R as usize { return Err(invalid("mvd.population.source").into()); }
        let index = self.index.as_deref().ok_or_else(|| invalid("mvd.population.index"))?;
        let rows = index.identity(source, epoch)?;
        let pin = IfcMvdPopulationPin { source: source_pin, request: request_pin, request_type: std::any::TypeId::of::<R>(), epoch, index: index as *const IfcSourceIndex as usize, rows, original: self.state.original.unwrap() };
        run(IfcMvdPopulationView { source, request, index, pin }, cx).map(Some)
    }
    fn close_metadata() -> RetirementDemand { RetirementDemand { copy_bytes: 2 * size_of::<ShapeCursor>() + 4 * size_of::<State>() + 4 * size_of::<usize>(), depth: 1, ..Default::default() } }
    fn can_close(&self, cx: &StepContext<'_>) -> Result<(), ValueError> { if self.state.original.is_some_and(|original| !original.matches(cx)) { Err(invalid("mvd.population.original")) } else { Ok(()) } }
    pub(super) fn begin_close(&mut self, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        self.can_close(cx)?;
        if self.state.closing { return Ok(true); }
        let demand = Self::close_metadata();
        if cx.retirement_should_yield() || !retained_clone_grant_admits(cx.retained_grant(), demand) { return Ok(false); }
        self.shape = ShapeCursor::default();
        self.state.original = Some(OriginalSourceBinding::borrow(cx));
        self.state.closing = true;
        self.state.close_phase = 1;
        cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })?;
        cx.consume_fuel(1);
        Ok(true)
    }
    pub(super) fn close_demands(&self) -> Result<RetirementDemand, ValueError> {
        let child = if !self.state.closing { return Ok(Self::close_metadata()); } else { match self.state.close_phase {
            1 => self.retiring.next_owner_demands::<Box<IfcSourceIndex>>(self.index.is_some(), &None, 32768)?,
            2 => self.retiring.next_owner_demands::<SnapshotSource>(self.source.is_some(), &None, 32768)?,
            3 => self.retiring.next_owner_demands::<Arc<IfcSnapshot>>(self.unborn.is_some(), &None, 32768)?,
            4 => self.retiring.next_owner_demands::<IfcMvdPopulationRequestSource<R>>(self.request.is_some(), &None, 32768)?,
            _ => return Ok(Self::close_metadata()),
        }};
        plus(Self::close_metadata(), child)
    }
    pub(super) fn terminal_is_empty(&self) -> bool { self.state.closed && self.index.is_none() && self.retiring.is_empty() && self.source.is_none() && self.unborn.is_none() && self.request.is_none() }
    pub(super) fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        self.can_close(cx)?;
        if !self.state.closing { self.begin_close(cx)?; return Ok(false); }
        if self.terminal_is_empty() { return Ok(true); }
        let demand = self.close_demands()?;
        if cx.retirement_should_yield() || !retained_clone_grant_admits(cx.retained_grant(), demand) { return Ok(false); }
        let phase = self.state.close_phase;
        let empty = match phase { 1 => self.index.is_none(), 2 => self.source.is_none(), 3 => self.unborn.is_none(), 4 => self.request.is_none(), _ => true } && self.retiring.is_empty();
        cx.consume_retained(RetainedCloneProgress { copied_items: usize::from(empty), copied_bytes: Self::close_metadata().copy_bytes, ..Default::default() })?;
        if empty {
            if phase >= 5 { self.state.closed = true; self.state.binding = None; } else { self.state.close_phase += 1; }
            cx.consume_fuel(1);
        } else {
            let close = &mut self.retiring;
            macro_rules! step { ($owner:expr, $ty:ty) => {{
                let child = close.next_owner_demands::<$ty>($owner.is_some(), &None, 32768)?;
                semio_framework_job::close_original_retained_step(cx, child, |grant| if close.is_empty() { close.begin_granted($owner, grant)?.ok_or_else(|| invalid("mvd.population.close")) } else { close.step_granted(grant) })?;
            }} }
            match phase { 1 => step!(&mut self.index, Box<IfcSourceIndex>), 2 => step!(&mut self.source, SnapshotSource), 3 => step!(&mut self.unborn, Arc<IfcSnapshot>), 4 => step!(&mut self.request, IfcMvdPopulationRequestSource<R>), _ => return Err(invalid("mvd.population.close").into()) }
        }
        Ok(self.terminal_is_empty())
    }
    #[cfg(test)]
    fn metadata(&self) -> State { self.state }
}
