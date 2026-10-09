//! 🗑️ Original Discard ownership commits separately from admitted displaced-session retirement.
use crate::{TimeTravelBase,TimeTravelEvent,TimeTravelPending,TimeTravelRefusal,TimeTravelSession,TimeTravelStage};
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retirement::controlled::ControlledRetirement,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use std::mem::size_of;

/// 🎟️ Keeps the original command and displaced native fields until each granted frontier empties.
pub struct TimeTravelDiscardCursor {
    event:Option<TimeTravelEvent>,
    bound:Option<(usize,u64,u32,TimeTravelBase)>,
    planned_stage:Option<TimeTravelStage>,
    session:Option<ControlledRetirement<TimeTravelSession>>,
    pending:Option<ControlledRetirement<TimeTravelPending>>,
    phase:u8,
    closing:bool,
    result:Option<Result<TimeTravelStage,TimeTravelRefusal>>,
    done:bool,
}
impl TimeTravelDiscardCursor {
    pub fn admission_copy_bytes()->usize{size_of::<Option<TimeTravelEvent>>()+size_of::<Self>()}
    /// 📨️ Transfers only a supported original scalar event after item and depth admission.
    pub fn admit_original(event:&mut Option<TimeTravelEvent>,grant:RetainedCloneGrant)->Result<Option<(Self,RetainedCloneProgress)>,ValueError>{
        if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<Self::admission_copy_bytes(){return Ok(None)}
        if !matches!(event,Some(TimeTravelEvent::Discard{..})){return Err(unsupported("discard owner requires its original Discard command"))}
        Ok(Some((Self{event:event.take(),bound:None,planned_stage:None,session:None,pending:None,phase:0,closing:false,result:None,done:false},copied(Self::admission_copy_bytes()))))
    }
    /// 📏️ Borrows every actual next currency without granting or changing custody.
    pub fn retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if self.done{return Ok(RetirementDemand::default())}
        if let Some(copy_bytes)=self.close_header_copy_bytes(){return Ok(RetirementDemand{copy_bytes,depth:1,..Default::default()})}
        if let Some(owner)=self.session.as_ref().filter(|owner|!owner.terminal_is_empty()){return demand(owner,body)}
        if let Some(owner)=self.pending.as_ref().filter(|owner|!owner.terminal_is_empty()){return demand(owner,body)}
        Ok(RetirementDemand{depth:1,..Default::default()})
    }
    pub fn is_validating(&self)->bool{!self.closing&&!self.done&&self.phase==0}
    pub fn prepared_stage(&self)->Option<TimeTravelStage>{if !self.closing&&self.phase==1{self.planned_stage}else{None}}
    fn validation_copy_bytes(&self,session:&TimeTravelSession)->usize{
        if !matches!(self.event.as_ref(),Some(TimeTravelEvent::Discard{generation})if *generation==session.generation)||session.stage!=TimeTravelStage::Editing||session.pending.is_none(){size_of::<Option<Result<TimeTravelStage,TimeTravelRefusal>>>()+size_of::<u8>()}else{size_of::<Option<(usize,u64,u32,TimeTravelBase)>>()+size_of::<Option<TimeTravelStage>>()+size_of::<u8>()}
    }
    pub fn validation_demands(&self,session:&TimeTravelSession)->RetirementDemand{RetirementDemand{copy_bytes:self.validation_copy_bytes(session),depth:1,..Default::default()}}
    pub fn commit_demands(&self,session:&TimeTravelSession)->Result<RetirementDemand,ValueError>{
        if self.closing||self.phase!=1{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"Discard requires its validated original before explicit commit"))}
        if self.bound!=Some((session as *const _ as usize,session.id,session.generation,session.base))||session.stage!=TimeTravelStage::Editing{return Err(unsupported("Discard session authority changed before ownership transfer"))}
        let pending=session.pending.as_ref().ok_or_else(||unsupported("Discard pending original is absent"))?;
        let fields=if pending.return_stage==TimeTravelStage::Reviewing{size_of::<Option<TimeTravelPending>>()+size_of::<Option<ControlledRetirement<TimeTravelPending>>>()+size_of::<TimeTravelStage>()}else{2*size_of::<TimeTravelSession>()+size_of::<Option<ControlledRetirement<TimeTravelSession>>>()};
        Ok(RetirementDemand{copy_bytes:fields+size_of::<Option<Result<TimeTravelStage,TimeTravelRefusal>>>()+size_of::<u8>(),depth:1,..Default::default()})
    }
    fn close_header_copy_bytes(&self)->Option<usize>{
        if self.done{return Some(0)}
        if !self.closing{return Some(size_of::<bool>()+if self.phase<2{size_of::<u8>()}else{0})}
        if let Some(owner)=self.session.as_ref(){return owner.terminal_is_empty().then_some(size_of::<Option<ControlledRetirement<TimeTravelSession>>>())}
        if let Some(owner)=self.pending.as_ref(){return owner.terminal_is_empty().then_some(size_of::<Option<ControlledRetirement<TimeTravelPending>>>())}
        Some(size_of::<Option<TimeTravelEvent>>()+size_of::<Option<(usize,u64,u32,TimeTravelBase)>>()+size_of::<Option<TimeTravelStage>>()+size_of::<bool>())
    }
    /// 🪪️ Leaves the original Session unchanged while its Store effects are separately admitted.
    pub fn validate(&mut self,session:&TimeTravelSession,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        if !self.is_validating(){return Ok(idle())}
        let demand=self.validation_demands(session);
        if !admits(demand,grant){return Ok(idle())}
            let Some(TimeTravelEvent::Discard{generation})=self.event.as_ref() else{return Err(unsupported("original Discard command is absent"))};
            if *generation!=session.generation{self.result=Some(Err(TimeTravelRefusal::Stale));self.phase=3;return Ok(RetainedCloneStep::Progress(copied(demand.copy_bytes)))}
            let Some(pending)=session.pending.as_ref().filter(|_|session.stage==TimeTravelStage::Editing) else{self.result=Some(Err(TimeTravelRefusal::Illegal));self.phase=3;return Ok(RetainedCloneStep::Progress(copied(demand.copy_bytes)))};
            if pending.return_stage==TimeTravelStage::Reviewing&&session.report.is_none()&&!session.accepted.is_empty(){return Err(unsupported("Discard replay input assembly has no bounded authority"))}
            self.bound=Some((session as *const _ as usize,session.id,session.generation,session.base));self.planned_stage=Some(if pending.return_stage==TimeTravelStage::Reviewing{TimeTravelStage::Reviewing}else{TimeTravelStage::Inactive});self.phase=1;
            Ok(RetainedCloneStep::Progress(copied(demand.copy_bytes)))
    }
    /// 🎟️ The caller commits only after its original Store effects have reached admitted custody.
    pub fn commit(&mut self,session:&mut TimeTravelSession,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        let demand=self.commit_demands(session)?;
        if !admits(demand,grant){return Ok(idle())}
        if self.closing||self.phase!=1{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"Discard requires its validated original before explicit commit"))}
        if self.bound!=Some((session as *const _ as usize,session.id,session.generation,session.base))||session.stage!=TimeTravelStage::Editing{return Err(unsupported("Discard session authority changed before ownership transfer"))}
        let Some(pending)=session.pending.as_ref() else{return Err(unsupported("Discard pending original is absent"))};
        let next_stage=if pending.return_stage==TimeTravelStage::Reviewing{TimeTravelStage::Reviewing}else{TimeTravelStage::Inactive};
        if self.planned_stage!=Some(next_stage){return Err(unsupported("Discard prospective Store stage changed before original ownership transfer"))}
        if pending.return_stage==TimeTravelStage::Reviewing{
            if session.report.is_none()&&!session.accepted.is_empty(){return Err(unsupported("Discard replay input assembly has no bounded authority"))}
            let original=session.pending.take().unwrap();
            match ControlledRetirement::new(original){Ok(owner)=>self.pending=Some(owner),Err((error,original))=>{session.pending=Some(original);return Err(error)}}
            session.stage=TimeTravelStage::Reviewing;
        }else{
            let next=TimeTravelSession{id:session.id,generation:session.generation.wrapping_add(1),base:session.base,..Default::default()};
            let original=std::mem::replace(session,next);
            match ControlledRetirement::new(original){Ok(owner)=>self.session=Some(owner),Err((error,original))=>{*session=original;return Err(error)}}
        }
        self.result=Some(Ok(session.stage));self.phase=2;
        Ok(RetainedCloneStep::Progress(copied(demand.copy_bytes)))
    }
    /// 🛑️ Cancels before publication or continues closing an already committed original residue.
    pub fn begin_close(&mut self){self.closing=true;if self.phase<2{self.phase=3}}
    pub fn is_closing(&self)->bool{self.closing}
    pub fn result(&self)->Option<Result<TimeTravelStage,TimeTravelRefusal>>{if self.terminal_is_empty(){self.result}else{None}}
    pub fn terminal_is_empty(&self)->bool{self.done&&self.event.is_none()&&self.bound.is_none()&&self.planned_stage.is_none()&&self.session.is_none()&&self.pending.is_none()}
    fn residue(&self)->Option<&dyn semio_framework_value::ErasedSnapshotRetirement>{
        if let Some(owner)=self.session.as_ref().filter(|owner|!owner.terminal_is_empty()){return Some(owner)}
        self.pending.as_ref().filter(|owner|!owner.terminal_is_empty()).map(|owner|owner as &dyn semio_framework_value::ErasedSnapshotRetirement)
    }
    /// ♻️ One granted original child unit precedes a distinct zero-allocation header turn.
    pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        if self.done{return Ok(RetainedCloneStep::Complete(Default::default()))}
        let demand=self.retirement_demands(grant.maximum_copy_bytes)?;
        if !admits(demand,grant){return Ok(idle())}
        if !self.closing{self.begin_close();return Ok(RetainedCloneStep::Progress(copied(demand.copy_bytes)))}
        if let Some(owner)=self.session.as_mut(){if !owner.terminal_is_empty(){let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};let step=owner.step(child)?;let step=semio_framework_value::retained_clone::admit_retained_clone_close(child,step,owner.terminal_is_empty(),"original Discard Session")?;return Ok(RetainedCloneStep::Progress(step.progress()))}self.session=None;return Ok(RetainedCloneStep::Progress(copied(demand.copy_bytes)))}
        if let Some(owner)=self.pending.as_mut(){if !owner.terminal_is_empty(){let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};let step=owner.step(child)?;let step=semio_framework_value::retained_clone::admit_retained_clone_close(child,step,owner.terminal_is_empty(),"original Discard pending")?;return Ok(RetainedCloneStep::Progress(step.progress()))}self.pending=None;return Ok(RetainedCloneStep::Progress(copied(demand.copy_bytes)))}
        self.event=None;self.bound=None;self.planned_stage=None;self.done=true;
        Ok(RetainedCloneStep::Complete(copied(demand.copy_bytes)))
    }
}
impl semio_framework_value::ErasedSnapshotRetirement for TimeTravelDiscardCursor {
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{Self::close_step(self,grant)}
    fn next_copy_byte_demand(&self)->Result<usize,ValueError>{self.close_header_copy_bytes().map_or_else(||self.residue().map_or(Ok(0),|owner|owner.next_copy_byte_demand()),Ok)}
    fn next_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{if self.close_header_copy_bytes().is_some(){return Ok(0)}self.residue().map_or(Ok(0),|owner|owner.next_capacity_byte_demand(body))}
    fn next_release_byte_demand(&self)->Result<usize,ValueError>{if self.close_header_copy_bytes().is_some(){return Ok(0)}self.residue().map_or(Ok(0),|owner|owner.next_release_byte_demand())}
    fn next_depth_demand(&self)->Result<usize,ValueError>{if self.done{return Ok(0)}if self.close_header_copy_bytes().is_some(){return Ok(1)}self.residue().map_or(Ok(1),|owner|owner.next_depth_demand()?.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"Discard child depth overflow")))}
    fn terminal_is_empty(&self)->bool{Self::terminal_is_empty(self)}
}
impl Drop for TimeTravelDiscardCursor{fn drop(&mut self){assert!(self.terminal_is_empty(),"original Discard custody dropped before admitted terminal closure")}}
fn demand<T:semio_framework_value::retirement::RetireOwned>(owner:&ControlledRetirement<T>,body:usize)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(body)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"Discard child depth overflow"))?})}
fn admits(demand:RetirementDemand,grant:RetainedCloneGrant)->bool{grant.maximum_items>0&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth}
fn item()->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,..Default::default()}}
fn copied(copied_bytes:usize)->RetainedCloneProgress{RetainedCloneProgress{copied_bytes,..item()}}
fn idle()->RetainedCloneStep{RetainedCloneStep::Progress(Default::default())}
fn unsupported(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::UnsupportedOwner,message)}
