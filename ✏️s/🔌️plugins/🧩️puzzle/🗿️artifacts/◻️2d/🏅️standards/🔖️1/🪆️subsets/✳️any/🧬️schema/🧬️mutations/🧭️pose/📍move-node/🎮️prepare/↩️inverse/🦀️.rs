//! ↩️ Admits inverse identifier ownership directly from the original borrowed move payload.

use crate::standards::v1::subsets::any::schema::preparation::native_preparation_child::{identifier::{IdentifierCopy,Text},list_reserve,pay,settled_step};
use semio_framework_value::{OriginalRetirementPoll, RetainedAllocationEffect, RetirementDemand, RetirementPollExit, ValueError};
use crate::standards::v1::subsets::any::schema::lookup::borrowed::BorrowedIdentity;
use super::{Puzzle2dMoveNodePlan, Puzzle2dMoveNodePreparationCursor, Puzzle2dMoveNodePreparationStep};
use crate::standards::v1::subsets::any::schema::mutations::MoveNode;
use crate::standards::v1::subsets::any::schema::snapshot::Puzzle2dSnapshot;




use crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;
use semio_framework_value::retained_clone::{RetainedCloneClose, RetainedCloneProgress, RetainedCloneRef, RetainedCloneStep};
use semio_framework_value_persistent::list::PagedList;
use std::{mem::{ManuallyDrop, size_of}, ops::{Deref, DerefMut}};

type Inverse = PagedList<Puzzle2dMutation, {usize::MAX}>;

pub struct Puzzle2dMoveNodeInverseCursor { state: ManuallyDrop<Puzzle2dMoveNodeInverseState> }

#[doc(hidden)]
pub struct Puzzle2dMoveNodeInverseState {
    source: BorrowedIdentity,
    mutation: BorrowedIdentity,
    preparation: Puzzle2dMoveNodePreparationCursor,
    identifier: IdentifierCopy,
    identifier_value: Option<Text>,
    plan: Option<Puzzle2dMoveNodePlan>,
    inverse: Option<Inverse>,
    retirement: RetainedCloneClose,
    pub phase: u8,
    closing: bool,
}

impl Deref for Puzzle2dMoveNodeInverseCursor { type Target = Puzzle2dMoveNodeInverseState; fn deref(&self) -> &Self::Target { &self.state } }
impl DerefMut for Puzzle2dMoveNodeInverseCursor { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.state } }

impl Default for Puzzle2dMoveNodeInverseCursor {
    fn default() -> Self {
        Self { state: ManuallyDrop::new(Puzzle2dMoveNodeInverseState { source: Default::default(), mutation: Default::default(), preparation: Default::default(), identifier: Default::default(), identifier_value: None, plan: None, inverse: Some(PagedList::with_page_items(1)), retirement: Default::default(), phase: 0, closing: false }) }
    }
}

