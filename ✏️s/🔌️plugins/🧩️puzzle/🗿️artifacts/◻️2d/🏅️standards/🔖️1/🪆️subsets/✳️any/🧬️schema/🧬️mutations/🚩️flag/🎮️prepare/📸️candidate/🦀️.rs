//! 📸️ Produces a native candidate with granted snapshot copying and scalar placement.

use crate::standards::v1::subsets::any::schema::mutations::native_preparation_child::{Puzzle2dCloseAxis,close_child_demand,close_demand_methods};
use super::{Puzzle2dFlagIntent,Puzzle2dFlagDisposition,Puzzle2dFlagPlan,Puzzle2dFlagPreparationCursor,Puzzle2dFlagPreparationStep,Puzzle2dSnapshot};
use semio_framework_value::{ValueError, ValueRefusalKind, retirement::controlled::ControlledRetirement, retained_clone::{ RetainedCloneBinding, RetainedCloneCursor, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneStep}};
use std::{mem::{ManuallyDrop, size_of}, ops::{Deref, DerefMut}};

pub struct Puzzle2dFlagCandidate { pub plan: Puzzle2dFlagPlan, pub snapshot: Option<Puzzle2dSnapshot> }

pub struct Puzzle2dFlagCandidateCursor<T:Puzzle2dFlagIntent> { state: ManuallyDrop<Puzzle2dFlagCandidateState<T>> }

#[doc(hidden)]
pub struct Puzzle2dFlagCandidateState<T:Puzzle2dFlagIntent> {
    source: Option<RetainedCloneBinding>,
    mutation: Option<RetainedCloneBinding>,
    preparation: Puzzle2dFlagPreparationCursor<T>,
    snapshot_clone: semio_framework_value::retained_clone::RetainedFieldCursor<Puzzle2dSnapshot>,
    plan: Option<Puzzle2dFlagPlan>,
    candidate: Option<Puzzle2dSnapshot>,
    candidate_close: Option<ControlledRetirement<Puzzle2dSnapshot>>,
    phase: u8,
    closing: bool,
}

impl<T:Puzzle2dFlagIntent> Deref for Puzzle2dFlagCandidateCursor<T> { type Target = Puzzle2dFlagCandidateState<T>; fn deref(&self) -> &Self::Target { &self.state } }
impl<T:Puzzle2dFlagIntent> DerefMut for Puzzle2dFlagCandidateCursor<T> { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.state } }

impl<T:Puzzle2dFlagIntent> Default for Puzzle2dFlagCandidateCursor<T> {
    fn default() -> Self { Self { state: ManuallyDrop::new(Puzzle2dFlagCandidateState { source: None, mutation: None, preparation: Default::default(), snapshot_clone: semio_framework_value::retained_clone::RetainedFieldCursor::<Puzzle2dSnapshot>::default(), plan: None, candidate: None, candidate_close: None, phase: 0, closing: false }) } }
}

