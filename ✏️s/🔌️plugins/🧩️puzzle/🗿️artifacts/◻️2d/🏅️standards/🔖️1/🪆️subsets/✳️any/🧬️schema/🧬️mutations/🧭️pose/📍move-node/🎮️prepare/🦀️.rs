//! 🎮️ Borrowed move semantics produce a scalar plan before inverse or diagnostic ownership.

use crate::standards::v1::subsets::any::schema::mutations::move_node::MoveNode;
use crate::{Puzzle2dSnapshot, standards::v1::subsets::any::schema::lookup::{Puzzle2dLookupStep, pay}};
use crate::standards::v1::subsets::any::schema::lookup::borrowed::{BorrowedIdentity,BorrowedNodeLookup};
use semio_framework_value::{OriginalRetirementPoll, RetainedAllocationEffect, RetirementPollExit, ValueError, ValueRefusalKind, retained_clone::{RetainedCloneProgress, RetainedCloneRef, RetainedCloneStep}};

#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;

#[path = "📸️candidate/🦀️.rs"]
pub mod candidate;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Puzzle2dMoveNodeDisposition { Changed, NoOp, TargetMissing, NonfiniteX, NonfiniteY }

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Puzzle2dMoveNodePlan {
    pub disposition: Puzzle2dMoveNodeDisposition,
    pub node_index: Option<usize>,
    pub previous_position: Option<[f64; 2]>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Puzzle2dMoveNodePreparationStep { Pending(RetainedCloneProgress), Complete { plan: Puzzle2dMoveNodePlan, progress: RetainedCloneProgress } }

pub struct Puzzle2dMoveNodePreparationCursor {
    source: BorrowedIdentity,
    mutation: BorrowedIdentity,
    lookup: BorrowedNodeLookup,
    node_index: Option<usize>,
    nonfinite: Option<Puzzle2dMoveNodeDisposition>,
    phase: u8,
    output: Option<Puzzle2dMoveNodePlan>,
    spent: bool,
    closing: bool,
}

impl Default for Puzzle2dMoveNodePreparationCursor {
    fn default() -> Self { Self { source: Default::default(), mutation: Default::default(), lookup: BorrowedNodeLookup::default(), node_index: None, nonfinite: None, phase: 0, output: None, spent: false, closing: false } }
}

impl Puzzle2dMoveNodePreparationCursor {
    pub fn advance(&mut self, source: RetainedCloneRef<'_, Puzzle2dSnapshot>, mutation: RetainedCloneRef<'_, MoveNode>, effect: &mut RetainedAllocationEffect<'_>) -> Result<Puzzle2dMoveNodePreparationStep, ValueError> {
        let grant = effect.grant();
        if self.closing || self.spent { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "move preparation is closing or spent")); }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(Puzzle2dMoveNodePreparationStep::Pending(Default::default())); }
        if let Some(progress)=self.source.bind(source,effect)?{return Ok(Puzzle2dMoveNodePreparationStep::Pending(progress))}
        if let Some(progress)=self.mutation.bind(mutation,effect)?{return Ok(Puzzle2dMoveNodePreparationStep::Pending(progress))}
        if let Some(plan) = self.output { return Ok(Puzzle2dMoveNodePreparationStep::Complete { plan, progress: Default::default() }); }
        if self.phase == 0 {
            return match self.lookup.advance(source, mutation.project(1, |payload| &payload.id), effect)? {
                Puzzle2dLookupStep::Pending(progress) => Ok(Puzzle2dMoveNodePreparationStep::Pending(progress)),
                Puzzle2dLookupStep::Complete { location, progress } => { self.node_index = location.map(|location| location.outer); self.lookup.take(); self.phase = 1; Ok(Puzzle2dMoveNodePreparationStep::Pending(progress)) },
            };
        }
        if self.phase == 1 || self.phase == 2 {
            if grant.maximum_copy_bytes < size_of::<f64>() || !pay(effect)? { return Ok(Puzzle2dMoveNodePreparationStep::Pending(Default::default())); }
            let (value, disposition) = if self.phase == 1 { (mutation.get().new_x, Puzzle2dMoveNodeDisposition::NonfiniteX) } else { (mutation.get().new_y, Puzzle2dMoveNodeDisposition::NonfiniteY) };
            if self.nonfinite.is_none() && !value.is_finite() { self.nonfinite = Some(disposition); }
            self.phase += 1;
            return Ok(Puzzle2dMoveNodePreparationStep::Pending(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<f64>(), ..Default::default() }));
        }
        if grant.maximum_copy_bytes < 4 * size_of::<f64>() || !pay(effect)? { return Ok(Puzzle2dMoveNodePreparationStep::Pending(Default::default())); }
        let previous_position = self.node_index.map(|index| { let node = source.get().nodes.get(index).expect("immutable move preparation node"); [node.x, node.y] });
        let disposition = self.nonfinite.unwrap_or_else(|| match previous_position { None => Puzzle2dMoveNodeDisposition::TargetMissing, Some([x,y]) if x == mutation.get().new_x && y == mutation.get().new_y => Puzzle2dMoveNodeDisposition::NoOp, Some(_) => Puzzle2dMoveNodeDisposition::Changed });
        let plan = Puzzle2dMoveNodePlan { disposition, node_index: self.node_index, previous_position };
        self.output = Some(plan);
        Ok(Puzzle2dMoveNodePreparationStep::Complete { plan, progress: RetainedCloneProgress { copied_items: 1, copied_bytes: 4 * size_of::<f64>(), ..Default::default() } })
    }

    pub fn take(&mut self) -> Option<Puzzle2dMoveNodePlan> { if self.closing { return None; } let output = self.output.take(); if output.is_some() { self.spent = true; } output }

    pub fn begin_close(&mut self) { self.closing = true; self.lookup.begin_close(); }

    pub fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> {
        if self.output.is_some() { return Ok(size_of_val(&self.output)); }
        if !self.lookup.terminal_is_empty() { return self.lookup.next_close_copy_byte_demand(); }
        Ok((if self.source.is_empty(){&self.mutation}else{&self.source}).close_demand().copy_bytes)
    }
    pub fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> {
        if self.output.is_some() { return Ok(0); }
        if !self.lookup.terminal_is_empty() { return self.lookup.next_close_capacity_byte_demand(body); }
        Ok(0)
    }
    pub fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
        if self.output.is_some() { return Ok(0); }
        if !self.lookup.terminal_is_empty() { return self.lookup.next_close_release_byte_demand(); }
        Ok(0)
    }
    pub fn next_close_depth_demand(&self) -> Result<usize, ValueError> {
        if self.output.is_some() { return Ok(1); }
        if !self.lookup.terminal_is_empty() { return self.lookup.next_close_depth_demand(); }
        Ok((if self.source.is_empty(){&self.mutation}else{&self.source}).close_demand().depth)
    }
    pub fn close_step(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> OriginalRetirementPoll { let result = self.close_stepped(effect); RetirementPollExit::settle(effect, result) }

    fn close_stepped(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneStep, RetirementPollExit> {
        let grant = effect.grant();
        if !self.closing { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "move preparation closure was not started").into()); }
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if self.output.is_some() { let bytes=size_of_val(&self.output);if grant.maximum_copy_bytes<bytes||!pay(effect)?{return Ok(RetainedCloneStep::Progress(Default::default()));}self.output=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})); }
        if !self.lookup.terminal_is_empty() { return Ok(RetainedCloneStep::Progress(RetirementPollExit::resume(self.lookup.close_step(effect))?.progress())); }
        let progress=(if self.source.is_empty(){&mut self.mutation}else{&mut self.source}).close(effect)?;
        Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
    }

    pub fn terminal_is_empty(&self) -> bool { self.closing && self.output.is_none() && self.lookup.terminal_is_empty() && self.source.is_empty() && self.mutation.is_empty() }
}


