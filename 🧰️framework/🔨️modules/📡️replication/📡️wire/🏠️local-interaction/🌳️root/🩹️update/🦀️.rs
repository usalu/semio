//! ⏮️ Atomic three-map candidates retain first-party inputs and compose actual typed ownership.

use super::{DomainSelection,LocalInteractionRoot,SelectionMode};
use super::super::LocalInteractionDomainPatch;
use semio_framework_value_ordered::ordered::{SharedOwner, UpdateCursor};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::retirement::RetireOwned;
use semio_framework_value::retirement::controlled::ControlledRetirement;
use semio_framework_value::{OriginalRetirementPoll, RetainedAllocationEffect, RetainedWorkOutcome, RetainedWorkPoll, ValueError, ValueRefusalKind};
use std::mem::ManuallyDrop;

#[doc(hidden)]
#[derive(Default)]
pub struct PatchState {selection:Option<SharedOwner<DomainSelection>>,mode:Option<SharedOwner<SelectionMode>>,granularity:Option<SharedOwner<String>>}
impl PatchState {fn is_empty(&self)->bool{self.selection.is_none()&&self.mode.is_none()&&self.granularity.is_none()}}
semio_framework_value::artifact_retire_struct!(PatchState {selection,mode,granularity});

#[must_use="interaction patch inputs require transfer or typed retirement"]
pub struct LocalInteractionRootPatch {owned:ManuallyDrop<PatchState>}
impl LocalInteractionRootPatch {
    /// 📥️ Moves already admitted immutable owners without allocation or payload copies.
    pub fn from_shared(selection:Option<SharedOwner<DomainSelection>>,mode:Option<SharedOwner<SelectionMode>>,granularity:Option<SharedOwner<String>>)->Self {Self {owned:ManuallyDrop::new(PatchState {selection,mode,granularity})}}
    /// 🧊️ Cold input construction allocates shared headers without retained credit.
    pub fn from_cold(patch:LocalInteractionDomainPatch)->Self {Self::from_shared(patch.selection.map(SharedOwner::from_cold),patch.active_mode.map(SharedOwner::from_cold),patch.active_granularity.map(SharedOwner::from_cold))}
    pub fn selection(&self)->Option<&SharedOwner<DomainSelection>>{self.owned.selection.as_ref()}
    pub fn retire(self)->Result<ControlledRetirement<Self>,(ValueError,Self)>{ControlledRetirement::new(self)}
}
impl RetireOwned for LocalInteractionRootPatch {
    type RetirementCursor=<PatchState as RetireOwned>::RetirementCursor;
    fn retirement_seed(mut self)->Self::RetirementCursor{std::mem::take(&mut *self.owned).retirement_seed()}
    fn retirement_birth_bytes(&self)->Option<usize>{self.owned.retirement_birth_bytes()}
    fn controlled_retirement_supported()->bool{true}
}
impl Drop for LocalInteractionRootPatch {
    fn drop(&mut self){if !self.owned.is_empty(){assert!(std::thread::panicking(),"interaction patch abandoned before terminal ownership");return;}unsafe{ManuallyDrop::drop(&mut self.owned);}}
}

struct MapUpdate {selection:Option<UpdateCursor<DomainSelection>>,mode:Option<UpdateCursor<SelectionMode>>,granularity:Option<UpdateCursor<String>>}
semio_framework_value::artifact_retire_struct!(MapUpdate {selection,mode,granularity});
impl MapUpdate {
    fn selection(cursor:UpdateCursor<DomainSelection>)->Self {Self {selection:Some(cursor),mode:None,granularity:None}}
    fn mode(cursor:UpdateCursor<SelectionMode>)->Self {Self {selection:None,mode:Some(cursor),granularity:None}}
    fn granularity(cursor:UpdateCursor<String>)->Self {Self {selection:None,mode:None,granularity:Some(cursor)}}
    fn is_complete(&self)->bool {self.selection.as_ref().is_none_or(UpdateCursor::is_complete)&&self.mode.as_ref().is_none_or(UpdateCursor::is_complete)&&self.granularity.as_ref().is_none_or(UpdateCursor::is_complete)}
    fn advance(&mut self,grant:RetainedCloneGrant,maximum_work_units:u64)->RetainedWorkOutcome {
        if let Some(cursor)=self.selection.as_mut() {return cursor.advance(grant,maximum_work_units);}
        if let Some(cursor)=self.mode.as_mut() {return cursor.advance(grant,maximum_work_units);}
        if let Some(cursor)=self.granularity.as_mut() {return cursor.advance(grant,maximum_work_units);}
        RetainedWorkOutcome {result:Ok(paused()),work:0}
    }
    fn copy_demand(&self)->usize {self.selection.as_ref().map_or(0,UpdateCursor::next_copy_byte_demand)+self.mode.as_ref().map_or(0,UpdateCursor::next_copy_byte_demand)+self.granularity.as_ref().map_or(0,UpdateCursor::next_copy_byte_demand)}
    fn capacity_demand(&self)->Result<usize,ValueError> {Ok(self.selection.as_ref().map_or(Ok(0),UpdateCursor::next_capacity_byte_demand)?+self.mode.as_ref().map_or(Ok(0),UpdateCursor::next_capacity_byte_demand)?+self.granularity.as_ref().map_or(Ok(0),UpdateCursor::next_capacity_byte_demand)?)}
    fn depth_demand(&self)->usize {self.selection.as_ref().map_or(0,UpdateCursor::next_depth_demand).max(self.mode.as_ref().map_or(0,UpdateCursor::next_depth_demand)).max(self.granularity.as_ref().map_or(0,UpdateCursor::next_depth_demand))}
}

