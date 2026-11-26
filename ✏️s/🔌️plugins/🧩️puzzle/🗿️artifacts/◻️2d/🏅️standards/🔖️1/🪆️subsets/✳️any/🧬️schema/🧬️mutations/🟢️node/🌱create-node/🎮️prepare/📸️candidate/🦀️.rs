//! 📸️ Copies native snapshot and borrowed node ownership before bounded ordered placement.

use crate::standards::v1::subsets::any::schema::lookup::borrowed::BorrowedIdentity;
use crate::standards::v1::subsets::any::schema::preparation::native_preparation_child::{cursor_advance, cursor_retire, cursor_take, edit_progress, edit_step, pay, settled_step};
use semio_framework_value::{OriginalRetirementPoll, RetainedAllocationEffect, RetirementDemand, RetirementPollExit};
use crate::standards::v1::subsets::any::schema::preparation::native_preparation_child::Puzzle2dCloseAxis;
use super::{Puzzle2dCreateNodeDisposition, Puzzle2dCreateNodePlan, Puzzle2dCreateNodePreparationCursor, Puzzle2dCreateNodePreparationStep};
use crate::standards::v1::subsets::any::schema::mutations::CreateNode;
use crate::standards::v1::subsets::any::schema::snapshot::Puzzle2dSnapshot;
use crate::standards::v1::subsets::any::schema::records::Puzzle2dNode;
use semio_framework_value::{ValueError, retained_clone::{RetainedCloneClose, RetainedFieldCursor, RetainedCloneCursor, RetainedCloneProgress, RetainedCloneRef, RetainedCloneStep}};
use semio_framework_value_persistent::list::PagedListEditCursor;
use std::{mem::{ManuallyDrop, size_of}, ops::{Deref, DerefMut}};

pub struct Puzzle2dCreateNodeCandidate { pub plan: Puzzle2dCreateNodePlan, pub snapshot: Option<Puzzle2dSnapshot> }
pub struct Puzzle2dCreateNodeCandidateCursor { state: ManuallyDrop<Puzzle2dCreateNodeCandidateState> }

enum CloneOwner {
    Snapshot(RetainedFieldCursor<Puzzle2dSnapshot>),
    Node(RetainedFieldCursor<Puzzle2dNode>),
}

impl CloneOwner {
    fn snapshot(&mut self)->Result<&mut RetainedFieldCursor<Puzzle2dSnapshot>,ValueError>{
        match self{Self::Snapshot(cursor)=>Ok(cursor),Self::Node(_)=>Err(refusal("create snapshot clone is absent"))}
    }

    fn node(&mut self)->Result<&mut RetainedFieldCursor<Puzzle2dNode>,ValueError>{
        match self{Self::Node(cursor)=>Ok(cursor),Self::Snapshot(_)=>Err(refusal("create node clone is absent"))}
    }

    fn terminal_is_empty(&self)->bool{
        match self{Self::Snapshot(cursor)=>cursor.terminal_is_empty(),Self::Node(cursor)=>cursor.terminal_is_empty()}
    }

    fn close(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError>{
        match self{Self::Snapshot(cursor)=>cursor_retire(cursor,effect),Self::Node(cursor)=>cursor_retire(cursor,effect)}
    }

    fn close_demand(&self,axis:Puzzle2dCloseAxis)->Result<usize,ValueError>{
        match self{Self::Snapshot(cursor)=>axis.retained::<Puzzle2dSnapshot,_>(cursor),Self::Node(cursor)=>axis.retained::<Puzzle2dNode,_>(cursor)}
    }
}

#[doc(hidden)]
pub struct Puzzle2dCreateNodeCandidateState {
    source: BorrowedIdentity,
    mutation: BorrowedIdentity,
    preparation: Puzzle2dCreateNodePreparationCursor,
    clone_owner: CloneOwner,
    plan: Option<Puzzle2dCreateNodePlan>,
    candidate: Option<Puzzle2dSnapshot>,
    pub pending: Option<Puzzle2dNode>,
    insertion: Option<PagedListEditCursor>,
    retirement: RetainedCloneClose,
    pub phase: u8,
    closing: bool,
}

impl Deref for Puzzle2dCreateNodeCandidateCursor { type Target = Puzzle2dCreateNodeCandidateState; fn deref(&self) -> &Self::Target { &self.state } }
impl DerefMut for Puzzle2dCreateNodeCandidateCursor { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.state } }

