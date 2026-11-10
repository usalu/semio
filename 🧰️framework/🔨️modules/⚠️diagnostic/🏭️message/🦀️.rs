//! 🏭️ Original diagnostic construction keeps every partial allocation until admitted publication or close.
use crate::{Fault,FaultCode,FaultOrigin,FaultScope,Severity};
use semio_framework_value::{RetainedWorkOutcome,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetirementDemand,ValueError,ValueRefusalKind,retirement::{RetireOwned,RetirementCursor,RetirementStep}};
const MESSAGE_CODE:&str="app.message";
/// 🛡️ SAFETY: the layout of `FaultScope` is non-zero-sized and is the exact layout passed to the global allocator; the pointer is checked non-null and fully initialised by `write` before `Box::from_raw` takes sole ownership, so the box deallocates with the matching layout.
fn allocate_scope()->Option<Box<FaultScope>>{let layout=std::alloc::Layout::new::<FaultScope>();let pointer=std::ptr::NonNull::new(unsafe{std::alloc::alloc(layout)}.cast::<FaultScope>())?;Some(unsafe{pointer.as_ptr().write(FaultScope::default());Box::from_raw(pointer.as_ptr())})}
pub struct FaultMessageOwner{fault:Option<Fault>,message:String,code:String,failure:Option<&'static str>}
impl FaultMessageOwner{
 pub const fn empty()->Self{Self{fault:None,message:String::new(),code:String::new(),failure:None}}
 pub fn construction_demand(message:&str)->Result<RetirementDemand,ValueError>{let copy_bytes=message.len().checked_add(MESSAGE_CODE.len()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"Fault message extent overflow"))?;let capacity_bytes=copy_bytes.checked_add(std::mem::size_of::<FaultScope>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"Fault scope extent overflow"))?;Ok(RetirementDemand{copy_bytes,capacity_bytes,depth:1,..Default::default()})}
 pub fn construct(message:&str,grant:RetainedCloneGrant)->RetainedWorkOutcome<(Self,RetainedCloneProgress)>{
  let demand=match Self::construction_demand(message){Ok(demand)=>demand,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};if grant.maximum_items==0||grant.maximum_depth<demand.depth||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes{return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"Fault message birth requires its complete original quote")),work:0}}
  let result=(||{
  let mut owner=Self::empty();let mut progress=RetainedCloneProgress{copied_items:1,..Default::default()};
  if owner.message.try_reserve_exact(message.len()).is_err(){owner.failure=Some("Fault message allocation failed");return Ok((owner,progress));}
  progress.retained_capacity_bytes+=owner.message.capacity();owner.message.push_str(message);progress.copied_bytes+=message.len();
  if owner.code.try_reserve_exact(MESSAGE_CODE.len()).is_err(){owner.failure=Some("Fault code allocation failed");return Ok((owner,progress));}
  progress.retained_capacity_bytes+=owner.code.capacity();owner.code.push_str(MESSAGE_CODE);progress.copied_bytes+=MESSAGE_CODE.len();
  let Some(scope)=allocate_scope()else{owner.failure=Some("Fault scope allocation failed");return Ok((owner,progress));};
  progress.retained_capacity_bytes+=std::mem::size_of::<FaultScope>();owner.fault=Some(Fault{retained_progress:Default::default(),origin:FaultOrigin::App,code:FaultCode(std::mem::take(&mut owner.code)),severity:Severity::Error,message:std::mem::take(&mut owner.message),scope,span:None,source:None,domain:None,causes:Vec::new(),params:None,retryable:false});Ok((owner,progress))
  })();RetainedWorkOutcome{result,work:1}
 }
 pub fn fault(&self)->Option<&Fault>{self.fault.as_ref()}
 pub fn failure(&self)->Option<&'static str>{self.failure}
 pub fn ready(&self)->bool{self.fault.is_some()}
 pub fn terminal_is_empty(&self)->bool{self.fault.is_none()&&self.message.capacity()==0&&self.code.capacity()==0&&self.failure.is_none()}
 pub fn take_demand(&self)->RetirementDemand{RetirementDemand{copy_bytes:0,depth:usize::from(!self.terminal_is_empty()),..Default::default()}}
 pub fn take_step(&mut self,output:&mut Option<Fault>,grant:RetainedCloneGrant)->RetainedWorkOutcome{
  if output.is_some(){return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"Fault publication requires an empty receiver")),work:1}}
  if self.failure.is_some(){return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"Fault message construction failed with retained partial custody")),work:0}}
  if !self.ready(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}let demand=self.take_demand();if grant.maximum_items==0||grant.maximum_depth<demand.depth||grant.maximum_copy_bytes<demand.copy_bytes{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
  *output=self.fault.take();RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})),work:1}
 }
 pub fn close_demand(&self)->RetirementDemand{
  let release_bytes=if let Some(fault)=&self.fault{if fault.message.capacity()>0{fault.message.capacity()}else if fault.code.0.capacity()>0{fault.code.0.capacity()}else{std::mem::size_of::<FaultScope>()}}else if self.message.capacity()>0{self.message.capacity()}else{self.code.capacity()};RetirementDemand{release_bytes,depth:usize::from(!self.terminal_is_empty()),..Default::default()}
 }
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome{
  if self.terminal_is_empty(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0};}let d=self.close_demand();if grant.maximum_items==0||grant.maximum_depth<d.depth||grant.maximum_release_bytes<d.release_bytes{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0};}
  if let Some(fault)=&mut self.fault{if fault.message.capacity()>0{fault.message=String::new();}else if fault.code.0.capacity()>0{fault.code.0=String::new();}else{self.fault=None;}}
  else if self.message.capacity()>0{self.message=String::new();}else if self.code.capacity()>0{self.code=String::new();}else{self.failure=None;}
  let progress=RetainedCloneProgress{copied_items:1,released_bytes:d.release_bytes,..Default::default()};RetainedWorkOutcome{result:Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)}),work:1}
 }
}
impl Drop for FaultMessageOwner{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"Fault construction retains its original candidate or partial backing");}}
#[doc(hidden)]
pub struct FaultMessageRetirement{owner:FaultMessageOwner}
impl RetirementCursor for FaultMessageRetirement{
 fn next_demand(&self,_:usize)->Result<RetirementDemand,ValueError>{Ok(self.owner.close_demand())}
 semio_framework_value_retirement::native_retirement_reply!();
 fn next_computational_work(&self,_:usize)->Result<u64,ValueError>{Ok(1)}
 fn terminal_is_empty(&self)->bool{self.owner.terminal_is_empty()}
 fn next_close_byte_demand(&self)->Option<usize>{Some(self.owner.close_demand().release_bytes)}
 fn next_birth_bytes(&self,_:usize)->Option<usize>{Some(0)}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(std::mem::size_of::<Self>())}
}
impl FaultMessageRetirement{
 fn close_step(&mut self,_body:usize,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->RetirementStep{let turn=self.owner.close_step(effect.grant());match turn.result{Ok(step)=>{if self.owner.terminal_is_empty()&&step.progress()==Default::default(){RetirementStep::Complete(turn.work)}else{RetirementStep::Progress(step.progress(),turn.work)}},Err(error)=>RetirementStep::Failure(error.into(),turn.work)}}
}
impl RetireOwned for FaultMessageOwner{
 type RetirementCursor=FaultMessageRetirement;
 fn retirement_seed(self)->FaultMessageRetirement{FaultMessageRetirement{owner:self}}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<FaultMessageRetirement>())}
 fn controlled_retirement_supported()->bool{true}
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
