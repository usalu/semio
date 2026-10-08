//! 📸️ Copies native snapshot and borrowed connection ownership before bounded ordered placement.

use super::{ConnectHandles, Puzzle2dConnectHandlesDisposition, Puzzle2dConnectHandlesPlan, Puzzle2dConnectHandlesPreparationCursor, Puzzle2dConnectHandlesPreparationStep, Puzzle2dSnapshot};
use crate::Puzzle2dEdge;
use super::edge::Puzzle2dConnectEdgeCursor;
use semio_framework_value::{SnapshotRetirementStep, ValueError, ValueRefusalKind, list::PagedListEditCursor, retirement::controlled::ControlledRetirement, retained_clone::{RetainedClone, RetainedCloneBinding, RetainedFieldCursor, RetainedCloneCursor, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneStep, ordered_map::BoundedOrdGrant}};
use crate::standards::v1::subsets::any::schema::mutations::native_preparation_child::Puzzle2dPreparationChild;
use std::{mem::{ManuallyDrop, size_of}, ops::{Deref, DerefMut}};

pub struct Puzzle2dConnectHandlesCandidate { pub plan: Puzzle2dConnectHandlesPlan, pub snapshot: Option<Puzzle2dSnapshot> }
pub struct Puzzle2dConnectHandlesCandidateCursor { state: ManuallyDrop<Puzzle2dConnectHandlesCandidateState> }

#[doc(hidden)]
pub struct Puzzle2dConnectHandlesCandidateState {
    source: Option<RetainedCloneBinding>,
    mutation: Option<RetainedCloneBinding>,
    preparation: Puzzle2dPreparationChild<Puzzle2dConnectHandlesPreparationCursor>,
    snapshot_clone: RetainedFieldCursor<Puzzle2dSnapshot>,
    edge_assembly: Puzzle2dPreparationChild<Puzzle2dConnectEdgeCursor>,
    plan: Option<Puzzle2dConnectHandlesPlan>,
    candidate: Option<Puzzle2dSnapshot>,
    pending: Option<Puzzle2dEdge>,
    insertion: Option<PagedListEditCursor>,
    candidate_close: Option<ControlledRetirement<Puzzle2dSnapshot>>,
    pending_close: Option<ControlledRetirement<Puzzle2dEdge>>,
    phase: u8,
    closing: bool,
}

impl Deref for Puzzle2dConnectHandlesCandidateCursor { type Target = Puzzle2dConnectHandlesCandidateState; fn deref(&self) -> &Self::Target { &self.state } }
impl DerefMut for Puzzle2dConnectHandlesCandidateCursor { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.state } }

impl Default for Puzzle2dConnectHandlesCandidateCursor {
    fn default() -> Self { Self { state: ManuallyDrop::new(Puzzle2dConnectHandlesCandidateState { source: None, mutation: None, preparation: Default::default(), snapshot_clone: RetainedFieldCursor::<Puzzle2dSnapshot>::default(), edge_assembly: Default::default(), plan: None, candidate: None, pending: None, insertion: None, candidate_close: None, pending_close: None, phase: 0, closing: false }) } }
}

