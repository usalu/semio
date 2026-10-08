//! 🎮️ Borrowed move semantics produce a scalar plan before inverse or diagnostic ownership.

use super::MoveNode;
use crate::{Puzzle2dSnapshot, standards::v1::subsets::any::schema::snapshot::lookup::{Puzzle2dLookupCursor, Puzzle2dLookupScope, Puzzle2dLookupStep}};
use semio_framework_value::{ValueError, ValueRefusalKind, retained_clone::{RetainedCloneBinding, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneStep}};

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
    source: Option<RetainedCloneBinding>,
    mutation: Option<RetainedCloneBinding>,
    lookup: Puzzle2dLookupCursor,
    node_index: Option<usize>,
    nonfinite: Option<Puzzle2dMoveNodeDisposition>,
    phase: u8,
    output: Option<Puzzle2dMoveNodePlan>,
    spent: bool,
    closing: bool,
}

impl Default for Puzzle2dMoveNodePreparationCursor {
    fn default() -> Self { Self { source: None, mutation: None, lookup: Puzzle2dLookupCursor::new(Puzzle2dLookupScope::Node), node_index: None, nonfinite: None, phase: 0, output: None, spent: false, closing: false } }
}

impl Puzzle2dMoveNodePreparationCursor {
    pub fn advance(&mut self, source: RetainedCloneRef<'_, Puzzle2dSnapshot>, mutation: RetainedCloneRef<'_, MoveNode>, grant: RetainedCloneGrant) -> Result<Puzzle2dMoveNodePreparationStep, ValueError> {
        if self.closing || self.spent { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "move preparation is closing or spent")); }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(Puzzle2dMoveNodePreparationStep::Pending(Default::default())); }
        source.bind(&mut self.source)?;
        mutation.bind(&mut self.mutation)?;
        if let Some(plan) = self.output { return Ok(Puzzle2dMoveNodePreparationStep::Complete { plan, progress: Default::default() }); }
        if self.phase == 0 {
            return match self.lookup.advance(source, mutation.project(1, |payload| &payload.id), grant)? {
                Puzzle2dLookupStep::Pending(progress) => Ok(Puzzle2dMoveNodePreparationStep::Pending(progress)),
                Puzzle2dLookupStep::Complete { location, progress } => { self.node_index = location.map(|location| location.outer); self.lookup.take(); self.phase = 1; Ok(Puzzle2dMoveNodePreparationStep::Pending(progress)) },
            };
        }
        if self.phase == 1 || self.phase == 2 {
            if grant.maximum_copy_bytes < std::mem::size_of::<f64>() { return Ok(Puzzle2dMoveNodePreparationStep::Pending(Default::default())); }
            let (value, disposition) = if self.phase == 1 { (mutation.get().new_x, Puzzle2dMoveNodeDisposition::NonfiniteX) } else { (mutation.get().new_y, Puzzle2dMoveNodeDisposition::NonfiniteY) };
            if self.nonfinite.is_none() && !value.is_finite() { self.nonfinite = Some(disposition); }
            self.phase += 1;
            return Ok(Puzzle2dMoveNodePreparationStep::Pending(RetainedCloneProgress { copied_items: 1, copied_bytes: std::mem::size_of::<f64>(), ..Default::default() }));
        }
        if grant.maximum_copy_bytes < 4 * std::mem::size_of::<f64>() { return Ok(Puzzle2dMoveNodePreparationStep::Pending(Default::default())); }
        let previous_position = self.node_index.map(|index| { let node = source.get().nodes.get(index).expect("immutable move preparation node"); [node.x, node.y] });
        let disposition = self.nonfinite.unwrap_or_else(|| match previous_position { None => Puzzle2dMoveNodeDisposition::TargetMissing, Some([x,y]) if x == mutation.get().new_x && y == mutation.get().new_y => Puzzle2dMoveNodeDisposition::NoOp, Some(_) => Puzzle2dMoveNodeDisposition::Changed });
        let plan = Puzzle2dMoveNodePlan { disposition, node_index: self.node_index, previous_position };
        self.output = Some(plan);
        Ok(Puzzle2dMoveNodePreparationStep::Complete { plan, progress: RetainedCloneProgress { copied_items: 1, copied_bytes: 4 * std::mem::size_of::<f64>(), ..Default::default() } })
    }

    pub fn take(&mut self) -> Option<Puzzle2dMoveNodePlan> { if self.closing { return None; } let output = self.output.take(); if output.is_some() { self.spent = true; } output }

    pub fn begin_close(&mut self) { self.closing = true; self.lookup.begin_close(); }

    pub fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> {
        if self.output.is_some() { return Ok(std::mem::size_of_val(&self.output)); }
        if !self.lookup.terminal_is_empty() { return self.lookup.next_close_copy_byte_demand(); }
        RetainedCloneBinding::copy_demand(if self.source.is_some() { &self.source } else { &self.mutation })
    }
    pub fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> {
        if self.output.is_some() { return Ok(0); }
        if !self.lookup.terminal_is_empty() { return self.lookup.next_close_capacity_byte_demand(body); }
        RetainedCloneBinding::capacity_demand(if self.source.is_some() { &self.source } else { &self.mutation }, body)
    }
    pub fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
        if self.output.is_some() { return Ok(0); }
        if !self.lookup.terminal_is_empty() { return self.lookup.next_close_release_byte_demand(); }
        RetainedCloneBinding::release_demand(if self.source.is_some() { &self.source } else { &self.mutation })
    }
    pub fn next_close_depth_demand(&self) -> Result<usize, ValueError> {
        if self.output.is_some() { return Ok(1); }
        if !self.lookup.terminal_is_empty() { return self.lookup.next_close_depth_demand(); }
        RetainedCloneBinding::depth_demand(if self.source.is_some() { &self.source } else { &self.mutation })
    }
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if !self.closing { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "move preparation closure was not started")); }
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if self.output.is_some() { let bytes=std::mem::size_of_val(&self.output);if grant.maximum_copy_bytes<bytes{return Ok(RetainedCloneStep::Progress(Default::default()));}self.output=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})); }
        if !self.lookup.terminal_is_empty() { return self.lookup.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress())); }
        let step=RetainedCloneBinding::close_one(if self.source.is_some(){&mut self.source}else{&mut self.mutation},grant)?;
        Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})
    }

    pub fn terminal_is_empty(&self) -> bool { self.closing && self.output.is_none() && self.lookup.terminal_is_empty() && self.source.is_none() && self.mutation.is_none() }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
