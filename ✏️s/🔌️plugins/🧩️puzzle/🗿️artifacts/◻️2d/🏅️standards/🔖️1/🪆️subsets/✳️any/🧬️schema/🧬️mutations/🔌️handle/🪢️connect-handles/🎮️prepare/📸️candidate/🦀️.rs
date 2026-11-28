//! 📸️ Copies native snapshot and borrowed connection ownership before bounded ordered placement.

use crate::standards::v1::subsets::any::schema::preparation::native_preparation_child::{controlled_step, cursor_advance, cursor_retire, cursor_taken, edit_progress, edit_step, list_reserve, pay, settled_step};
use semio_framework_value::{OriginalRetirementPoll, RetainedAllocationEffect, RetirementPollExit};
use crate::standards::v1::subsets::any::schema::preparation::native_preparation_child::Puzzle2dCloseAxis;
use super::{Puzzle2dConnectHandlesDisposition, Puzzle2dConnectHandlesPlan, Puzzle2dConnectHandlesPreparationCursor, Puzzle2dConnectHandlesPreparationStep};
use crate::standards::v1::subsets::any::schema::mutations::ConnectHandles;
use crate::standards::v1::subsets::any::schema::snapshot::Puzzle2dSnapshot;
use crate::standards::v1::subsets::any::schema::records::Puzzle2dEdge;
use super::edge::Puzzle2dConnectEdgeCursor;
use semio_framework_value::{ValueError, ValueRefusalKind, retirement::controlled::ControlledRetirement, retained_clone::{RetainedCloneBinding, RetainedFieldCursor, RetainedCloneCursor, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneStep}};
use semio_framework_value_persistent::list::PagedListEditCursor;
use crate::standards::v1::subsets::any::schema::preparation::native_preparation_child::Puzzle2dPreparationChild;
use std::{mem::{ManuallyDrop, size_of}, ops::{Deref, DerefMut}};

pub struct Puzzle2dConnectHandlesCandidate { pub plan: Puzzle2dConnectHandlesPlan, pub snapshot: Option<Puzzle2dSnapshot> }
/// 🪆️ Holds only the sequential child or retirement that is alive, so preparation, snapshot copy, edge assembly and the two retirements share one slot.
pub enum Puzzle2dCandidateStage { Idle, Prepare(Puzzle2dPreparationChild<Puzzle2dConnectHandlesPreparationCursor>), Clone(RetainedFieldCursor<Puzzle2dSnapshot>), Edge(Puzzle2dPreparationChild<Puzzle2dConnectEdgeCursor>), CloseEdge(ControlledRetirement<Puzzle2dEdge>), CloseSnapshot(ControlledRetirement<Puzzle2dSnapshot>) }
impl Puzzle2dCandidateStage {
    fn terminal_is_empty(&self)->bool { match self { Self::Idle => true, Self::Prepare(child) => child.terminal_is_empty(), Self::Clone(cursor) => cursor.terminal_is_empty(), Self::Edge(child) => child.terminal_is_empty(), Self::CloseEdge(owner) => owner.terminal_is_empty(), Self::CloseSnapshot(owner) => owner.terminal_is_empty() } }
}
impl Puzzle2dConnectHandlesCandidateState {
    fn prep(&mut self)->&mut Puzzle2dPreparationChild<Puzzle2dConnectHandlesPreparationCursor> { if matches!(self.stage, Puzzle2dCandidateStage::Idle) { self.stage = Puzzle2dCandidateStage::Prepare(Default::default()); } match &mut self.stage { Puzzle2dCandidateStage::Prepare(child) => child, _ => unreachable!("connect candidate phases are sequential") } }
    fn clone_cursor(&mut self)->&mut RetainedFieldCursor<Puzzle2dSnapshot> { if matches!(self.stage, Puzzle2dCandidateStage::Idle) { self.stage = Puzzle2dCandidateStage::Clone(Default::default()); } match &mut self.stage { Puzzle2dCandidateStage::Clone(cursor) => cursor, _ => unreachable!("connect candidate phases are sequential") } }
    fn edge(&mut self)->&mut Puzzle2dPreparationChild<Puzzle2dConnectEdgeCursor> { if matches!(self.stage, Puzzle2dCandidateStage::Idle) { self.stage = Puzzle2dCandidateStage::Edge(Default::default()); } match &mut self.stage { Puzzle2dCandidateStage::Edge(child) => child, _ => unreachable!("connect candidate phases are sequential") } }
}

pub struct Puzzle2dConnectHandlesCandidateCursor { state: ManuallyDrop<Puzzle2dConnectHandlesCandidateState> }

