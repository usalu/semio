//! ↩️ Admits inverse identifier ownership directly from the original borrowed move payload.

use super::{MoveNode, Puzzle2dMoveNodePlan, Puzzle2dMoveNodePreparationCursor, Puzzle2dMoveNodePreparationStep, Puzzle2dSnapshot};
use crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;
use semio_framework_value::{SnapshotRetirementStep, ValueError, ValueRefusalKind, list::PagedList, paged::PagedUtf8, retirement::controlled::ControlledRetirement, retained_clone::{RetainedClone, RetainedCloneBinding, RetainedCloneCursor, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneStep, ordered_map::BoundedOrdGrant}};
use std::{mem::{ManuallyDrop, size_of}, ops::{Deref, DerefMut}};

pub struct Puzzle2dMoveNodeInverseCursor { state: ManuallyDrop<Puzzle2dMoveNodeInverseState> }

#[doc(hidden)]
pub struct Puzzle2dMoveNodeInverseState {
    source: Option<RetainedCloneBinding>,
    mutation: Option<RetainedCloneBinding>,
    preparation: Puzzle2dMoveNodePreparationCursor,
    identifier: <PagedUtf8<{usize::MAX}> as RetainedClone>::Cursor,
    plan: Option<Puzzle2dMoveNodePlan>,
    pending: Option<MoveNode>,
    inverse: PagedList<Puzzle2dMutation, {usize::MAX}>,
    pending_close: Option<ControlledRetirement<MoveNode>>,
    inverse_close: Option<ControlledRetirement<PagedList<Puzzle2dMutation, {usize::MAX}>>>,
    phase: u8,
    closing: bool,
}

impl Deref for Puzzle2dMoveNodeInverseCursor { type Target = Puzzle2dMoveNodeInverseState; fn deref(&self) -> &Self::Target { &self.state } }
impl DerefMut for Puzzle2dMoveNodeInverseCursor { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.state } }

impl Default for Puzzle2dMoveNodeInverseCursor {
    fn default() -> Self {
        Self { state: ManuallyDrop::new(Puzzle2dMoveNodeInverseState { source: None, mutation: None, preparation: Default::default(), identifier: PagedUtf8::retained_clone_cursor(), plan: None, pending: None, inverse: PagedList::default(), pending_close: None, inverse_close: None, phase: 0, closing: false }) }
    }
}