#[derive(Default)]
struct CandidateCleanup {selection:Option<semio_framework_value_ordered::ordered::OrderedMap<DomainSelection>>,mode:Option<semio_framework_value_ordered::ordered::OrderedMap<SelectionMode>>,granularity:Option<semio_framework_value_ordered::ordered::OrderedMap<String>>,current:Option<MapUpdate>}
semio_framework_value::artifact_retire_struct!(CandidateCleanup {selection,mode,granularity,current});

struct UpdateState {candidate:Option<LocalInteractionRoot>,domain:Option<SharedOwner<String>>,patch:PatchState,current:Option<MapUpdate>,pending:Option<CandidateCleanup>,cleanup:Option<ControlledRetirement<CandidateCleanup>>,phase:u8}
impl RetireOwned for UpdateState {
    type RetirementCursor=semio_framework_value::retirement_fields_type!(Option<LocalInteractionRoot>,Option<SharedOwner<String>>,PatchState,Option<MapUpdate>,Option<CandidateCleanup>,Option<ControlledRetirement<CandidateCleanup>>);
    fn retirement_seed(self)->Self::RetirementCursor{let Self {candidate,domain,patch,current,pending,cleanup,..}=self;semio_framework_value::retirement_fields_seed!(candidate,domain,patch,current,pending,cleanup)}
    fn retirement_birth_bytes(&self)->Option<usize>{semio_framework_value::retirement::sequence_birth_bytes(&[semio_framework_value::retirement::deferred_birth_bytes_for(&self.candidate),semio_framework_value::retirement::deferred_birth_bytes_for(&self.domain),semio_framework_value::retirement::deferred_birth_bytes_for(&self.patch),semio_framework_value::retirement::deferred_birth_bytes_for(&self.current),semio_framework_value::retirement::deferred_birth_bytes_for(&self.pending),semio_framework_value::retirement::deferred_birth_bytes_for(&self.cleanup)])}
    fn controlled_retirement_supported()->bool{true}
}
impl UpdateState {fn complete(&self)->bool{self.phase==3&&self.current.is_none()&&self.pending.is_none()&&self.cleanup.is_none()}}

enum OwnerState {Running(UpdateState),Closing(ControlledRetirement<UpdateState>),Empty}

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum LocalInteractionWork {Metadata,Comparison,Retirement}