impl Puzzle2dMoveNodeInverseCursor {
    pub fn advance(&mut self, source: RetainedCloneRef<'_, Puzzle2dSnapshot>, mutation: RetainedCloneRef<'_, MoveNode>, body:usize,effect:&mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneStep, ValueError> {
        let grant=effect.grant();
        if self.closing || self.phase == 8 { return Err(refusal("owned move inverse is closing or spent")); }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if let Some(progress)=self.source.bind(source,effect)?{return Ok(RetainedCloneStep::Progress(progress))}
        if let Some(progress)=self.mutation.bind(mutation,effect)?{return Ok(RetainedCloneStep::Progress(progress))}
        match self.phase {
            0 => {
                let step = self.preparation.advance(source, mutation, effect)?;
                let progress = match step {
                    Puzzle2dMoveNodePreparationStep::Pending(progress) => progress,
                    Puzzle2dMoveNodePreparationStep::Complete { plan, progress } => { self.plan = self.preparation.take(); self.preparation.begin_close(); self.phase = 1; if self.plan != Some(plan) { return Err(refusal("move inverse lost its scalar plan")); } progress },
                };
                Ok(RetainedCloneStep::Progress(progress))
            }
            1 => {
                let step = settled_step(self.preparation.close_step(effect))?;
                if self.preparation.terminal_is_empty() { self.phase = if self.plan.and_then(|plan| plan.previous_position).is_some() { 2 } else { 6 }; }
                Ok(close_progress(step))
            }
            2 => {
                let step = self.identifier.advance(&mutation.get().id, body, effect)?;
                if matches!(step, RetainedCloneStep::Complete(_)) { self.phase = 3; }
                Ok(close_progress(step))
            }
            3 => {
                let state = &mut *self.state;
                let step = state.identifier.take(&mut state.identifier_value, effect)?;
                if state.identifier_value.is_some() { state.phase = 4; }
                Ok(close_progress(step))
            }
            4 => {
                let state = &mut *self.state;
                let step = state.identifier.close(&mut state.identifier_value, effect)?;
                if state.identifier.terminal_is_empty() { state.phase = 5; }
                Ok(close_progress(step))
            }
            5 => {
                let state = &mut *self.state;
                let inverse = state.inverse.as_mut().ok_or_else(|| refusal("move inverse output page is absent"))?;
                if !inverse.has_reserved_slot() {
                    let progress = list_reserve(inverse, effect)?.unwrap_or_default();
                    return Ok(RetainedCloneStep::Progress(progress));
                }
                let bytes = size_of::<Puzzle2dMutation>() + size_of::<Option<Text>>();
                if grant.maximum_copy_bytes < bytes || !pay(effect)? { return Ok(RetainedCloneStep::Progress(Default::default())); }
                let [new_x, new_y] = state.plan.and_then(|plan| plan.previous_position).ok_or_else(|| refusal("move inverse has no retained prior position"))?;
                let id = state.identifier_value.take().ok_or_else(|| refusal("move inverse lost its literal identifier"))?;
                if let Err(Puzzle2dMutation::MoveNode(payload)) = inverse.push_reserved(Puzzle2dMutation::MoveNode(MoveNode { id, new_x, new_y })) {
                    state.identifier_value = Some(payload.id);
                    return Err(refusal("move inverse lost its admitted page"));
                }
                state.phase = 6;
                Ok(RetainedCloneStep::Progress(paid(bytes)))
            }
            6 => {
                if grant.maximum_copy_bytes < size_of::<PagedList<Puzzle2dMutation, {usize::MAX}>>()||!pay(effect)? { return Ok(RetainedCloneStep::Progress(Default::default())); }
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
        self.inverse.take()
    }

    pub fn begin_close(&mut self) { self.closing = true; self.preparation.begin_close();  }

    #[cfg(feature = "testkit")]
    #[doc(hidden)]
    pub fn advance_test_demand(&self, mutation: &MoveNode, body: usize) -> Result<Option<RetirementDemand>, ValueError> {
        if self.phase == 2 { return self.identifier.advance_demand(&mutation.id, body).map(Some); }
        if self.phase == 4 { return self.identifier.close_demand(body).map(Some); }
        if let Some(inverse) = self.inverse.as_ref().filter(|inverse| self.phase == 5 && !inverse.has_reserved_slot()) {
            return Ok(Some(RetirementDemand {
                copy_bytes: inverse.next_reserve_copy_byte_demand()?,
                capacity_bytes: inverse.next_allocation_bytes()?,
                depth: inverse.next_reserve_depth_demand()?,
                ..Default::default()
            }));
        }
        Ok(None)
    }

    #[cfg(feature = "testkit")]
    #[doc(hidden)]
    pub fn current_identifier_chunk_bytes(&self) -> usize { self.identifier.current_chunk_body_bytes() }

    pub fn close_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if !self.preparation.terminal_is_empty() {
            return Ok(RetirementDemand {
                copy_bytes: self.preparation.next_close_copy_byte_demand()?,
                capacity_bytes: self.preparation.next_close_capacity_byte_demand(body)?,
                release_bytes: self.preparation.next_close_release_byte_demand()?,
                depth: self.preparation.next_close_depth_demand()?,
            });
        }
        if !self.identifier.terminal_is_empty() { return self.identifier.close_demand(body); }
        if !self.retirement.is_empty() { return self.retirement.demands(body); }
        if self.identifier_value.is_some() { return self.retirement.next_owner_demands::<Text>(true, &None, body); }
        if self.inverse.as_ref().is_some_and(|inverse| !inverse.terminal_is_empty()) {
            return self.retirement.next_owner_demands::<Inverse>(true, &None, body);
        }
        if self.plan.is_some() {
            return Ok(RetirementDemand { copy_bytes: size_of::<Option<Puzzle2dMoveNodePlan>>(), depth: 1, ..Default::default() });
        }
        Ok(if self.source.is_empty() { self.mutation.close_demand() } else { self.source.close_demand() })
    }

    pub fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demand(0)?.copy_bytes)}
    pub fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.close_demand(body)?.capacity_bytes)}
    pub fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demand(0)?.release_bytes)}
    pub fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(self.close_demand(0)?.depth)}

    pub fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{let result=self.close_stepped(effect);RetirementPollExit::settle(effect,result)}
    fn close_stepped(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,RetirementPollExit>{
        let grant = effect.grant();
        if !self.closing { return Err(refusal("move inverse closure was not started").into()); }
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !self.preparation.terminal_is_empty() { return RetirementPollExit::resume(self.preparation.close_step(effect)).map(close_progress); }
        let state = &mut *self.state;
        if !state.identifier.terminal_is_empty() { return Ok(close_progress(state.identifier.close(&mut state.identifier_value, effect)?)); }
        if !state.retirement.is_empty() { return RetirementPollExit::resume(state.retirement.step_granted(grant.maximum_copy_bytes, effect)).map(close_progress); }
        let birth = if state.identifier_value.is_some() {
            Some(state.retirement.begin_granted(&mut state.identifier_value, effect))
        } else if state.inverse.as_ref().is_some_and(|inverse| !inverse.terminal_is_empty()) {
            Some(state.retirement.begin_granted(&mut state.inverse, effect))
        } else {
            None
        };
        if let Some(turn) = birth {
            return Ok(RetainedCloneStep::Progress(turn.result?.map(|step| step.progress()).unwrap_or_default()));
        }
        if state.plan.is_some() {
            let bytes = size_of::<Option<Puzzle2dMoveNodePlan>>();
            if grant.maximum_copy_bytes < bytes || !pay(effect)? { return Ok(RetainedCloneStep::Progress(Default::default())); }
            state.plan = None;
            return Ok(RetainedCloneStep::Progress(paid(bytes)));
        }
        let progress = if state.source.is_empty() { state.mutation.close(effect)? } else { state.source.close(effect)? };
        Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(progress) } else { RetainedCloneStep::Progress(progress) })
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.closing && self.preparation.terminal_is_empty() && self.identifier.terminal_is_empty()
            && self.identifier_value.is_none() && self.plan.is_none() && self.inverse.as_ref().is_none_or(PagedList::terminal_is_empty)
            && self.retirement.is_empty() && self.source.is_empty() && self.mutation.is_empty()
    }
}

impl Drop for Puzzle2dMoveNodeInverseCursor {
    fn drop(&mut self) { let empty = self.terminal_is_empty(); assert!(std::thread::panicking() || empty, "owned move inverse abandoned before controlled closure"); if empty { unsafe { ManuallyDrop::drop(&mut self.state); } } }
}

fn close_progress(step: RetainedCloneStep) -> RetainedCloneStep { RetainedCloneStep::Progress(step.progress()) }

fn paid(bytes: usize) -> RetainedCloneProgress { RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() } }

fn refusal(message: &'static str) -> ValueError { ValueError::invariant(message) }


