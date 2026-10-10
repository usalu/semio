//! 🎟️ The original paid semantic result remains a small descriptor while payload headers stay inside the producer.
use super::*;
use std::mem::{MaybeUninit};

/// 🚦️ Semantic absence carries no invented empty payload carrier.
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum JobOutcomeKind{Yield,PreviewReady,CheckpointReady{applied_progress:u64},Complete,Cancelled,Fault}

/// 🎟️ Owns the original admission and immutable native payload addresses until a paid acknowledgement.
#[derive(Debug)]
pub struct JobOutcomeDescriptor{kind:JobOutcomeKind,admission:JobOutcomeAdmission,original:[Option<usize>;2],acknowledged:bool}

/// 🤝️ Borrows the same descriptor admission and the exact original producer payloads without another grant.
#[derive(Debug)]
pub enum JobOutcomeView<'a>{
 Yield{admission:&'a JobOutcomeAdmission},
 PreviewReady{payload:&'a RetainedJobPayload,admission:&'a JobOutcomeAdmission},
 CheckpointReady{state:&'a RetainedJobPayload,applied_progress:u64,admission:&'a JobOutcomeAdmission},
 Complete{state:Option<&'a RetainedJobPayload>,output:Option<&'a RetainedJobPayload>,admission:&'a JobOutcomeAdmission},
 Cancelled{admission:&'a JobOutcomeAdmission},
 Fault{detail:&'a RetainedJobPayload,admission:&'a JobOutcomeAdmission},
}
fn address(original:Option<&RetainedJobPayload>)->Option<usize>{original.map(|original|original as*const _ as usize)}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"semantic descriptor requires its original paid result and same native payload")}

impl JobOutcomeDescriptor{
 pub(super) fn from_original(kind:JobOutcomeKind,admission:JobOutcomeAdmission,original:[Option<&RetainedJobPayload>;2])->Self{Self{kind,admission,original:original.map(address),acknowledged:false}}
 pub fn kind(&self)->JobOutcomeKind{self.kind}
 pub fn admission(&self)->&JobOutcomeAdmission{&self.admission}
 pub fn is_terminal(&self)->bool{matches!(self.kind,JobOutcomeKind::Complete|JobOutcomeKind::Cancelled|JobOutcomeKind::Fault)}
 pub fn is_acknowledged(&self)->bool{self.acknowledged}
 fn matches(&self,kind:JobOutcomeKind,original:[Option<&RetainedJobPayload>;2])->Result<(),ValueError>{if self.acknowledged||kind!=self.kind||original.map(address)!=self.original{return Err(invalid())}Ok(())}
 pub fn yielded(&self)->Result<JobOutcomeView<'_>,ValueError>{self.matches(JobOutcomeKind::Yield,[None,None])?;Ok(JobOutcomeView::Yield{admission:&self.admission})}
 pub fn preview<'a>(&'a self,payload:&'a RetainedJobPayload)->Result<JobOutcomeView<'a>,ValueError>{self.matches(JobOutcomeKind::PreviewReady,[Some(payload),None])?;Ok(JobOutcomeView::PreviewReady{payload,admission:&self.admission})}
 pub fn checkpoint<'a>(&'a self,state:&'a RetainedJobPayload)->Result<JobOutcomeView<'a>,ValueError>{let JobOutcomeKind::CheckpointReady{applied_progress}=self.kind else{return Err(invalid())};self.matches(self.kind,[Some(state),None])?;Ok(JobOutcomeView::CheckpointReady{state,applied_progress,admission:&self.admission})}
 pub fn complete<'a>(&'a self,state:Option<&'a RetainedJobPayload>,output:Option<&'a RetainedJobPayload>)->Result<JobOutcomeView<'a>,ValueError>{self.matches(JobOutcomeKind::Complete,[state,output])?;Ok(JobOutcomeView::Complete{state,output,admission:&self.admission})}
 pub fn cancelled(&self)->Result<JobOutcomeView<'_>,ValueError>{self.matches(JobOutcomeKind::Cancelled,[None,None])?;Ok(JobOutcomeView::Cancelled{admission:&self.admission})}
 pub fn fault<'a>(&'a self,detail:&'a RetainedJobPayload)->Result<JobOutcomeView<'a>,ValueError>{self.matches(JobOutcomeKind::Fault,[Some(detail),None])?;Ok(JobOutcomeView::Fault{detail,admission:&self.admission})}
 /// 🔔️ The exclusive result loan ends before its original semantic descriptor acknowledges one metadata unit.
 pub fn acknowledge(&mut self,grant:RetainedCloneGrant)->RetainedCloneStep{if self.acknowledged{return RetainedCloneStep::Complete(Default::default())}if grant.maximum_items==0||grant.maximum_depth==0{return RetainedCloneStep::Progress(Default::default())}self.acknowledged=true;RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:0,..Default::default()})}
}
impl Drop for JobOutcomeDescriptor{fn drop(&mut self){assert!(std::thread::panicking()||self.acknowledged,"original semantic descriptor requires funded acknowledgement before resumption or closure");}}
impl JobOutcomeView<'_>{
 pub fn admission(&self)->&JobOutcomeAdmission{match self{Self::Yield{admission}|Self::PreviewReady{admission,..}|Self::CheckpointReady{admission,..}|Self::Complete{admission,..}|Self::Cancelled{admission}|Self::Fault{admission,..}=>admission}}
 pub fn is_terminal(&self)->bool{matches!(self,Self::Complete{..}|Self::Cancelled{..}|Self::Fault{..})}
}

/// 📬️ The exclusive worker result slot acknowledges its child before a separate metadata removal.
pub(super) struct JobOutcomeDescriptorSlot{original:MaybeUninit<JobOutcomeDescriptor>,present:bool}
impl JobOutcomeDescriptorSlot{
 pub(super) unsafe fn initialize_empty(target:*mut Self){unsafe{std::ptr::addr_of_mut!((*target).present).write(false);}}
 pub(super) fn empty()->Self{Self{original:MaybeUninit::uninit(),present:false}}
 pub(super) fn is_empty(&self)->bool{!self.present}
 pub(super) fn original(&self)->Option<&JobOutcomeDescriptor>{if self.present{Some(unsafe{self.original.assume_init_ref()})}else{None}}
 pub(super) fn retain(&mut self,original:JobOutcomeDescriptor)->Result<(),JobOutcomeDescriptor>{if self.present{return Err(original)}self.original.write(original);self.present=true;Ok(())}
 pub(super) fn retirement_demands(&self)->RetirementDemand{RetirementDemand{copy_bytes:0,depth:if self.original().is_some_and(|original|!original.is_acknowledged()){2}else{usize::from(self.present)},..Default::default()}}
 pub(super) fn close_step(&mut self,grant:RetainedCloneGrant)->RetainedCloneStep{if !self.present{return RetainedCloneStep::Complete(Default::default())}let demand=self.retirement_demands();if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_depth<demand.depth{return RetainedCloneStep::Progress(Default::default())}let original=unsafe{self.original.assume_init_mut()};if !original.is_acknowledged(){return RetainedCloneStep::Progress(original.acknowledge(RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant}).progress())}unsafe{self.original.assume_init_drop();}self.present=false;RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:0,..Default::default()})}
}
impl Drop for JobOutcomeDescriptorSlot{fn drop(&mut self){assert!(std::thread::panicking()||self.is_empty(),"exclusive worker semantic descriptor requires original paid acknowledgement and removal");}}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
