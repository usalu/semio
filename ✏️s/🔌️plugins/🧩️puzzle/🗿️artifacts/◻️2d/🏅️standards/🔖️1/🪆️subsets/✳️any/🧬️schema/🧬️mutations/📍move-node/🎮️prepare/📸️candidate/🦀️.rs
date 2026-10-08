//! 📸️ Produces a native candidate with granted snapshot copying and scalar placement.

use super::{MoveNode, Puzzle2dMoveNodeDisposition, Puzzle2dMoveNodePlan, Puzzle2dMoveNodePreparationCursor, Puzzle2dMoveNodePreparationStep, Puzzle2dSnapshot};
use semio_framework_value::{SnapshotRetirementStep, ValueError, ValueRefusalKind, retirement::controlled::ControlledRetirement, retained_clone::{RetainedClone, RetainedCloneBinding, RetainedCloneCursor, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneStep, ordered_map::BoundedOrdGrant}};
use std::{mem::{ManuallyDrop, size_of}, ops::{Deref, DerefMut}};

pub struct Puzzle2dMoveNodeCandidate { pub plan: Puzzle2dMoveNodePlan, pub snapshot: Option<Puzzle2dSnapshot> }

pub struct Puzzle2dMoveNodeCandidateCursor { state: ManuallyDrop<Puzzle2dMoveNodeCandidateState> }

#[doc(hidden)]
pub struct Puzzle2dMoveNodeCandidateState {
    source: Option<RetainedCloneBinding>,
    mutation: Option<RetainedCloneBinding>,
    preparation: Puzzle2dMoveNodePreparationCursor,
    snapshot_clone: <Puzzle2dSnapshot as RetainedClone>::Cursor,
    plan: Option<Puzzle2dMoveNodePlan>,
    candidate: Option<Puzzle2dSnapshot>,
    candidate_close: Option<ControlledRetirement<Puzzle2dSnapshot>>,
    phase: u8,
    closing: bool,
}

impl Deref for Puzzle2dMoveNodeCandidateCursor { type Target = Puzzle2dMoveNodeCandidateState; fn deref(&self) -> &Self::Target { &self.state } }
impl DerefMut for Puzzle2dMoveNodeCandidateCursor { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.state } }

impl Default for Puzzle2dMoveNodeCandidateCursor {
    fn default() -> Self { Self { state: ManuallyDrop::new(Puzzle2dMoveNodeCandidateState { source: None, mutation: None, preparation: Default::default(), snapshot_clone: Puzzle2dSnapshot::retained_clone_cursor(), plan: None, candidate: None, candidate_close: None, phase: 0, closing: false }) } }
}

