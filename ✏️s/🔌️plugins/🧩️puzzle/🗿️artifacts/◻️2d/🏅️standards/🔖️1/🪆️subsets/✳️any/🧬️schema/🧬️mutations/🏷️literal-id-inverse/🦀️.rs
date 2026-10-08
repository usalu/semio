//! 🏷️ Typed literal inverses retain only their original native identifier ownership.

use super::Puzzle2dMutation;
use semio_framework_value::{ValueError, ValueRefusalKind, list::PagedList, paged::PagedUtf8, retirement::controlled::ControlledRetirement, retained_clone::{RetainedClone, RetainedCloneBinding, RetainedCloneCursor, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneStep}};
use std::{mem::{ManuallyDrop, size_of}, ops::{Deref, DerefMut}};

pub trait Puzzle2dLiteralIdInverse: Send + Sync { fn inverse_id(&self) -> &PagedUtf8<{usize::MAX}>; fn inverse_payload(id: PagedUtf8<{usize::MAX}>) -> Puzzle2dMutation; }

pub struct Puzzle2dLiteralIdInverseCursor<M: Puzzle2dLiteralIdInverse> { state: ManuallyDrop<Puzzle2dLiteralIdInverseState>, marker: std::marker::PhantomData<M> }

#[doc(hidden)]
pub struct Puzzle2dLiteralIdInverseState {
    mutation: Option<RetainedCloneBinding>,
    identifier: <PagedUtf8<{usize::MAX}> as RetainedClone>::Cursor,
    pending: Option<Puzzle2dMutation>,
    inverse: PagedList<Puzzle2dMutation, {usize::MAX}>,
    pending_close: Option<ControlledRetirement<Puzzle2dMutation>>,
    inverse_close: Option<ControlledRetirement<PagedList<Puzzle2dMutation, {usize::MAX}>>>,
    phase: u8,
    closing: bool,
}

impl<M: Puzzle2dLiteralIdInverse> Deref for Puzzle2dLiteralIdInverseCursor<M> { type Target = Puzzle2dLiteralIdInverseState; fn deref(&self) -> &Self::Target { &self.state } }
impl<M: Puzzle2dLiteralIdInverse> DerefMut for Puzzle2dLiteralIdInverseCursor<M> { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.state } }

impl<M: Puzzle2dLiteralIdInverse> Default for Puzzle2dLiteralIdInverseCursor<M> {
    fn default() -> Self { Self { state: ManuallyDrop::new(Puzzle2dLiteralIdInverseState { mutation: None, identifier: PagedUtf8::retained_clone_cursor(), pending: None, inverse: PagedList::default(), pending_close: None, inverse_close: None, phase: 0, closing: false }), marker: std::marker::PhantomData } }
}

