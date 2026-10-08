//! ↩️ Owns the original rename identifier and old name through separately granted native pages.

use super::{owned::{DrawingOwnedRenamePlan, DrawingOwnedRenamePreparationCursor, DrawingOwnedRenamePreparationStep}, RenameLayer};
use crate::{DrawingMutation, DrawingSnapshot};
use semio_framework_value::{ValueError, ValueRefusalKind, list::PagedList, paged::PagedUtf8, retirement::controlled::ControlledRetirement, retained_clone::{RetainedClone, RetainedCloneCursor, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneStep, RetainedOwnedProjection}};
use std::{mem::{ManuallyDrop, size_of}, ops::{Deref, DerefMut}};

pub struct DrawingRenameInverseCursor { state: ManuallyDrop<DrawingRenameInverseState> }

#[doc(hidden)]
pub struct DrawingRenameInverseState {
    preparation: DrawingOwnedRenamePreparationCursor,
    identifier: <PagedUtf8<{usize::MAX}> as RetainedClone>::Cursor,
    name: <PagedUtf8<{usize::MAX}> as RetainedClone>::Cursor,
    plan: Option<DrawingOwnedRenamePlan>,
    pending: Option<RenameLayer>,
    inverse: PagedList<DrawingMutation, {usize::MAX}>,
    pending_close: Option<ControlledRetirement<RenameLayer>>,
    inverse_close: Option<ControlledRetirement<PagedList<DrawingMutation, {usize::MAX}>>>,
    phase: u8,
    closing: bool,
}

impl Deref for DrawingRenameInverseCursor { type Target = DrawingRenameInverseState; fn deref(&self) -> &Self::Target { &self.state } }
impl DerefMut for DrawingRenameInverseCursor { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.state } }

impl DrawingRenameInverseCursor {
    pub fn new(source: RetainedOwnedProjection<DrawingSnapshot>) -> Result<Self, ValueError> {
        Ok(Self { state: ManuallyDrop::new(DrawingRenameInverseState { preparation: DrawingOwnedRenamePreparationCursor::new(source)?, identifier: PagedUtf8::retained_clone_cursor(), name: PagedUtf8::retained_clone_cursor(), plan: None, pending: None, inverse: PagedList::default(), pending_close: None, inverse_close: None, phase: 0, closing: false }) })
    }