#[must_use="interaction candidates require complete publication or typed retirement"]
pub struct LocalInteractionRootUpdate {owned:ManuallyDrop<OwnerState>}
impl LocalInteractionRoot {
    /// 🩹️ Captures exact immutable inputs and three root aliases without traversing source payloads.
    pub fn begin_domain_patch(&self,domain:SharedOwner<String>,mut patch:LocalInteractionRootPatch)->LocalInteractionRootUpdate {
        LocalInteractionRootUpdate {owned:ManuallyDrop::new(OwnerState::Running(UpdateState {candidate:Some(self.clone()),domain:Some(domain),patch:std::mem::take(&mut *patch.owned),current:None,pending:None,cleanup:None,phase:0}))}
    }
}
fn metadata()->RetainedCloneStep{RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,..Default::default()})}
/// 💰️ Records one pure bounded action on the caller's effect exactly once.
fn pay(effect:&mut RetainedAllocationEffect<'_>,work:&mut u128)->Result<(),ValueError>{match semio_framework_value::retirement::record_retirement_work(effect,1){Ok(paid)=>{*work+=paid;Ok(())},Err((error,paid))=>{*work+=paid;Err(error)}}}
/// 💰️ Records the work a grant-shaped ordered cursor already performed onto the caller's effect.
fn owe(effect:&mut RetainedAllocationEffect<'_>,owed:u128,work:&mut u128)->Result<(),ValueError>{if owed==0{return Ok(())}let owed=u64::try_from(owed).map_err(|_|ValueError::literal(ValueRefusalKind::WorkLimit,"interaction cursor work exceeds its counter"))?;pay_many(effect,owed,work)}
fn pay_many(effect:&mut RetainedAllocationEffect<'_>,owed:u64,work:&mut u128)->Result<(),ValueError>{match semio_framework_value::retirement::record_retirement_work(effect,owed){Ok(paid)=>{*work+=paid;Ok(())},Err((error,paid))=>{*work+=paid;Err(error)}}}
fn paused()->RetainedCloneStep{RetainedCloneStep::Progress(RetainedCloneProgress::default())}
impl LocalInteractionRootUpdate {
    pub fn is_complete(&self)->bool{matches!(&*self.owned,OwnerState::Running(state) if state.complete())}
    pub fn take(&mut self)->Option<LocalInteractionRoot>{match &mut *self.owned{OwnerState::Running(state) if state.complete()=>state.candidate.take(),_=>None}}
    pub fn terminal_is_empty(&self)->bool{match &*self.owned{OwnerState::Closing(owner)=>owner.terminal_is_empty(),OwnerState::Empty=>true,_=>false}}
    pub fn work_kind(&self)->LocalInteractionWork{match &*self.owned{OwnerState::Closing(_)=>LocalInteractionWork::Retirement,OwnerState::Running(state) if state.cleanup.is_some()||state.pending.is_some()=>LocalInteractionWork::Retirement,OwnerState::Running(state) if state.current.as_ref().is_some_and(|current|!current.is_complete())=>LocalInteractionWork::Comparison,_=>LocalInteractionWork::Metadata}}
    pub fn next_copy_byte_demand(&self)->Result<usize,ValueError>{match &*self.owned{OwnerState::Closing(owner)=>owner.next_copy_byte_demand(),OwnerState::Running(state)=>state.cleanup.as_ref().map_or_else(||Ok(state.current.as_ref().map_or(0,MapUpdate::copy_demand)),ControlledRetirement::next_copy_byte_demand),OwnerState::Empty=>Ok(0)}}
    pub fn next_capacity_byte_demand(&self,maximum_copy_bytes:usize)->Result<usize,ValueError>{match &*self.owned{OwnerState::Closing(owner)=>owner.next_capacity_byte_demand(maximum_copy_bytes),OwnerState::Running(state)=>state.cleanup.as_ref().map_or_else(||state.current.as_ref().map_or(Ok(0),MapUpdate::capacity_demand),|owner|owner.next_capacity_byte_demand(maximum_copy_bytes)),OwnerState::Empty=>Ok(0)}}
    pub fn next_release_byte_demand(&self)->Result<usize,ValueError>{match &*self.owned{OwnerState::Closing(owner)=>owner.next_release_byte_demand(),OwnerState::Running(state)=>state.cleanup.as_ref().map_or(Ok(0),ControlledRetirement::next_release_byte_demand),OwnerState::Empty=>Ok(0)}}
    pub fn next_depth_demand(&self)->Result<usize,ValueError>{match &*self.owned{OwnerState::Closing(owner)=>owner.next_depth_demand(),OwnerState::Running(state) if state.complete()=>Ok(0),OwnerState::Running(state)=>{let child=state.cleanup.as_ref().map_or_else(||Ok(state.current.as_ref().map_or(0,MapUpdate::depth_demand)),ControlledRetirement::next_depth_demand)?;child.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"interaction nested depth overflow"))},OwnerState::Empty=>Ok(0)}}
    pub fn advance(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll {
        let grant=effect.grant();
        let done=|result:Result<RetainedCloneStep,ValueError>,work:u128|RetainedWorkPoll::Complete(RetainedWorkOutcome {result:result.map_err(Into::into),work});
        if self.is_complete(){return done(Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default())),0);}
        if grant.maximum_items==0||effect.maximum_work_units()==0{return done(Ok(paused()),0);}
        match self.next_depth_demand(){Ok(depth) if grant.maximum_depth<depth=>return done(Err(ValueError::literal(ValueRefusalKind::DepthLimit,"interaction update exceeds admitted depth")),0),Err(error)=>return done(Err(error),0),Ok(_)=>{}}
        let OwnerState::Running(state)=&mut *self.owned else{return done(Ok(paused()),0);};
        let child=RetainedCloneGrant {maximum_depth:grant.maximum_depth.saturating_sub(1),..grant};
        let mut work=0;
        let invalid=|message:&'static str|ValueError::literal(ValueRefusalKind::InvariantViolated,message);
        if let Some(owner)=state.cleanup.as_mut(){
            if !owner.terminal_is_empty(){return owner.step(child.maximum_copy_bytes,&mut effect.nested(child));}
            if let Err(error)=pay(effect,&mut work){return done(Err(error),work);}
            state.cleanup.take();state.phase+=1;return done(Ok(metadata()),work);
        }
        if state.pending.is_some(){
            if let Err(error)=pay(effect,&mut work){return done(Err(error),work);}
            let Some(pending)=state.pending.take()else{return done(Err(invalid("interaction cleanup vanished")),work);};
            match ControlledRetirement::new(pending){Ok(owner)=>state.cleanup=Some(owner),Err((error,pending))=>{state.pending=Some(pending);return done(Err(error),work);}}
            return done(Ok(metadata()),work);
        }
        if let Some(current)=state.current.as_mut(){if !current.is_complete(){
            let turn=current.advance(child,effect.maximum_work_units());
            if let Err(error)=owe(effect,turn.work,&mut work){return done(Err(error),work);}
            return done(turn.result,work);
        }}
        if state.current.is_some(){
            if let Err(error)=pay(effect,&mut work){return done(Err(error),work);}
            let (Some(mut current),Some(root))=(state.current.take(),state.candidate.as_mut())else{return done(Err(invalid("interaction candidate vanished")),work);};
            let mut cleanup=CandidateCleanup::default();
            if let Some(cursor)=current.selection.as_mut(){match cursor.take_result(){Some(result)=>cleanup.selection=Some(std::mem::replace(&mut root.selection,result)),None=>{state.current=Some(current);return done(Err(invalid("interaction selection result is absent")),work);}}}
            else if let Some(cursor)=current.mode.as_mut(){match cursor.take_result(){Some(result)=>cleanup.mode=Some(std::mem::replace(&mut root.active_mode,result)),None=>{state.current=Some(current);return done(Err(invalid("interaction mode result is absent")),work);}}}
            else if let Some(cursor)=current.granularity.as_mut(){match cursor.take_result(){Some(result)=>cleanup.granularity=Some(std::mem::replace(&mut root.active_granularity,result)),None=>{state.current=Some(current);return done(Err(invalid("interaction granularity result is absent")),work);}}}
            cleanup.current=Some(current);state.pending=Some(cleanup);return done(Ok(metadata()),work);
        }
        let (Some(root),Some(domain))=(state.candidate.as_ref(),state.domain.as_ref().cloned())else{return done(Err(invalid("interaction update lost its candidate or domain")),work);};
        if state.phase>2{return done(Err(invalid("interaction update phase is invalid")),work);}
        if let Err(error)=pay(effect,&mut work){return done(Err(error),work);}
        state.current=Some(match state.phase{0=>MapUpdate::selection(match state.patch.selection.take(){Some(value)=>root.selection.begin_set_shared(domain,value),None=>root.selection.begin_remove_shared(domain)}),1=>MapUpdate::mode(match state.patch.mode.take(){Some(value)=>root.active_mode.begin_set_shared(domain,value),None=>root.active_mode.begin_remove_shared(domain)}),_=>MapUpdate::granularity(match state.patch.granularity.take(){Some(value)=>root.active_granularity.begin_set_shared(domain,value),None=>root.active_granularity.begin_remove_shared(domain)})});
        done(Ok(metadata()),work)
    }
    /// 🛑️ Cancellation transfers all partial state inline to the same defining typed retirement driver.
    pub fn begin_close(&mut self)->Result<(),ValueError>{
        let state=std::mem::replace(&mut *self.owned,OwnerState::Empty);
        match state{OwnerState::Running(state)=>match ControlledRetirement::new(state){Ok(owner)=>*self.owned=OwnerState::Closing(owner),Err((error,state))=>{*self.owned=OwnerState::Running(state);return Err(error);}},state=>*self.owned=state}Ok(())
    }
    pub fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll {match &mut *self.owned{OwnerState::Closing(owner)=>owner.step(effect.grant().maximum_copy_bytes,effect),OwnerState::Empty=>RetainedWorkPoll::Complete(RetainedWorkOutcome {result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default())),work:0}),_=>RetainedWorkPoll::Complete(RetainedWorkOutcome {result:Ok(paused()),work:0})}}
}
impl Drop for LocalInteractionRootUpdate {
    fn drop(&mut self){if !self.terminal_is_empty(){assert!(std::thread::panicking(),"interaction candidate abandoned before terminal ownership");return;}unsafe{ManuallyDrop::drop(&mut self.owned);}}
}