impl Default for Puzzle2dCreateNodeCandidateCursor {
    fn default() -> Self { Self { state: ManuallyDrop::new(Puzzle2dCreateNodeCandidateState { source: Default::default(), mutation: Default::default(), preparation: Default::default(), clone_owner: CloneOwner::Snapshot(Default::default()), plan: None, candidate: None, pending: None, insertion: None, retirement: Default::default(), phase: 0, closing: false }) } }
}

impl Puzzle2dCreateNodeCandidateCursor {
    pub fn advance(&mut self, source: RetainedCloneRef<'_, Puzzle2dSnapshot>, mutation: RetainedCloneRef<'_, CreateNode>, body:usize,effect:&mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneStep, ValueError> {
        let grant=effect.grant();
        if self.closing || self.phase == 13 { return Err(refusal("create candidate is closing or spent")); }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if let Some(progress)=self.source.bind(source,effect)?{return Ok(RetainedCloneStep::Progress(progress))}
        if let Some(progress)=self.mutation.bind(mutation,effect)?{return Ok(RetainedCloneStep::Progress(progress))}
        match self.phase {
            0 => {
                let step = self.preparation.advance(source, mutation, effect)?;
                let progress = match step { Puzzle2dCreateNodePreparationStep::Pending(progress) => progress, Puzzle2dCreateNodePreparationStep::Complete { plan, progress } => { self.plan = self.preparation.take(); self.preparation.begin_close(); self.phase = 1; if self.plan != Some(plan) { return Err(refusal("create candidate lost its scalar plan")); } progress } };
                Ok(RetainedCloneStep::Progress(progress))
            }
            1 => {
                let step = settled_step(self.preparation.close_step(effect))?;
                if self.preparation.terminal_is_empty() { self.phase = if self.plan.is_some_and(|plan| plan.disposition == Puzzle2dCreateNodeDisposition::Changed) { 2 } else { 11 }; }
                Ok(close_progress(step))
            }
            2 => {
                let step = cursor_advance(self.clone_owner.snapshot()?,source,body,effect)?;
                if matches!(step, RetainedCloneStep::Complete(_)) { self.phase = 3; }
                Ok(RetainedCloneStep::Progress(step.progress()))
            }
            3 => {
                let state=&mut *self.state;
                let step=cursor_take(state.clone_owner.snapshot()?,&mut state.candidate,effect)?;
                if state.candidate.is_some(){state.phase=4;}
                Ok(close_progress(step))
            }
            4 => {
                let step=self.clone_owner.close(effect)?;
                if self.clone_owner.terminal_is_empty() { self.phase = 14; }
                Ok(close_progress(step))
            }
            14 => {
                let bytes=size_of::<CloneOwner>();
                if grant.maximum_copy_bytes<bytes||!pay(effect)?{return Ok(RetainedCloneStep::Progress(Default::default()));}
                self.clone_owner=CloneOwner::Node(Default::default());
                self.phase=5;
                Ok(payload_progress(bytes))
            }
            5 => {
                let step = cursor_advance(self.clone_owner.node()?,mutation.project(1, |payload| &payload.node),body,effect)?;
                if matches!(step, RetainedCloneStep::Complete(_)) { self.phase = 6; }
                Ok(RetainedCloneStep::Progress(step.progress()))
            }
            6 => {
                let state=&mut *self.state;
                let step=cursor_take(state.clone_owner.node()?,&mut state.pending,effect)?;
                if state.pending.is_some(){state.phase=7;}
                Ok(close_progress(step))
            }
            7 => {
                let step=self.clone_owner.close(effect)?;
                if self.clone_owner.terminal_is_empty() { self.phase = 8; }
                Ok(close_progress(step))
            }
            8 => {
                let bytes=size_of::<Option<PagedListEditCursor>>();
                if grant.maximum_copy_bytes < bytes||!pay(effect)? { return Ok(RetainedCloneStep::Progress(Default::default())); }
                let position = self.plan.and_then(|plan| plan.position).ok_or_else(|| refusal("create candidate has no admitted insertion position"))?;
                let length = self.candidate.as_ref().ok_or_else(|| refusal("create candidate lost its snapshot"))?.nodes.len();
                self.insertion = Some(PagedListEditCursor::insert(position, length));self.phase = 9;
                Ok(payload_progress(bytes))
            }
            9 => {
                let state = &mut **self;
                let nodes = &mut state.candidate.as_mut().ok_or_else(|| refusal("create insertion lost its snapshot"))?.nodes;
                let insertion = state.insertion.as_mut().ok_or_else(|| refusal("create insertion lost its ordered cursor"))?;
                let Some(step) = edit_step(insertion, nodes, &mut state.pending, effect)? else { return Ok(RetainedCloneStep::Progress(Default::default())); };
                if step.complete { state.phase = 10; }
                Ok(RetainedCloneStep::Progress(edit_progress(step)))
            }
            10 => { let bytes=size_of::<Option<PagedListEditCursor>>();if grant.maximum_copy_bytes<bytes||!pay(effect)? { return Ok(RetainedCloneStep::Progress(Default::default())); } self.insertion = None;self.phase = 11;Ok(payload_progress(bytes)) }
            11 => {
                if grant.maximum_copy_bytes < size_of::<Puzzle2dCreateNodeCandidate>()||!pay(effect)? { return Ok(RetainedCloneStep::Progress(Default::default())); }
                self.phase = 12;
                Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<Puzzle2dCreateNodeCandidate>(), retained_capacity_bytes: 0, released_bytes: 0 }))
            }
            12 => Ok(RetainedCloneStep::Complete(Default::default())),
            _ => Err(refusal("create candidate has an unknown ownership phase")),
        }
    }

    pub fn take(&mut self) -> Option<Puzzle2dCreateNodeCandidate> { if self.closing || self.phase != 12 { return None; } self.phase = 13;Some(Puzzle2dCreateNodeCandidate { plan: self.plan.take()?, snapshot: self.candidate.take() }) }
    pub fn begin_close(&mut self) { self.closing = true;self.preparation.begin_close(); }

    #[cfg(feature="testkit")]
    #[doc(hidden)]
    pub fn advance_test_demand(&self,source:RetainedCloneRef<'_,Puzzle2dSnapshot>,mutation:RetainedCloneRef<'_,CreateNode>,body:usize)->Result<Option<RetirementDemand>,ValueError>{
        use crate::standards::v1::subsets::any::schema::preparation::native_preparation_child::{cursor_retire_demand,edit_fault};
        Ok(match (&self.clone_owner,self.phase){
            (CloneOwner::Snapshot(cursor),2)=>Some(cursor.advance_demands(source,body,u64::MAX)?),
            (CloneOwner::Snapshot(cursor),3)=>Some(cursor.take_demand()?),
            (CloneOwner::Snapshot(cursor),4)=>Some(cursor_retire_demand::<Puzzle2dSnapshot,_>(cursor,body)?),
            (CloneOwner::Node(cursor),5)=>Some(cursor.advance_demands(mutation.project(1,|payload|&payload.node),body,u64::MAX)?),
            (CloneOwner::Node(cursor),6)=>Some(cursor.take_demand()?),
            (CloneOwner::Node(cursor),7)=>Some(cursor_retire_demand::<Puzzle2dNode,_>(cursor,body)?),
            (_,9)=>{
                let candidate=self.candidate.as_ref().ok_or_else(||refusal("create candidate lost its snapshot"))?;
                let insertion=self.insertion.as_ref().ok_or_else(||refusal("create candidate lost its insertion cursor"))?;
                Some(insertion.step_demand(&candidate.nodes,&self.pending).map_err(edit_fault)?)
            }
            _=>None,
        })
    }

    fn close_demand(&self,axis:Puzzle2dCloseAxis)->Result<usize,ValueError>{
        if !self.preparation.terminal_is_empty(){return match axis {Puzzle2dCloseAxis::Copy=>self.preparation.next_close_copy_byte_demand(),Puzzle2dCloseAxis::Capacity(body)=>self.preparation.next_close_capacity_byte_demand(body),Puzzle2dCloseAxis::Release=>self.preparation.next_close_release_byte_demand(),Puzzle2dCloseAxis::Depth=>self.preparation.next_close_depth_demand()}}
        if !self.clone_owner.terminal_is_empty(){return self.clone_owner.close_demand(axis)}
        if self.pending.is_some(){return axis.owned::<Puzzle2dNode>(&self.retirement,true)}
        if self.candidate.is_some()||!self.retirement.is_empty(){return axis.owned::<Puzzle2dSnapshot>(&self.retirement,self.candidate.is_some())}
        if self.insertion.is_some(){return axis.inline::<Option<PagedListEditCursor>>()}
        if self.plan.is_some(){return axis.inline::<Option<Puzzle2dCreateNodePlan>>()}
        Ok(axis.select(if self.source.is_empty(){self.mutation.close_demand()}else{self.source.close_demand()}))
    }

    pub fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{self.close_demand(Puzzle2dCloseAxis::Copy)}
    pub fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{self.close_demand(Puzzle2dCloseAxis::Capacity(body))}
    pub fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{self.close_demand(Puzzle2dCloseAxis::Release)}
    pub fn next_close_depth_demand(&self)->Result<usize,ValueError>{self.close_demand(Puzzle2dCloseAxis::Depth)}

    pub fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{let result=self.close_stepped(effect);RetirementPollExit::settle(effect,result)}
    fn close_stepped(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,RetirementPollExit>{
        let grant=effect.grant();
        if !self.closing { return Err(refusal("create candidate closure was not started").into()); }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !self.preparation.terminal_is_empty() { return RetirementPollExit::resume(self.preparation.close_step(effect)).map(close_progress); }
        if !self.clone_owner.terminal_is_empty() { return Ok(close_progress(self.clone_owner.close(effect)?)); }
        let state=&mut *self.state;
        if !state.retirement.is_empty(){return RetirementPollExit::resume(state.retirement.step_granted(grant.maximum_copy_bytes,effect)).map(close_progress);}
        if state.pending.is_some()||state.candidate.is_some() {
            let turn=if state.pending.is_some(){state.retirement.begin_granted(&mut state.pending,effect)}
                else{state.retirement.begin_granted(&mut state.candidate,effect)};
            return Ok(RetainedCloneStep::Progress(turn.result?.map(|step|step.progress()).unwrap_or_default()));
        }
        if self.insertion.is_some() { let bytes=size_of::<Option<PagedListEditCursor>>();if grant.maximum_copy_bytes<bytes||!pay(effect)?{return Ok(RetainedCloneStep::Progress(Default::default()));}self.insertion=None;return Ok(payload_progress(bytes)); }
        if self.plan.is_some() { let bytes=size_of::<Option<Puzzle2dCreateNodePlan>>();if grant.maximum_copy_bytes<bytes||!pay(effect)?{return Ok(RetainedCloneStep::Progress(Default::default()));}self.plan=None;return Ok(payload_progress(bytes)); }
        let progress=if self.source.is_empty(){self.mutation.close(effect)?}else{self.source.close(effect)?};
        Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
    }

    pub fn terminal_is_empty(&self) -> bool { self.closing && self.preparation.terminal_is_empty() && self.clone_owner.terminal_is_empty() && self.plan.is_none() && self.candidate.is_none() && self.pending.is_none() && self.insertion.is_none() && self.retirement.is_empty() && self.source.is_empty() && self.mutation.is_empty() }
}

impl Drop for Puzzle2dCreateNodeCandidateCursor { fn drop(&mut self) { let empty = self.terminal_is_empty();assert!(std::thread::panicking() || empty, "create candidate abandoned before controlled closure");if empty { unsafe { ManuallyDrop::drop(&mut self.state); } } } }

fn payload_progress(bytes: usize) -> RetainedCloneStep { RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, retained_capacity_bytes: 0, released_bytes: 0 }) }
fn close_progress(step: RetainedCloneStep) -> RetainedCloneStep { RetainedCloneStep::Progress(step.progress()) }
fn refusal(message: &'static str) -> ValueError { ValueError::invariant(message) }


