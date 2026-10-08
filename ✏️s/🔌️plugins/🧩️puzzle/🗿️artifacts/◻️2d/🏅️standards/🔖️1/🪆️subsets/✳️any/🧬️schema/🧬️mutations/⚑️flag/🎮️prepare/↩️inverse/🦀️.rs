//! ⚑️ Grants actual native prior-flag inverse ownership without copying the original payload.

use crate::standards::v1::subsets::any::schema::mutations::native_preparation_child::{Puzzle2dCloseAxis,close_child_demand,close_demand_methods};
use super::{Puzzle2dFlagIntent,Puzzle2dFlagPlan,Puzzle2dFlagPreparationCursor,Puzzle2dFlagPreparationStep,Puzzle2dSnapshot};
use crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;
use semio_framework_value::{ValueError, ValueRefusalKind, list::PagedList, paged::PagedUtf8, retirement::controlled::ControlledRetirement, retained_clone::{RetainedClone, RetainedCloneBinding, RetainedCloneCursor, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneStep}};
use std::{mem::{ManuallyDrop, size_of}, ops::{Deref, DerefMut}};

pub struct Puzzle2dFlagInverseCursor<T:Puzzle2dFlagIntent> { state: ManuallyDrop<Puzzle2dFlagInverseState<T>> }

#[doc(hidden)]
pub struct Puzzle2dFlagInverseState<T:Puzzle2dFlagIntent> {
    source: Option<RetainedCloneBinding>,
    mutation: Option<RetainedCloneBinding>,
    preparation: Puzzle2dFlagPreparationCursor<T>,
    identifier: <PagedUtf8<{usize::MAX}> as RetainedClone>::Cursor,
    plan: Option<Puzzle2dFlagPlan>,
    pending: Option<Puzzle2dMutation>,
    inverse: PagedList<Puzzle2dMutation, {usize::MAX}>,
    pending_close: Option<ControlledRetirement<Puzzle2dMutation>>,
    inverse_close: Option<ControlledRetirement<PagedList<Puzzle2dMutation, {usize::MAX}>>>,
    phase: u8,
    closing: bool,
}

impl<T:Puzzle2dFlagIntent> Deref for Puzzle2dFlagInverseCursor<T> { type Target = Puzzle2dFlagInverseState<T>; fn deref(&self) -> &Self::Target { &self.state } }
impl<T:Puzzle2dFlagIntent> DerefMut for Puzzle2dFlagInverseCursor<T> { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.state } }

impl<T:Puzzle2dFlagIntent> Default for Puzzle2dFlagInverseCursor<T> {
    fn default() -> Self {
        Self { state: ManuallyDrop::new(Puzzle2dFlagInverseState { source: None, mutation: None, preparation: Default::default(), identifier: PagedUtf8::retained_clone_cursor(), plan: None, pending: None, inverse: PagedList::default(), pending_close: None, inverse_close: None, phase: 0, closing: false }) }
    }
}

