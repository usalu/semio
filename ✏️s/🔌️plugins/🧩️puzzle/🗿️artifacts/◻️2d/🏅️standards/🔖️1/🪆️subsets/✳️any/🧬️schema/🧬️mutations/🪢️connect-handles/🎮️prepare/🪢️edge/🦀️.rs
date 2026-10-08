//! 🪢️ Assembles actual native edge fields through separately granted ownership turns.

use super::ConnectHandles;
use crate::Puzzle2dEdge;
use semio_framework_value::{ValueError, ValueRefusalKind, paged::PagedUtf8, retirement::controlled::ControlledRetirement, retained_clone::{RetainedClone, RetainedCloneBinding, RetainedCloneCursor, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneStep}};
use std::{mem::{ManuallyDrop, size_of}, ops::{Deref, DerefMut}};

type Text = PagedUtf8<{usize::MAX}>;
pub struct Puzzle2dConnectEdgeCursor { state: ManuallyDrop<Puzzle2dConnectEdgeState> }

#[doc(hidden)]
pub struct Puzzle2dConnectEdgeState {
    mutation: Option<RetainedCloneBinding>,
    text: <Text as RetainedClone>::Cursor,
    optional: <Option<Text> as RetainedClone>::Cursor,
    edge: Option<Puzzle2dEdge>,
    retirement: Option<ControlledRetirement<Puzzle2dEdge>>,
    scalar: u8,
    field: u8,
    child: u8,
    phase: u8,
    closing: bool,
}

impl Deref for Puzzle2dConnectEdgeCursor { type Target = Puzzle2dConnectEdgeState; fn deref(&self) -> &Self::Target { &self.state } }
impl DerefMut for Puzzle2dConnectEdgeCursor { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.state } }
impl Default for Puzzle2dConnectEdgeCursor {
    fn default() -> Self { Self { state: ManuallyDrop::new(Puzzle2dConnectEdgeState { mutation: None, text: Text::retained_clone_cursor(), optional: Option::<Text>::retained_clone_cursor(), edge: None, retirement: None, scalar: 0, field: 0, child: 0, phase: 0, closing: false }) } }
}