    pub fn advance(&mut self, payload: RetainedCloneRef<'_, RenameLayer>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.closing || self.phase == 10 { return Err(refusal("Drawing rename inverse is closing or spent")); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if let Some(plan) = &mut self.plan { plan.check_original(payload)?; }
        match self.phase {
            0 => {
                let step = self.preparation.advance(payload, grant)?;
                let progress = match step {
                    DrawingOwnedRenamePreparationStep::Pending(progress) => progress,
                    DrawingOwnedRenamePreparationStep::Complete { progress, .. } => { self.plan = self.preparation.take(); self.preparation.begin_close(); self.phase = 1; progress },
                };
                Ok(RetainedCloneStep::Progress(progress))
            }
            1 => {
                let step = self.preparation.close_granted(grant)?;
                if self.preparation.terminal_is_empty() { self.phase = if self.plan.as_ref().map(DrawingOwnedRenamePlan::inverse_name).transpose()?.flatten().is_some() { 2 } else { 8 }; }
                Ok(RetainedCloneStep::Progress(step.progress()))
            }
            2 => {
                let step = self.identifier.advance(payload.project(1, |payload| &payload.layer_id), grant)?;
                if matches!(step, RetainedCloneStep::Complete(_)) { self.phase = 3; }
                Ok(RetainedCloneStep::Progress(step.progress()))
            }
            3 => {
                let state = &mut *self.state;
                let source = state.plan.as_ref().ok_or_else(|| refusal("Drawing rename inverse has no retained plan"))?.inverse_name()?.ok_or_else(|| refusal("Drawing rename inverse has no original name"))?;
                let step = state.name.advance(source, grant)?;
                if matches!(step, RetainedCloneStep::Complete(_)) { state.phase = 4; }
                Ok(RetainedCloneStep::Progress(step.progress()))
            }
            4 => {
                if grant.maximum_copy_bytes < size_of::<RenameLayer>() { return Ok(RetainedCloneStep::Progress(Default::default())); }
                let layer_id = self.identifier.take().ok_or_else(|| refusal("Drawing rename inverse identifier has no completed owner"))?;
                let new_name = self.name.take().expect("completed Drawing inverse name");
                self.pending = Some(RenameLayer { layer_id, new_name });
                self.identifier.begin_close();
                self.name.begin_close();
                self.phase = 5;
                Ok(payload_progress(size_of::<RenameLayer>()))
            }
            5 => {
                let step = self.identifier.close_granted(grant)?;
                if self.identifier.terminal_is_empty() { self.phase = 6; }
                Ok(RetainedCloneStep::Progress(step.progress()))
            }
            6 => {
                let step = self.name.close_granted(grant)?;
                if self.name.terminal_is_empty() { self.phase = 7; }
                Ok(RetainedCloneStep::Progress(step.progress()))
            }
            7 => {
                if !self.inverse.has_reserved_slot() {
                    let demand = self.inverse.next_capacity_allocation_bytes(1).map_err(ValueError::from)?.ok_or_else(|| refusal("Drawing rename inverse has no page demand"))?;
                    if demand > grant.maximum_capacity_bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
                    let progress = self.inverse.reserve_one(grant.maximum_capacity_bytes).map_err(|error| ValueError::from(error.refusal()))?;
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: usize::from(progress.progressed), copied_bytes: 0, retained_capacity_bytes: progress.allocated_bytes, released_bytes: 0 }));
                }
                if grant.maximum_copy_bytes < size_of::<DrawingMutation>() { return Ok(RetainedCloneStep::Progress(Default::default())); }
                let mutation = DrawingMutation::RenameLayer(self.pending.take().ok_or_else(|| refusal("Drawing rename inverse lost its literal owner"))?);
                if let Err(DrawingMutation::RenameLayer(payload)) = self.inverse.push_reserved(mutation) { self.pending = Some(payload); return Err(refusal("Drawing rename inverse lost its admitted slot")); }
                self.phase = 8;
                Ok(payload_progress(size_of::<DrawingMutation>()))
            }
            8 => {
                if grant.maximum_copy_bytes < size_of::<PagedList<DrawingMutation, {usize::MAX}>>() { return Ok(RetainedCloneStep::Progress(Default::default())); }
                self.phase = 9;
                Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<PagedList<DrawingMutation, {usize::MAX}>>(), retained_capacity_bytes: 0 , released_bytes: 0 }))
            }
            9 => Ok(RetainedCloneStep::Complete(Default::default())),
            _ => Err(refusal("Drawing rename inverse has an unknown ownership phase")),
        }
    }

    pub fn take(&mut self) -> Option<PagedList<DrawingMutation, {usize::MAX}>> {
        if self.closing || self.phase != 9 { return None; }
        self.phase = 10;
        Some(std::mem::take(&mut self.inverse))
    }

    pub fn begin_close(&mut self) { self.closing = true; self.preparation.begin_close(); self.identifier.begin_close(); self.name.begin_close(); if let Some(plan) = &mut self.plan { plan.begin_close(); } }

    pub fn close_granted(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if !self.closing { return Err(refusal("Drawing rename inverse closure was not begun")); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !self.preparation.terminal_is_empty() { return self.preparation.close_granted(grant).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if !self.identifier.terminal_is_empty() { return self.identifier.close_granted(grant).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if !self.name.terminal_is_empty() { return self.name.close_granted(grant).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if let Some(owner) = self.pending_close.as_mut() {
            let step = owner.step(grant)?;
            if owner.terminal_is_empty() { self.pending_close = None; }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.pending.is_some() {
            if grant.maximum_copy_bytes < size_of::<RenameLayer>() { return Ok(RetainedCloneStep::Progress(Default::default())); }
            match ControlledRetirement::new(self.pending.take().unwrap()) { Ok(owner) => self.pending_close = Some(owner), Err((error, owner)) => { self.pending = Some(owner); return Err(error); } }
            return Ok(payload_progress(size_of::<RenameLayer>()));
        }
        if let Some(owner) = self.inverse_close.as_mut() {
            let step = owner.step(grant)?;
            if owner.terminal_is_empty() { self.inverse_close = None; }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if !self.inverse.terminal_is_empty() {
            if grant.maximum_copy_bytes < size_of::<PagedList<DrawingMutation, {usize::MAX}>>() { return Ok(RetainedCloneStep::Progress(Default::default())); }
            match ControlledRetirement::new(std::mem::take(&mut self.inverse)) { Ok(owner) => self.inverse_close = Some(owner), Err((error, owner)) => { self.inverse = owner; return Err(error); } }
            return Ok(payload_progress(size_of::<PagedList<DrawingMutation, {usize::MAX}>>()));
        }
        if let Some(plan) = &mut self.plan {
            let step = plan.close_granted(grant)?;
            if plan.terminal_is_empty() { self.plan = None; }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        Ok(RetainedCloneStep::Complete(Default::default()))
    }

    pub fn terminal_is_empty(&self) -> bool { self.closing && self.preparation.terminal_is_empty() && self.identifier.terminal_is_empty() && self.name.terminal_is_empty() && self.plan.is_none() && self.pending.is_none() && self.inverse.terminal_is_empty() && self.pending_close.is_none() && self.inverse_close.is_none() }
}

impl Drop for DrawingRenameInverseCursor {
    fn drop(&mut self) { let empty = self.terminal_is_empty(); assert!(std::thread::panicking() || empty, "Drawing rename inverse reached Drop before granted closure"); if empty { unsafe { ManuallyDrop::drop(&mut self.state); } } }
}

fn payload_progress(bytes: usize) -> RetainedCloneStep { RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, retained_capacity_bytes: 0, released_bytes: 0 }) }
fn refusal(message: &str) -> ValueError { ValueError::new(ValueRefusalKind::InvariantViolated, message) }
