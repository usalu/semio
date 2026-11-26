//! 📸️ Produces a native candidate with granted snapshot copying and scalar placement.

use crate::standards::v1::subsets::any::schema::lookup::borrowed::BorrowedIdentity;
use crate::standards::v1::subsets::any::schema::preparation::native_preparation_child::{cursor_advance, cursor_retire, cursor_take, pay, settled_step};
use semio_framework_value::{OriginalRetirementPoll, RetainedAllocationEffect, RetirementDemand, RetirementPollExit};
use crate::standards::v1::subsets::any::schema::preparation::native_preparation_child::Puzzle2dCloseAxis;
use super::{Puzzle2dMoveNodeDisposition, Puzzle2dMoveNodePlan, Puzzle2dMoveNodePreparationCursor, Puzzle2dMoveNodePreparationStep};
use crate::standards::v1::subsets::any::schema::mutations::MoveNode;
use crate::standards::v1::subsets::any::schema::snapshot::Puzzle2dSnapshot;
use semio_framework_value::{ValueError, retained_clone::{RetainedCloneClose, RetainedCloneCursor, RetainedCloneProgress, RetainedCloneRef, RetainedCloneStep, RetainedFieldCursor}};
use std::{mem::{ManuallyDrop, size_of}, ops::{Deref, DerefMut}};

pub struct Puzzle2dMoveNodeCandidate { pub plan: Puzzle2dMoveNodePlan, pub snapshot: Option<Puzzle2dSnapshot> }

pub struct Puzzle2dMoveNodeCandidateCursor { state: ManuallyDrop<Puzzle2dMoveNodeCandidateState> }

#[doc(hidden)]
pub struct Puzzle2dMoveNodeCandidateState {
    source: BorrowedIdentity,
    mutation: BorrowedIdentity,
    preparation: Puzzle2dMoveNodePreparationCursor,
    snapshot_clone: RetainedFieldCursor<Puzzle2dSnapshot>,
    plan: Option<Puzzle2dMoveNodePlan>,
    candidate: Option<Puzzle2dSnapshot>,
    candidate_close: RetainedCloneClose,
    pub phase: u8,
    closing: bool,
}

impl Deref for Puzzle2dMoveNodeCandidateCursor { type Target = Puzzle2dMoveNodeCandidateState; fn deref(&self) -> &Self::Target { &self.state } }
impl DerefMut for Puzzle2dMoveNodeCandidateCursor { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.state } }

impl Default for Puzzle2dMoveNodeCandidateCursor {
    fn default() -> Self { Self { state: ManuallyDrop::new(Puzzle2dMoveNodeCandidateState { source: Default::default(), mutation: Default::default(), preparation: Default::default(), snapshot_clone: Default::default(), plan: None, candidate: None, candidate_close: Default::default(), phase: 0, closing: false }) } }
}

