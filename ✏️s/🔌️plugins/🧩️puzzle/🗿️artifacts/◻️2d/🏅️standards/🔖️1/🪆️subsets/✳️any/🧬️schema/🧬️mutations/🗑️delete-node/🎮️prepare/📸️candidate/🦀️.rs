//! 📸️ Resolves native first-ID deletion targets before copying and retiring paged candidate owners.

use crate::standards::v1::subsets::any::schema::mutations::native_preparation_child::{Puzzle2dCloseAxis,close_child_demand,close_demand_methods};
use super::{DeleteNode, Puzzle2dDeleteNodePreparationCursor, Puzzle2dDeleteNodePreparationStep, Puzzle2dSnapshot};
use crate::{Puzzle2dEdge, Puzzle2dNode};
use semio_framework_value::{ValueError, ValueRefusalKind, list::{PagedList, PagedListEditCursor}, retirement::controlled::ControlledRetirement, retained_clone::{RetainedClone, RetainedCloneBinding, RetainedFieldCursor, RetainedCloneCursor, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneStep, paged::PagedUtf8BoundedOrdCursor, ordered_map::{BoundedOrdCursor, BoundedOrdGrant, BoundedOrdProgress, BoundedOrdStep}}};
use crate::standards::v1::subsets::any::schema::mutations::native_preparation_child::Puzzle2dPreparationChild;
use std::{mem::{ManuallyDrop, size_of}, ops::{Deref, DerefMut}};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Puzzle2dDeleteNodeDisposition { Changed, Missing, DuplicateRemovalTarget }

pub struct Puzzle2dDeleteNodeCandidate { pub disposition: Puzzle2dDeleteNodeDisposition, pub snapshot: Option<Puzzle2dSnapshot> }
pub struct Puzzle2dDeleteNodeCandidateCursor { state: ManuallyDrop<Puzzle2dDeleteNodeCandidateState> }

#[doc(hidden)]
pub struct Puzzle2dDeleteNodeCandidateState {
    source: Option<RetainedCloneBinding>,
    mutation: Option<RetainedCloneBinding>,
    preparation: Puzzle2dPreparationChild<Puzzle2dDeleteNodePreparationCursor>,
    comparison: PagedUtf8BoundedOrdCursor<{usize::MAX}>,
    snapshot_clone: RetainedFieldCursor<Puzzle2dSnapshot>,
    removals: Option<PagedList<usize, {usize::MAX}>>,
    removal_close: Option<ControlledRetirement<PagedList<usize, {usize::MAX}>>>,
    candidate: Option<Puzzle2dSnapshot>,
    candidate_close: Option<ControlledRetirement<Puzzle2dSnapshot>>,
    pending_node: Option<Puzzle2dNode>,
    node_close: Option<ControlledRetirement<Puzzle2dNode>>,
    pending_edge: Option<Puzzle2dEdge>,
    edge_close: Option<ControlledRetirement<Puzzle2dEdge>>,
    edit: Option<PagedListEditCursor>,
    pending_index: Option<usize>,
    node: Option<usize>,
    event: usize,
    search: usize,
    position: usize,
    remaining: usize,
    matched: bool,
    disposition: Puzzle2dDeleteNodeDisposition,
    phase: u8,
    closing: bool,
}

impl Deref for Puzzle2dDeleteNodeCandidateCursor { type Target = Puzzle2dDeleteNodeCandidateState; fn deref(&self) -> &Self::Target { &self.state } }
impl DerefMut for Puzzle2dDeleteNodeCandidateCursor { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.state } }

impl Default for Puzzle2dDeleteNodeCandidateCursor {
    fn default() -> Self { Self { state: ManuallyDrop::new(Puzzle2dDeleteNodeCandidateState { source: None, mutation: None, preparation: Default::default(), comparison: Default::default(), snapshot_clone: RetainedFieldCursor::<Puzzle2dSnapshot>::default(), removals: Some(PagedList::new()), removal_close: None, candidate: None, candidate_close: None, pending_node: None, node_close: None, pending_edge: None, edge_close: None, edit: None, pending_index: None, node: None, event: 0, search: 0, position: 0, remaining: 0, matched: false, disposition: Puzzle2dDeleteNodeDisposition::Missing, phase: 0, closing: false }) } }
}

