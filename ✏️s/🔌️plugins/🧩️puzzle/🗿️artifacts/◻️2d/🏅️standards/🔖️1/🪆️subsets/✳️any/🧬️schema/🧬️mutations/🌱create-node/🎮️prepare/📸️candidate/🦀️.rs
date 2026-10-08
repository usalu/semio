//! 📸️ Copies native snapshot and borrowed node ownership before bounded ordered placement.

use crate::standards::v1::subsets::any::schema::mutations::native_preparation_child::{Puzzle2dCloseAxis,close_child_demand,close_demand_methods};
use super::{CreateNode, Puzzle2dCreateNodeDisposition, Puzzle2dCreateNodePlan, Puzzle2dCreateNodePreparationCursor, Puzzle2dCreateNodePreparationStep, Puzzle2dSnapshot};
use crate::Puzzle2dNode;
use semio_framework_value::{ValueError, ValueRefusalKind, list::PagedListEditCursor, retirement::controlled::ControlledRetirement, retained_clone::{RetainedClone, RetainedCloneBinding, RetainedFieldCursor, RetainedCloneCursor, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneStep}};
use std::{mem::{ManuallyDrop, size_of}, ops::{Deref, DerefMut}};

pub struct Puzzle2dCreateNodeCandidate { pub plan: Puzzle2dCreateNodePlan, pub snapshot: Option<Puzzle2dSnapshot> }
pub struct Puzzle2dCreateNodeCandidateCursor { state: ManuallyDrop<Puzzle2dCreateNodeCandidateState> }

#[doc(hidden)]
pub struct Puzzle2dCreateNodeCandidateState {
    source: Option<RetainedCloneBinding>,
    mutation: Option<RetainedCloneBinding>,
    preparation: Puzzle2dCreateNodePreparationCursor,
    snapshot_clone: RetainedFieldCursor<Puzzle2dSnapshot>,
    node_clone: RetainedFieldCursor<Puzzle2dNode>,
    plan: Option<Puzzle2dCreateNodePlan>,
    candidate: Option<Puzzle2dSnapshot>,
    pending: Option<Puzzle2dNode>,
    insertion: Option<PagedListEditCursor>,
    candidate_close: Option<ControlledRetirement<Puzzle2dSnapshot>>,
    pending_close: Option<ControlledRetirement<Puzzle2dNode>>,
    phase: u8,
    closing: bool,
}

impl Deref for Puzzle2dCreateNodeCandidateCursor { type Target = Puzzle2dCreateNodeCandidateState; fn deref(&self) -> &Self::Target { &self.state } }
impl DerefMut for Puzzle2dCreateNodeCandidateCursor { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.state } }

impl Default for Puzzle2dCreateNodeCandidateCursor {
    fn default() -> Self { Self { state: ManuallyDrop::new(Puzzle2dCreateNodeCandidateState { source: None, mutation: None, preparation: Default::default(), snapshot_clone: RetainedFieldCursor::<Puzzle2dSnapshot>::default(), node_clone: RetainedFieldCursor::<Puzzle2dNode>::default(), plan: None, candidate: None, pending: None, insertion: None, candidate_close: None, pending_close: None, phase: 0, closing: false }) } }
}