impl Puzzle2dConnectEdgeCursor {
    pub fn advance(&mut self, mutation: RetainedCloneRef<'_, ConnectHandles>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.closing || self.phase == 5 { return Err(refusal("edge assembly is closing or spent")); }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        mutation.bind(&mut self.mutation)?;
        if self.phase == 0 {
            if grant.maximum_copy_bytes < size_of::<Puzzle2dEdge>() { return Ok(RetainedCloneStep::Progress(Default::default())); }
            self.edge = Some(Puzzle2dEdge { id: Text::default(), source: Text::default(), target: Text::default(), edge_kind: None, gap: 0.0, shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0, x: 0.0, y: 0.0, source_tip: None, target_tip: None, visible: None, locked: None });
            self.phase = 1;
            return Ok(copy(size_of::<Puzzle2dEdge>()));
        }
        if self.phase == 1 {
            if grant.maximum_copy_bytes < size_of::<f64>() { return Ok(RetainedCloneStep::Progress(Default::default())); }
            let payload = mutation.get();
            let scalar = self.scalar;
            let edge = self.edge.as_mut().ok_or_else(|| refusal("edge assembly lost its native owner"))?;
            match scalar { 0 => edge.gap = payload.gap, 1 => edge.shift = payload.shift, 2 => edge.rise = payload.rise, 3 => edge.rotation = payload.rotation, 4 => edge.turn = payload.turn, 5 => edge.tilt = payload.tilt, 6 => edge.x = payload.x, _ => edge.y = payload.y }
            self.scalar += 1;
            if self.scalar == 8 { self.phase = 2; }
            return Ok(copy(size_of::<f64>()));
        }
        if self.phase == 2 {
            let field = self.field;
            if self.child == 0 {
                let step = if field < 3 { self.text.advance(mutation.project(field as usize + 1, |payload| match field { 0 => &payload.id, 1 => &payload.source, _ => &payload.target }), grant)? }
                else { self.optional.advance(mutation.project(field as usize + 1, |payload| match field { 3 => &payload.edge_kind, 4 => &payload.source_tip, _ => &payload.target_tip }), grant)? };
                if matches!(step, RetainedCloneStep::Complete(_)) { self.child = 1; }
                return Ok(RetainedCloneStep::Progress(step.progress()));
            }
            if self.child == 1 {
                let bytes = if field < 3 { size_of::<Text>() } else { size_of::<Option<Text>>() };
                if grant.maximum_copy_bytes < bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
                if field < 3 {
                    let value = self.text.take().ok_or_else(|| refusal("edge field completed without text ownership"))?;
                    let edge = self.edge.as_mut().ok_or_else(|| refusal("edge assembly lost its native owner"))?;
                    match field { 0 => edge.id = value, 1 => edge.source = value, _ => edge.target = value }
                    self.text.begin_close();
                } else {
                    let value = self.optional.take().ok_or_else(|| refusal("edge field completed without optional ownership"))?;
                    let edge = self.edge.as_mut().ok_or_else(|| refusal("edge assembly lost its native owner"))?;
                    match field { 3 => edge.edge_kind = value, 4 => edge.source_tip = value, _ => edge.target_tip = value }
                    self.optional.begin_close();
                }
                self.child = 2;
                return Ok(copy(bytes));
            }
            let step = if field < 3 { self.text.close_step(grant)? } else { self.optional.close_step(grant)? };
            let empty = if field < 3 { self.text.terminal_is_empty() } else { self.optional.terminal_is_empty() };
            if empty {
                if field < 3 { self.text = Text::retained_clone_cursor(); } else { self.optional = Option::<Text>::retained_clone_cursor(); }
                self.field += 1;
                self.child = 0;
                if self.field == 6 { self.phase = 3; }
            }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.phase == 3 {
            if grant.maximum_copy_bytes < size_of::<Puzzle2dEdge>() { return Ok(RetainedCloneStep::Progress(Default::default())); }
            self.phase = 4;
            return Ok(RetainedCloneStep::Complete(copy(size_of::<Puzzle2dEdge>()).progress()));
        }
        Ok(RetainedCloneStep::Complete(Default::default()))
    }

    pub fn take(&mut self) -> Option<Puzzle2dEdge> { if self.closing || self.phase != 4 { return None; } self.phase = 5; self.edge.take() }
    pub fn begin_close(&mut self) { self.closing = true; self.text.begin_close(); self.optional.begin_close(); }
    pub fn next_close_copy_byte_demand(&self) -> Result<usize,ValueError> {
        if !self.text.terminal_is_empty(){return self.text.next_close_copy_byte_demand();}
        if !self.optional.terminal_is_empty(){return self.optional.next_close_copy_byte_demand();}
        if let Some(owner)=self.retirement.as_ref(){return owner.next_copy_byte_demand();}
        if self.edge.is_some(){return Ok(size_of::<Puzzle2dEdge>());}
        RetainedCloneBinding::copy_demand(&self.mutation)
    }
    pub fn next_close_capacity_byte_demand(&self,body:usize) -> Result<usize,ValueError> {
        if !self.text.terminal_is_empty(){return self.text.next_close_capacity_byte_demand(body);}
        if !self.optional.terminal_is_empty(){return self.optional.next_close_capacity_byte_demand(body);}
        if let Some(owner)=self.retirement.as_ref(){return owner.next_capacity_byte_demand(body);}
        if self.edge.is_some(){return Ok(0);}
        RetainedCloneBinding::capacity_demand(&self.mutation,body)
    }
    pub fn next_close_release_byte_demand(&self) -> Result<usize,ValueError> {
        if !self.text.terminal_is_empty(){return self.text.next_close_release_byte_demand();}
        if !self.optional.terminal_is_empty(){return self.optional.next_close_release_byte_demand();}
        if let Some(owner)=self.retirement.as_ref(){return owner.next_release_byte_demand();}
        if self.edge.is_some(){return Ok(0);}
        RetainedCloneBinding::release_demand(&self.mutation)
    }
    pub fn next_close_depth_demand(&self) -> Result<usize,ValueError> {
        if !self.text.terminal_is_empty(){return self.text.next_close_depth_demand();}
        if !self.optional.terminal_is_empty(){return self.optional.next_close_depth_demand();}
        if let Some(owner)=self.retirement.as_ref(){return owner.next_depth_demand();}
        if self.edge.is_some(){return Ok(1);}
        RetainedCloneBinding::depth_demand(&self.mutation)
    }
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if !self.closing { return Err(refusal("edge assembly closure was not started")); }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !self.text.terminal_is_empty() { return self.text.close_step(grant).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if !self.optional.terminal_is_empty() { return self.optional.close_step(grant).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if let Some(owner) = self.retirement.as_mut() { let step = owner.step(grant)?; if owner.terminal_is_empty() { self.retirement = None; } return Ok(RetainedCloneStep::Progress(step.progress())); }
        if self.edge.is_some() {
            if grant.maximum_copy_bytes < size_of::<Puzzle2dEdge>() { return Ok(RetainedCloneStep::Progress(Default::default())); }
            match ControlledRetirement::new(self.edge.take().unwrap()) { Ok(owner) => self.retirement = Some(owner), Err((error, edge)) => { self.edge = Some(edge); return Err(error); } }
            return Ok(copy(size_of::<Puzzle2dEdge>()));
        }
        let step = RetainedCloneBinding::close_one(&mut self.mutation, grant)?;
        Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) })
    }
    pub fn terminal_is_empty(&self) -> bool { self.closing && self.text.terminal_is_empty() && self.optional.terminal_is_empty() && self.edge.is_none() && self.retirement.is_none() && self.mutation.is_none() }
}

impl Drop for Puzzle2dConnectEdgeCursor { fn drop(&mut self) { let empty = self.terminal_is_empty(); assert!(std::thread::panicking() || empty, "edge assembly abandoned before controlled closure"); if empty { unsafe { ManuallyDrop::drop(&mut self.state); } } } }
fn copy(bytes: usize) -> RetainedCloneStep { RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() }) }
fn refusal(message: &str) -> ValueError { ValueError::new(ValueRefusalKind::InvariantViolated, message) }

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
