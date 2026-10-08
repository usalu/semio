//! 🎮️ Borrowed move semantics produce a scalar plan before inverse or diagnostic ownership.

use super::MoveNode;
use crate::{Puzzle2dSnapshot, standards::v1::subsets::any::schema::snapshot::lookup::{Puzzle2dLookupCursor, Puzzle2dLookupScope, Puzzle2dLookupStep}};
use semio_framework_value::{SnapshotRetirementStep, ValueError, ValueRefusalKind, retained_clone::{RetainedCloneBinding, RetainedCloneRef, ordered_map::{BoundedOrdGrant, BoundedOrdProgress}}};

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
pub enum Puzzle2dMoveNodePreparationStep { Pending(BoundedOrdProgress), Complete { plan: Puzzle2dMoveNodePlan, progress: BoundedOrdProgress } }

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
    pub fn advance(&mut self, source: RetainedCloneRef<'_, Puzzle2dSnapshot>, mutation: RetainedCloneRef<'_, MoveNode>, grant: BoundedOrdGrant) -> Result<Puzzle2dMoveNodePreparationStep, ValueError> {
        if self.closing || self.spent { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "move preparation is closing or spent")); }
        if grant.maximum_items == 0 { return Ok(Puzzle2dMoveNodePreparationStep::Pending(BoundedOrdProgress::default())); }
        source.bind(&mut self.source)?;
        mutation.bind(&mut self.mutation)?;
        if let Some(plan) = self.output { return Ok(Puzzle2dMoveNodePreparationStep::Complete { plan, progress: BoundedOrdProgress::default() }); }
        if self.phase == 0 {
            return match self.lookup.advance(source, mutation.project(1, |payload| &payload.id), grant)? {
                Puzzle2dLookupStep::Pending(progress) => Ok(Puzzle2dMoveNodePreparationStep::Pending(progress)),
                Puzzle2dLookupStep::Complete { location, progress } => { self.node_index = location.map(|location| location.outer); self.lookup.take(); self.phase = 1; Ok(Puzzle2dMoveNodePreparationStep::Pending(progress)) },
            };
        }
        if self.phase == 1 || self.phase == 2 {
            if grant.maximum_bytes < std::mem::size_of::<f64>() { return Ok(Puzzle2dMoveNodePreparationStep::Pending(BoundedOrdProgress::default())); }
            let (value, disposition) = if self.phase == 1 { (mutation.get().new_x, Puzzle2dMoveNodeDisposition::NonfiniteX) } else { (mutation.get().new_y, Puzzle2dMoveNodeDisposition::NonfiniteY) };
            if self.nonfinite.is_none() && !value.is_finite() { self.nonfinite = Some(disposition); }
            self.phase += 1;
            return Ok(Puzzle2dMoveNodePreparationStep::Pending(BoundedOrdProgress { compared_items: 1, compared_bytes: std::mem::size_of::<f64>() }));
        }
        if grant.maximum_bytes < 4 * std::mem::size_of::<f64>() { return Ok(Puzzle2dMoveNodePreparationStep::Pending(BoundedOrdProgress::default())); }
        let previous_position = self.node_index.map(|index| { let node = source.get().nodes.get(index).expect("immutable move preparation node"); [node.x, node.y] });
        let disposition = self.nonfinite.unwrap_or_else(|| match previous_position { None => Puzzle2dMoveNodeDisposition::TargetMissing, Some([x,y]) if x == mutation.get().new_x && y == mutation.get().new_y => Puzzle2dMoveNodeDisposition::NoOp, Some(_) => Puzzle2dMoveNodeDisposition::Changed });
        let plan = Puzzle2dMoveNodePlan { disposition, node_index: self.node_index, previous_position };
        self.output = Some(plan);
        Ok(Puzzle2dMoveNodePreparationStep::Complete { plan, progress: BoundedOrdProgress { compared_items: 1, compared_bytes: 4 * std::mem::size_of::<f64>() } })
    }

    pub fn take(&mut self) -> Option<Puzzle2dMoveNodePlan> { if self.closing { return None; } let output = self.output.take(); if output.is_some() { self.spent = true; } output }

    pub fn begin_close(&mut self) { self.closing = true; self.lookup.begin_close(); }

    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, ValueError> {
        if !self.closing { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "move preparation closure was not started")); }
        if self.output.is_some() { if maximum_items == 0 { return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); } self.output = None; return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }); }
        let step = self.lookup.close_step(maximum_items, maximum_bytes)?;
        if step != SnapshotRetirementStep::Complete { return Ok(step); }
        let step = RetainedCloneBinding::close_one(&mut self.source, maximum_items)?;
        if step != SnapshotRetirementStep::Complete { return Ok(step); }
        RetainedCloneBinding::close_one(&mut self.mutation, maximum_items)
    }

    pub fn terminal_is_empty(&self) -> bool { self.closing && self.output.is_none() && self.lookup.terminal_is_empty() && self.source.is_none() && self.mutation.is_none() }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
