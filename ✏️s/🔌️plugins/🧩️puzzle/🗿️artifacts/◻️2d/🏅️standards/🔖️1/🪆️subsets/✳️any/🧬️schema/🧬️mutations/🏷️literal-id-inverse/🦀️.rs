//! 🏷️ Typed literal inverses retain only their original native identifier ownership.

use crate::standards::v1::subsets::any::schema::preparation::native_preparation_child::{controlled_step, cursor_advance, cursor_retire, cursor_retire_demand, cursor_taken, edit_progress, edit_step, list_reserve, pay, settled_step};
use semio_framework_value::{OriginalRetirementPoll, RetainedAllocationEffect, RetirementPollExit};
use crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;
use semio_framework_value::{ValueError, ValueRefusalKind, paged::PagedUtf8, retirement::controlled::ControlledRetirement, retained_clone::{RetainedClone, RetainedCloneBinding, RetainedCloneCursor, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneStep}};
use semio_framework_value_persistent::list::PagedList;
use std::{mem::{ManuallyDrop, size_of}, ops::{Deref, DerefMut}};

pub trait Puzzle2dLiteralIdInverse: Send + Sync { fn inverse_id(&self) -> &PagedUtf8<{usize::MAX}>; fn inverse_payload(id: PagedUtf8<{usize::MAX}>) -> Puzzle2dMutation; }

impl Puzzle2dLiteralIdInverse for crate::standards::v1::subsets::any::schema::mutations::CreateNode {
    fn inverse_id(&self) -> &PagedUtf8<{usize::MAX}> { &self.node.id }
    fn inverse_payload(id: PagedUtf8<{usize::MAX}>) -> Puzzle2dMutation { Puzzle2dMutation::DeleteNode(crate::standards::v1::subsets::any::schema::mutations::delete_node::DeleteNode { id }) }
}

impl Puzzle2dLiteralIdInverse for crate::standards::v1::subsets::any::schema::mutations::ConnectHandles {
    fn inverse_id(&self) -> &PagedUtf8<{usize::MAX}> { &self.id }
    fn inverse_payload(id: PagedUtf8<{usize::MAX}>) -> Puzzle2dMutation { Puzzle2dMutation::DisconnectHandles(crate::standards::v1::subsets::any::schema::mutations::disconnect_handles::DisconnectHandles { id }) }
}

pub struct Puzzle2dLiteralIdInverseCursor<M: Puzzle2dLiteralIdInverse> { state: ManuallyDrop<Puzzle2dLiteralIdInverseState>, marker: std::marker::PhantomData<M> }

/// 🧺️ Holds the literal payload and each of its sequential controlled closes in one shared slot.
#[doc(hidden)]
pub enum Puzzle2dLiteralIdInverseHeld { Empty, Payload(Puzzle2dMutation), PayloadClose(ControlledRetirement<Puzzle2dMutation>), InverseClose(ControlledRetirement<PagedList<Puzzle2dMutation, {usize::MAX}>>) }

#[doc(hidden)]
pub struct Puzzle2dLiteralIdInverseState {
    mutation: Option<RetainedCloneBinding>,
    identifier: <PagedUtf8<{usize::MAX}> as RetainedClone>::Cursor,
    inverse: PagedList<Puzzle2dMutation, {usize::MAX}>,
    held: Puzzle2dLiteralIdInverseHeld,
    phase: u8,
    closing: bool,
}

impl<M: Puzzle2dLiteralIdInverse> Deref for Puzzle2dLiteralIdInverseCursor<M> { type Target = Puzzle2dLiteralIdInverseState; fn deref(&self) -> &Self::Target { &self.state } }
impl<M: Puzzle2dLiteralIdInverse> DerefMut for Puzzle2dLiteralIdInverseCursor<M> { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.state } }

impl<M: Puzzle2dLiteralIdInverse> Default for Puzzle2dLiteralIdInverseCursor<M> {
    fn default() -> Self { Self { state: ManuallyDrop::new(Puzzle2dLiteralIdInverseState { mutation: None, identifier: PagedUtf8::retained_clone_cursor(), inverse: PagedList::default(), held: Puzzle2dLiteralIdInverseHeld::Empty, phase: 0, closing: false }), marker: std::marker::PhantomData } }
}

