//! 🩹️ Atomic three-map candidates retain first-party inputs and compose actual typed ownership.

use super::{DomainSelection,LocalInteractionRoot,SelectionMode};
use super::super::LocalInteractionDomainPatch;
use crate::value::{ordered::{SharedOwner,UpdateCursor},retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep},retirement::{RetireOwned,RetirementCursor,controlled::ControlledRetirement},ValueError,ValueRefusalKind};
use std::mem::ManuallyDrop;

#[derive(Default)]
struct PatchState {selection:Option<SharedOwner<DomainSelection>>,mode:Option<SharedOwner<SelectionMode>>,granularity:Option<SharedOwner<String>>}
impl PatchState {fn is_empty(&self)->bool{self.selection.is_none()&&self.mode.is_none()&&self.granularity.is_none()}}
crate::value::artifact_retire_struct!(PatchState {selection,mode,granularity});

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
    fn retirement(mut self)->Box<dyn RetirementCursor>{std::mem::take(&mut *self.owned).retirement()}
    fn retirement_birth_bytes(&self)->Option<usize>{self.owned.retirement_birth_bytes()}
    fn controlled_retirement_supported()->bool{true}
}
impl Drop for LocalInteractionRootPatch {
    fn drop(&mut self){if !self.owned.is_empty(){assert!(std::thread::panicking(),"interaction patch abandoned before terminal ownership");return;}unsafe{ManuallyDrop::drop(&mut self.owned);}}
}