impl Puzzle2dCreateNodeCandidateCursor {
    pub fn advance(&mut self, source: RetainedCloneRef<'_, Puzzle2dSnapshot>, mutation: RetainedCloneRef<'_, CreateNode>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.closing || self.phase == 13 { return Err(refusal("create candidate is closing or spent")); }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        source.bind(&mut self.source)?;
        mutation.bind(&mut self.mutation)?;
        match self.phase {
            0 => {
                let step = self.preparation.advance(source, mutation, grant)?;
                let progress = match step { Puzzle2dCreateNodePreparationStep::Pending(progress) => progress, Puzzle2dCreateNodePreparationStep::Complete { plan, progress } => { self.plan = self.preparation.take(); self.preparation.begin_close(); self.phase = 1; if self.plan != Some(plan) { return Err(refusal("create candidate lost its scalar plan")); } progress } };
                Ok(RetainedCloneStep::Progress(progress))
            }
            1 => {
                let step = self.preparation.close_step(grant)?;
                if self.preparation.terminal_is_empty() { self.phase = if self.plan.is_some_and(|plan| plan.disposition == Puzzle2dCreateNodeDisposition::Changed) { 2 } else { 11 }; }
                Ok(close_progress(step))
            }
            2 => {
                let step = self.snapshot_clone.advance(source, grant)?;
                if matches!(step, RetainedCloneStep::Complete(_)) { self.phase = 3; }
                Ok(RetainedCloneStep::Progress(step.progress()))
            }
            3 => {
                if grant.maximum_copy_bytes < size_of::<Puzzle2dSnapshot>() { return Ok(RetainedCloneStep::Progress(Default::default())); }
                self.candidate = Some(self.snapshot_clone.take().ok_or_else(|| refusal("create candidate snapshot completed without its owner"))?);
                self.snapshot_clone.begin_close();self.phase = 4;
                Ok(payload_progress(size_of::<Puzzle2dSnapshot>()))
            }
            4 => {
                let step = self.snapshot_clone.close_step(grant)?;
                if self.snapshot_clone.terminal_is_empty() { self.phase = 5; }
                Ok(RetainedCloneStep::Progress(step.progress()))
            }
            5 => {
                let step = self.node_clone.advance(mutation.project(1, |payload| &payload.node), grant)?;
                if matches!(step, RetainedCloneStep::Complete(_)) { self.phase = 6; }
                Ok(RetainedCloneStep::Progress(step.progress()))
            }
            6 => {
                if grant.maximum_copy_bytes < size_of::<Puzzle2dNode>() { return Ok(RetainedCloneStep::Progress(Default::default())); }
                self.pending = Some(self.node_clone.take().ok_or_else(|| refusal("create candidate node completed without its owner"))?);
                self.node_clone.begin_close();self.phase = 7;
                Ok(payload_progress(size_of::<Puzzle2dNode>()))
            }
            7 => {
                let step = self.node_clone.close_step(grant)?;
                if self.node_clone.terminal_is_empty() { self.phase = 8; }
                Ok(RetainedCloneStep::Progress(step.progress()))
            }
            8 => {
                if grant.maximum_copy_bytes < size_of::<PagedListEditCursor>() { return Ok(RetainedCloneStep::Progress(Default::default())); }
                let position = self.plan.and_then(|plan| plan.position).ok_or_else(|| refusal("create candidate has no admitted insertion position"))?;
                let length = self.candidate.as_ref().ok_or_else(|| refusal("create candidate lost its snapshot"))?.nodes.len();
                self.insertion = Some(PagedListEditCursor::insert(position, length));self.phase = 9;
                Ok(payload_progress(size_of::<PagedListEditCursor>()))
            }
            9 => {
                let state = &mut **self;
                let nodes = &mut state.candidate.as_mut().ok_or_else(|| refusal("create insertion lost its snapshot"))?.nodes;
                let capacity = state.pending.is_some() && !nodes.has_reserved_slot();
                let bytes = if capacity { grant.maximum_capacity_bytes } else { grant.maximum_copy_bytes };
                if capacity {
                    let demand = nodes.next_capacity_allocation_bytes(1).map_err(ValueError::from)?.ok_or_else(|| refusal("create insertion lost its capacity demand"))?;
                    if demand > bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
                }
                let step = state.insertion.as_mut().ok_or_else(|| refusal("create insertion lost its ordered cursor"))?.step(nodes, &mut state.pending, 1, bytes).map_err(ValueError::from)?;
                if step.complete { state.phase = 10; }
                Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: step.moved_items, copied_bytes: step.progress.placed_bytes, retained_capacity_bytes: step.progress.allocated_bytes, released_bytes: step.progress.released_allocation_bytes }))
            }
            10 => { self.insertion = None;self.phase = 11;Ok(payload_progress(0)) }
            11 => {
                if grant.maximum_copy_bytes < size_of::<Puzzle2dCreateNodeCandidate>() { return Ok(RetainedCloneStep::Progress(Default::default())); }
                self.phase = 12;
                Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<Puzzle2dCreateNodeCandidate>(), retained_capacity_bytes: 0, released_bytes: 0 }))
            }
            12 => Ok(RetainedCloneStep::Complete(Default::default())),
            _ => Err(refusal("create candidate has an unknown ownership phase")),
        }
    }

    pub fn take(&mut self) -> Option<Puzzle2dCreateNodeCandidate> { if self.closing || self.phase != 12 { return None; } self.phase = 13;Some(Puzzle2dCreateNodeCandidate { plan: self.plan.take()?, snapshot: self.candidate.take() }) }
    pub fn begin_close(&mut self) { self.closing = true;self.preparation.begin_close();self.snapshot_clone.begin_close();self.node_clone.begin_close(); }

    fn close_demand(&self,axis:Puzzle2dCloseAxis)->Result<usize,ValueError>{
        if !self.preparation.terminal_is_empty(){return close_child_demand!(axis,self.preparation)}
        if !self.snapshot_clone.terminal_is_empty(){return axis.retained::<Puzzle2dSnapshot,_>(&self.snapshot_clone)}
        if !self.node_clone.terminal_is_empty(){return axis.retained::<Puzzle2dNode,_>(&self.node_clone)}
        if let Some(owner)=self.pending_close.as_ref(){return axis.retirement(owner)}
        if self.pending.is_some(){return axis.inline::<Puzzle2dNode>()}
        if let Some(owner)=self.candidate_close.as_ref(){return axis.retirement(owner)}
        if self.candidate.is_some(){return axis.inline::<Puzzle2dSnapshot>()}
        if self.insertion.is_some(){return axis.inline::<Option<PagedListEditCursor>>()}
        if self.plan.is_some(){return axis.inline::<Option<Puzzle2dCreateNodePlan>>()}
        axis.binding(if self.source.is_some(){&self.source}else{&self.mutation})
    }

    close_demand_methods!(next_close_copy_byte_demand,next_close_capacity_byte_demand,next_close_release_byte_demand,next_close_depth_demand);

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if !self.closing { return Err(refusal("create candidate closure was not started")); }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !self.preparation.terminal_is_empty() { return self.preparation.close_step(grant).map(close_progress); }
        if !self.snapshot_clone.terminal_is_empty() { return self.snapshot_clone.close_step(grant).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if !self.node_clone.terminal_is_empty() { return self.node_clone.close_step(grant).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if let Some(owner) = self.pending_close.as_mut() { let step = owner.step(grant)?;if owner.terminal_is_empty() { self.pending_close = None; }return Ok(RetainedCloneStep::Progress(step.progress())); }
        if self.pending.is_some() {
            if grant.maximum_copy_bytes < size_of::<Puzzle2dNode>() { return Ok(RetainedCloneStep::Progress(Default::default())); }
            match ControlledRetirement::new(self.pending.take().unwrap()) { Ok(owner) => self.pending_close = Some(owner), Err((error, owner)) => { self.pending = Some(owner);return Err(error); } }
            return Ok(payload_progress(size_of::<Puzzle2dNode>()));
        }
        if let Some(owner) = self.candidate_close.as_mut() { let step = owner.step(grant)?;if owner.terminal_is_empty() { self.candidate_close = None; }return Ok(RetainedCloneStep::Progress(step.progress())); }
        if self.candidate.is_some() {
            if grant.maximum_copy_bytes < size_of::<Puzzle2dSnapshot>() { return Ok(RetainedCloneStep::Progress(Default::default())); }
            match ControlledRetirement::new(self.candidate.take().unwrap()) { Ok(owner) => self.candidate_close = Some(owner), Err((error, owner)) => { self.candidate = Some(owner);return Err(error); } }
            return Ok(payload_progress(size_of::<Puzzle2dSnapshot>()));
        }
        if self.insertion.is_some() { let bytes=size_of::<Option<PagedListEditCursor>>();if grant.maximum_copy_bytes<bytes{return Ok(RetainedCloneStep::Progress(Default::default()));}self.insertion=None;return Ok(payload_progress(bytes)); }
        if self.plan.is_some() { let bytes=size_of::<Option<Puzzle2dCreateNodePlan>>();if grant.maximum_copy_bytes<bytes{return Ok(RetainedCloneStep::Progress(Default::default()));}self.plan=None;return Ok(payload_progress(bytes)); }
        let step = RetainedCloneBinding::close_one(if self.source.is_some(){&mut self.source}else{&mut self.mutation}, grant)?;
        Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { close_progress(step) })
    }

    pub fn terminal_is_empty(&self) -> bool { self.closing && self.preparation.terminal_is_empty() && self.snapshot_clone.terminal_is_empty() && self.node_clone.terminal_is_empty() && self.plan.is_none() && self.candidate.is_none() && self.pending.is_none() && self.insertion.is_none() && self.candidate_close.is_none() && self.pending_close.is_none() && self.source.is_none() && self.mutation.is_none() }
}

impl Drop for Puzzle2dCreateNodeCandidateCursor { fn drop(&mut self) { let empty = self.terminal_is_empty();assert!(std::thread::panicking() || empty, "create candidate abandoned before controlled closure");if empty { unsafe { ManuallyDrop::drop(&mut self.state); } } } }

fn payload_progress(bytes: usize) -> RetainedCloneStep { RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, retained_capacity_bytes: 0, released_bytes: 0 }) }
fn close_progress(step: RetainedCloneStep) -> RetainedCloneStep { RetainedCloneStep::Progress(step.progress()) }
fn refusal(message: &str) -> ValueError { ValueError::new(ValueRefusalKind::InvariantViolated, message) }

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