impl Puzzle2dConnectHandlesCandidateCursor {
    pub fn advance(&mut self, source: RetainedCloneRef<'_, Puzzle2dSnapshot>, mutation: RetainedCloneRef<'_, ConnectHandles>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.closing || self.phase == 13 { return Err(refusal("connect candidate is closing or spent")); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        source.bind(&mut self.source)?;
        mutation.bind(&mut self.mutation)?;
        match self.phase {
            0 => {
                if let Some(step)=self.preparation.ensure(grant)? {return Ok(step);}
                let step = self.preparation.owner_mut().advance(source, mutation, BoundedOrdGrant { maximum_items: 1, maximum_bytes: grant.maximum_copy_bytes })?;
                let progress = match step { Puzzle2dConnectHandlesPreparationStep::Pending(progress) => progress, Puzzle2dConnectHandlesPreparationStep::Complete { plan, progress } => { self.plan = self.preparation.owner_mut().take(); self.preparation.begin_close(); self.phase = 1; if self.plan != Some(plan) { return Err(refusal("connect candidate lost its scalar plan")); } progress } };
                Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: progress.compared_items, copied_bytes: progress.compared_bytes, retained_capacity_bytes: 0, released_bytes: 0 }))
            }
            1 => {
                let step = self.preparation.close_granted(grant)?;
                if matches!(step,RetainedCloneStep::Complete(_)) { self.phase = if self.plan.is_some_and(|plan| plan.disposition == Puzzle2dConnectHandlesDisposition::Changed) { 2 } else { 11 }; }
                Ok(RetainedCloneStep::Progress(step.progress()))
            }
            2 => {
                let step = self.snapshot_clone.advance(source, grant)?;
                if matches!(step, RetainedCloneStep::Complete(_)) { self.phase = 3; }
                Ok(RetainedCloneStep::Progress(step.progress()))
            }
            3 => {
                if grant.maximum_copy_bytes < size_of::<Puzzle2dSnapshot>() { return Ok(RetainedCloneStep::Progress(Default::default())); }
                self.candidate = Some(self.snapshot_clone.take().ok_or_else(|| refusal("connect candidate snapshot completed without its owner"))?);
                self.snapshot_clone.begin_close();self.phase = 4;
                Ok(payload_progress(size_of::<Puzzle2dSnapshot>()))
            }
            4 => {
                let step = self.snapshot_clone.close_granted(grant)?;
                if self.snapshot_clone.terminal_is_empty() { self.phase = 5; }
                Ok(RetainedCloneStep::Progress(step.progress()))
            }
            5 => {
                if let Some(step)=self.edge_assembly.ensure(grant)? {return Ok(step);}
                let step = self.edge_assembly.owner_mut().advance(mutation, grant)?;
                if matches!(step, RetainedCloneStep::Complete(_)) { self.phase = 6; }
                Ok(RetainedCloneStep::Progress(step.progress()))
            }
            6 => {
                if grant.maximum_copy_bytes < size_of::<Puzzle2dEdge>() { return Ok(RetainedCloneStep::Progress(Default::default())); }
                self.pending = Some(self.edge_assembly.owner_mut().take().ok_or_else(|| refusal("connect candidate edge completed without its owner"))?);
                self.edge_assembly.begin_close();self.phase = 7;
                Ok(payload_progress(size_of::<Puzzle2dEdge>()))
            }
            7 => {
                let step = self.edge_assembly.close_granted(grant)?;
                if self.edge_assembly.terminal_is_empty() { self.phase = 8; }
                Ok(RetainedCloneStep::Progress(step.progress()))
            }
            8 => {
                if grant.maximum_copy_bytes < size_of::<PagedListEditCursor>() { return Ok(RetainedCloneStep::Progress(Default::default())); }
                let position = self.plan.and_then(|plan| plan.position).ok_or_else(|| refusal("connect candidate has no admitted insertion position"))?;
                let length = self.candidate.as_ref().ok_or_else(|| refusal("connect candidate lost its snapshot"))?.edges.len();
                self.insertion = Some(PagedListEditCursor::insert(position, length));self.phase = 9;
                Ok(payload_progress(size_of::<PagedListEditCursor>()))
            }
            9 => {
                let state = &mut **self;
                let edges = &mut state.candidate.as_mut().ok_or_else(|| refusal("connect insertion lost its snapshot"))?.edges;
                let capacity = state.pending.is_some() && !edges.has_reserved_slot();
                let bytes = if capacity { grant.maximum_capacity_bytes } else { grant.maximum_copy_bytes };
                if capacity {
                    let demand = edges.next_capacity_allocation_bytes(1).map_err(ValueError::from)?.ok_or_else(|| refusal("connect insertion lost its capacity demand"))?;
                    if demand > bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
                }
                let step = state.insertion.as_mut().ok_or_else(|| refusal("connect insertion lost its ordered cursor"))?.step(edges, &mut state.pending, 1, bytes).map_err(ValueError::from)?;
                if step.complete { state.phase = 10; }
                Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: step.moved_items, copied_bytes: step.progress.placed_bytes, retained_capacity_bytes: step.progress.allocated_bytes, released_bytes: step.progress.released_allocation_bytes }))
            }
            10 => { self.insertion = None;self.phase = 11;Ok(payload_progress(0)) }
            11 => {
                if grant.maximum_copy_bytes < size_of::<Puzzle2dConnectHandlesCandidate>() { return Ok(RetainedCloneStep::Progress(Default::default())); }
                self.phase = 12;
                Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<Puzzle2dConnectHandlesCandidate>(), retained_capacity_bytes: 0, released_bytes: 0 }))
            }
            12 => Ok(RetainedCloneStep::Complete(Default::default())),
            _ => Err(refusal("connect candidate has an unknown ownership phase")),
        }
    }

    pub fn take(&mut self) -> Option<Puzzle2dConnectHandlesCandidate> { if self.closing || self.phase != 12 { return None; } self.phase = 13;Some(Puzzle2dConnectHandlesCandidate { plan: self.plan.take()?, snapshot: self.candidate.take() }) }
    pub fn begin_close(&mut self) { self.closing = true;self.preparation.begin_close();self.snapshot_clone.begin_close();self.edge_assembly.begin_close(); }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if !self.closing { return Err(refusal("connect candidate closure was not started")); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !self.preparation.terminal_is_empty() { return self.preparation.close_granted(grant); }
        if !self.snapshot_clone.terminal_is_empty() { return self.snapshot_clone.close_granted(grant).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if !self.edge_assembly.terminal_is_empty() { return self.edge_assembly.close_granted(grant).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if let Some(owner) = self.pending_close.as_mut() { let step = owner.step(grant)?;if owner.terminal_is_empty() { self.pending_close = None; }return Ok(RetainedCloneStep::Progress(step.progress())); }
        if self.pending.is_some() {
            if grant.maximum_copy_bytes < size_of::<Puzzle2dEdge>() { return Ok(RetainedCloneStep::Progress(Default::default())); }
            match ControlledRetirement::new(self.pending.take().unwrap()) { Ok(owner) => self.pending_close = Some(owner), Err((error, owner)) => { self.pending = Some(owner);return Err(error); } }
            return Ok(payload_progress(size_of::<Puzzle2dEdge>()));
        }
        if let Some(owner) = self.candidate_close.as_mut() { let step = owner.step(grant)?;if owner.terminal_is_empty() { self.candidate_close = None; }return Ok(RetainedCloneStep::Progress(step.progress())); }
        if self.candidate.is_some() {
            if grant.maximum_copy_bytes < size_of::<Puzzle2dSnapshot>() { return Ok(RetainedCloneStep::Progress(Default::default())); }
            match ControlledRetirement::new(self.candidate.take().unwrap()) { Ok(owner) => self.candidate_close = Some(owner), Err((error, owner)) => { self.candidate = Some(owner);return Err(error); } }
            return Ok(payload_progress(size_of::<Puzzle2dSnapshot>()));
        }
        if self.insertion.take().is_some() { return Ok(release_progress(0)); }
        if self.plan.take().is_some() { return Ok(release_progress(0)); }
        let step = RetainedCloneBinding::close_one(&mut self.source, 1)?;
        if step != SnapshotRetirementStep::Complete { return Ok(close_progress(step)); }
        let step = RetainedCloneBinding::close_one(&mut self.mutation, 1)?;
        Ok(if step == SnapshotRetirementStep::Complete { RetainedCloneStep::Complete(Default::default()) } else { close_progress(step) })
    }

    pub fn terminal_is_empty(&self) -> bool { self.closing && self.preparation.terminal_is_empty() && self.snapshot_clone.terminal_is_empty() && self.edge_assembly.terminal_is_empty() && self.plan.is_none() && self.candidate.is_none() && self.pending.is_none() && self.insertion.is_none() && self.candidate_close.is_none() && self.pending_close.is_none() && self.source.is_none() && self.mutation.is_none() }
}

impl Drop for Puzzle2dConnectHandlesCandidateCursor { fn drop(&mut self) { let empty = self.terminal_is_empty();assert!(std::thread::panicking() || empty, "connect candidate abandoned before controlled closure");if empty { unsafe { ManuallyDrop::drop(&mut self.state); } } } }

fn payload_progress(bytes: usize) -> RetainedCloneStep { RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, retained_capacity_bytes: 0, released_bytes: 0 }) }
fn close_progress(step: SnapshotRetirementStep) -> RetainedCloneStep { RetainedCloneStep::Progress(match step { SnapshotRetirementStep::Pending { released_items, released_bytes } => RetainedCloneProgress { copied_items: released_items, copied_bytes: 0, retained_capacity_bytes: 0, released_bytes }, _ => Default::default() }) }
fn refusal(message: &str) -> ValueError { ValueError::new(ValueRefusalKind::InvariantViolated, message) }

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

fn release_progress(bytes: usize) -> RetainedCloneStep { RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: bytes, ..Default::default() }) }
