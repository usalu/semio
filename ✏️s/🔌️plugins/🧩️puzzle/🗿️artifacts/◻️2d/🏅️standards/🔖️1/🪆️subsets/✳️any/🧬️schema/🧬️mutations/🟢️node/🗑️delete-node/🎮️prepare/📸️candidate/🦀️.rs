//! 📸️ Resolves native first-ID deletion targets before copying and retiring paged candidate owners.

use crate::standards::v1::subsets::any::schema::preparation::native_preparation_child::{Puzzle2dCloseAxis, cursor_advance, cursor_retire, cursor_take, edit_progress, edit_step, pay, settled_step};
use crate::standards::v1::subsets::any::schema::lookup::borrowed::{BorrowedComparison,BorrowedIdentity};
use super::{Puzzle2dDeleteNodePreparationCursor, Puzzle2dDeleteNodePreparationStep};
use crate::standards::v1::subsets::any::schema::mutations::DeleteNode;
use crate::Puzzle2dSnapshot;
use {crate::standards::v1::subsets::any::schema::records::Puzzle2dEdge, crate::standards::v1::subsets::any::schema::records::Puzzle2dNode};
use semio_framework_value::{OriginalRetirementPoll, RetainedAllocationEffect, RetirementDemand, RetirementPollExit, ValueError, retained_clone::{RetainedCloneClose, RetainedFieldCursor, RetainedCloneCursor, RetainedCloneProgress, RetainedCloneRef, RetainedCloneStep}};
use semio_framework_value_persistent::{list::PagedList, list::PagedListEditCursor};
use crate::standards::v1::subsets::any::schema::preparation::native_preparation_child::Puzzle2dPreparationChild;
use std::{mem::{ManuallyDrop, size_of}, ops::{Deref, DerefMut}};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Puzzle2dDeleteNodeDisposition { Changed, Missing, DuplicateRemovalTarget }

pub struct Puzzle2dDeleteNodeCandidate { pub disposition: Puzzle2dDeleteNodeDisposition, pub snapshot: Option<Puzzle2dSnapshot> }
pub struct Puzzle2dDeleteNodeCandidateCursor { state: ManuallyDrop<Puzzle2dDeleteNodeCandidateState> }

enum Removed {
    Node(Option<Puzzle2dNode>),
    Edge(Option<Puzzle2dEdge>),
}

impl Removed {
    fn is_empty(&self)->bool{
        match self{Self::Node(node)=>node.is_none(),Self::Edge(edge)=>edge.is_none()}
    }

    fn quote(&self,axis:Puzzle2dCloseAxis,retirement:&RetainedCloneClose)->Result<usize,ValueError>{
        match self{Self::Node(node)=>axis.owned::<Puzzle2dNode>(retirement,node.is_some()),Self::Edge(edge)=>axis.owned::<Puzzle2dEdge>(retirement,edge.is_some())}
    }

    #[cfg(feature="testkit")]
    fn demand(&self,body:usize,retirement:&RetainedCloneClose)->Result<RetirementDemand,ValueError>{
        match self{
            Self::Node(node)=>retirement.next_owner_demands::<Puzzle2dNode>(node.is_some(),&None,body),
            Self::Edge(edge)=>retirement.next_owner_demands::<Puzzle2dEdge>(edge.is_some(),&None,body),
        }
    }

    fn retire(&mut self,retirement:&mut RetainedCloneClose,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError>{
        let turn=match self{Self::Node(node)=>retirement.begin_granted(node,effect),Self::Edge(edge)=>retirement.begin_granted(edge,effect)};
        Ok(close_progress(turn.result?.unwrap_or(RetainedCloneStep::Progress(Default::default()))))
    }
}

#[doc(hidden)]
pub struct Puzzle2dDeleteNodeCandidateState {
    source: BorrowedIdentity,
    mutation: BorrowedIdentity,
    preparation: Puzzle2dPreparationChild<Puzzle2dDeleteNodePreparationCursor>,
    comparison: BorrowedComparison,
    snapshot_clone: RetainedFieldCursor<Puzzle2dSnapshot>,
    removals: Option<PagedList<usize, {usize::MAX}>>,
    candidate: Option<Puzzle2dSnapshot>,
    removed: Removed,
    retirement: RetainedCloneClose,
    edit: Option<PagedListEditCursor>,
    pending_index: Option<usize>,
    node: Option<usize>,
    event: usize,
    search: usize,
    position: usize,
    remaining: usize,
    matched: bool,
    disposition: Puzzle2dDeleteNodeDisposition,
    pub phase: u8,
    closing: bool,
}

impl Deref for Puzzle2dDeleteNodeCandidateCursor { type Target = Puzzle2dDeleteNodeCandidateState; fn deref(&self) -> &Self::Target { &self.state } }
impl DerefMut for Puzzle2dDeleteNodeCandidateCursor { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.state } }