impl<M: Puzzle2dLiteralIdInverse> Puzzle2dLiteralIdInverseCursor<M> {
    pub fn advance(&mut self, mutation: RetainedCloneRef<'_, M>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.closing || self.phase == 6 { return Err(refusal("literal inverse is closing or spent")); }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        mutation.bind(&mut self.mutation)?;
        match self.phase {
            0 => {
                let step = self.identifier.advance(mutation.project(1, M::inverse_id), grant)?;
                if matches!(step, RetainedCloneStep::Complete(_)) { self.phase = 1; }
                Ok(RetainedCloneStep::Progress(step.progress()))
            }
            1 => {
                if grant.maximum_copy_bytes < size_of::<Puzzle2dMutation>() { return Ok(RetainedCloneStep::Progress(Default::default())); }
                let id = self.identifier.take().ok_or_else(|| refusal("literal inverse completed without identifier ownership"))?;
                self.pending = Some(M::inverse_payload(id));
                self.identifier.begin_close();
                self.phase = 2;
                Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<Puzzle2dMutation>(), retained_capacity_bytes: 0, released_bytes: 0 }))
            }
            2 => {
                let step = self.identifier.close_step(grant)?;
                if self.identifier.terminal_is_empty() { self.phase = 3; }
                Ok(RetainedCloneStep::Progress(step.progress()))
            }
            3 => {
                if !self.inverse.has_reserved_slot() {
                    let demand = self.inverse.next_capacity_allocation_bytes(1).map_err(ValueError::from)?.ok_or_else(|| refusal("literal inverse lost its page demand"))?;
                    if demand > grant.maximum_capacity_bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
                    let progress = self.inverse.reserve_one(grant.maximum_capacity_bytes).map_err(|error| ValueError::from(error.refusal()))?;
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: usize::from(progress.progressed), copied_bytes: 0, retained_capacity_bytes: progress.allocated_bytes, released_bytes: 0 }));
                }
                if grant.maximum_copy_bytes < size_of::<Puzzle2dMutation>() { return Ok(RetainedCloneStep::Progress(Default::default())); }
                let operation = self.pending.take().ok_or_else(|| refusal("literal inverse lost its literal payload"))?;
                if let Err(payload) = self.inverse.push_reserved(operation) { self.pending = Some(payload); return Err(refusal("literal inverse lost its admitted page")); }
                self.phase = 4;
                Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<Puzzle2dMutation>(), retained_capacity_bytes: 0, released_bytes: 0 }))
            }
            4 => {
                if grant.maximum_copy_bytes < size_of::<PagedList<Puzzle2dMutation, {usize::MAX}>>() { return Ok(RetainedCloneStep::Progress(Default::default())); }
                self.phase = 5;
                Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<PagedList<Puzzle2dMutation, {usize::MAX}>>(), retained_capacity_bytes: 0 , released_bytes: 0 }))
            }
            5 => Ok(RetainedCloneStep::Complete(Default::default())),
            _ => Err(refusal("literal inverse has an unknown ownership phase")),
        }
    }

    pub fn take(&mut self) -> Option<PagedList<Puzzle2dMutation, {usize::MAX}>> { if self.closing || self.phase != 5 { return None; } self.phase = 6; Some(std::mem::take(&mut self.inverse)) }
    pub fn begin_close(&mut self) { self.closing = true; self.identifier.begin_close(); }

    pub fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> {
        if !self.identifier.terminal_is_empty() { return self.identifier.next_close_copy_byte_demand(); }
        if let Some(owner)=self.pending_close.as_ref() { return owner.next_copy_byte_demand(); }
        if self.pending.is_some() { return Ok(size_of::<Puzzle2dMutation>()); }
        if let Some(owner)=self.inverse_close.as_ref() { return owner.next_copy_byte_demand(); }
        if !self.inverse.terminal_is_empty() { return Ok(size_of::<PagedList<Puzzle2dMutation,{usize::MAX}>>()); }
        RetainedCloneBinding::copy_demand(&self.mutation)
    }
    pub fn next_close_capacity_byte_demand(&self, body:usize) -> Result<usize, ValueError> {
        if !self.identifier.terminal_is_empty() { return self.identifier.next_close_capacity_byte_demand(body); }
        if let Some(owner)=self.pending_close.as_ref() { return owner.next_capacity_byte_demand(body); }
        if self.pending.is_some() { return Ok(0); }
        if let Some(owner)=self.inverse_close.as_ref() { return owner.next_capacity_byte_demand(body); }
        if !self.inverse.terminal_is_empty() { return Ok(0); }
        RetainedCloneBinding::capacity_demand(&self.mutation,body)
    }
    pub fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
        if !self.identifier.terminal_is_empty() { return self.identifier.next_close_release_byte_demand(); }
        if let Some(owner)=self.pending_close.as_ref() { return owner.next_release_byte_demand(); }
        if self.pending.is_some() { return Ok(0); }
        if let Some(owner)=self.inverse_close.as_ref() { return owner.next_release_byte_demand(); }
        if !self.inverse.terminal_is_empty() { return Ok(0); }
        RetainedCloneBinding::release_demand(&self.mutation)
    }
    pub fn next_close_depth_demand(&self) -> Result<usize, ValueError> {
        if !self.identifier.terminal_is_empty() { return self.identifier.next_close_depth_demand(); }
        if let Some(owner)=self.pending_close.as_ref() { return owner.next_depth_demand(); }
        if self.pending.is_some() { return Ok(1); }
        if let Some(owner)=self.inverse_close.as_ref() { return owner.next_depth_demand(); }
        if !self.inverse.terminal_is_empty() { return Ok(1); }
        RetainedCloneBinding::depth_demand(&self.mutation)
    }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if !self.closing { return Err(refusal("literal inverse closure was not started")); }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
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
        let step = RetainedCloneBinding::close_one(&mut self.mutation, grant)?;
        Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) })
    }

    pub fn terminal_is_empty(&self) -> bool { self.closing && self.identifier.terminal_is_empty() && self.pending.is_none() && self.inverse.terminal_is_empty() && self.pending_close.is_none() && self.inverse_close.is_none() && self.mutation.is_none() }
}

impl<M: Puzzle2dLiteralIdInverse> Drop for Puzzle2dLiteralIdInverseCursor<M> {
    fn drop(&mut self) { let empty = self.terminal_is_empty(); assert!(std::thread::panicking() || empty, "literal inverse abandoned before controlled closure"); if empty { unsafe { ManuallyDrop::drop(&mut self.state); } } }
}

fn refusal(message: &str) -> ValueError { ValueError::new(ValueRefusalKind::InvariantViolated, message) }
