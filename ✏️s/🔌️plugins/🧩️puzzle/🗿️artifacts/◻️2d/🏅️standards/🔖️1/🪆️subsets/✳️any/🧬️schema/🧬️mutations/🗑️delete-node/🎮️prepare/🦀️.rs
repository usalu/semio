//! 🗑️ Borrowed deletion streams first-node and cascade ordinals without owning source records.

use super::DeleteNode;
use crate::{Puzzle2dSnapshot, standards::v1::subsets::any::schema::snapshot::lookup::{Puzzle2dLookupCursor, Puzzle2dLookupScope, Puzzle2dLookupStep}};
use semio_framework_value::{SnapshotRetirementStep, ValueError, ValueRefusalKind, retained_clone::{RetainedCloneBinding, RetainedCloneRef, paged::PagedUtf8BoundedOrdCursor, ordered_map::{BoundedOrdCursor, BoundedOrdGrant, BoundedOrdProgress, BoundedOrdStep}}};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Puzzle2dDeleteNodePreparationStep {
    Pending(BoundedOrdProgress),
    Node { index: Option<usize>, progress: BoundedOrdProgress },
    Edge { index: usize, progress: BoundedOrdProgress },
    Complete(BoundedOrdProgress),
}

pub struct Puzzle2dDeleteNodePreparationCursor {
    source: Option<RetainedCloneBinding>,
    mutation: Option<RetainedCloneBinding>,
    lookup: Puzzle2dLookupCursor,
    comparison: PagedUtf8BoundedOrdCursor<{usize::MAX}>,
    node: Option<usize>,
    edge: usize,
    handle: usize,
    endpoint: bool,
    matched: bool,
    phase: u8,
    closing: bool,
}

impl Default for Puzzle2dDeleteNodePreparationCursor {
    fn default() -> Self { Self { source: None, mutation: None, lookup: Puzzle2dLookupCursor::new(Puzzle2dLookupScope::Node), comparison: Default::default(), node: None, edge: 0, handle: 0, endpoint: false, matched: false, phase: 0, closing: false } }
}