impl Default for Puzzle2dDeleteNodeCandidateCursor {
    fn default() -> Self { Self { state: ManuallyDrop::new(Puzzle2dDeleteNodeCandidateState { source: Default::default(), mutation: Default::default(), preparation: Default::default(), comparison: Default::default(), snapshot_clone: Default::default(), removals: Some(PagedList::with_page_items(64)), candidate: None, removed: Removed::Node(None), retirement: Default::default(), edit: None, pending_index: None, node: None, event: 0, search: 0, position: 0, remaining: 0, matched: false, disposition: Puzzle2dDeleteNodeDisposition::Missing, phase: 0, closing: false }) } }
}

impl Puzzle2dDeleteNodeCandidateCursor {
    pub fn advance(&mut self, source: RetainedCloneRef<'_, Puzzle2dSnapshot>, mutation: RetainedCloneRef<'_, DeleteNode>, body:usize,effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneStep, ValueError> {
        let grant = effect.grant();
        if self.closing || self.phase == 23 { return Err(refusal("delete candidate is closing or spent")); }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if let Some(progress)=self.source.bind(source,effect)?{return Ok(RetainedCloneStep::Progress(progress))}
        if let Some(progress)=self.mutation.bind(mutation,effect)?{return Ok(RetainedCloneStep::Progress(progress))}
        match self.phase {
            0 => {
                if let Some(step)=self.preparation.ensure(effect)? {return Ok(step);}
                let step = self.preparation.owner_mut().advance(source, mutation, effect)?;
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
                let (ordering, progress) = self.comparison.compare(left, right, effect)?;
                let Some(ordering) = ordering else { return Ok(RetainedCloneStep::Progress(progress)); };
                self.matched = ordering == std::cmp::Ordering::Equal;self.phase = 3;Ok(RetainedCloneStep::Progress(progress))
            }
            3 => {
                let progress=self.comparison.close(effect)?;
                if self.comparison.is_empty() { if self.matched { self.position = 0;self.phase = 4; } else { self.search += 1;self.phase = 2; } }
                Ok(RetainedCloneStep::Progress(progress))
            }
            4 => {
                if !pay(effect)? { return Ok(RetainedCloneStep::Progress(Default::default())); }
                let indices = self.removals.as_ref().ok_or_else(|| refusal("delete cascade lost its ordinal owner"))?;
                match indices.get(self.position).copied() {
                    Some(index) if index == self.search => { self.disposition = Puzzle2dDeleteNodeDisposition::DuplicateRemovalTarget;self.preparation.begin_close();self.phase = 8; }
                    Some(index) if index < self.search => self.position += 1,
                    _ => self.phase = 5,
                }
                Ok(payload_progress(0))
            }
            5 => {
                if grant.maximum_copy_bytes < size_of::<PagedListEditCursor>() + size_of::<usize>() || !pay(effect)? { return Ok(RetainedCloneStep::Progress(Default::default())); }
                self.edit = Some(PagedListEditCursor::insert(self.position, self.removals.as_ref().unwrap().len()));self.pending_index = Some(self.search);self.phase = 6;
                Ok(payload_progress(size_of::<PagedListEditCursor>() + size_of::<usize>()))
            }
            6 => {
                let state = &mut **self;
                let indices = state.removals.as_mut().ok_or_else(|| refusal("delete sorting lost its ordinal owner"))?;
                let edit = state.edit.as_mut().ok_or_else(|| refusal("delete sorting lost its ordered cursor"))?;
                let Some(step) = edit_step(edit, indices, &mut state.pending_index, effect)? else { return Ok(RetainedCloneStep::Progress(Default::default())); };
                if step.complete { state.phase = 7; }
                Ok(RetainedCloneStep::Progress(edit_progress(step)))
            }
            7 => { if !pay(effect)? { return Ok(RetainedCloneStep::Progress(Default::default())); } self.edit = None;self.phase = 0;Ok(payload_progress(0)) }
            8 => {
                let step = settled_step(self.preparation.close_step(effect))?;
                if matches!(step,RetainedCloneStep::Complete(_)) { self.phase = 9; }
                Ok(RetainedCloneStep::Progress(step.progress()))
            }
            9 => { if !pay(effect)? { return Ok(RetainedCloneStep::Progress(Default::default())); } self.phase = if self.disposition == Puzzle2dDeleteNodeDisposition::Changed { 10 } else { 21 };Ok(payload_progress(0)) }
            10 => {
                let step = cursor_advance(&mut self.snapshot_clone, source, body, effect)?;
                if matches!(step, RetainedCloneStep::Complete(_)) { self.phase = 11; }
                Ok(RetainedCloneStep::Progress(step.progress()))
            }
            11 => {
                let state=&mut *self.state;
                let step=cursor_take(&mut state.snapshot_clone,&mut state.candidate,effect)?;
                if state.candidate.is_some(){state.phase=12;}
                Ok(close_progress(step))
            }
            12 => { let step = cursor_retire::<Puzzle2dSnapshot, _>(&mut self.snapshot_clone, effect)?;if self.snapshot_clone.terminal_is_empty() { self.phase = 13; }Ok(RetainedCloneStep::Progress(step.progress())) }
            13 => {
                if grant.maximum_copy_bytes < size_of::<PagedListEditCursor>() || !pay(effect)? { return Ok(RetainedCloneStep::Progress(Default::default())); }
                let index = self.node.ok_or_else(|| refusal("delete candidate lost its first node ordinal"))?;
                self.edit = Some(PagedListEditCursor::remove(index, self.candidate.as_ref().unwrap().nodes.len()));self.phase = 14;
                Ok(payload_progress(size_of::<PagedListEditCursor>()))
            }
            14 => {
                let state = &mut **self;
                let Removed::Node(node)=&mut state.removed else{return Err(refusal("delete candidate lost its node receiver"));};
                let Some(step) = edit_step(state.edit.as_mut().unwrap(), &mut state.candidate.as_mut().unwrap().nodes, node, effect)? else { return Ok(RetainedCloneStep::Progress(Default::default())); };
                if step.complete { state.edit = None;state.phase = 15; }
                Ok(RetainedCloneStep::Progress(edit_progress(step)))
            }
            15 => {
                let state=&mut *self.state;
                let step=state.removed.retire(&mut state.retirement,effect)?;
                if !state.retirement.is_empty(){state.phase=16;}
                Ok(step)
            }
            16 => {
                let step=settled_step(self.retirement.step_granted(body,effect))?;
                if self.retirement.is_empty(){self.remaining=self.removals.as_ref().unwrap().len();self.phase=17;}
                Ok(close_progress(step))
            }
            17 => {
                if self.remaining == 0 { if !pay(effect)? { return Ok(RetainedCloneStep::Progress(Default::default())); } self.phase = 21;return Ok(payload_progress(0)); }
                if matches!(self.removed,Removed::Node(None)){
                    let bytes=size_of::<Removed>();
                    if grant.maximum_copy_bytes<bytes||!pay(effect)?{return Ok(RetainedCloneStep::Progress(Default::default()));}
                    self.removed=Removed::Edge(None);
                    return Ok(payload_progress(bytes));
                }
                if grant.maximum_copy_bytes < size_of::<PagedListEditCursor>() || !pay(effect)? { return Ok(RetainedCloneStep::Progress(Default::default())); }
                self.remaining -= 1;let index = *self.removals.as_ref().unwrap().get(self.remaining).unwrap();
                self.edit = Some(PagedListEditCursor::remove(index, self.candidate.as_ref().unwrap().edges.len()));self.phase = 18;
                Ok(payload_progress(size_of::<PagedListEditCursor>()))
            }
            18 => {
                let state = &mut **self;
                let Removed::Edge(edge)=&mut state.removed else{return Err(refusal("delete candidate lost its edge receiver"));};
                let Some(step) = edit_step(state.edit.as_mut().unwrap(), &mut state.candidate.as_mut().unwrap().edges, edge, effect)? else { return Ok(RetainedCloneStep::Progress(Default::default())); };
                if step.complete { state.edit = None;state.phase = 19; }
                Ok(RetainedCloneStep::Progress(edit_progress(step)))
            }
            19 => {
                let state=&mut *self.state;
                let step=state.removed.retire(&mut state.retirement,effect)?;
                if !state.retirement.is_empty(){state.phase=20;}
                Ok(step)
            }
            20 => { let step=settled_step(self.retirement.step_granted(body,effect))?;if self.retirement.is_empty(){self.phase=17;}Ok(close_progress(step)) }
            21 => {
                if grant.maximum_copy_bytes < size_of::<Puzzle2dDeleteNodeCandidate>() || !pay(effect)? { return Ok(RetainedCloneStep::Progress(Default::default())); }
                self.phase = 22;Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<Puzzle2dDeleteNodeCandidate>(), retained_capacity_bytes: 0, released_bytes: 0 }))
            }
            22 => Ok(RetainedCloneStep::Complete(Default::default())),
            _ => Err(refusal("delete candidate has an unknown ownership phase")),
        }
    }

    pub fn take(&mut self) -> Option<Puzzle2dDeleteNodeCandidate> { if self.closing || self.phase != 22 { return None; }self.phase = 23;Some(Puzzle2dDeleteNodeCandidate { disposition: self.disposition, snapshot: self.candidate.take() }) }
    pub fn begin_close(&mut self) { self.closing = true;self.preparation.begin_close(); }

    #[cfg(feature="testkit")]
    #[doc(hidden)]
    pub fn advance_test_demand(&self,source:RetainedCloneRef<'_,Puzzle2dSnapshot>,body:usize)->Result<Option<RetirementDemand>,ValueError>{
        use crate::standards::v1::subsets::any::schema::preparation::native_preparation_child::{cursor_retire_demand,edit_fault};
        Ok(match self.phase{
            6=>Some(self.edit.as_ref().unwrap().step_demand(self.removals.as_ref().unwrap(),&self.pending_index).map_err(edit_fault)?),
            10=>Some(self.snapshot_clone.advance_demands(source,body,u64::MAX)?),
            11=>Some(self.snapshot_clone.take_demand()?),
            12=>Some(cursor_retire_demand::<Puzzle2dSnapshot,_>(&self.snapshot_clone,body)?),
            14=>{
                let Removed::Node(node)=&self.removed else{return Err(refusal("delete candidate lost its node receiver"));};
                Some(self.edit.as_ref().unwrap().step_demand(&self.candidate.as_ref().unwrap().nodes,node).map_err(edit_fault)?)
            }
            18=>{
                let Removed::Edge(edge)=&self.removed else{return Err(refusal("delete candidate lost its edge receiver"));};
                Some(self.edit.as_ref().unwrap().step_demand(&self.candidate.as_ref().unwrap().edges,edge).map_err(edit_fault)?)
            }
            15|19=>Some(self.removed.demand(body,&self.retirement)?),
            16|20=>Some(self.retirement.demands(body)?),
            _=>None,
        })
    }

    fn close_demand(&self,axis:Puzzle2dCloseAxis)->Result<usize,ValueError>{
        if !self.preparation.terminal_is_empty(){return match axis {Puzzle2dCloseAxis::Copy=>self.preparation.next_close_copy_byte_demand(),Puzzle2dCloseAxis::Capacity(body)=>self.preparation.next_close_capacity_byte_demand(body),Puzzle2dCloseAxis::Release=>self.preparation.next_close_release_byte_demand(),Puzzle2dCloseAxis::Depth=>self.preparation.next_close_depth_demand()}}
        if !self.comparison.is_empty(){return Ok(axis.select(self.comparison.close_demand()))}
        if !self.snapshot_clone.terminal_is_empty(){return axis.retained::<Puzzle2dSnapshot,_>(&self.snapshot_clone)}
        if !self.retirement.is_empty(){return axis.owned::<()>(&self.retirement,false)}
        if !self.removed.is_empty(){return self.removed.quote(axis,&self.retirement)}
        if self.candidate.is_some(){return axis.owned::<Puzzle2dSnapshot>(&self.retirement,true)}
        if self.removals.as_ref().is_some_and(|owner|!owner.terminal_is_empty()){return axis.owned::<PagedList<usize,{usize::MAX}>>(&self.retirement,true)}
        if self.edit.is_some(){return axis.inline::<Option<PagedListEditCursor>>()}
        if self.pending_index.is_some(){return axis.inline::<Option<usize>>()}
        Ok(axis.select(if self.source.is_empty(){self.mutation.close_demand()}else{self.source.close_demand()}))
    }

    pub fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{self.close_demand(Puzzle2dCloseAxis::Copy)}
    pub fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{self.close_demand(Puzzle2dCloseAxis::Capacity(body))}
    pub fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{self.close_demand(Puzzle2dCloseAxis::Release)}
    pub fn next_close_depth_demand(&self)->Result<usize,ValueError>{self.close_demand(Puzzle2dCloseAxis::Depth)}

    pub fn close_step(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> OriginalRetirementPoll { let result = self.close_stepped(effect); RetirementPollExit::settle(effect, result) }

    fn close_stepped(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneStep, RetirementPollExit> {
        let grant = effect.grant();
        if !self.closing { return Err(refusal("delete candidate closure was not started").into()); }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !self.preparation.terminal_is_empty() { return Ok(RetainedCloneStep::Progress(RetirementPollExit::resume(self.preparation.close_step(effect))?.progress())); }
        if !self.comparison.is_empty() { return Ok(RetainedCloneStep::Progress(self.comparison.close(effect)?)); }
        if !self.snapshot_clone.terminal_is_empty() { return Ok(RetainedCloneStep::Progress(cursor_retire::<Puzzle2dSnapshot, _>(&mut self.snapshot_clone, effect)?.progress())); }
        let state=&mut *self.state;
        if !state.retirement.is_empty(){return RetirementPollExit::resume(state.retirement.step_granted(grant.maximum_copy_bytes,effect)).map(close_progress);}
        if !state.removed.is_empty(){return Ok(state.removed.retire(&mut state.retirement,effect)?);}
        if state.candidate.is_some()||state.removals.as_ref().is_some_and(|owner|!owner.terminal_is_empty()){
            let turn=if state.candidate.is_some(){state.retirement.begin_granted(&mut state.candidate,effect)}
                else{state.retirement.begin_granted(&mut state.removals,effect)};
            return Ok(close_progress(turn.result?.unwrap_or(RetainedCloneStep::Progress(Default::default()))));
        }
        if self.edit.is_some() {let bytes=size_of::<Option<PagedListEditCursor>>();if grant.maximum_copy_bytes<bytes||!pay(effect)?{return Ok(RetainedCloneStep::Progress(Default::default()));}self.edit=None;return Ok(payload_progress(bytes));}
        if self.pending_index.is_some() {let bytes=size_of::<Option<usize>>();if grant.maximum_copy_bytes<bytes||!pay(effect)?{return Ok(RetainedCloneStep::Progress(Default::default()));}self.pending_index=None;return Ok(payload_progress(bytes));}
        let progress=if self.source.is_empty(){self.mutation.close(effect)?}else{self.source.close(effect)?};
        Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
    }

    pub fn terminal_is_empty(&self) -> bool { self.closing && self.preparation.terminal_is_empty() && self.comparison.is_empty() && self.snapshot_clone.terminal_is_empty() && self.removals.as_ref().is_none_or(PagedList::terminal_is_empty) && self.candidate.is_none() && self.removed.is_empty() && self.retirement.is_empty() && self.edit.is_none() && self.pending_index.is_none() && self.source.is_empty() && self.mutation.is_empty() }
}

impl Drop for Puzzle2dDeleteNodeCandidateCursor { fn drop(&mut self) { let empty = self.terminal_is_empty();assert!(std::thread::panicking() || empty, "delete candidate abandoned before controlled closure");if empty { unsafe { ManuallyDrop::drop(&mut self.state); } } } }

fn payload_progress(bytes: usize) -> RetainedCloneStep { RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, retained_capacity_bytes: 0, released_bytes: 0 }) }
fn close_progress(step: RetainedCloneStep) -> RetainedCloneStep { RetainedCloneStep::Progress(step.progress()) }
fn refusal(message: &'static str) -> ValueError { ValueError::invariant(message) }