impl<T:Puzzle2dFlagIntent> Puzzle2dFlagInverseCursor<T> {
    pub fn advance(&mut self, source: RetainedCloneRef<'_, Puzzle2dSnapshot>, mutation: RetainedCloneRef<'_, T>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.closing || self.phase == 8 { return Err(refusal("owned flag inverse is closing or spent")); }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        source.bind(&mut self.source)?;
        mutation.bind(&mut self.mutation)?;
        match self.phase {
            0 => {
                let step = self.preparation.advance(source, mutation, grant)?;
                let progress = match step {
                    Puzzle2dFlagPreparationStep::Pending(progress) => progress,
                    Puzzle2dFlagPreparationStep::Complete { plan, progress } => { self.plan = self.preparation.take(); self.preparation.begin_close(); self.phase = 1; if self.plan != Some(plan) { return Err(refusal("flag inverse lost its scalar plan")); } progress },
                };
                Ok(RetainedCloneStep::Progress(progress))
            }
            1 => {
                let step = self.preparation.close_step(grant)?;
                if self.preparation.terminal_is_empty() { self.phase = if self.plan.and_then(|plan| plan.previous).is_some() { 2 } else { 6 }; }
                Ok(close_progress(step))
            }
            2 => {
                let index=self.plan.and_then(|plan|plan.index).ok_or_else(||refusal("flag inverse lost its found ordinal"))?;
                let step = self.identifier.advance(source.project(1, |snapshot| T::identifier_at(snapshot,index)), grant)?;
                if matches!(step, RetainedCloneStep::Complete(_)) { self.phase = 3; }
                Ok(RetainedCloneStep::Progress(step.progress()))
            }
            3 => {
                if grant.maximum_copy_bytes < size_of::<Puzzle2dMutation>() { return Ok(RetainedCloneStep::Progress(Default::default())); }
                let id = self.identifier.take().ok_or_else(|| refusal("flag inverse identifier completed without its owner"))?;
                let previous = self.plan.and_then(|plan| plan.previous).ok_or_else(|| refusal("flag inverse has no original prior optional state"))?;
                self.pending = Some(T::restore(id,previous));
                self.identifier.begin_close();
                self.phase = 4;
                Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<Puzzle2dMutation>(), retained_capacity_bytes: 0, released_bytes: 0 }))
            }
            4 => {
                let step = self.identifier.close_step(grant)?;
                if self.identifier.terminal_is_empty() { self.phase = 5; }
                Ok(RetainedCloneStep::Progress(step.progress()))
            }
            5 => {
                if !self.inverse.has_reserved_slot() {
                    let demand = self.inverse.next_capacity_allocation_bytes(1).map_err(ValueError::from)?.ok_or_else(|| refusal("flag inverse lost its page demand"))?;
                    if demand > grant.maximum_capacity_bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
                    let progress = self.inverse.reserve_one(grant.maximum_capacity_bytes).map_err(|error| ValueError::from(error.refusal()))?;
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: usize::from(progress.progressed), copied_bytes: 0, retained_capacity_bytes: progress.allocated_bytes, released_bytes: 0 }));
                }
                if grant.maximum_copy_bytes < size_of::<Puzzle2dMutation>() { return Ok(RetainedCloneStep::Progress(Default::default())); }
                let mutation = self.pending.take().ok_or_else(|| refusal("flag inverse lost its original prior payload"))?;
                if let Err(payload) = self.inverse.push_reserved(mutation) { self.pending = Some(payload); return Err(refusal("flag inverse lost its admitted page")); }
                self.phase = 6;
                Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<Puzzle2dMutation>(), retained_capacity_bytes: 0, released_bytes: 0 }))
            }
            6 => {
                if grant.maximum_copy_bytes < size_of::<PagedList<Puzzle2dMutation, {usize::MAX}>>() { return Ok(RetainedCloneStep::Progress(Default::default())); }
                self.phase = 7;
                Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<PagedList<Puzzle2dMutation, {usize::MAX}>>(), retained_capacity_bytes: 0 , released_bytes: 0 }))
            }
            7 => Ok(RetainedCloneStep::Complete(Default::default())),
            _ => Err(refusal("flag inverse has an unknown ownership phase")),
        }
    }

    pub fn take(&mut self) -> Option<PagedList<Puzzle2dMutation, {usize::MAX}>> {
        if self.closing || self.phase != 7 { return None; }
        self.phase = 8;
        Some(std::mem::take(&mut self.inverse))
    }

    pub fn begin_close(&mut self) { self.closing = true; self.preparation.begin_close(); self.identifier.begin_close(); }

    fn close_demand(&self,axis:Puzzle2dCloseAxis)->Result<usize,ValueError>{
        if !self.preparation.terminal_is_empty(){return close_child_demand!(axis,self.preparation)}
        if !self.identifier.terminal_is_empty(){return axis.retained::<PagedUtf8<{usize::MAX}>,_>(&self.identifier)}
        if let Some(owner)=self.pending_close.as_ref(){return axis.retirement(owner)}
        if self.pending.is_some(){return axis.inline::<Puzzle2dMutation>()}
        if let Some(owner)=self.inverse_close.as_ref(){return axis.retirement(owner)}
        if !self.inverse.terminal_is_empty(){return axis.inline::<PagedList<Puzzle2dMutation,{usize::MAX}>>()}
        if self.plan.is_some(){return axis.inline::<Option<Puzzle2dFlagPlan>>()}
        axis.binding(if self.source.is_some(){&self.source}else{&self.mutation})
    }

    close_demand_methods!(next_close_copy_byte_demand,next_close_capacity_byte_demand,next_close_release_byte_demand,next_close_depth_demand);

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if !self.closing { return Err(refusal("flag inverse closure was not started")); }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !self.preparation.terminal_is_empty() { return self.preparation.close_step(grant).map(close_progress); }
        if !self.identifier.terminal_is_empty() { return self.identifier.close_step(grant).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if let Some(owner) = self.pending_close.as_mut() {
            let step = owner.step(grant)?;
            if owner.terminal_is_empty() { self.pending_close = None; }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.pending.is_some() {
            if grant.maximum_copy_bytes < size_of::<Puzzle2dMutation>() { return Ok(RetainedCloneStep::Progress(Default::default())); }
            match ControlledRetirement::new(self.pending.take().unwrap()) { Ok(owner) => self.pending_close = Some(owner), Err((error, owner)) => { self.pending = Some(owner); return Err(error); } }
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<Puzzle2dMutation>(), retained_capacity_bytes: 0, released_bytes: 0 }));
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
        if self.plan.is_some() {let bytes=size_of::<Option<Puzzle2dFlagPlan>>();if grant.maximum_copy_bytes<bytes{return Ok(RetainedCloneStep::Progress(Default::default()));}self.plan=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,copied_bytes:bytes,..Default::default()}));}
        let step = RetainedCloneBinding::close_one(if self.source.is_some(){&mut self.source}else{&mut self.mutation}, grant)?;
        Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{close_progress(step)})
    }

    pub fn terminal_is_empty(&self) -> bool { self.closing && self.preparation.terminal_is_empty() && self.identifier.terminal_is_empty() && self.plan.is_none() && self.pending.is_none() && self.inverse.terminal_is_empty() && self.pending_close.is_none() && self.inverse_close.is_none() && self.source.is_none() && self.mutation.is_none() }
}

impl<T:Puzzle2dFlagIntent> Drop for Puzzle2dFlagInverseCursor<T> {
    fn drop(&mut self) { let empty = self.terminal_is_empty(); assert!(std::thread::panicking() || empty, "owned flag inverse abandoned before controlled closure"); if empty { unsafe { ManuallyDrop::drop(&mut self.state); } } }
}

fn close_progress(step: RetainedCloneStep) -> RetainedCloneStep { RetainedCloneStep::Progress(step.progress()) }

fn refusal(message: &str) -> ValueError { ValueError::new(ValueRefusalKind::InvariantViolated, message) }

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