impl<T:Puzzle2dFlagIntent> Puzzle2dFlagCandidateCursor<T> {
    pub fn advance(&mut self, source: RetainedCloneRef<'_, Puzzle2dSnapshot>, mutation: RetainedCloneRef<'_, T>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.closing || self.phase == 9 { return Err(refusal("flag candidate is closing or spent")); }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if let Some(progress)=source.bind(&mut self.source,grant)?{return Ok(RetainedCloneStep::Progress(progress))}
        if let Some(progress)=mutation.bind(&mut self.mutation,grant)?{return Ok(RetainedCloneStep::Progress(progress))}
        match self.phase {
            0 => {
                let step = self.preparation.advance(source, mutation, grant)?;
                let progress = match step { Puzzle2dFlagPreparationStep::Pending(progress) => progress, Puzzle2dFlagPreparationStep::Complete { plan, progress } => { self.plan = self.preparation.take(); self.preparation.begin_close(); self.phase = 1; if self.plan != Some(plan) { return Err(refusal("flag candidate lost its scalar plan")); } progress } };
                Ok(RetainedCloneStep::Progress(progress))
            }
            1 => {
                let step = self.preparation.close_step(grant)?;
                if self.preparation.terminal_is_empty() { self.phase = if self.plan.is_some_and(|plan| plan.disposition == Puzzle2dFlagDisposition::Changed) { 2 } else { 7 }; }
                Ok(close_progress(step))
            }
            2 => {
                let step = self.snapshot_clone.advance(source, grant)?;
                if matches!(step, RetainedCloneStep::Complete(_)) { self.phase = 3; }
                Ok(RetainedCloneStep::Progress(step.progress()))
            }
            3 => {
                if grant.maximum_copy_bytes < size_of::<Puzzle2dSnapshot>() { return Ok(RetainedCloneStep::Progress(Default::default())); }
                self.candidate = Some(self.snapshot_clone.take().ok_or_else(|| refusal("flag candidate clone completed without its owner"))?);
                self.snapshot_clone.begin_close();
                self.phase = 4;
                Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<Puzzle2dSnapshot>(), retained_capacity_bytes: 0, released_bytes: 0 }))
            }
            4 => {
                let step = self.snapshot_clone.close_step(grant)?;
                if self.snapshot_clone.terminal_is_empty() { self.phase = 5; }
                Ok(RetainedCloneStep::Progress(step.progress()))
            }
            5 => {
                if grant.maximum_copy_bytes < size_of::<Option<bool>>() { return Ok(RetainedCloneStep::Progress(Default::default())); }
                let index=self.plan.and_then(|plan|plan.index).ok_or_else(||refusal("changed flag candidate has no original location"))?;
                let candidate=self.candidate.as_mut().ok_or_else(||refusal("changed flag candidate lost its native snapshot"))?;
                T::place(candidate,index,mutation.get().next())?;self.phase=7;
                Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Option<bool>>(),retained_capacity_bytes:0,released_bytes:0}))
            }
            7 => {
                if grant.maximum_copy_bytes < size_of::<Puzzle2dFlagCandidate>() { return Ok(RetainedCloneStep::Progress(Default::default())); }
                self.phase = 8;
                Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<Puzzle2dFlagCandidate>(), retained_capacity_bytes: 0, released_bytes: 0 }))
            }
            8 => Ok(RetainedCloneStep::Complete(Default::default())),
            _ => Err(refusal("flag candidate has an unknown ownership phase")),
        }
    }

    pub fn take(&mut self) -> Option<Puzzle2dFlagCandidate> { if self.closing || self.phase != 8 { return None; } self.phase = 9; Some(Puzzle2dFlagCandidate { plan: self.plan.take()?, snapshot: self.candidate.take() }) }

    pub fn begin_close(&mut self) { self.closing = true; self.preparation.begin_close(); self.snapshot_clone.begin_close(); }

    fn close_demand(&self,axis:Puzzle2dCloseAxis)->Result<usize,ValueError>{
        if !self.preparation.terminal_is_empty(){return close_child_demand!(axis,self.preparation)}
        if !self.snapshot_clone.terminal_is_empty(){return axis.retained::<Puzzle2dSnapshot,_>(&self.snapshot_clone)}
        if let Some(owner)=self.candidate_close.as_ref(){return axis.retirement(owner)}
        if self.candidate.is_some(){return axis.inline::<Puzzle2dSnapshot>()}
        if self.plan.is_some(){return axis.inline::<Option<Puzzle2dFlagPlan>>()}
        axis.binding(if self.source.is_some(){&self.source}else{&self.mutation})
    }

    close_demand_methods!(next_close_copy_byte_demand,next_close_capacity_byte_demand,next_close_release_byte_demand,next_close_depth_demand);

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if !self.closing { return Err(refusal("flag candidate closure was not started")); }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !self.preparation.terminal_is_empty() { return self.preparation.close_step(grant).map(close_progress); }
        if !self.snapshot_clone.terminal_is_empty() { return self.snapshot_clone.close_step(grant).map(|step| RetainedCloneStep::Progress(step.progress())); }
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
        if self.plan.is_some() {let bytes=size_of::<Option<Puzzle2dFlagPlan>>();if grant.maximum_copy_bytes<bytes{return Ok(RetainedCloneStep::Progress(Default::default()));}self.plan=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,copied_bytes:bytes,..Default::default()}));}
        let step = RetainedCloneBinding::close_one(if self.source.is_some(){&mut self.source}else{&mut self.mutation}, grant)?;
        Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{close_progress(step)})
    }

    pub fn terminal_is_empty(&self) -> bool { self.closing && self.preparation.terminal_is_empty() && self.snapshot_clone.terminal_is_empty() && self.plan.is_none() && self.candidate.is_none() && self.candidate_close.is_none() && self.source.is_none() && self.mutation.is_none() }
}

impl<T:Puzzle2dFlagIntent> Drop for Puzzle2dFlagCandidateCursor<T> { fn drop(&mut self) { let empty = self.terminal_is_empty(); assert!(std::thread::panicking() || empty, "flag candidate abandoned before controlled closure"); if empty { unsafe { ManuallyDrop::drop(&mut self.state); } } } }

fn close_progress(step: RetainedCloneStep) -> RetainedCloneStep { RetainedCloneStep::Progress(step.progress()) }
fn refusal(message: &str) -> ValueError { ValueError::new(ValueRefusalKind::InvariantViolated, message) }

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