impl Puzzle2dMoveNodeCandidateCursor {
    pub fn advance(&mut self, source: RetainedCloneRef<'_, Puzzle2dSnapshot>, mutation: RetainedCloneRef<'_, MoveNode>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.closing || self.phase == 9 { return Err(refusal("move candidate is closing or spent")); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        source.bind(&mut self.source)?;
        mutation.bind(&mut self.mutation)?;
        match self.phase {
            0 => {
                let step = self.preparation.advance(source, mutation, BoundedOrdGrant { maximum_items: 1, maximum_bytes: grant.maximum_copy_bytes })?;
                let progress = match step { Puzzle2dMoveNodePreparationStep::Pending(progress) => progress, Puzzle2dMoveNodePreparationStep::Complete { plan, progress } => { self.plan = self.preparation.take(); self.preparation.begin_close(); self.phase = 1; if self.plan != Some(plan) { return Err(refusal("move candidate lost its scalar plan")); } progress } };
                Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: progress.compared_items, copied_bytes: progress.compared_bytes, retained_capacity_bytes: 0, released_bytes: 0 }))
            }
            1 => {
                let step = self.preparation.close_step(1, grant.maximum_release_bytes)?;
                if step == SnapshotRetirementStep::Complete { self.phase = if self.plan.is_some_and(|plan| plan.disposition == Puzzle2dMoveNodeDisposition::Changed) { 2 } else { 7 }; }
                Ok(close_progress(step))
            }
            2 => {
                let step = self.snapshot_clone.advance(source, grant)?;
                if matches!(step, RetainedCloneStep::Complete(_)) { self.phase = 3; }
                Ok(RetainedCloneStep::Progress(step.progress()))
            }
            3 => {
                if grant.maximum_copy_bytes < size_of::<Puzzle2dSnapshot>() { return Ok(RetainedCloneStep::Progress(Default::default())); }
                self.candidate = Some(self.snapshot_clone.take().ok_or_else(|| refusal("move candidate clone completed without its owner"))?);
                self.snapshot_clone.begin_close();
                self.phase = 4;
                Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<Puzzle2dSnapshot>(), retained_capacity_bytes: 0, released_bytes: 0 }))
            }
            4 => {
                let step = self.snapshot_clone.close_granted(grant)?;
                if self.snapshot_clone.terminal_is_empty() { self.phase = 5; }
                Ok(RetainedCloneStep::Progress(step.progress()))
            }
            5 | 6 => {
                if grant.maximum_copy_bytes < size_of::<f64>() { return Ok(RetainedCloneStep::Progress(Default::default())); }
                let index = self.plan.and_then(|plan| plan.node_index).ok_or_else(|| refusal("changed candidate has no native location"))?;
                let x_turn = self.phase == 5;
                let node = self.candidate.as_mut().and_then(|candidate| candidate.nodes.get_mut(index)).ok_or_else(|| refusal("move candidate lost its native node"))?;
                if x_turn { node.x = mutation.get().new_x; } else { node.y = mutation.get().new_y; }
                self.phase += 1;
                Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<f64>(), retained_capacity_bytes: 0, released_bytes: 0 }))
            }
            7 => {
                if grant.maximum_copy_bytes < size_of::<Puzzle2dMoveNodeCandidate>() { return Ok(RetainedCloneStep::Progress(Default::default())); }
                self.phase = 8;
                Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<Puzzle2dMoveNodeCandidate>(), retained_capacity_bytes: 0, released_bytes: 0 }))
            }
            8 => Ok(RetainedCloneStep::Complete(Default::default())),
            _ => Err(refusal("move candidate has an unknown ownership phase")),
        }
    }

    pub fn take(&mut self) -> Option<Puzzle2dMoveNodeCandidate> { if self.closing || self.phase != 8 { return None; } self.phase = 9; Some(Puzzle2dMoveNodeCandidate { plan: self.plan.take()?, snapshot: self.candidate.take() }) }

    pub fn begin_close(&mut self) { self.closing = true; self.preparation.begin_close(); self.snapshot_clone.begin_close(); }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if !self.closing { return Err(refusal("move candidate closure was not started")); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !self.preparation.terminal_is_empty() { return self.preparation.close_step(1, grant.maximum_release_bytes).map(close_progress); }
        if !self.snapshot_clone.terminal_is_empty() { return self.snapshot_clone.close_granted(grant).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if let Some(owner) = self.candidate_close.as_mut() {
            let step = owner.step(grant)?;
            if owner.terminal_is_empty() { self.candidate_close = None; }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.candidate.is_some() {
            if grant.maximum_copy_bytes < size_of::<Puzzle2dSnapshot>() { return Ok(RetainedCloneStep::Progress(Default::default())); }
            match ControlledRetirement::new(self.candidate.take().unwrap()) { Ok(owner) => self.candidate_close = Some(owner), Err((error, owner)) => { self.candidate = Some(owner); return Err(error); } }
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<Puzzle2dSnapshot>(), retained_capacity_bytes: 0, released_bytes: 0 }));
        }
        if self.plan.take().is_some() { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })); }
        let step = RetainedCloneBinding::close_one(&mut self.source, 1)?;
        if step != SnapshotRetirementStep::Complete { return Ok(close_progress(step)); }
        let step = RetainedCloneBinding::close_one(&mut self.mutation, 1)?;
        Ok(if step == SnapshotRetirementStep::Complete { RetainedCloneStep::Complete(Default::default()) } else { close_progress(step) })
    }

    pub fn terminal_is_empty(&self) -> bool { self.closing && self.preparation.terminal_is_empty() && self.snapshot_clone.terminal_is_empty() && self.plan.is_none() && self.candidate.is_none() && self.candidate_close.is_none() && self.source.is_none() && self.mutation.is_none() }
}

impl Drop for Puzzle2dMoveNodeCandidateCursor { fn drop(&mut self) { let empty = self.terminal_is_empty(); assert!(std::thread::panicking() || empty, "move candidate abandoned before controlled closure"); if empty { unsafe { ManuallyDrop::drop(&mut self.state); } } } }

fn close_progress(step: SnapshotRetirementStep) -> RetainedCloneStep { RetainedCloneStep::Progress(match step { SnapshotRetirementStep::Pending { released_items, released_bytes } => RetainedCloneProgress { copied_items: released_items, copied_bytes: 0, retained_capacity_bytes: 0, released_bytes }, _ => Default::default() }) }
fn refusal(message: &str) -> ValueError { ValueError::new(ValueRefusalKind::InvariantViolated, message) }

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