impl Puzzle2dMoveNodeCandidateCursor {
    pub fn advance(&mut self, source: RetainedCloneRef<'_, Puzzle2dSnapshot>, mutation: RetainedCloneRef<'_, MoveNode>, body:usize,effect:&mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneStep, ValueError> {
        let grant=effect.grant();
        if self.closing || self.phase == 9 { return Err(refusal("move candidate is closing or spent")); }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if let Some(progress)=self.source.bind(source,effect)?{return Ok(RetainedCloneStep::Progress(progress))}
        if let Some(progress)=self.mutation.bind(mutation,effect)?{return Ok(RetainedCloneStep::Progress(progress))}
        match self.phase {
            0 => {
                let step = self.preparation.advance(source, mutation, effect)?;
                let progress = match step { Puzzle2dMoveNodePreparationStep::Pending(progress) => progress, Puzzle2dMoveNodePreparationStep::Complete { plan, progress } => { self.plan = self.preparation.take(); self.preparation.begin_close(); self.phase = 1; if self.plan != Some(plan) { return Err(refusal("move candidate lost its scalar plan")); } progress } };
                Ok(RetainedCloneStep::Progress(progress))
            }
            1 => {
                let step = settled_step(self.preparation.close_step(effect))?;
                if self.preparation.terminal_is_empty() { self.phase = if self.plan.is_some_and(|plan| plan.disposition == Puzzle2dMoveNodeDisposition::Changed) { 2 } else { 7 }; }
                Ok(close_progress(step))
            }
            2 => {
                let step = cursor_advance(&mut self.snapshot_clone,source,body,effect)?;
                if matches!(step, RetainedCloneStep::Complete(_)) { self.phase = 3; }
                Ok(RetainedCloneStep::Progress(step.progress()))
            }
            3 => {
                let state=&mut *self.state;
                let step=cursor_take(&mut state.snapshot_clone,&mut state.candidate,effect)?;
                if state.candidate.is_some(){state.phase=4;}
                Ok(close_progress(step))
            }
            4 => {
                let step=cursor_retire(&mut self.snapshot_clone,effect)?;
                if self.snapshot_clone.terminal_is_empty() { self.phase = 5; }
                Ok(RetainedCloneStep::Progress(step.progress()))
            }
            5 | 6 => {
                if grant.maximum_copy_bytes < size_of::<f64>()||!pay(effect)? { return Ok(RetainedCloneStep::Progress(Default::default())); }
                let index = self.plan.and_then(|plan| plan.node_index).ok_or_else(|| refusal("changed candidate has no native location"))?;
                let x_turn = self.phase == 5;
                let node = self.candidate.as_mut().and_then(|candidate| candidate.nodes.get_mut(index)).ok_or_else(|| refusal("move candidate lost its native node"))?;
                if x_turn { node.x = mutation.get().new_x; } else { node.y = mutation.get().new_y; }
                self.phase += 1;
                Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<f64>(), retained_capacity_bytes: 0, released_bytes: 0 }))
            }
            7 => {
                if grant.maximum_copy_bytes < size_of::<Puzzle2dMoveNodeCandidate>()||!pay(effect)? { return Ok(RetainedCloneStep::Progress(Default::default())); }
                self.phase = 8;
                Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<Puzzle2dMoveNodeCandidate>(), retained_capacity_bytes: 0, released_bytes: 0 }))
            }
            8 => Ok(RetainedCloneStep::Complete(Default::default())),
            _ => Err(refusal("move candidate has an unknown ownership phase")),
        }
    }

    pub fn take(&mut self) -> Option<Puzzle2dMoveNodeCandidate> { if self.closing || self.phase != 8 { return None; } self.phase = 9; Some(Puzzle2dMoveNodeCandidate { plan: self.plan.take()?, snapshot: self.candidate.take() }) }

    pub fn begin_close(&mut self) { self.closing = true; self.preparation.begin_close();  }

    #[cfg(feature = "testkit")]
    #[doc(hidden)]
    pub fn advance_test_demand(&self,source:RetainedCloneRef<'_,Puzzle2dSnapshot>,body:usize)->Result<Option<RetirementDemand>,ValueError>{
        Ok(match self.phase {
            2=>Some(self.snapshot_clone.advance_demands(source,body,u64::MAX)?),
            3=>Some(self.snapshot_clone.take_demand()?),
            4=>Some(crate::standards::v1::subsets::any::schema::preparation::native_preparation_child::cursor_retire_demand::<Puzzle2dSnapshot,_>(&self.snapshot_clone,body)?),
            _=>None,
        })
    }

    fn close_demand(&self,axis:Puzzle2dCloseAxis)->Result<usize,ValueError>{
        if !self.preparation.terminal_is_empty(){return match axis {Puzzle2dCloseAxis::Copy=>self.preparation.next_close_copy_byte_demand(),Puzzle2dCloseAxis::Capacity(body)=>self.preparation.next_close_capacity_byte_demand(body),Puzzle2dCloseAxis::Release=>self.preparation.next_close_release_byte_demand(),Puzzle2dCloseAxis::Depth=>self.preparation.next_close_depth_demand()}}
        if !self.snapshot_clone.terminal_is_empty(){return axis.retained::<Puzzle2dSnapshot,_>(&self.snapshot_clone)}
        if self.candidate.is_some()||!self.candidate_close.is_empty(){
            return axis.owned::<Puzzle2dSnapshot>(&self.candidate_close,self.candidate.is_some());
        }
        if self.plan.is_some(){return axis.inline::<Option<Puzzle2dMoveNodePlan>>()}
        Ok(axis.select(if self.source.is_empty(){self.mutation.close_demand()}else{self.source.close_demand()}))
    }

    pub fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{self.close_demand(Puzzle2dCloseAxis::Copy)}
    pub fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{self.close_demand(Puzzle2dCloseAxis::Capacity(body))}
    pub fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{self.close_demand(Puzzle2dCloseAxis::Release)}
    pub fn next_close_depth_demand(&self)->Result<usize,ValueError>{self.close_demand(Puzzle2dCloseAxis::Depth)}

    pub fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{let result=self.close_stepped(effect);RetirementPollExit::settle(effect,result)}
    fn close_stepped(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,RetirementPollExit>{
        let grant=effect.grant();
        if !self.closing { return Err(refusal("move candidate closure was not started").into()); }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !self.preparation.terminal_is_empty() { return RetirementPollExit::resume(self.preparation.close_step(effect)).map(close_progress); }
        if !self.snapshot_clone.terminal_is_empty() { return Ok(RetainedCloneStep::Progress(cursor_retire(&mut self.snapshot_clone,effect)?.progress())); }
        if !self.candidate_close.is_empty() {
            return RetirementPollExit::resume(self.candidate_close.step_granted(grant.maximum_copy_bytes,effect)).map(close_progress);
        }
        if self.candidate.is_some() {
            let state=&mut *self.state;
            let step=state.candidate_close.begin_granted(&mut state.candidate,effect).result?;
            return Ok(RetainedCloneStep::Progress(step.map(|step|step.progress()).unwrap_or_default()));
        }
        if self.plan.is_some() {let bytes=size_of::<Option<Puzzle2dMoveNodePlan>>();if grant.maximum_copy_bytes<bytes||!pay(effect)?{return Ok(RetainedCloneStep::Progress(Default::default()));}self.plan=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,copied_bytes:bytes,..Default::default()}));}
        let progress=if self.source.is_empty(){self.mutation.close(effect)?}else{self.source.close(effect)?};
        Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
    }

    pub fn terminal_is_empty(&self) -> bool { self.closing && self.preparation.terminal_is_empty() && self.snapshot_clone.terminal_is_empty() && self.plan.is_none() && self.candidate.is_none() && self.candidate_close.is_empty() && self.source.is_empty() && self.mutation.is_empty() }
}

impl Drop for Puzzle2dMoveNodeCandidateCursor { fn drop(&mut self) { let empty = self.terminal_is_empty(); assert!(std::thread::panicking() || empty, "move candidate abandoned before controlled closure"); if empty { unsafe { ManuallyDrop::drop(&mut self.state); } } } }

fn close_progress(step: RetainedCloneStep) -> RetainedCloneStep { RetainedCloneStep::Progress(step.progress()) }
fn refusal(message: &'static str) -> ValueError { ValueError::invariant(message) }