impl Puzzle2dMoveNodeInverseCursor {
    pub fn advance(&mut self, source: RetainedCloneRef<'_, Puzzle2dSnapshot>, mutation: RetainedCloneRef<'_, MoveNode>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.closing || self.phase == 8 { return Err(refusal("owned move inverse is closing or spent")); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        source.bind(&mut self.source)?;
        mutation.bind(&mut self.mutation)?;
        match self.phase {
            0 => {
                let step = self.preparation.advance(source, mutation, BoundedOrdGrant { maximum_items: 1, maximum_bytes: grant.maximum_copy_bytes })?;
                let progress = match step {
                    Puzzle2dMoveNodePreparationStep::Pending(progress) => progress,
                    Puzzle2dMoveNodePreparationStep::Complete { plan, progress } => { self.plan = self.preparation.take(); self.preparation.begin_close(); self.phase = 1; if self.plan != Some(plan) { return Err(refusal("move inverse lost its scalar plan")); } progress },
                };
                Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: progress.compared_items, copied_bytes: progress.compared_bytes, retained_capacity_bytes: 0, released_bytes: 0 }))
            }
            1 => {
                let step = self.preparation.close_step(1, grant.maximum_release_bytes)?;
                if step == SnapshotRetirementStep::Complete { self.phase = if self.plan.and_then(|plan| plan.previous_position).is_some() { 2 } else { 6 }; }
                Ok(close_progress(step))
            }
            2 => {
                let step = self.identifier.advance(mutation.project(1, |payload| &payload.id), grant)?;
                if matches!(step, RetainedCloneStep::Complete(_)) { self.phase = 3; }
                Ok(RetainedCloneStep::Progress(step.progress()))
            }
            3 => {
                if grant.maximum_copy_bytes < size_of::<MoveNode>() { return Ok(RetainedCloneStep::Progress(Default::default())); }
                let id = self.identifier.take().ok_or_else(|| refusal("move inverse identifier completed without its owner"))?;
                let [new_x, new_y] = self.plan.and_then(|plan| plan.previous_position).ok_or_else(|| refusal("move inverse has no retained prior position"))?;
                self.pending = Some(MoveNode { id, new_x, new_y });
                self.identifier.begin_close();
                self.phase = 4;
                Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<MoveNode>(), retained_capacity_bytes: 0, released_bytes: 0 }))
            }
            4 => {
                let step = self.identifier.close_granted(grant)?;
                if self.identifier.terminal_is_empty() { self.phase = 5; }
                Ok(RetainedCloneStep::Progress(step.progress()))
            }
            5 => {
                if !self.inverse.has_reserved_slot() {
                    let demand = self.inverse.next_capacity_allocation_bytes(1).map_err(ValueError::from)?.ok_or_else(|| refusal("move inverse lost its page demand"))?;
                    if demand > grant.maximum_capacity_bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
                    let progress = self.inverse.reserve_one(grant.maximum_capacity_bytes).map_err(|error| ValueError::from(error.refusal()))?;
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: usize::from(progress.progressed), copied_bytes: 0, retained_capacity_bytes: progress.allocated_bytes, released_bytes: 0 }));
                }
                if grant.maximum_copy_bytes < size_of::<Puzzle2dMutation>() { return Ok(RetainedCloneStep::Progress(Default::default())); }
                let mutation = Puzzle2dMutation::MoveNode(self.pending.take().ok_or_else(|| refusal("move inverse lost its literal payload"))?);
                if let Err(Puzzle2dMutation::MoveNode(payload)) = self.inverse.push_reserved(mutation) { self.pending = Some(payload); return Err(refusal("move inverse lost its admitted page")); }
                self.phase = 6;
                Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<Puzzle2dMutation>(), retained_capacity_bytes: 0, released_bytes: 0 }))
            }
            6 => {
                if grant.maximum_copy_bytes < size_of::<PagedList<Puzzle2dMutation, {usize::MAX}>>() { return Ok(RetainedCloneStep::Progress(Default::default())); }
                self.phase = 7;
                Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<PagedList<Puzzle2dMutation, {usize::MAX}>>(), retained_capacity_bytes: 0 , released_bytes: 0 }))
            }
            7 => Ok(RetainedCloneStep::Complete(Default::default())),
            _ => Err(refusal("move inverse has an unknown ownership phase")),
        }
    }

    pub fn take(&mut self) -> Option<PagedList<Puzzle2dMutation, {usize::MAX}>> {
        if self.closing || self.phase != 7 { return None; }
        self.phase = 8;
        Some(std::mem::take(&mut self.inverse))
    }

    pub fn begin_close(&mut self) { self.closing = true; self.preparation.begin_close(); self.identifier.begin_close(); }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if !self.closing { return Err(refusal("move inverse closure was not started")); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !self.preparation.terminal_is_empty() { return self.preparation.close_step(1, grant.maximum_release_bytes).map(close_progress); }
        if !self.identifier.terminal_is_empty() { return self.identifier.close_granted(grant).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if let Some(owner) = self.pending_close.as_mut() {
            let step = owner.step(grant)?;
            if owner.terminal_is_empty() { self.pending_close = None; }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.pending.is_some() {
            if grant.maximum_copy_bytes < size_of::<MoveNode>() { return Ok(RetainedCloneStep::Progress(Default::default())); }
            match ControlledRetirement::new(self.pending.take().unwrap()) { Ok(owner) => self.pending_close = Some(owner), Err((error, owner)) => { self.pending = Some(owner); return Err(error); } }
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<MoveNode>(), retained_capacity_bytes: 0, released_bytes: 0 }));
        }
        if let Some(owner) = self.inverse_close.as_mut() {
            let step = owner.step(grant)?;
            if owner.terminal_is_empty() { self.inverse_close = None; }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if !self.inverse.terminal_is_empty() {
            if grant.maximum_copy_bytes < size_of::<PagedList<Puzzle2dMutation, {usize::MAX}>>() { return Ok(RetainedCloneStep::Progress(Default::default())); }
            match ControlledRetirement::new(std::mem::take(&mut self.inverse)) { Ok(owner) => self.inverse_close = Some(owner), Err((error, owner)) => { self.inverse = owner; return Err(error); } }
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<PagedList<Puzzle2dMutation, {usize::MAX}>>(), retained_capacity_bytes: 0 , released_bytes: 0 }));
        }
        if self.plan.take().is_some() { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })); }
        let step = RetainedCloneBinding::close_one(&mut self.source, 1)?;
        if step != SnapshotRetirementStep::Complete { return Ok(close_progress(step)); }
        let step = RetainedCloneBinding::close_one(&mut self.mutation, 1)?;
        Ok(if step == SnapshotRetirementStep::Complete { RetainedCloneStep::Complete(Default::default()) } else { close_progress(step) })
    }

    pub fn terminal_is_empty(&self) -> bool { self.closing && self.preparation.terminal_is_empty() && self.identifier.terminal_is_empty() && self.plan.is_none() && self.pending.is_none() && self.inverse.terminal_is_empty() && self.pending_close.is_none() && self.inverse_close.is_none() && self.source.is_none() && self.mutation.is_none() }
}

impl Drop for Puzzle2dMoveNodeInverseCursor {
    fn drop(&mut self) { let empty = self.terminal_is_empty(); assert!(std::thread::panicking() || empty, "owned move inverse abandoned before controlled closure"); if empty { unsafe { ManuallyDrop::drop(&mut self.state); } } }
}

fn close_progress(step: SnapshotRetirementStep) -> RetainedCloneStep {
    RetainedCloneStep::Progress(match step { SnapshotRetirementStep::Pending { released_items, released_bytes } => RetainedCloneProgress { copied_items: released_items, copied_bytes: 0, retained_capacity_bytes: 0, released_bytes }, _ => Default::default() })
}

fn refusal(message: &str) -> ValueError { ValueError::new(ValueRefusalKind::InvariantViolated, message) }

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
