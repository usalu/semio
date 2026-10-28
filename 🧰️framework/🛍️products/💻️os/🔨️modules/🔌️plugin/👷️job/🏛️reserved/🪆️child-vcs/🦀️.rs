//! 🪆️ A retained child VCS unit keeps the exact request and erased typed command through result, acknowledgement and physical closure.
use semio_framework_os_kernel::os_store::{SpaceMember,MemberNativeVcsCommand,ArtifactNativeVcsRequest,ArtifactNativeVcsPhase,ArtifactNativeVcsResult};
use semio_framework_job::{StepContext,OriginalTurnFailure,OriginalSourceBinding,OriginalConstructionDemand,admit_original_construction,close_original_retained_step};
use semio_framework_value::{RetirementDemand,RetainedCloneStep,RetainedCloneProgress,ValueError,ValueRefusalKind,retained_clone::RetainedCloneClose};
use std::mem::{size_of,ManuallyDrop};
/// 🎮️ The caller owns one actual member command; no method creates a replacement decoder or caller.
pub struct ChildVcsCommandUnit{binding:OriginalSourceBinding,decoder:(u64,u64),body:usize,request:ManuallyDrop<Option<ArtifactNativeVcsRequest>>,command:ManuallyDrop<Option<Box<dyn MemberNativeVcsCommand>>>,result:Option<ArtifactNativeVcsResult>,retirement:RetainedCloneClose,phase:u8,closing:bool}
pub(super) fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original child VCS unit caller, decoder or ownership phase changed")}
pub(super) fn header<T>()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<T>(),depth:1,..Default::default()}}
pub(super) fn original(d:RetirementDemand)->OriginalConstructionDemand{OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:d.capacity_bytes,release_bytes:d.release_bytes,depth:d.depth}}
pub(super) fn paid(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
impl ChildVcsCommandUnit{
 pub fn capture_demands()->RetirementDemand{RetirementDemand{capacity_bytes:size_of::<Self>(),..header::<(Self,Option<ArtifactNativeVcsRequest>,Option<ArtifactNativeVcsRequest>)>()}}
 /// 🎟️ The exact request owner transfers once inside the original caller's funded construction.
 pub fn admit(request:&mut Option<ArtifactNativeVcsRequest>,cx:&mut StepContext<'_>,output:&mut Option<Box<Self>>)->Result<bool,OriginalTurnFailure>{if request.is_none(){return Err(invalid().into())}let binding=OriginalSourceBinding::borrow(cx);let body=cx.retained_grant().maximum_copy_bytes;let d=Self::capture_demands();admit_original_construction(cx,output,original(d),|control|Ok((Box::new(Self{binding,decoder:(control.ledger_identity(),control.retirement_recipient_identity().ok_or_else(invalid)?),body,request:ManuallyDrop::new(request.take()),command:ManuallyDrop::new(None),result:None,retirement:Default::default(),phase:0,closing:false}),paid(d))))}
 fn validate(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{if !self.binding.matches(cx)||cx.native_receiving_identity()!=Some(self.decoder){return Err(invalid().into())}Ok(())}
 pub fn next_construction_demands<M:SpaceMember>(&self,member:&M)->Option<RetirementDemand>{match self.phase{0=>Some(member.member_native_vcs_capture_demands()),1=>self.command.as_ref().and_then(|c|c.next_construction_demands()),_=>None}}
 pub fn phase(&self)->u8{self.phase}
 pub fn ready(&self)->bool{self.phase==6&&!self.closing&&self.result.is_some()}
 /// ⏭️ Actual member capture, publication, result and acknowledgement each advance on their own original turn.
 pub fn advance<M:SpaceMember>(&mut self,member:&mut M,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.validate(cx)?;if self.closing||cx.is_cancelled(){return Err(ValueError::literal(ValueRefusalKind::Canceled,"original child VCS unit was cancelled").into())}if self.ready(){return Ok(true)}match self.phase{
 0=>{if member.admit_member_native_vcs_command(&mut self.request,cx,&mut self.command)?{self.phase=1}},
 1=>{let step=member.advance_member_native_vcs_command(self.command.as_mut().ok_or_else(invalid)?.as_mut(),cx)?;if step.phase==ArtifactNativeVcsPhase::AwaitingAck{self.phase=2}},
 2=>{if member.take_member_native_vcs_result(self.command.as_mut().ok_or_else(invalid)?.as_mut(),cx,&mut self.result)?{self.phase=3}},
 3=>{if member.ack_member_native_vcs_command(self.command.as_mut().ok_or_else(invalid)?.as_mut(),cx)?{self.phase=4}},
 4=>{if cx.with_original_retirement(|cx|member.close_member_native_vcs_command(self.command.as_mut().ok_or_else(invalid)?.as_mut(),cx))?{self.phase=5}},
 5=>{let command=self.command.as_ref().ok_or_else(invalid)?;if !command.terminal_is_empty(){return Err(invalid().into())}let mut d=command.release_demands();d.copy_bytes=size_of::<(Option<Box<dyn MemberNativeVcsCommand>>,u8)>();close_original_retained_step(cx,d,|_|{self.command.take();self.phase=6;Ok(RetainedCloneStep::Progress(paid(d)))})?;},_=>return Err(invalid().into()),}Ok(self.ready())}
 pub fn take_result(&mut self,cx:&mut StepContext<'_>,output:&mut Option<ArtifactNativeVcsResult>)->Result<bool,OriginalTurnFailure>{self.validate(cx)?;if !self.ready(){return Err(invalid().into())}let d=header::<(ArtifactNativeVcsResult,Option<ArtifactNativeVcsResult>,u8)>();admit_original_construction(cx,output,original(d),|_|{self.phase=7;Ok((self.result.take().ok_or_else(invalid)?,paid(d)))})}
 pub fn close_demands<M:SpaceMember>(&self,member:&M)->Result<RetirementDemand,ValueError>{if !self.closing{return Ok(header::<bool>())}if let Some(command)=self.command.as_ref(){if command.terminal_is_empty(){let mut d=command.release_demands();d.copy_bytes=size_of::<Option<Box<dyn MemberNativeVcsCommand>>>();return Ok(d)}return member.member_native_vcs_close_demands(command.as_ref(),self.body)}if !self.retirement.is_empty(){return self.retirement.demands(self.body)}if self.request.is_some(){return self.retirement.next_owner_demands::<ArtifactNativeVcsRequest>(true,&None,self.body)}if self.result.is_some(){return Ok(header::<Option<ArtifactNativeVcsResult>>())}Ok(Default::default())}
 /// ♻️ The exact member and receiving continuation close every initialized command before the unit frame can release.
 pub fn close_step<M:SpaceMember>(&mut self,member:&mut M,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.validate(cx)?;if self.terminal_is_empty(){return Ok(true)}let d=self.close_demands(member)?;if !self.closing{close_original_retained_step(cx,d,|_|{self.closing=true;Ok(RetainedCloneStep::Progress(paid(d)))})?;return Ok(false)}if let Some(command)=self.command.as_mut(){if !command.terminal_is_empty(){member.close_member_native_vcs_command(command.as_mut(),cx)?;return Ok(false)}close_original_retained_step(cx,d,|_|{self.command.take();Ok(RetainedCloneStep::Progress(paid(d)))})?;return Ok(false)}close_original_retained_step(cx,d,|g|{if !self.retirement.is_empty(){return self.retirement.select(self.body)?.step(g)}if self.request.is_some(){return Ok(self.retirement.begin_granted(&mut self.request,g)?.ok_or_else(invalid)?)}if self.result.is_some(){self.result=None;return Ok(RetainedCloneStep::Progress(paid(d)))}Ok(RetainedCloneStep::Complete(Default::default()))})?;Ok(self.terminal_is_empty())}
 pub fn terminal_is_empty(&self)->bool{self.request.is_none()&&self.command.is_none()&&self.result.is_none()&&self.retirement.is_empty()}
 pub fn release_demands()->RetirementDemand{RetirementDemand{release_bytes:size_of::<Self>(),depth:1,..Default::default()}}
}
impl Drop for ChildVcsCommandUnit{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original child VCS unit requires funded closure")}}

#[path="📌️checkpoint/🦀️.rs"]
pub mod checkpoint;