enum MapUpdate {Selection(UpdateCursor<DomainSelection>),Mode(UpdateCursor<SelectionMode>),Granularity(UpdateCursor<String>)}
impl MapUpdate {
    fn is_complete(&self)->bool{match self{Self::Selection(cursor)=>cursor.is_complete(),Self::Mode(cursor)=>cursor.is_complete(),Self::Granularity(cursor)=>cursor.is_complete()}}
    fn advance(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{match self{Self::Selection(cursor)=>cursor.advance(grant),Self::Mode(cursor)=>cursor.advance(grant),Self::Granularity(cursor)=>cursor.advance(grant)}}
    fn copy_demand(&self)->usize{match self{Self::Selection(cursor)=>cursor.next_copy_byte_demand(),Self::Mode(cursor)=>cursor.next_copy_byte_demand(),Self::Granularity(cursor)=>cursor.next_copy_byte_demand()}}
    fn capacity_demand(&self)->Result<usize,ValueError>{match self{Self::Selection(cursor)=>cursor.next_capacity_byte_demand(),Self::Mode(cursor)=>cursor.next_capacity_byte_demand(),Self::Granularity(cursor)=>cursor.next_capacity_byte_demand()}}
    fn depth_demand(&self)->usize{match self{Self::Selection(cursor)=>cursor.next_depth_demand(),Self::Mode(cursor)=>cursor.next_depth_demand(),Self::Granularity(cursor)=>cursor.next_depth_demand()}}
}
impl RetireOwned for MapUpdate {
    fn retirement(self)->Box<dyn RetirementCursor>{match self{Self::Selection(cursor)=>cursor.retirement(),Self::Mode(cursor)=>cursor.retirement(),Self::Granularity(cursor)=>cursor.retirement()}}
    fn retirement_birth_bytes(&self)->Option<usize>{match self{Self::Selection(cursor)=>cursor.retirement_birth_bytes(),Self::Mode(cursor)=>cursor.retirement_birth_bytes(),Self::Granularity(cursor)=>cursor.retirement_birth_bytes()}}
    fn controlled_retirement_supported()->bool{true}
}

#[derive(Default)]
struct CandidateCleanup {selection:Option<crate::value::ordered::OrderedMap<DomainSelection>>,mode:Option<crate::value::ordered::OrderedMap<SelectionMode>>,granularity:Option<crate::value::ordered::OrderedMap<String>>,current:Option<MapUpdate>}
crate::value::artifact_retire_struct!(CandidateCleanup {selection,mode,granularity,current});

struct UpdateState {candidate:Option<LocalInteractionRoot>,domain:Option<SharedOwner<String>>,patch:PatchState,current:Option<MapUpdate>,pending:Option<CandidateCleanup>,cleanup:Option<ControlledRetirement<CandidateCleanup>>,phase:u8}
impl RetireOwned for UpdateState {
    fn retirement(self)->Box<dyn RetirementCursor>{let Self {candidate,domain,patch,current,pending,cleanup,..}=self;crate::value::retirement::sequence(vec![crate::value::retirement::deferred(candidate),crate::value::retirement::deferred(domain),crate::value::retirement::deferred(patch),crate::value::retirement::deferred(current),crate::value::retirement::deferred(pending),crate::value::retirement::deferred(cleanup)])}
    fn retirement_birth_bytes(&self)->Option<usize>{crate::value::retirement::sequence_birth_bytes(&[crate::value::retirement::deferred_birth_bytes_for(&self.candidate),crate::value::retirement::deferred_birth_bytes_for(&self.domain),crate::value::retirement::deferred_birth_bytes_for(&self.patch),crate::value::retirement::deferred_birth_bytes_for(&self.current),crate::value::retirement::deferred_birth_bytes_for(&self.pending),crate::value::retirement::deferred_birth_bytes_for(&self.cleanup)])}
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
    pub fn advance(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        if self.is_complete(){return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()));}
        if grant.maximum_items==0{return Ok(paused());}
        if grant.maximum_depth<self.next_depth_demand()?{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"interaction update exceeds admitted depth"));}
        let OwnerState::Running(state)=&mut *self.owned else{return Ok(paused());};
        if let Some(owner)=state.cleanup.as_mut(){if !owner.terminal_is_empty(){return owner.step(RetainedCloneGrant {maximum_depth:grant.maximum_depth-1,..grant});}state.cleanup.take();state.phase+=1;return Ok(metadata());}
        if let Some(pending)=state.pending.take(){match ControlledRetirement::new(pending){Ok(owner)=>state.cleanup=Some(owner),Err((error,pending))=>{state.pending=Some(pending);return Err(error);}}return Ok(metadata());}
        if let Some(current)=state.current.as_mut(){if !current.is_complete(){return current.advance(RetainedCloneGrant {maximum_depth:grant.maximum_depth-1,..grant});}}
        if let Some(mut current)=state.current.take(){
            let root=state.candidate.as_mut().unwrap();let mut cleanup=CandidateCleanup::default();
            match &mut current{MapUpdate::Selection(cursor)=>cleanup.selection=Some(std::mem::replace(&mut root.selection,cursor.take_result().unwrap())),MapUpdate::Mode(cursor)=>cleanup.mode=Some(std::mem::replace(&mut root.active_mode,cursor.take_result().unwrap())),MapUpdate::Granularity(cursor)=>cleanup.granularity=Some(std::mem::replace(&mut root.active_granularity,cursor.take_result().unwrap()))}
            cleanup.current=Some(current);state.pending=Some(cleanup);return Ok(metadata());
        }
        let root=state.candidate.as_ref().unwrap();let domain=state.domain.as_ref().unwrap().clone();
        state.current=Some(match state.phase{0=>MapUpdate::Selection(match state.patch.selection.take(){Some(value)=>root.selection.begin_set_shared(domain,value),None=>root.selection.begin_remove_shared(domain)}),1=>MapUpdate::Mode(match state.patch.mode.take(){Some(value)=>root.active_mode.begin_set_shared(domain,value),None=>root.active_mode.begin_remove_shared(domain)}),2=>MapUpdate::Granularity(match state.patch.granularity.take(){Some(value)=>root.active_granularity.begin_set_shared(domain,value),None=>root.active_granularity.begin_remove_shared(domain)}),_=>unreachable!()});Ok(metadata())
    }
    /// 🛑️ Cancellation transfers all partial state inline to the same defining typed retirement driver.
    pub fn begin_close(&mut self)->Result<(),ValueError>{
        let state=std::mem::replace(&mut *self.owned,OwnerState::Empty);
        match state{OwnerState::Running(state)=>match ControlledRetirement::new(state){Ok(owner)=>*self.owned=OwnerState::Closing(owner),Err((error,state))=>{*self.owned=OwnerState::Running(state);return Err(error);}},state=>*self.owned=state}Ok(())
    }
    pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{match &mut *self.owned{OwnerState::Closing(owner)=>owner.step(grant),OwnerState::Empty=>Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default())),_=>Ok(paused())}}
}
impl Drop for LocalInteractionRootUpdate {
    fn drop(&mut self){if !self.terminal_is_empty(){assert!(std::thread::panicking(),"interaction candidate abandoned before terminal ownership");return;}unsafe{ManuallyDrop::drop(&mut self.owned);}}
}