impl Puzzle2dDeleteNodePreparationCursor {
    pub fn advance(&mut self, source: RetainedCloneRef<'_, Puzzle2dSnapshot>, mutation: RetainedCloneRef<'_, DeleteNode>, grant: BoundedOrdGrant) -> Result<Puzzle2dDeleteNodePreparationStep, ValueError> {
        if self.closing { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "delete preparation is closing")); }
        if grant.maximum_items == 0 { return Ok(Puzzle2dDeleteNodePreparationStep::Pending(BoundedOrdProgress::default())); }
        source.bind(&mut self.source)?;
        mutation.bind(&mut self.mutation)?;
        let item = BoundedOrdProgress { compared_items: 1, compared_bytes: 0 };
        match self.phase {
            0 => match self.lookup.advance(source, mutation.project(1, |payload| &payload.id), grant)? {
                Puzzle2dLookupStep::Pending(progress) => Ok(Puzzle2dDeleteNodePreparationStep::Pending(progress)),
                Puzzle2dLookupStep::Complete { location, progress } => { self.node = location.map(|location| location.outer); self.lookup.take(); self.lookup.begin_close(); self.phase = 1; Ok(Puzzle2dDeleteNodePreparationStep::Pending(progress)) },
            },
            1 => match self.lookup.close_step(grant.maximum_items, grant.maximum_bytes)? {
                SnapshotRetirementStep::Complete => { self.phase = 2; Ok(Puzzle2dDeleteNodePreparationStep::Pending(item)) },
                SnapshotRetirementStep::Pending { released_items, released_bytes } => Ok(Puzzle2dDeleteNodePreparationStep::Pending(BoundedOrdProgress { compared_items: released_items, compared_bytes: released_bytes })),
                SnapshotRetirementStep::Blocked => Ok(Puzzle2dDeleteNodePreparationStep::Pending(BoundedOrdProgress::default())),
            },
            2 => {
                let bytes = std::mem::size_of::<Option<usize>>();
                if grant.maximum_bytes < bytes { return Ok(Puzzle2dDeleteNodePreparationStep::Pending(BoundedOrdProgress::default())); }
                self.phase = 3;
                Ok(Puzzle2dDeleteNodePreparationStep::Node { index: self.node, progress: BoundedOrdProgress { compared_items: 1, compared_bytes: bytes } })
            }
            3 => {
                if self.node.is_none_or(|index| source.get().nodes.get(index).expect("immutable deletion node").handles.is_empty()) || self.edge == source.get().edges.len() { self.phase = 8; }
                else { self.phase = 4; }
                Ok(Puzzle2dDeleteNodePreparationStep::Pending(item))
            }
            4 => {
                let node = self.node.expect("deletion lookup completed");
                if self.handle == source.get().nodes.get(node).expect("immutable deletion node").handles.len() { self.edge += 1; self.handle = 0; self.endpoint = false; self.phase = 3; return Ok(Puzzle2dDeleteNodePreparationStep::Pending(item)); }
                let (edge, handle, endpoint) = (self.edge, self.handle, self.endpoint);
                let left = source.project(1, |snapshot| { let edge = snapshot.edges.get(edge).expect("immutable deletion edge"); if endpoint { &edge.target } else { &edge.source } });
                let right = source.project(1, |snapshot| &snapshot.nodes.get(node).expect("immutable deletion node").handles.get(handle).expect("immutable deletion handle").id);
                match self.comparison.compare(left, right, grant)? {
                    BoundedOrdStep::Progress(progress) => Ok(Puzzle2dDeleteNodePreparationStep::Pending(progress)),
                    BoundedOrdStep::Complete { ordering, progress } => { self.matched = ordering == std::cmp::Ordering::Equal; self.comparison.begin_close(); self.phase = 5; Ok(Puzzle2dDeleteNodePreparationStep::Pending(progress)) },
                }
            }
            5 => match self.comparison.close_step(grant.maximum_items, grant.maximum_bytes)? {
                SnapshotRetirementStep::Complete => { self.comparison = Default::default(); self.phase = if self.matched { 7 } else { 6 }; Ok(Puzzle2dDeleteNodePreparationStep::Pending(item)) },
                SnapshotRetirementStep::Pending { released_items, released_bytes } => Ok(Puzzle2dDeleteNodePreparationStep::Pending(BoundedOrdProgress { compared_items: released_items, compared_bytes: released_bytes })),
                SnapshotRetirementStep::Blocked => Ok(Puzzle2dDeleteNodePreparationStep::Pending(BoundedOrdProgress::default())),
            },
            6 => { if self.endpoint { self.handle += 1; self.endpoint = false; } else { self.endpoint = true; } self.phase = 4; Ok(Puzzle2dDeleteNodePreparationStep::Pending(item)) }
            7 => {
                let bytes = std::mem::size_of::<usize>();
                if grant.maximum_bytes < bytes { return Ok(Puzzle2dDeleteNodePreparationStep::Pending(BoundedOrdProgress::default())); }
                let index = self.edge;
                self.edge += 1; self.handle = 0; self.endpoint = false; self.matched = false; self.phase = 3;
                Ok(Puzzle2dDeleteNodePreparationStep::Edge { index, progress: BoundedOrdProgress { compared_items: 1, compared_bytes: bytes } })
            }
            8 => { self.phase = 9; Ok(Puzzle2dDeleteNodePreparationStep::Complete(item)) }
            _ => Ok(Puzzle2dDeleteNodePreparationStep::Complete(BoundedOrdProgress::default())),
        }
    }

    pub fn begin_close(&mut self) { self.closing = true; self.lookup.begin_close(); self.comparison.begin_close(); }

    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, ValueError> {
        if !self.closing { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "delete preparation closure was not started")); }
        let step = self.lookup.close_step(maximum_items, maximum_bytes)?;
        if step != SnapshotRetirementStep::Complete { return Ok(step); }
        let step = self.comparison.close_step(maximum_items, maximum_bytes)?;
        if step != SnapshotRetirementStep::Complete { return Ok(step); }
        let step = RetainedCloneBinding::close_one(&mut self.source, maximum_items)?;
        if step != SnapshotRetirementStep::Complete { return Ok(step); }
        RetainedCloneBinding::close_one(&mut self.mutation, maximum_items)
    }

    pub fn terminal_is_empty(&self) -> bool { self.closing && self.lookup.terminal_is_empty() && self.comparison.terminal_is_empty() && self.source.is_none() && self.mutation.is_none() }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;

#[path = "📸️candidate/🦀️.rs"]
pub mod candidate;
