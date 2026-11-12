//! 🗑️ Borrowed deletion streams first-node and cascade ordinals without owning source records.

use crate::standards::v1::subsets::any::schema::mutations::delete_node::DeleteNode;
use crate::{Puzzle2dSnapshot, standards::v1::subsets::any::schema::lookup::{Puzzle2dIdentifierComparison, Puzzle2dLookupCursor, Puzzle2dLookupScope, Puzzle2dLookupStep, pay, settled_step}};
use semio_framework_value::{OriginalRetirementPoll, RetainedAllocationEffect, RetirementPollExit, ValueError, ValueRefusalKind, retained_clone::{RetainedCloneBinding, RetainedCloneProgress, RetainedCloneRef, RetainedCloneStep}};

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
    comparison: Puzzle2dIdentifierComparison,
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
    pub fn advance(&mut self, source: RetainedCloneRef<'_, Puzzle2dSnapshot>, mutation: RetainedCloneRef<'_, DeleteNode>, effect: &mut RetainedAllocationEffect<'_>) -> Result<Puzzle2dDeleteNodePreparationStep, ValueError> {
        let grant = effect.grant();
        if self.closing { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "delete preparation is closing")); }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(Puzzle2dDeleteNodePreparationStep::Pending(Default::default())); }
        if let Some(progress)=source.bind(&mut self.source,effect).result?{return Ok(Puzzle2dDeleteNodePreparationStep::Pending(progress))}
        if let Some(progress)=mutation.bind(&mut self.mutation,effect).result?{return Ok(Puzzle2dDeleteNodePreparationStep::Pending(progress))}
        let item = RetainedCloneProgress { copied_items: 1, ..Default::default() };
        match self.phase {
            0 => match self.lookup.advance(source, mutation.project(1, |payload| &payload.id), effect)? {
                Puzzle2dLookupStep::Pending(progress) => Ok(Puzzle2dDeleteNodePreparationStep::Pending(progress)),
                Puzzle2dLookupStep::Complete { location, progress } => { self.node = location.map(|location| location.outer); self.lookup.take(); self.lookup.begin_close(); self.phase = 1; Ok(Puzzle2dDeleteNodePreparationStep::Pending(progress)) },
            },
            1 => { let step=settled_step(self.lookup.close_step(effect))?;if self.lookup.terminal_is_empty(){self.phase=2;}Ok(Puzzle2dDeleteNodePreparationStep::Pending(step.progress())) },
            2 => {
                let bytes = size_of::<Option<usize>>();
                if grant.maximum_copy_bytes < bytes || !pay(effect)? { return Ok(Puzzle2dDeleteNodePreparationStep::Pending(Default::default())); }
                self.phase = 3;
                Ok(Puzzle2dDeleteNodePreparationStep::Node { index: self.node, progress: RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() } })
            }
            3 => {
                if !pay(effect)? { return Ok(Puzzle2dDeleteNodePreparationStep::Pending(Default::default())); }
                if self.node.is_none_or(|index| source.get().nodes.get(index).expect("immutable deletion node").handles.is_empty()) || self.edge == source.get().edges.len() { self.phase = 8; }
                else { self.phase = 4; }
                Ok(Puzzle2dDeleteNodePreparationStep::Pending(item))
            }
            4 => {
                let node = self.node.expect("deletion lookup completed");
                if self.handle == source.get().nodes.get(node).expect("immutable deletion node").handles.len() { if !pay(effect)? { return Ok(Puzzle2dDeleteNodePreparationStep::Pending(Default::default())); } self.edge += 1; self.handle = 0; self.endpoint = false; self.phase = 3; return Ok(Puzzle2dDeleteNodePreparationStep::Pending(item)); }
                let (edge, handle, endpoint) = (self.edge, self.handle, self.endpoint);
                let left = source.project(1, |snapshot| { let edge = snapshot.edges.get(edge).expect("immutable deletion edge"); if endpoint { &edge.target } else { &edge.source } });
                let right = source.project(1, |snapshot| &snapshot.nodes.get(node).expect("immutable deletion node").handles.get(handle).expect("immutable deletion handle").id);
                let (ordering, progress) = self.comparison.compare(left, right, effect)?;
                let Some(ordering) = ordering else { return Ok(Puzzle2dDeleteNodePreparationStep::Pending(progress)); };
                self.matched = ordering == std::cmp::Ordering::Equal; self.phase = 5; Ok(Puzzle2dDeleteNodePreparationStep::Pending(progress))
            }
            5 => {let step=settled_step(self.comparison.close_step(effect))?;if self.comparison.terminal_is_empty(){self.comparison.reset();self.phase=if self.matched{7}else{6};}Ok(Puzzle2dDeleteNodePreparationStep::Pending(step.progress()))},
            6 => { if !pay(effect)? { return Ok(Puzzle2dDeleteNodePreparationStep::Pending(Default::default())); } if self.endpoint { self.handle += 1; self.endpoint = false; } else { self.endpoint = true; } self.phase = 4; Ok(Puzzle2dDeleteNodePreparationStep::Pending(item)) }
            7 => {
                let bytes = size_of::<usize>();
                if grant.maximum_copy_bytes < bytes || !pay(effect)? { return Ok(Puzzle2dDeleteNodePreparationStep::Pending(Default::default())); }
                let index = self.edge;
                self.edge += 1; self.handle = 0; self.endpoint = false; self.matched = false; self.phase = 3;
                Ok(Puzzle2dDeleteNodePreparationStep::Edge { index, progress: RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() } })
            }
            8 => { if !pay(effect)? { return Ok(Puzzle2dDeleteNodePreparationStep::Pending(Default::default())); } self.phase = 9; Ok(Puzzle2dDeleteNodePreparationStep::Complete(item)) }
            _ => Ok(Puzzle2dDeleteNodePreparationStep::Complete(Default::default())),
        }
    }

    pub fn begin_close(&mut self) { self.closing = true; self.lookup.begin_close(); }

    pub fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{if !self.lookup.terminal_is_empty(){return self.lookup.next_close_copy_byte_demand();}if !self.comparison.terminal_is_empty(){return self.comparison.next_close_copy_byte_demand();}RetainedCloneBinding::copy_demand(if self.source.is_some(){&self.source}else{&self.mutation})}
    pub fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{if !self.lookup.terminal_is_empty(){return self.lookup.next_close_capacity_byte_demand(body);}if !self.comparison.terminal_is_empty(){return self.comparison.next_close_capacity_byte_demand(body);}RetainedCloneBinding::capacity_demand(if self.source.is_some(){&self.source}else{&self.mutation},body)}
    pub fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{if !self.lookup.terminal_is_empty(){return self.lookup.next_close_release_byte_demand();}if !self.comparison.terminal_is_empty(){return self.comparison.next_close_release_byte_demand();}RetainedCloneBinding::release_demand(if self.source.is_some(){&self.source}else{&self.mutation})}
    pub fn next_close_depth_demand(&self)->Result<usize,ValueError>{if !self.lookup.terminal_is_empty(){return self.lookup.next_close_depth_demand();}if !self.comparison.terminal_is_empty(){return self.comparison.next_close_depth_demand();}RetainedCloneBinding::depth_demand(if self.source.is_some(){&self.source}else{&self.mutation})}
    pub fn close_step(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> OriginalRetirementPoll { let result = self.close_stepped(effect); RetirementPollExit::settle(effect, result) }

    fn close_stepped(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneStep, RetirementPollExit> {
        let grant = effect.grant();
        if !self.closing { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "delete preparation closure was not started").into()); }
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}if grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
        if !self.lookup.terminal_is_empty(){return Ok(RetainedCloneStep::Progress(RetirementPollExit::resume(self.lookup.close_step(effect))?.progress()));}
        if !self.comparison.terminal_is_empty(){return Ok(RetainedCloneStep::Progress(RetirementPollExit::resume(self.comparison.close_step(effect))?.progress()));}
        let step=RetirementPollExit::resume(RetainedCloneBinding::close_one(if self.source.is_some(){&mut self.source}else{&mut self.mutation},grant.maximum_copy_bytes,effect))?;Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})
    }

    pub fn terminal_is_empty(&self) -> bool { self.closing && self.lookup.terminal_is_empty() && self.comparison.terminal_is_empty() && self.source.is_none() && self.mutation.is_none() }
}



#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;

#[path = "📸️candidate/🦀️.rs"]
pub mod candidate;

impl crate::standards::v1::subsets::any::schema::preparation::native_preparation_child::Puzzle2dNativePreparation for Puzzle2dDeleteNodePreparationCursor {
    fn begin_close(&mut self){Puzzle2dDeleteNodePreparationCursor::begin_close(self);}
    fn close_step(&mut self,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->semio_framework_value::OriginalRetirementPoll{Puzzle2dDeleteNodePreparationCursor::close_step(self,effect)}
    fn terminal_is_empty(&self)->bool{Puzzle2dDeleteNodePreparationCursor::terminal_is_empty(self)}
    fn next_close_copy_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Puzzle2dDeleteNodePreparationCursor::next_close_copy_byte_demand(self)}
    fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,semio_framework_value::ValueError>{Puzzle2dDeleteNodePreparationCursor::next_close_capacity_byte_demand(self,body)}
    fn next_close_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Puzzle2dDeleteNodePreparationCursor::next_close_release_byte_demand(self)}
    fn next_close_depth_demand(&self)->Result<usize,semio_framework_value::ValueError>{Puzzle2dDeleteNodePreparationCursor::next_close_depth_demand(self)}
}