impl Puzzle2dDeleteNodeCandidateCursor {
    pub fn advance(&mut self, source: RetainedCloneRef<'_, Puzzle2dSnapshot>, mutation: RetainedCloneRef<'_, DeleteNode>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.closing || self.phase == 23 { return Err(refusal("delete candidate is closing or spent")); }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        source.bind(&mut self.source)?;
        mutation.bind(&mut self.mutation)?;
        match self.phase {
            0 => {
                if let Some(step)=self.preparation.ensure(grant)? {return Ok(step);}
                let step = self.preparation.owner_mut().advance(source, mutation, grant)?;
                let progress = match step {
                    Puzzle2dDeleteNodePreparationStep::Pending(progress) => progress,
                    Puzzle2dDeleteNodePreparationStep::Node { index, progress } => { self.node = index;self.disposition = if index.is_some() { Puzzle2dDeleteNodeDisposition::Changed } else { Puzzle2dDeleteNodeDisposition::Missing };progress }
                    Puzzle2dDeleteNodePreparationStep::Edge { index, progress } => { self.event = index;self.search = 0;self.phase = 2;progress }
                    Puzzle2dDeleteNodePreparationStep::Complete(progress) => { self.preparation.begin_close();self.phase = 8;progress }
                };
                Ok(RetainedCloneStep::Progress(progress))
            }
            2 => {
                let (search, event) = (self.search, self.event);
                if search == source.get().edges.len() { return Err(refusal("delete cascade lost its original edge identifier")); }
                let left = source.project(1, |snapshot| &snapshot.edges.get(search).expect("immutable first-ID edge").id);
                let right = source.project(1, |snapshot| &snapshot.edges.get(event).expect("immutable cascade edge").id);
                match self.comparison.compare(left, right, BoundedOrdGrant { maximum_items: 1, maximum_bytes: grant.maximum_copy_bytes })? {
                    BoundedOrdStep::Progress(progress) => Ok(comparison_progress(progress)),
                    BoundedOrdStep::Complete { ordering, progress } => { self.matched = ordering == std::cmp::Ordering::Equal;self.comparison.begin_close();self.phase = 3;Ok(comparison_progress(progress)) }
                }
            }
            3 => {
                let step = self.comparison.close_step(grant)?;
                if self.comparison.terminal_is_empty() { self.comparison = Default::default();if self.matched { self.position = 0;self.phase = 4; } else { self.search += 1;self.phase = 2; } }
                Ok(close_progress(step))
            }
            4 => {
                let indices = self.removals.as_ref().ok_or_else(|| refusal("delete cascade lost its ordinal owner"))?;
                match indices.get(self.position).copied() {
                    Some(index) if index == self.search => { self.disposition = Puzzle2dDeleteNodeDisposition::DuplicateRemovalTarget;self.preparation.begin_close();self.phase = 8; }
                    Some(index) if index < self.search => self.position += 1,
                    _ => self.phase = 5,
                }
                Ok(payload_progress(0))
            }
            5 => {
                if grant.maximum_copy_bytes < size_of::<PagedListEditCursor>() + size_of::<usize>() { return Ok(RetainedCloneStep::Progress(Default::default())); }
                self.edit = Some(PagedListEditCursor::insert(self.position, self.removals.as_ref().unwrap().len()));self.pending_index = Some(self.search);self.phase = 6;
                Ok(payload_progress(size_of::<PagedListEditCursor>() + size_of::<usize>()))
            }
            6 => {
                let state = &mut **self;
                let indices = state.removals.as_mut().ok_or_else(|| refusal("delete sorting lost its ordinal owner"))?;
                let capacity = state.pending_index.is_some() && !indices.has_reserved_slot();
                let bytes = if capacity { grant.maximum_capacity_bytes } else { grant.maximum_copy_bytes };
                if capacity && indices.next_capacity_allocation_bytes(1).map_err(ValueError::from)?.is_some_and(|demand| demand > bytes) { return Ok(RetainedCloneStep::Progress(Default::default())); }
                let step = state.edit.as_mut().ok_or_else(|| refusal("delete sorting lost its ordered cursor"))?.step(indices, &mut state.pending_index, 1, bytes).map_err(ValueError::from)?;
                if step.complete { state.phase = 7; }
                Ok(edit_progress(step))
            }
            7 => { self.edit = None;self.phase = 0;Ok(payload_progress(0)) }
            8 => {
                let step = self.preparation.close_granted(grant)?;
                if matches!(step,RetainedCloneStep::Complete(_)) { self.phase = 9; }
                Ok(RetainedCloneStep::Progress(step.progress()))
            }
            9 => { self.phase = if self.disposition == Puzzle2dDeleteNodeDisposition::Changed { 10 } else { 21 };Ok(payload_progress(0)) }
            10 => {
                let step = self.snapshot_clone.advance(source, grant)?;
                if matches!(step, RetainedCloneStep::Complete(_)) { self.phase = 11; }
                Ok(RetainedCloneStep::Progress(step.progress()))
            }
            11 => {
                if grant.maximum_copy_bytes < size_of::<Puzzle2dSnapshot>() { return Ok(RetainedCloneStep::Progress(Default::default())); }
                self.candidate = Some(self.snapshot_clone.take().ok_or_else(|| refusal("delete snapshot clone completed without an owner"))?);self.snapshot_clone.begin_close();self.phase = 12;
                Ok(payload_progress(size_of::<Puzzle2dSnapshot>()))
            }
            12 => { let step = self.snapshot_clone.close_step(grant)?;if self.snapshot_clone.terminal_is_empty() { self.phase = 13; }Ok(RetainedCloneStep::Progress(step.progress())) }
            13 => {
                if grant.maximum_copy_bytes < size_of::<PagedListEditCursor>() { return Ok(RetainedCloneStep::Progress(Default::default())); }
                let index = self.node.ok_or_else(|| refusal("delete candidate lost its first node ordinal"))?;
                self.edit = Some(PagedListEditCursor::remove(index, self.candidate.as_ref().unwrap().nodes.len()));self.phase = 14;
                Ok(payload_progress(size_of::<PagedListEditCursor>()))
            }
            14 => {
                let state = &mut **self;
                let step = state.edit.as_mut().unwrap().step(&mut state.candidate.as_mut().unwrap().nodes, &mut state.pending_node, 1, grant.maximum_copy_bytes).map_err(ValueError::from)?;
                if step.complete { state.edit = None;state.phase = 15; }
                Ok(edit_progress(step))
            }
            15 => {
                if grant.maximum_copy_bytes < size_of::<Puzzle2dNode>() { return Ok(RetainedCloneStep::Progress(Default::default())); }
                match ControlledRetirement::new(self.pending_node.take().ok_or_else(|| refusal("delete node removal lost its owner"))?) { Ok(owner) => self.node_close = Some(owner), Err((error, owner)) => { self.pending_node = Some(owner);return Err(error); } }
                self.phase = 16;Ok(payload_progress(size_of::<Puzzle2dNode>()))
            }
            16 => {
                let owner = self.node_close.as_mut().unwrap();let step = owner.step(grant)?;
                if owner.terminal_is_empty() { self.node_close = None;self.remaining = self.removals.as_ref().unwrap().len();self.phase = 17; }
                Ok(RetainedCloneStep::Progress(step.progress()))
            }
            17 => {
                if self.remaining == 0 { self.phase = 21;return Ok(payload_progress(0)); }
                if grant.maximum_copy_bytes < size_of::<PagedListEditCursor>() { return Ok(RetainedCloneStep::Progress(Default::default())); }
                self.remaining -= 1;let index = *self.removals.as_ref().unwrap().get(self.remaining).unwrap();
                self.edit = Some(PagedListEditCursor::remove(index, self.candidate.as_ref().unwrap().edges.len()));self.phase = 18;
                Ok(payload_progress(size_of::<PagedListEditCursor>()))
            }
            18 => {
                let state = &mut **self;
                let step = state.edit.as_mut().unwrap().step(&mut state.candidate.as_mut().unwrap().edges, &mut state.pending_edge, 1, grant.maximum_copy_bytes).map_err(ValueError::from)?;
                if step.complete { state.edit = None;state.phase = 19; }
                Ok(edit_progress(step))
            }
            19 => {
                if grant.maximum_copy_bytes < size_of::<Puzzle2dEdge>() { return Ok(RetainedCloneStep::Progress(Default::default())); }
                match ControlledRetirement::new(self.pending_edge.take().ok_or_else(|| refusal("delete edge removal lost its owner"))?) { Ok(owner) => self.edge_close = Some(owner), Err((error, owner)) => { self.pending_edge = Some(owner);return Err(error); } }
                self.phase = 20;Ok(payload_progress(size_of::<Puzzle2dEdge>()))
            }
            20 => { let owner = self.edge_close.as_mut().unwrap();let step = owner.step(grant)?;if owner.terminal_is_empty() { self.edge_close = None;self.phase = 17; }Ok(RetainedCloneStep::Progress(step.progress())) }
            21 => {
                if grant.maximum_copy_bytes < size_of::<Puzzle2dDeleteNodeCandidate>() { return Ok(RetainedCloneStep::Progress(Default::default())); }
                self.phase = 22;Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<Puzzle2dDeleteNodeCandidate>(), retained_capacity_bytes: 0, released_bytes: 0 }))
            }
            22 => Ok(RetainedCloneStep::Complete(Default::default())),
            _ => Err(refusal("delete candidate has an unknown ownership phase")),
        }
    }

    pub fn take(&mut self) -> Option<Puzzle2dDeleteNodeCandidate> { if self.closing || self.phase != 22 { return None; }self.phase = 23;Some(Puzzle2dDeleteNodeCandidate { disposition: self.disposition, snapshot: self.candidate.take() }) }
    pub fn begin_close(&mut self) { self.closing = true;self.preparation.begin_close();self.comparison.begin_close();self.snapshot_clone.begin_close(); }

    fn close_demand(&self,axis:Puzzle2dCloseAxis)->Result<usize,ValueError>{
        if !self.preparation.terminal_is_empty(){return close_child_demand!(axis,self.preparation)}
        if !self.comparison.terminal_is_empty(){return match axis{Puzzle2dCloseAxis::Copy=>self.comparison.next_close_copy_byte_demand(),Puzzle2dCloseAxis::Capacity(body)=>self.comparison.next_close_capacity_byte_demand(body),Puzzle2dCloseAxis::Release=>self.comparison.next_close_release_byte_demand(),Puzzle2dCloseAxis::Depth=>BoundedOrdCursor::next_close_depth_demand(&self.comparison)}}
        if !self.snapshot_clone.terminal_is_empty(){return axis.retained::<Puzzle2dSnapshot,_>(&self.snapshot_clone)}
        if let Some(owner)=self.node_close.as_ref(){return axis.retirement(owner)}
        if self.pending_node.is_some(){return axis.inline::<Puzzle2dNode>()}
        if let Some(owner)=self.edge_close.as_ref(){return axis.retirement(owner)}
        if self.pending_edge.is_some(){return axis.inline::<Puzzle2dEdge>()}
        if let Some(owner)=self.candidate_close.as_ref(){return axis.retirement(owner)}
        if self.candidate.is_some(){return axis.inline::<Puzzle2dSnapshot>()}
        if let Some(owner)=self.removal_close.as_ref(){return axis.retirement(owner)}
        if self.removals.is_some(){return axis.inline::<PagedList<usize,{usize::MAX}>>()}
        if self.edit.is_some(){return axis.inline::<Option<PagedListEditCursor>>()}
        if self.pending_index.is_some(){return axis.inline::<Option<usize>>()}
        axis.binding(if self.source.is_some(){&self.source}else{&self.mutation})
    }

    close_demand_methods!(next_close_copy_byte_demand,next_close_capacity_byte_demand,next_close_release_byte_demand,next_close_depth_demand);

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if !self.closing { return Err(refusal("delete candidate closure was not started")); }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !self.preparation.terminal_is_empty() { return self.preparation.close_granted(grant); }
        if !self.comparison.terminal_is_empty() { return self.comparison.close_step(grant).map(close_progress); }
        if !self.snapshot_clone.terminal_is_empty() { return self.snapshot_clone.close_step(grant).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if let Some(owner) = self.node_close.as_mut() { let step = owner.step(grant)?;if owner.terminal_is_empty() { self.node_close = None; }return Ok(RetainedCloneStep::Progress(step.progress())); }
        if self.pending_node.is_some() {
            if grant.maximum_copy_bytes < size_of::<Puzzle2dNode>() { return Ok(RetainedCloneStep::Progress(Default::default())); }
            match ControlledRetirement::new(self.pending_node.take().unwrap()) { Ok(owner) => self.node_close = Some(owner), Err((error, owner)) => { self.pending_node = Some(owner);return Err(error); } }return Ok(payload_progress(size_of::<Puzzle2dNode>()));
        }
        if let Some(owner) = self.edge_close.as_mut() { let step = owner.step(grant)?;if owner.terminal_is_empty() { self.edge_close = None; }return Ok(RetainedCloneStep::Progress(step.progress())); }
        if self.pending_edge.is_some() {
            if grant.maximum_copy_bytes < size_of::<Puzzle2dEdge>() { return Ok(RetainedCloneStep::Progress(Default::default())); }
            match ControlledRetirement::new(self.pending_edge.take().unwrap()) { Ok(owner) => self.edge_close = Some(owner), Err((error, owner)) => { self.pending_edge = Some(owner);return Err(error); } }return Ok(payload_progress(size_of::<Puzzle2dEdge>()));
        }
        if let Some(owner) = self.candidate_close.as_mut() { let step = owner.step(grant)?;if owner.terminal_is_empty() { self.candidate_close = None; }return Ok(RetainedCloneStep::Progress(step.progress())); }
        if self.candidate.is_some() {
            if grant.maximum_copy_bytes < size_of::<Puzzle2dSnapshot>() { return Ok(RetainedCloneStep::Progress(Default::default())); }
            match ControlledRetirement::new(self.candidate.take().unwrap()) { Ok(owner) => self.candidate_close = Some(owner), Err((error, owner)) => { self.candidate = Some(owner);return Err(error); } }return Ok(payload_progress(size_of::<Puzzle2dSnapshot>()));
        }
        if let Some(owner) = self.removal_close.as_mut() { let step = owner.step(grant)?;if owner.terminal_is_empty() { self.removal_close = None; }return Ok(RetainedCloneStep::Progress(step.progress())); }
        if self.removals.is_some() {
            if grant.maximum_copy_bytes < size_of::<PagedList<usize, {usize::MAX}>>() { return Ok(RetainedCloneStep::Progress(Default::default())); }
            match ControlledRetirement::new(self.removals.take().unwrap()) { Ok(owner) => self.removal_close = Some(owner), Err((error, owner)) => { self.removals = Some(owner);return Err(error); } }return Ok(payload_progress(size_of::<PagedList<usize, {usize::MAX}>>()));
        }
        if self.edit.is_some() {let bytes=size_of::<Option<PagedListEditCursor>>();if grant.maximum_copy_bytes<bytes{return Ok(RetainedCloneStep::Progress(Default::default()));}self.edit=None;return Ok(payload_progress(bytes));}
        if self.pending_index.is_some() {let bytes=size_of::<Option<usize>>();if grant.maximum_copy_bytes<bytes{return Ok(RetainedCloneStep::Progress(Default::default()));}self.pending_index=None;return Ok(payload_progress(bytes));}
        let step = RetainedCloneBinding::close_one(if self.source.is_some(){&mut self.source}else{&mut self.mutation}, grant)?;
        Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { close_progress(step) })
    }

    pub fn terminal_is_empty(&self) -> bool { self.closing && self.preparation.terminal_is_empty() && self.comparison.terminal_is_empty() && self.snapshot_clone.terminal_is_empty() && self.removals.is_none() && self.removal_close.is_none() && self.candidate.is_none() && self.candidate_close.is_none() && self.pending_node.is_none() && self.node_close.is_none() && self.pending_edge.is_none() && self.edge_close.is_none() && self.edit.is_none() && self.pending_index.is_none() && self.source.is_none() && self.mutation.is_none() }
}

impl Drop for Puzzle2dDeleteNodeCandidateCursor { fn drop(&mut self) { let empty = self.terminal_is_empty();assert!(std::thread::panicking() || empty, "delete candidate abandoned before controlled closure");if empty { unsafe { ManuallyDrop::drop(&mut self.state); } } } }

fn payload_progress(bytes: usize) -> RetainedCloneStep { RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, retained_capacity_bytes: 0, released_bytes: 0 }) }
fn comparison_progress(progress: BoundedOrdProgress) -> RetainedCloneStep { RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: progress.compared_items, copied_bytes: progress.compared_bytes, retained_capacity_bytes: 0, released_bytes: 0 }) }
fn close_progress(step: RetainedCloneStep) -> RetainedCloneStep { RetainedCloneStep::Progress(step.progress()) }
fn edit_progress(step: semio_framework_value::list::PagedListEditStep) -> RetainedCloneStep { RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: step.moved_items, copied_bytes: step.progress.placed_bytes, retained_capacity_bytes: step.progress.allocated_bytes, released_bytes: step.progress.released_allocation_bytes }) }
fn refusal(message: &str) -> ValueError { ValueError::new(ValueRefusalKind::InvariantViolated, message) }

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