impl<M: Puzzle2dLiteralIdInverse> Puzzle2dLiteralIdInverseCursor<M> {
    pub fn advance(&mut self, mutation: RetainedCloneRef<'_, M>, body:usize,effect:&mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneStep, ValueError> {
        let grant=effect.grant();
        if self.closing || self.phase == 6 { return Err(refusal("literal inverse is closing or spent")); }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if let Some(progress)=mutation.bind(&mut self.mutation,effect).result?{return Ok(RetainedCloneStep::Progress(progress))}
        match self.phase {
            0 => {
                let step = cursor_advance(&mut self.identifier,mutation.project(1, M::inverse_id),body,effect)?;
                if matches!(step, RetainedCloneStep::Complete(_)) { self.phase = 1; }
                Ok(RetainedCloneStep::Progress(step.progress()))
            }
            1 => {
                if grant.maximum_copy_bytes < size_of::<Puzzle2dMutation>() { return Ok(RetainedCloneStep::Progress(Default::default())); }
                let id=match cursor_taken(&mut self.identifier,effect)?{Some(owner)=>owner,None=>return Ok(RetainedCloneStep::Progress(Default::default()))};
                self.held = Puzzle2dLiteralIdInverseHeld::Payload(M::inverse_payload(id));
                
                self.phase = 2;
                Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<Puzzle2dMutation>(), retained_capacity_bytes: 0, released_bytes: 0 }))
            }
            2 => {
                let step=cursor_retire(&mut self.identifier,effect)?;
                if self.identifier.terminal_is_empty() { self.phase = 3; }
                Ok(RetainedCloneStep::Progress(step.progress()))
            }
            3 => {
                if !self.inverse.has_reserved_slot() { let Some(progress)=list_reserve(&mut self.inverse,effect)? else { return Ok(RetainedCloneStep::Progress(Default::default())); }; return Ok(RetainedCloneStep::Progress(progress)); }
                if grant.maximum_copy_bytes < size_of::<Puzzle2dMutation>()||!pay(effect)? { return Ok(RetainedCloneStep::Progress(Default::default())); }
                let operation = match std::mem::replace(&mut self.held, Puzzle2dLiteralIdInverseHeld::Empty) { Puzzle2dLiteralIdInverseHeld::Payload(operation) => operation, other => { self.held = other; return Err(refusal("literal inverse lost its literal payload")); } };
                if let Err(payload) = self.inverse.push_reserved(operation) { self.held = Puzzle2dLiteralIdInverseHeld::Payload(payload); return Err(refusal("literal inverse lost its admitted page")); }
                self.phase = 4;
                Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<Puzzle2dMutation>(), retained_capacity_bytes: 0, released_bytes: 0 }))
            }
            4 => {
                if grant.maximum_copy_bytes < size_of::<PagedList<Puzzle2dMutation, {usize::MAX}>>()||!pay(effect)? { return Ok(RetainedCloneStep::Progress(Default::default())); }
                self.phase = 5;
                Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<PagedList<Puzzle2dMutation, {usize::MAX}>>(), retained_capacity_bytes: 0 , released_bytes: 0 }))
            }
            5 => Ok(RetainedCloneStep::Complete(Default::default())),
            _ => Err(refusal("literal inverse has an unknown ownership phase")),
        }
    }

    pub fn take(&mut self) -> Option<PagedList<Puzzle2dMutation, {usize::MAX}>> { if self.closing || self.phase != 5 { return None; } self.phase = 6; Some(std::mem::take(&mut self.inverse)) }
    pub fn begin_close(&mut self) { self.closing = true;  }

    pub fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> {
        if !self.identifier.terminal_is_empty() { return cursor_retire_demand(&self.identifier,0).map(|demand| demand.copy_bytes); }
        match &self.held { Puzzle2dLiteralIdInverseHeld::PayloadClose(owner) => return owner.next_copy_byte_demand(), Puzzle2dLiteralIdInverseHeld::Payload(_) => return Ok(size_of::<Puzzle2dMutation>()), Puzzle2dLiteralIdInverseHeld::InverseClose(owner) => return owner.next_copy_byte_demand(), Puzzle2dLiteralIdInverseHeld::Empty => {} }
        if !self.inverse.terminal_is_empty() { return Ok(size_of::<PagedList<Puzzle2dMutation,{usize::MAX}>>()); }
        RetainedCloneBinding::copy_demand(&self.mutation)
    }
    pub fn next_close_capacity_byte_demand(&self, body:usize) -> Result<usize, ValueError> {
        if !self.identifier.terminal_is_empty() { return cursor_retire_demand(&self.identifier,body).map(|demand| demand.capacity_bytes); }
        match &self.held { Puzzle2dLiteralIdInverseHeld::PayloadClose(owner) => return owner.next_capacity_byte_demand(body), Puzzle2dLiteralIdInverseHeld::Payload(_) => return Ok(0), Puzzle2dLiteralIdInverseHeld::InverseClose(owner) => return owner.next_capacity_byte_demand(body), Puzzle2dLiteralIdInverseHeld::Empty => {} }
        if !self.inverse.terminal_is_empty() { return Ok(0); }
        RetainedCloneBinding::capacity_demand(&self.mutation,body)
    }
    pub fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
        if !self.identifier.terminal_is_empty() { return cursor_retire_demand(&self.identifier,0).map(|demand| demand.release_bytes); }
        match &self.held { Puzzle2dLiteralIdInverseHeld::PayloadClose(owner) => return owner.next_release_byte_demand(), Puzzle2dLiteralIdInverseHeld::Payload(_) => return Ok(0), Puzzle2dLiteralIdInverseHeld::InverseClose(owner) => return owner.next_release_byte_demand(), Puzzle2dLiteralIdInverseHeld::Empty => {} }
        if !self.inverse.terminal_is_empty() { return Ok(0); }
        RetainedCloneBinding::release_demand(&self.mutation)
    }
    pub fn next_close_depth_demand(&self) -> Result<usize, ValueError> {
        if !self.identifier.terminal_is_empty() { return cursor_retire_demand(&self.identifier,0).map(|demand| demand.depth); }
        match &self.held { Puzzle2dLiteralIdInverseHeld::PayloadClose(owner) => return owner.next_depth_demand(), Puzzle2dLiteralIdInverseHeld::Payload(_) => return Ok(1), Puzzle2dLiteralIdInverseHeld::InverseClose(owner) => return owner.next_depth_demand(), Puzzle2dLiteralIdInverseHeld::Empty => {} }
        if !self.inverse.terminal_is_empty() { return Ok(1); }
        RetainedCloneBinding::depth_demand(&self.mutation)
    }

    pub fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{let result=self.close_stepped(effect);RetirementPollExit::settle(effect,result)}
    fn close_stepped(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,RetirementPollExit>{
        let grant=effect.grant();
        if !self.closing { return Err(refusal("literal inverse closure was not started").into()); }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !self.identifier.terminal_is_empty() { return Ok(RetainedCloneStep::Progress(cursor_retire(&mut self.identifier,effect)?.progress())); }
        match &mut self.held {
            Puzzle2dLiteralIdInverseHeld::PayloadClose(owner) => {
                let step = controlled_step(owner,effect)?;
                if owner.terminal_is_empty() { self.held = Puzzle2dLiteralIdInverseHeld::Empty; }
                return Ok(RetainedCloneStep::Progress(step.progress()));
            }
            Puzzle2dLiteralIdInverseHeld::InverseClose(owner) => {
                let step = controlled_step(owner,effect)?;
                if owner.terminal_is_empty() { self.held = Puzzle2dLiteralIdInverseHeld::Empty; }
                return Ok(RetainedCloneStep::Progress(step.progress()));
            }
            Puzzle2dLiteralIdInverseHeld::Payload(_) => {
                if grant.maximum_copy_bytes < size_of::<Puzzle2dMutation>()||!pay(effect)? { return Ok(RetainedCloneStep::Progress(Default::default())); }
                let Puzzle2dLiteralIdInverseHeld::Payload(payload) = std::mem::replace(&mut self.held, Puzzle2dLiteralIdInverseHeld::Empty) else { unreachable!("payload slot checked") };
                match ControlledRetirement::new(payload) { Ok(owner) => self.held = Puzzle2dLiteralIdInverseHeld::PayloadClose(owner), Err((error, owner)) => { self.held = Puzzle2dLiteralIdInverseHeld::Payload(owner); return Err(error.into()); } }
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<Puzzle2dMutation>(), retained_capacity_bytes: 0, released_bytes: 0 }));
            }
            Puzzle2dLiteralIdInverseHeld::Empty => {}
        }
        if !self.inverse.terminal_is_empty() {
            if grant.maximum_copy_bytes < size_of::<PagedList<Puzzle2dMutation, {usize::MAX}>>()||!pay(effect)? { return Ok(RetainedCloneStep::Progress(Default::default())); }
            match ControlledRetirement::new(std::mem::take(&mut self.inverse)) { Ok(owner) => self.held = Puzzle2dLiteralIdInverseHeld::InverseClose(owner), Err((error, owner)) => { self.inverse = owner; return Err(error.into()); } }
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<PagedList<Puzzle2dMutation, {usize::MAX}>>(), retained_capacity_bytes: 0 , released_bytes: 0 }));
        }
        let step = RetirementPollExit::resume(RetainedCloneBinding::close_one(&mut self.mutation,grant.maximum_copy_bytes,effect))?;
        Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) })
    }

    pub fn terminal_is_empty(&self) -> bool { self.closing && self.identifier.terminal_is_empty() && matches!(self.held, Puzzle2dLiteralIdInverseHeld::Empty) && self.inverse.terminal_is_empty() && self.mutation.is_none() }
}

impl<M: Puzzle2dLiteralIdInverse> Drop for Puzzle2dLiteralIdInverseCursor<M> {
    fn drop(&mut self) { let empty = self.terminal_is_empty(); assert!(std::thread::panicking() || empty, "literal inverse abandoned before controlled closure"); if empty { unsafe { ManuallyDrop::drop(&mut self.state); } } }
}

fn refusal(message: &str) -> ValueError { ValueError::new(ValueRefusalKind::InvariantViolated, message) }
