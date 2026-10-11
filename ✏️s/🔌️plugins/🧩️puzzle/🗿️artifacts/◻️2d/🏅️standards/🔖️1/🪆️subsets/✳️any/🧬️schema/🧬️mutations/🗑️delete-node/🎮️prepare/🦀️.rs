//! 🗑️ Borrowed deletion streams first-node and cascade ordinals without owning source records.

use super::DeleteNode;
use crate::{Puzzle2dSnapshot, standards::v1::subsets::any::schema::snapshot::lookup::{Puzzle2dLookupCursor, Puzzle2dLookupScope, Puzzle2dLookupStep}};
use semio_framework_value::{ValueError, ValueRefusalKind, retained_clone::{RetainedCloneBinding, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneStep, paged::PagedUtf8BoundedOrdCursor, ordered_map::{BoundedOrdCursor, BoundedOrdGrant, BoundedOrdStep}}};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Puzzle2dDeleteNodePreparationStep {
    Pending(RetainedCloneProgress),
    Node { index: Option<usize>, progress: RetainedCloneProgress },
    Edge { index: usize, progress: RetainedCloneProgress },
    Complete(RetainedCloneProgress),
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
    pub fn advance(&mut self, source: RetainedCloneRef<'_, Puzzle2dSnapshot>, mutation: RetainedCloneRef<'_, DeleteNode>, grant: RetainedCloneGrant) -> Result<Puzzle2dDeleteNodePreparationStep, ValueError> {
        if self.closing { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "delete preparation is closing")); }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(Puzzle2dDeleteNodePreparationStep::Pending(Default::default())); }
        if let Some(progress)=source.bind(&mut self.source,grant)?{return Ok(Puzzle2dDeleteNodePreparationStep::Pending(progress))}
        if let Some(progress)=mutation.bind(&mut self.mutation,grant)?{return Ok(Puzzle2dDeleteNodePreparationStep::Pending(progress))}
        let item = RetainedCloneProgress { copied_items: 1, ..Default::default() };
        match self.phase {
            0 => match self.lookup.advance(source, mutation.project(1, |payload| &payload.id), grant)? {
                Puzzle2dLookupStep::Pending(progress) => Ok(Puzzle2dDeleteNodePreparationStep::Pending(progress)),
                Puzzle2dLookupStep::Complete { location, progress } => { self.node = location.map(|location| location.outer); self.lookup.take(); self.lookup.begin_close(); self.phase = 1; Ok(Puzzle2dDeleteNodePreparationStep::Pending(progress)) },
            },
            1 => { let step=self.lookup.close_step(grant)?;if self.lookup.terminal_is_empty(){self.phase=2;}Ok(Puzzle2dDeleteNodePreparationStep::Pending(step.progress())) },
            2 => {
                let bytes = std::mem::size_of::<Option<usize>>();
                if grant.maximum_copy_bytes < bytes { return Ok(Puzzle2dDeleteNodePreparationStep::Pending(Default::default())); }
                self.phase = 3;
                Ok(Puzzle2dDeleteNodePreparationStep::Node { index: self.node, progress: RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() } })
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
                match self.comparison.compare(left, right, BoundedOrdGrant { maximum_items:grant.maximum_items,maximum_bytes:grant.maximum_copy_bytes }, grant)? {
                    BoundedOrdStep::Authority(progress) => Ok(Puzzle2dDeleteNodePreparationStep::Pending(progress)),
                    BoundedOrdStep::Progress(progress) => Ok(Puzzle2dDeleteNodePreparationStep::Pending(RetainedCloneProgress{copied_items:progress.compared_items,copied_bytes:progress.compared_bytes,..Default::default()})),
                    BoundedOrdStep::Complete { ordering, progress } => { self.matched = ordering == std::cmp::Ordering::Equal; self.comparison.begin_close(); self.phase = 5; Ok(Puzzle2dDeleteNodePreparationStep::Pending(RetainedCloneProgress{copied_items:progress.compared_items,copied_bytes:progress.compared_bytes,..Default::default()})) },
                }
            }
            5 => {let step=self.comparison.close_step(grant)?;if self.comparison.terminal_is_empty(){self.comparison=Default::default();self.phase=if self.matched{7}else{6};}Ok(Puzzle2dDeleteNodePreparationStep::Pending(step.progress()))},
            6 => { if self.endpoint { self.handle += 1; self.endpoint = false; } else { self.endpoint = true; } self.phase = 4; Ok(Puzzle2dDeleteNodePreparationStep::Pending(item)) }
            7 => {
                let bytes = std::mem::size_of::<usize>();
                if grant.maximum_copy_bytes < bytes { return Ok(Puzzle2dDeleteNodePreparationStep::Pending(Default::default())); }
                let index = self.edge;
                self.edge += 1; self.handle = 0; self.endpoint = false; self.matched = false; self.phase = 3;
                Ok(Puzzle2dDeleteNodePreparationStep::Edge { index, progress: RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() } })
            }
            8 => { self.phase = 9; Ok(Puzzle2dDeleteNodePreparationStep::Complete(item)) }
            _ => Ok(Puzzle2dDeleteNodePreparationStep::Complete(Default::default())),
        }
    }

    pub fn begin_close(&mut self) { self.closing = true; self.lookup.begin_close(); self.comparison.begin_close(); }

    pub fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{if !self.lookup.terminal_is_empty(){return self.lookup.next_close_copy_byte_demand();}if !self.comparison.terminal_is_empty(){return self.comparison.next_close_copy_byte_demand();}RetainedCloneBinding::copy_demand(if self.source.is_some(){&self.source}else{&self.mutation})}
    pub fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{if !self.lookup.terminal_is_empty(){return self.lookup.next_close_capacity_byte_demand(body);}if !self.comparison.terminal_is_empty(){return self.comparison.next_close_capacity_byte_demand(body);}RetainedCloneBinding::capacity_demand(if self.source.is_some(){&self.source}else{&self.mutation},body)}
    pub fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{if !self.lookup.terminal_is_empty(){return self.lookup.next_close_release_byte_demand();}if !self.comparison.terminal_is_empty(){return self.comparison.next_close_release_byte_demand();}RetainedCloneBinding::release_demand(if self.source.is_some(){&self.source}else{&self.mutation})}
    pub fn next_close_depth_demand(&self)->Result<usize,ValueError>{if !self.lookup.terminal_is_empty(){return self.lookup.next_close_depth_demand();}if !self.comparison.terminal_is_empty(){return BoundedOrdCursor::next_close_depth_demand(&self.comparison);}RetainedCloneBinding::depth_demand(if self.source.is_some(){&self.source}else{&self.mutation})}
    pub fn close_step(&mut self, grant:RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if !self.closing { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "delete preparation closure was not started")); }
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}if grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
        if !self.lookup.terminal_is_empty(){return self.lookup.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()));}
        if !self.comparison.terminal_is_empty(){return self.comparison.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()));}
        let step=RetainedCloneBinding::close_one(if self.source.is_some(){&mut self.source}else{&mut self.mutation},grant)?;Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})
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