#[doc(hidden)]
pub struct Puzzle2dConnectHandlesCandidateState {
    source: Option<RetainedCloneBinding>,
    mutation: Option<RetainedCloneBinding>,
    stage: Puzzle2dCandidateStage,
    plan: Option<Puzzle2dConnectHandlesPlan>,
    candidate: Option<Puzzle2dSnapshot>,
    pub pending: Option<Puzzle2dEdge>,
    insertion: Option<PagedListEditCursor>,
    pub phase: u8,
    closing: bool,
}

impl Deref for Puzzle2dConnectHandlesCandidateCursor { type Target = Puzzle2dConnectHandlesCandidateState; fn deref(&self) -> &Self::Target { &self.state } }
impl DerefMut for Puzzle2dConnectHandlesCandidateCursor { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.state } }

impl Default for Puzzle2dConnectHandlesCandidateCursor {
    fn default() -> Self { Self { state: ManuallyDrop::new(Puzzle2dConnectHandlesCandidateState { source: None, mutation: None, stage: Puzzle2dCandidateStage::Idle, plan: None, candidate: None, pending: None, insertion: None, phase: 0, closing: false }) } }
}

impl Puzzle2dConnectHandlesCandidateCursor {
    pub fn advance(&mut self, source: RetainedCloneRef<'_, Puzzle2dSnapshot>, mutation: RetainedCloneRef<'_, ConnectHandles>, body:usize,effect:&mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneStep, ValueError> {
        let grant=effect.grant();
        if self.closing || self.phase == 13 { return Err(refusal("connect candidate is closing or spent")); }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if let Some(progress)=source.bind(&mut self.source,effect).result?{return Ok(RetainedCloneStep::Progress(progress))}
        if let Some(progress)=mutation.bind(&mut self.mutation,effect).result?{return Ok(RetainedCloneStep::Progress(progress))}
        match self.phase {
            0 => {
                if let Some(step)=self.prep().ensure(effect)? {return Ok(step);}
                let step = self.prep().owner_mut().advance(source, mutation,effect)?;
                let progress = match step { Puzzle2dConnectHandlesPreparationStep::Pending(progress) => progress, Puzzle2dConnectHandlesPreparationStep::Complete { plan, progress } => { self.plan = self.prep().owner_mut().take(); self.prep().begin_close(); self.phase = 1; if self.plan != Some(plan) { return Err(refusal("connect candidate lost its scalar plan")); } progress } };
                Ok(RetainedCloneStep::Progress(progress))
            }
            1 => {
                let step = settled_step(self.prep().close_step(effect))?;
                if matches!(step,RetainedCloneStep::Complete(_)) { self.stage = Puzzle2dCandidateStage::Idle; self.phase = if self.plan.is_some_and(|plan| plan.disposition == Puzzle2dConnectHandlesDisposition::Changed) { 2 } else { 11 }; }
                Ok(RetainedCloneStep::Progress(step.progress()))
            }
            2 => {
                let step = cursor_advance(self.clone_cursor(),source,body,effect)?;
                if matches!(step, RetainedCloneStep::Complete(_)) { self.phase = 3; }
                Ok(RetainedCloneStep::Progress(step.progress()))
            }
            3 => {
                if grant.maximum_copy_bytes < size_of::<Puzzle2dSnapshot>() { return Ok(RetainedCloneStep::Progress(Default::default())); }
                let Some(owner)=cursor_taken(self.clone_cursor(),effect)? else { return Ok(RetainedCloneStep::Progress(Default::default())); }; self.candidate = Some(owner);
                self.phase = 4;
                Ok(payload_progress(size_of::<Puzzle2dSnapshot>()))
            }
            4 => {
                let step=cursor_retire(self.clone_cursor(),effect)?;
                if self.clone_cursor().terminal_is_empty() { self.stage = Puzzle2dCandidateStage::Idle; self.phase = 5; }
                Ok(RetainedCloneStep::Progress(step.progress()))
            }
            5 => {
                if let Some(step)=self.edge().ensure(effect)? {return Ok(step);}
                let step = self.edge().owner_mut().advance(mutation, body,effect)?;
                if matches!(step, RetainedCloneStep::Complete(_)) { self.phase = 6; }
                Ok(RetainedCloneStep::Progress(step.progress()))
            }
            6 => {
                if grant.maximum_copy_bytes < size_of::<Puzzle2dEdge>()||!pay(effect)? { return Ok(RetainedCloneStep::Progress(Default::default())); }
                let taken = self.edge().owner_mut().take().ok_or_else(|| refusal("connect candidate edge completed without its owner"))?;self.pending = Some(taken);
                self.edge().begin_close();self.phase = 7;
                Ok(payload_progress(size_of::<Puzzle2dEdge>()))
            }
            7 => {
                let step = settled_step(self.edge().close_step(effect))?;
                if self.edge().terminal_is_empty() { self.stage = Puzzle2dCandidateStage::Idle; self.phase = 8; }
                Ok(RetainedCloneStep::Progress(step.progress()))
            }
            8 => {
                if grant.maximum_copy_bytes < size_of::<PagedListEditCursor>()||!pay(effect)? { return Ok(RetainedCloneStep::Progress(Default::default())); }
                let position = self.plan.and_then(|plan| plan.position).ok_or_else(|| refusal("connect candidate has no admitted insertion position"))?;
                let length = self.candidate.as_ref().ok_or_else(|| refusal("connect candidate lost its snapshot"))?.edges.len();
                self.insertion = Some(PagedListEditCursor::insert(position, length));self.phase = 9;
                Ok(payload_progress(size_of::<PagedListEditCursor>()))
            }
            9 => {
                let state = &mut **self;
                let edges = &mut state.candidate.as_mut().ok_or_else(|| refusal("connect insertion lost its snapshot"))?.edges;
                let insertion = state.insertion.as_mut().ok_or_else(|| refusal("connect insertion lost its ordered cursor"))?;
                let Some(step) = edit_step(insertion, edges, &mut state.pending, effect)? else { return Ok(RetainedCloneStep::Progress(Default::default())); };
                if step.complete { state.phase = 10; }
                Ok(RetainedCloneStep::Progress(edit_progress(step)))
            }
            10 => { if !pay(effect)? { return Ok(RetainedCloneStep::Progress(Default::default())); } self.insertion = None;self.phase = 11;Ok(payload_progress(0)) }
            11 => {
                if grant.maximum_copy_bytes < size_of::<Puzzle2dConnectHandlesCandidate>()||!pay(effect)? { return Ok(RetainedCloneStep::Progress(Default::default())); }
                self.phase = 12;
                Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<Puzzle2dConnectHandlesCandidate>(), retained_capacity_bytes: 0, released_bytes: 0 }))
            }
            12 => Ok(RetainedCloneStep::Complete(Default::default())),
            _ => Err(refusal("connect candidate has an unknown ownership phase")),
        }
    }

    pub fn take(&mut self) -> Option<Puzzle2dConnectHandlesCandidate> { if self.closing || self.phase != 12 { return None; } self.phase = 13;Some(Puzzle2dConnectHandlesCandidate { plan: self.plan.take()?, snapshot: self.candidate.take() }) }
    pub fn begin_close(&mut self) { self.closing = true;match &mut self.stage { Puzzle2dCandidateStage::Prepare(child) => child.begin_close(), Puzzle2dCandidateStage::Edge(child) => child.begin_close(), _ => {} } }

    fn close_demand(&self,axis:Puzzle2dCloseAxis)->Result<usize,ValueError>{
        match &self.stage {
            Puzzle2dCandidateStage::Prepare(child) if !child.terminal_is_empty() => return match axis {Puzzle2dCloseAxis::Copy=>child.next_close_copy_byte_demand(),Puzzle2dCloseAxis::Capacity(body)=>child.next_close_capacity_byte_demand(body),Puzzle2dCloseAxis::Release=>child.next_close_release_byte_demand(),Puzzle2dCloseAxis::Depth=>child.next_close_depth_demand()},
            Puzzle2dCandidateStage::Clone(cursor) if !cursor.terminal_is_empty() => return axis.retained::<Puzzle2dSnapshot,_>(cursor),
            Puzzle2dCandidateStage::Edge(child) if !child.terminal_is_empty() => return match axis {Puzzle2dCloseAxis::Copy=>child.next_close_copy_byte_demand(),Puzzle2dCloseAxis::Capacity(body)=>child.next_close_capacity_byte_demand(body),Puzzle2dCloseAxis::Release=>child.next_close_release_byte_demand(),Puzzle2dCloseAxis::Depth=>child.next_close_depth_demand()},
            Puzzle2dCandidateStage::CloseEdge(owner) => return axis.retirement(owner),
            _ => {}
        }
        if self.pending.is_some(){return axis.inline::<Puzzle2dEdge>()}
        if let Puzzle2dCandidateStage::CloseSnapshot(owner)=&self.stage{return axis.retirement(owner)}
        if self.candidate.is_some(){return axis.inline::<Puzzle2dSnapshot>()}
        if self.insertion.is_some(){return axis.inline::<Option<PagedListEditCursor>>()}
        if self.plan.is_some(){return axis.inline::<Option<Puzzle2dConnectHandlesPlan>>()}
        axis.binding(if self.source.is_some(){&self.source}else{&self.mutation})
    }

    pub fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{self.close_demand(Puzzle2dCloseAxis::Copy)}
    pub fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{self.close_demand(Puzzle2dCloseAxis::Capacity(body))}
    pub fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{self.close_demand(Puzzle2dCloseAxis::Release)}
    pub fn next_close_depth_demand(&self)->Result<usize,ValueError>{self.close_demand(Puzzle2dCloseAxis::Depth)}

    pub fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{let result=self.close_stepped(effect);RetirementPollExit::settle(effect,result)}
    fn close_stepped(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,RetirementPollExit>{
        let grant=effect.grant();
        if !self.closing { return Err(refusal("connect candidate closure was not started").into()); }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        match &mut self.stage {
            Puzzle2dCandidateStage::Prepare(child) if !child.terminal_is_empty() => return Ok(RetainedCloneStep::Progress(RetirementPollExit::resume(child.close_step(effect))?.progress())),
            Puzzle2dCandidateStage::Clone(cursor) if !cursor.terminal_is_empty() => return Ok(RetainedCloneStep::Progress(cursor_retire(cursor,effect)?.progress())),
            Puzzle2dCandidateStage::Edge(child) if !child.terminal_is_empty() => return RetirementPollExit::resume(child.close_step(effect)).map(|step| RetainedCloneStep::Progress(step.progress())),
            Puzzle2dCandidateStage::CloseEdge(owner) => { let step = controlled_step(owner,effect)?; if owner.terminal_is_empty() { self.stage = Puzzle2dCandidateStage::Idle; } return Ok(RetainedCloneStep::Progress(step.progress())); }
            Puzzle2dCandidateStage::CloseSnapshot(owner) => { let step = controlled_step(owner,effect)?; if owner.terminal_is_empty() { self.stage = Puzzle2dCandidateStage::Idle; } return Ok(RetainedCloneStep::Progress(step.progress())); }
            _ => {}
        }
        if self.pending.is_some() {
            if grant.maximum_copy_bytes < size_of::<Puzzle2dEdge>()||!pay(effect)? { return Ok(RetainedCloneStep::Progress(Default::default())); }
            match ControlledRetirement::new(self.pending.take().unwrap()) { Ok(owner) => self.stage = Puzzle2dCandidateStage::CloseEdge(owner), Err((error, owner)) => { self.pending = Some(owner);return Err(error.into()); } }
            return Ok(payload_progress(size_of::<Puzzle2dEdge>()));
        }
        if !matches!(self.stage, Puzzle2dCandidateStage::Idle|Puzzle2dCandidateStage::CloseSnapshot(_)) { self.stage = Puzzle2dCandidateStage::Idle; }
        if self.candidate.is_some() {
            if grant.maximum_copy_bytes < size_of::<Puzzle2dSnapshot>()||!pay(effect)? { return Ok(RetainedCloneStep::Progress(Default::default())); }
            match ControlledRetirement::new(self.candidate.take().unwrap()) { Ok(owner) => self.stage = Puzzle2dCandidateStage::CloseSnapshot(owner), Err((error, owner)) => { self.candidate = Some(owner);return Err(error.into()); } }
            return Ok(payload_progress(size_of::<Puzzle2dSnapshot>()));
        }
        if self.insertion.is_some() { let bytes=size_of::<Option<PagedListEditCursor>>();if grant.maximum_copy_bytes<bytes||!pay(effect)?{return Ok(RetainedCloneStep::Progress(Default::default()));}self.insertion=None;return Ok(payload_progress(bytes)); }
        if self.plan.is_some() { let bytes=size_of::<Option<Puzzle2dConnectHandlesPlan>>();if grant.maximum_copy_bytes<bytes||!pay(effect)?{return Ok(RetainedCloneStep::Progress(Default::default()));}self.plan=None;return Ok(payload_progress(bytes)); }
        let step = RetirementPollExit::resume(RetainedCloneBinding::close_one(if self.source.is_some(){&mut self.source}else{&mut self.mutation},grant.maximum_copy_bytes,effect))?;
        Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { close_progress(step) })
    }

    pub fn terminal_is_empty(&self) -> bool { self.closing && self.stage.terminal_is_empty() && self.plan.is_none() && self.candidate.is_none() && self.pending.is_none() && self.insertion.is_none() && self.source.is_none() && self.mutation.is_none() }
}

impl Drop for Puzzle2dConnectHandlesCandidateCursor { fn drop(&mut self) { let empty = self.terminal_is_empty();assert!(std::thread::panicking() || empty, "connect candidate abandoned before controlled closure");if empty { unsafe { ManuallyDrop::drop(&mut self.state); } } } }

fn payload_progress(bytes: usize) -> RetainedCloneStep { RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, retained_capacity_bytes: 0, released_bytes: 0 }) }
fn close_progress(step: RetainedCloneStep) -> RetainedCloneStep { RetainedCloneStep::Progress(step.progress()) }
fn refusal(message: &str) -> ValueError { ValueError::new(ValueRefusalKind::InvariantViolated, message) }


