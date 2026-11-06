//! 🫴️ Native snapshot encoding borrows original allocation and independent full ownership authority.
use semio_framework_value::{NativeEncodeControl,RetainedCloneGrant};
use super::{NativeSnapshotBodyWallet,NativeSnapshotFailure};
#[path="🧾️settlement/🦀️.rs"]
mod settlement;
pub use settlement::NativeSnapshotEncodeSettlement;
#[path="📦️octets/🦀️.rs"]
mod octets;
/// 🎟️ Keeps the original caller's five-field grant immutable beside its cumulative native owner.
pub struct NativeSnapshotEncodeOwner<'owner,'control>{native:&'owner mut NativeEncodeControl<'control>,grant:RetainedCloneGrant,progress:RetainedCloneProgress,wallet:Option<&'owner mut RetainedCloneProgress>,pending:Option<&'owner mut Option<Box<dyn ErasedSnapshotRetirement>>>}
impl<'owner,'control> NativeSnapshotEncodeOwner<'owner,'control>{
 pub(super) fn borrowed_pending(native:&'owner mut NativeEncodeControl<'control>,grant:RetainedCloneGrant,wallet:&'owner mut RetainedCloneProgress,pending:Option<&'owner mut Option<Box<dyn ErasedSnapshotRetirement>>>)->Self{let mut owner=Self::borrowed(native,grant,wallet);owner.pending=pending;owner}
 pub(super) fn with_io<T,E:Into<NativeSnapshotFailure>>(&mut self,operation:impl FnOnce(&mut super::IoRunControl<'_,'control>)->RetainedWorkOutcome<T,E>)->RetainedWorkOutcome<T,NativeSnapshotFailure>{let mut run=super::IoRunControl::borrowed_encoding(self.native,self.grant,&mut self.progress,self.pending.as_deref_mut());let turn=operation(&mut run);RetainedWorkOutcome{result:turn.result.map_err(Into::into),work:turn.work}}
 pub(super) fn with_io_companion<T,E:Into<NativeSnapshotFailure>>(&mut self,decode:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'control>,operation:impl FnOnce(&mut super::IoRunControl<'_,'control>)->RetainedWorkOutcome<T,E>)->RetainedWorkOutcome<T,NativeSnapshotFailure>{let mut run=super::IoRunControl::borrowed_native(decode,self.native,self.grant,&mut self.progress,self.pending.as_deref_mut(),super::IoNativeDirection::Encode);let turn=operation(&mut run);RetainedWorkOutcome{result:turn.result.map_err(Into::into),work:turn.work}}
 /// 🧲️ Requires original native authority and an independently supplied full physical grant.
 pub fn new(native:&'owner mut NativeEncodeControl<'control>,grant:RetainedCloneGrant)->Self{Self{native,grant,progress:RetainedCloneProgress::default(),wallet:None,pending:None}}
 /// 🔗️ Reborrows the caller's existing physical wallet without resetting its accepted receipts.
 pub(super) fn borrowed(native:&'owner mut NativeEncodeControl<'control>,grant:RetainedCloneGrant,wallet:&'owner mut RetainedCloneProgress)->Self{let progress=*wallet;Self{native,grant,progress,wallet:Some(wallet),pending:None}}
 /// 📏️ Returns the exact original grant without promoting descriptive demands into authority.
 pub fn grant(&self)->RetainedCloneGrant{self.grant}
 /// 🧮 Returns unspent authority from the original caller's cumulative physical receipts.
 pub fn remaining_grant(&self)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:self.grant.maximum_items.saturating_sub(self.progress.copied_items),maximum_copy_bytes:self.grant.maximum_copy_bytes.saturating_sub(self.progress.copied_bytes),maximum_capacity_bytes:self.grant.maximum_capacity_bytes.saturating_sub(self.progress.retained_capacity_bytes),maximum_release_bytes:self.grant.maximum_release_bytes.saturating_sub(self.progress.released_bytes),maximum_depth:self.grant.maximum_depth}}
 /// 🧾 Observes only receipts accepted from actual owner turns.
 pub fn progress(&self)->RetainedCloneProgress{self.progress}
 /// 🔐 Refuses an overflowing or ungranted receipt before mutating the original wallet.
 pub fn record_progress(&mut self,progress:RetainedCloneProgress)->Result<(),ValueError>{let admitted=self.progress.fits(self.grant)&&progress.fits(self.remaining_grant());self.progress=self.progress.checked_add(progress).map_err(|error|error.with_retained_progress(progress))?;if !admitted{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot encoding receipt exceeds original grant").with_retained_progress(progress))}Ok(())}

 /// 🔭️ Reborrows the same original callback, allocation port and cumulative native receipt.
 pub fn native(&mut self)->&mut NativeEncodeControl<'control>{self.native}
 /// 💻️ Keeps actual parent custody and accepted receipts through original native scopes.
 pub fn scoped_native<O,E:Into<NativeSnapshotFailure>>(&mut self,maximum:usize,observer:&mut dyn FnMut(semio_framework_value::native_encoding::NativeEncodeProgress)->bool,operation:impl FnOnce(&mut NativeSnapshotEncodeOwner<'_,'_>)->RetainedWorkOutcome<O,E>)->RetainedWorkOutcome<O,NativeSnapshotFailure>{
  let grant=self.grant;let wallet=&mut self.progress;let pending=self.pending.as_deref_mut();let mut work=0;
  let result=self.native.scoped_maximum(maximum,|native|native.scoped_observer(observer,|native|{let mut original=NativeSnapshotEncodeOwner::borrowed(native,grant,wallet);original.pending=pending;let turn=operation(&mut original);work=turn.work;turn.result.map_err(Into::into)}));
  RetainedWorkOutcome{result:result.map_err(|error:NativeSnapshotFailure|error.with_retained_progress(self.progress)),work}
 }

}


impl Drop for NativeSnapshotEncodeOwner<'_, '_>{fn drop(&mut self){if let Some(wallet)=self.wallet.as_deref_mut(){*wallet=self.progress;}}}

use semio_framework_value::{ValueError,ValueRefusalKind,ErasedSnapshotRetirement,retirement::{RetireOwned,controlled::ControlledRetirement},retained_clone::{RetainedCloneStep,RetainedCloneProgress},RetainedWorkOutcome};
pub(crate) struct NativeSnapshotReceiving<T:RetireOwned,O:RetireOwned>{pending:std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,intermediate:ControlledRetirement<Option<T>>,action_marker:super::NativeSnapshotEffectProgress,action_failure:Option<NativeSnapshotFailure>,unwind_failure:Option<ControlledRetirement<NativeSnapshotFailure>>,output:ControlledRetirement<Option<O>>}
impl<T:RetireOwned,O:RetireOwned> NativeSnapshotReceiving<T,O>{
 fn terminal_pending(&self)->bool{self.pending.as_ref().is_some_and(|pending|pending.terminal_is_empty())}
 fn action_pending(&self)->bool{self.pending.is_none()&&self.intermediate.terminal_is_empty()&&self.unwind_failure.as_ref().is_none_or(|failure|failure.terminal_is_empty())&&self.action_failure.is_some()}
 fn active(&self)->&dyn ErasedSnapshotRetirement{if let Some(pending)=self.pending.as_ref(){pending.as_ref()}else if !self.intermediate.terminal_is_empty(){&self.intermediate}else if let Some(failure)=self.unwind_failure.as_ref().filter(|failure|!failure.terminal_is_empty()){failure}else{&self.output}}
}
impl<T:RetireOwned,O:RetireOwned> ErasedSnapshotRetirement for NativeSnapshotReceiving<T,O>{
 fn visit_original(&mut self,identity:std::any::TypeId,visitor:&mut dyn FnMut(&mut dyn std::any::Any))->bool{if let Some(pending)=self.pending.as_mut(){return pending.visit_original(identity,visitor)}if identity!=std::any::TypeId::of::<(T,O)>(){return false}match self.intermediate.original_mut(){Some(frame)=>{visitor(frame);true},None=>false}}
    fn next_demand(&self,body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{if self.terminal_is_empty(){return Ok(Default::default())}if self.action_pending(){return Ok(semio_framework_value::RetirementDemand{depth:2,..Default::default()})}let mut demand=if let Some(pending)=self.pending.as_ref(){semio_framework_value::factory_ticket_demands(pending,body)?}else{self.active().next_demand(body)?};demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"snapshot receiving child depth overflow"))?;Ok(demand)}
 fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{if self.terminal_is_empty(){Ok(0)}else if self.terminal_pending()||self.action_pending(){Ok(1)}else{self.active().next_computational_work(body)}}
 fn requires_original_control(&self)->bool{true}
 fn close_step_controlled(&mut self,grant:RetainedCloneGrant,control:&mut dyn semio_framework_value::OriginalRetirementControl)->semio_framework_value::OriginalRetirementOutcome{
  if !self.terminal_is_empty()&&control.failure_storage_pool().is_empty(){return semio_framework_value::refill_original_failure_storage(control,grant)}
  let demand=match self.next_demand(grant.maximum_copy_bytes){Ok(demand)=>demand,Err(error)=>return RetainedWorkOutcome{result:Err(error.into()),work:0}};
  if self.terminal_is_empty(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}
  if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
  if self.pending.is_some(){let mut child=grant;child.maximum_depth-=1;return semio_framework_value::close_factory_ticket_controlled(&mut self.pending,child,control)}
  if !self.intermediate.terminal_is_empty()&&ErasedSnapshotRetirement::requires_original_control(&self.intermediate){let mut child=grant;child.maximum_depth-=1;return ErasedSnapshotRetirement::close_step_controlled(&mut self.intermediate,child,control)}
  if self.intermediate.terminal_is_empty(){if let Some(failure)=self.unwind_failure.as_mut().filter(|failure|!failure.terminal_is_empty()&&ErasedSnapshotRetirement::requires_original_control(&**failure)){let mut child=grant;child.maximum_depth-=1;return ErasedSnapshotRetirement::close_step_controlled(failure,child,control)}}
  if self.intermediate.terminal_is_empty()&&self.unwind_failure.as_ref().is_none_or(|failure|failure.terminal_is_empty())&&self.action_failure.is_none()&&!self.output.terminal_is_empty()&&ErasedSnapshotRetirement::requires_original_control(&self.output){let mut child=grant;child.maximum_depth-=1;return ErasedSnapshotRetirement::close_step_controlled(&mut self.output,child,control)}
  let work=match self.next_computational_work(grant.maximum_copy_bytes){Ok(work)=>work,Err(error)=>return RetainedWorkOutcome{result:Err(error.into()),work:0}};
  semio_framework_value::run_original_retirement_work(control,work,|control|{if let Err(error)=control.charge(demand.capacity_bytes){return RetainedWorkOutcome{result:Err(error.into()),work:0}}self.close_step(grant)})
 }
 fn close_original_step(&mut self,original:&mut semio_framework_value::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->semio_framework_value::OriginalRetirementOutcome{
  if !self.terminal_is_empty()&&original.failure_storage_pool().is_empty(){return semio_framework_value::refill_original_failure_storage(original,grant)}
  let demand=match self.next_demand(grant.maximum_copy_bytes){Ok(demand)=>demand,Err(error)=>return RetainedWorkOutcome{result:Err(error.into()),work:0}};
  if self.terminal_is_empty(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}
  if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
  if self.pending.is_some(){let mut child=grant;child.maximum_depth-=1;return semio_framework_value::close_factory_ticket_original(&mut self.pending,original,child)}
  if !self.intermediate.terminal_is_empty()&&ErasedSnapshotRetirement::requires_original_control(&self.intermediate){let mut child=grant;child.maximum_depth-=1;return ErasedSnapshotRetirement::close_original_step(&mut self.intermediate,original,child)}
  if self.intermediate.terminal_is_empty(){if let Some(failure)=self.unwind_failure.as_mut().filter(|failure|!failure.terminal_is_empty()&&ErasedSnapshotRetirement::requires_original_control(&**failure)){let mut child=grant;child.maximum_depth-=1;return ErasedSnapshotRetirement::close_original_step(failure,original,child)}}
  if self.intermediate.terminal_is_empty()&&self.unwind_failure.as_ref().is_none_or(|failure|failure.terminal_is_empty())&&self.action_failure.is_none()&&!self.output.terminal_is_empty()&&ErasedSnapshotRetirement::requires_original_control(&self.output){let mut child=grant;child.maximum_depth-=1;return ErasedSnapshotRetirement::close_original_step(&mut self.output,original,child)}
  let work=match self.next_computational_work(grant.maximum_copy_bytes){Ok(work)=>work,Err(error)=>return RetainedWorkOutcome{result:Err(error.into()),work:0}};
  semio_framework_value::run_original_retirement_loan(original,work,|original|{if let Err(error)=original.charge(demand.capacity_bytes){return RetainedWorkOutcome{result:Err(error.into()),work:0}}self.close_step(grant)})
 }

 fn close_step(&mut self,grant:RetainedCloneGrant)->semio_framework_value::OriginalRetirementOutcome{let mut work=0u64;let result=(||->Result<RetainedCloneStep,semio_framework_value::OriginalRetirementFailure>{if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()))}if grant.maximum_depth<self.next_depth_demand()?{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"snapshot receiving child exceeds parent depth").into())}let mut child=grant;child.maximum_depth-=1;if self.pending.is_some(){let turn=semio_framework_value::close_factory_ticket(&mut self.pending,child);work+=turn.work;turn.result}else if !self.intermediate.terminal_is_empty(){let step=self.intermediate.step(child);work+=self.intermediate.last_turn_work();step.map_err(Into::into)}else if let Some(failure)=self.unwind_failure.as_mut().filter(|failure|!failure.terminal_is_empty()){let step=failure.step(child);work+=failure.last_turn_work();step.map_err(Into::into)}else if self.action_failure.is_some(){self.unwind_failure=Some(ControlledRetirement::retain(self.action_failure.take().unwrap()));work+=1;Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))}else{let step=self.output.step(child);work+=self.output.last_turn_work();step.map_err(Into::into)}})();RetainedWorkOutcome{result,work}}
 fn terminal_is_empty(&self)->bool{self.pending.is_none()&&self.intermediate.terminal_is_empty()&&self.action_failure.is_none()&&self.unwind_failure.as_ref().is_none_or(|failure|failure.terminal_is_empty())&&self.output.terminal_is_empty()}
 fn next_copy_byte_demand(&self)->Result<usize,ValueError>{if self.action_pending(){return Ok(0)}if let Some(pending)=self.pending.as_ref(){return Ok(semio_framework_value::factory_ticket_demands(pending,0)?.copy_bytes)}self.active().next_copy_byte_demand()}
 fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{if self.action_pending(){return Ok(0)}if let Some(pending)=self.pending.as_ref(){return Ok(semio_framework_value::factory_ticket_demands(pending,copy)?.capacity_bytes)}self.active().next_capacity_byte_demand(copy)}
 fn next_release_byte_demand(&self)->Result<usize,ValueError>{if self.action_pending(){return Ok(0)}if let Some(pending)=self.pending.as_ref(){return Ok(semio_framework_value::factory_ticket_demands(pending,0)?.release_bytes)}self.active().next_release_byte_demand()}
 fn next_depth_demand(&self)->Result<usize,ValueError>{if self.terminal_is_empty(){return Ok(0)}if self.action_pending(){return Ok(2)}let depth=if let Some(pending)=self.pending.as_ref(){semio_framework_value::factory_ticket_demands(pending,0)?.depth}else{self.active().next_depth_demand()?};depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"snapshot receiving child depth overflow"))}
}
pub(super) trait SnapshotAdmission:Sized{
 fn quote_frame(&mut self,demand:semio_framework_value::RetirementDemand)->Result<Option<RetainedCloneGrant>,ValueError>;
 fn submit_frame(&mut self,grant:RetainedCloneGrant,progress:RetainedCloneProgress)->Result<(),ValueError>;
 fn checkpoint(&mut self)->Result<(),ValueError>;
 fn prepare_action(&mut self)->Result<bool,ValueError>;
 fn commit_action(&mut self)->Result<(),ValueError>;
 fn cancel_action(&mut self)->Result<(),ValueError>;
 fn charge(&mut self,bytes:usize)->Result<(),ValueError>;
 fn with_retirement_owner<T,E:From<ValueError>>(&mut self,bytes:usize,operation:impl FnOnce(&mut Self)->(Result<T,E>,Option<Box<dyn ErasedSnapshotRetirement>>))->Result<T,E>;
}
macro_rules! snapshot_admission{
 ($native:ty,$quote:expr,$submit:expr)=>{impl SnapshotAdmission for $native{
  fn quote_frame(&mut self,demand:semio_framework_value::RetirementDemand)->Result<Option<RetainedCloneGrant>,ValueError>{($quote)(self,demand)}
  fn submit_frame(&mut self,grant:RetainedCloneGrant,progress:RetainedCloneProgress)->Result<(),ValueError>{($submit)(self,grant,progress)}
  fn checkpoint(&mut self)->Result<(),ValueError>{<$native>::checkpoint(self)}
  fn prepare_action(&mut self)->Result<bool,ValueError>{<$native>::prepare_work(self,1)}
  fn commit_action(&mut self)->Result<(),ValueError>{<$native>::commit_work(self,1)}
  fn cancel_action(&mut self)->Result<(),ValueError>{<$native>::cancel_work(self)}
  fn charge(&mut self,bytes:usize)->Result<(),ValueError>{<$native>::charge(self,bytes)}
  fn with_retirement_owner<T,E:From<ValueError>>(&mut self,bytes:usize,operation:impl FnOnce(&mut Self)->(Result<T,E>,Option<Box<dyn ErasedSnapshotRetirement>>))->Result<T,E>{<$native>::with_retirement_owner(self,bytes,operation)}
 }};
}
snapshot_admission!(NativeEncodeControl<'_>,|native:&mut NativeEncodeControl<'_>,demand:semio_framework_value::RetirementDemand|native.construction_grant(demand).map(Some),|native:&mut NativeEncodeControl<'_>,grant:RetainedCloneGrant,progress:RetainedCloneProgress|native.submit_constructed_progress(grant,progress));
snapshot_admission!(semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>,|native:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>,demand:semio_framework_value::RetirementDemand|native.construction_grant(demand).map(Some),|native:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>,grant:RetainedCloneGrant,progress:RetainedCloneProgress|native.submit_constructed_progress(grant,progress));
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
impl<'owner,'control> NativeSnapshotEncodeOwner<'owner,'control>{
 /// 🧾️ Runs one bounded original encoder effect with caller-retained progress and refusal slots.
 pub fn effect(&mut self,demand:RetainedCloneGrant,progress:&mut super::NativeSnapshotEffectProgress,unwind_failure:&mut Option<NativeSnapshotFailure>,effect:&mut impl FnMut(&mut bool,&mut RetainedCloneProgress)->Result<(),ValueError>)->RetainedWorkOutcome<(),NativeSnapshotFailure>{
  if unwind_failure.is_some(){return RetainedWorkOutcome{work:0,result:Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"snapshot encoder effect requires an empty unwind slot").into())}}
  let mut body=super::SnapshotBodyScope{wallet:NativeSnapshotBodyWallet{grant:self.grant,progress:self.progress},progress:&mut self.progress};let mut actual=None;let mut allocation=None;let mut outcome=None;let mut lower_unwind=None;let mut view=semio_framework_value::native_encoding::NativeEncodeEffectProgress{performed:&mut progress.performed,physical:&mut progress.physical,actual:&mut actual,allocation:&mut allocation,outcome:&mut outcome,settlement:&mut progress.settlement};
  let result=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||self.native.effect_allocated(self.grant,&mut body.wallet.progress,demand,None,&mut view,&mut lower_unwind,&mut |performed,physical,_|effect(performed,physical))));let observed=body.wallet.progress;match result{Ok(turn)=>RetainedWorkOutcome{result:turn.result.map_err(|error|NativeSnapshotFailure::from_encode_effect(error,observed)),work:turn.work},Err(payload)=>{if let Some(failure)=lower_unwind.take(){*unwind_failure=Some(NativeSnapshotFailure::from_encode_effect(failure,observed))}std::panic::resume_unwind(payload)}}
 }
 /// 🪢️ Moves one existing owner between caller-retained slots through the original encoder.
 pub fn move_existing<T>(&mut self,source:&mut Option<T>,output:&mut Option<T>,progress:&mut super::NativeSnapshotEffectProgress,unwind_failure:&mut Option<NativeSnapshotFailure>)->RetainedWorkOutcome<(),NativeSnapshotFailure>{
  if source.is_none()||output.is_some(){return RetainedWorkOutcome{work:0,result:Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"snapshot owner transfer requires a source and empty destination").into())}}
  self.effect(RetainedCloneGrant{maximum_items:1,maximum_depth:1,..Default::default()},progress,unwind_failure,&mut |performed,physical|{*output=source.take();*performed=true;*physical=RetainedCloneProgress{copied_items:1,..Default::default()};Ok(())})
 }
 /// 🪑️ Retains the caller's static cursor and accepted output until funded retirement completes.
 pub fn drive_cursor<T:RetireOwned,O:RetireOwned,E:Into<NativeSnapshotFailure>>(&mut self,cursor:&mut Option<T>,output:&mut Option<O>,advance:&mut impl FnMut(&mut T,&mut Option<O>,&mut super::NativeSnapshotEffectProgress,&mut Option<NativeSnapshotFailure>,&mut NativeEncodeControl<'_>,&mut NativeSnapshotBodyWallet)->RetainedWorkOutcome<(),E>)->RetainedWorkOutcome<(),NativeSnapshotFailure>{drive_cursor(self.native,self.grant,&mut self.progress,self.pending.as_deref_mut(),cursor,output,advance)}
 /// 🧾️ Runs a retained cursor with a reborrowed original wallet and callback authority.
 pub fn drive_cursor_receipted<T:RetireOwned,O:RetireOwned,E:Into<NativeSnapshotFailure>>(&mut self,cursor:&mut Option<T>,output:&mut Option<O>,advance:&mut impl FnMut(&mut T,&mut Option<O>,&mut super::NativeSnapshotEffectProgress,&mut Option<NativeSnapshotFailure>,&mut NativeSnapshotEncodeOwner<'_,'control>)->RetainedWorkOutcome<(),E>)->RetainedWorkOutcome<(),NativeSnapshotFailure>{drive_cursor(self.native,self.grant,&mut self.progress,self.pending.as_deref_mut(),cursor,output,&mut |cursor,output,marker,failure,native,body|{let mut owner=NativeSnapshotEncodeOwner::borrowed(native,body.grant,&mut body.progress);advance(cursor,output,marker,failure,&mut owner)})}
 pub fn receive<T:RetireOwned,O:RetireOwned,E:Into<NativeSnapshotFailure>>(&mut self,output:&mut Option<O>,operation:&mut impl FnMut(&mut Option<T>,&mut Option<O>,&mut super::NativeSnapshotEffectProgress,&mut Option<NativeSnapshotFailure>,&mut NativeEncodeControl<'_>,&mut NativeSnapshotBodyWallet)->RetainedWorkOutcome<(),E>)->RetainedWorkOutcome<(),NativeSnapshotFailure>{receive(self.native,self.grant,&mut self.progress,self.pending.as_deref_mut(),output,&mut |slot,result,marker,failure,native,body,_pending|operation(slot,result,marker,failure,native,body))}
 pub fn receive_nested<T:RetireOwned,O:RetireOwned,E:Into<NativeSnapshotFailure>>(&mut self,output:&mut Option<O>,operation:&mut impl FnMut(&mut Option<T>,&mut Option<O>,&mut super::NativeSnapshotEffectProgress,&mut Option<NativeSnapshotFailure>,&mut NativeSnapshotEncodeOwner<'_,'control>)->RetainedWorkOutcome<(),E>)->RetainedWorkOutcome<(),NativeSnapshotFailure>{
  receive(self.native,self.grant,&mut self.progress,self.pending.as_deref_mut(),output,&mut |slot,result,marker,failure,native,body,pending|{let mut original=NativeSnapshotEncodeOwner::borrowed(native,body.grant,&mut body.progress);original.pending=Some(pending);operation(slot,result,marker,failure,&mut original)})
 }
}

fn publication_progress<O>(grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
 let progress=RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Option<O>>(),..Default::default()};
 if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"snapshot publication requires original metadata depth"))}
 if !progress.fits(grant){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot publication exceeds original item grant"))}
 Ok(progress)
}
fn publish_output<T:RetireOwned,O:RetireOwned,N:SnapshotAdmission>(frame:&mut NativeSnapshotReceiving<T,O>,output:&mut Option<O>,native:&mut N,driver:&mut NativeSnapshotBodyWallet,performed:&mut RetainedCloneProgress,work:&mut u64)->Result<(),NativeSnapshotFailure>{
 let local=driver.remaining_grant();let progress=publication_progress::<O>(local)?;let demand=semio_framework_value::RetirementDemand{copy_bytes:size_of::<Option<O>>(),depth:1,..Default::default()};let quote=native.quote_frame(demand)?.unwrap_or(local);publication_progress::<O>(quote)?;
 if output.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"snapshot caller output slot must be empty").into())}
 if !frame.output.original_is_untouched()||frame.output.original().is_none_or(Option::is_none){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"snapshot output lost original ownership").into())}
 native.checkpoint()?;let next=work.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"snapshot publication work overflow"))?;if !native.prepare_action()?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"snapshot publication requires original work authority").into())}
 *output=frame.output.take_original().flatten();*work=next;let action=native.commit_action();let local=driver.record_progress(progress);*performed=driver.progress();let receipt=native.submit_frame(quote,progress);let mut failure=action.err().map(NativeSnapshotFailure::work_post);
 if let Err(error)=local{failure=Some(match failure{Some(failure)=>failure.with_context_refusal(error),None=>NativeSnapshotFailure::context(error)})}
 if let Err(error)=receipt{failure=Some(match failure{Some(failure)=>failure.with_receipt_refusal(error),None=>NativeSnapshotFailure::receipt(error)})}
 match failure{Some(failure)=>Err(failure),None=>Ok(())}
}

fn report_work(result:Result<(),super::NativeSnapshotFailure>,work:&mut u64,units:u64)->Result<(),super::NativeSnapshotFailure>{
 if let Some(total)=work.checked_add(units){*work=total;return result}
 let refusal=||ValueError::literal(ValueRefusalKind::WorkLimit,"native snapshot work report exceeds its original counter");
 match result{Ok(())=>Err(super::NativeSnapshotFailure::context(refusal())),Err(mut error)=>{if error.context_refusal.is_none(){error.context_refusal=Some(refusal())}Err(error)}}
}
fn post(result:Result<(),super::NativeSnapshotFailure>,driver:&mut NativeSnapshotBodyWallet,performed:&mut RetainedCloneProgress,progress:RetainedCloneProgress)->Result<(),super::NativeSnapshotFailure>{
 if result.as_ref().err().is_some_and(|error|error.receipt_refusal.is_some()){
  driver.progress=driver.progress.checked_add(progress).expect("actual original physical receipts fit address space");*performed=driver.progress();return result
 }
 let receipt=driver.record_progress(progress);*performed=driver.progress();
 match(result,receipt){(result,Ok(()))=>result,(Ok(()),Err(error))=>Err(super::NativeSnapshotFailure::receipt(error)),(Err(mut primary),Err(error))=>{primary.receipt_refusal=Some(error);Err(primary)}}
}
fn post_frame_birth<N:SnapshotAdmission>(native:&mut N,driver:&mut NativeSnapshotBodyWallet,performed:&mut RetainedCloneProgress,grant:RetainedCloneGrant,bytes:usize)->Result<(),super::NativeSnapshotFailure>{
 let progress=RetainedCloneProgress{copied_items:1,retained_capacity_bytes:bytes,..Default::default()};let action=native.commit_action();let local=driver.record_progress(progress);*performed=driver.progress();let receipt=native.submit_frame(grant,progress);let mut failure=action.err().map(super::NativeSnapshotFailure::work_post);
 if let Err(error)=local{failure=Some(match failure{Some(failure)=>failure.with_context_refusal(error),None=>super::NativeSnapshotFailure::context(error)})}
 if let Err(error)=receipt{failure=Some(match failure{Some(failure)=>failure.with_receipt_refusal(error),None=>super::NativeSnapshotFailure::receipt(error)})}
 match failure{Some(failure)=>Err(failure),None=>Ok(())}
}
fn release_empty_pending<N:SnapshotAdmission>(native:&mut N,pending:&mut Option<Box<dyn ErasedSnapshotRetirement>>,driver:&mut NativeSnapshotBodyWallet,performed:&mut RetainedCloneProgress,work:&mut u64)->Result<(),NativeSnapshotFailure>{
 let Some(owner)=pending.as_ref()else{return Ok(())};
 if !owner.terminal_is_empty(){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot parent retains a nonempty child").into())}
 let bytes=size_of_val(owner.as_ref());let local=driver.remaining_grant();let demand=semio_framework_value::RetirementDemand{release_bytes:bytes,depth:1,..Default::default()};
 if local.maximum_items==0||!semio_framework_value::retained_clone::retained_clone_grant_admits(local,demand){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"empty snapshot child release exceeds original wallet").into())}
 let quote=native.quote_frame(demand)?.unwrap_or(local);if quote.maximum_items==0||!semio_framework_value::retained_clone::retained_clone_grant_admits(quote,demand){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"empty snapshot child release exceeds recipient quote").into())}
 native.checkpoint()?;let next=work.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"snapshot child release work overflow"))?;if !native.prepare_action()?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"snapshot child release requires original work authority").into())}
 drop(pending.take());*work=next;let progress=RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()};let action=native.commit_action();let local=driver.record_progress(progress);*performed=driver.progress();let receipt=native.submit_frame(quote,progress);let mut failure=action.err().map(NativeSnapshotFailure::work_post);
 if let Err(error)=local{failure=Some(match failure{Some(failure)=>failure.with_context_refusal(error),None=>NativeSnapshotFailure::context(error)})}if let Err(error)=receipt{failure=Some(match failure{Some(failure)=>failure.with_receipt_refusal(error),None=>NativeSnapshotFailure::receipt(error)})}
 match failure{Some(failure)=>Err(failure),None=>Ok(())}
}
fn admission<T:RetireOwned,O:RetireOwned>(remaining:RetainedCloneGrant,parent:Option<&Option<Box<dyn ErasedSnapshotRetirement>>>)->Result<usize,super::NativeSnapshotFailure>{
 if !T::controlled_retirement_supported()||!O::controlled_retirement_supported(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"snapshot receiving fields require original retirement factories").into())}
 if remaining.maximum_items==0{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"snapshot receiving frame requires admitted item").into())}
 if remaining.maximum_depth<2{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"snapshot receiving frame requires parent and child depth").into())}
 let bytes=size_of::<NativeSnapshotReceiving<T,O>>();
 if bytes>remaining.maximum_capacity_bytes{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot receiving frame exceeds original capacity grant").into())}
 if parent.is_some_and(Option::is_some){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot parent retains its original refused child").into())}
 Ok(bytes)
}
fn retire_inputs<T:RetireOwned,O:RetireOwned,N:SnapshotAdmission>(frame:&mut NativeSnapshotReceiving<T,O>,native:&mut N,driver:&mut NativeSnapshotBodyWallet,performed:&mut RetainedCloneProgress,work:&mut u64)->Result<(),super::NativeSnapshotFailure>{
 while frame.pending.is_some()||!frame.intermediate.terminal_is_empty(){
  let mut grant=driver.remaining_grant();grant.maximum_depth-=1;
  if grant.maximum_items==0{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"snapshot retirement exhausted original item grant").into())}
  let closing_child=frame.pending.is_some();let header=if !closing_child&&frame.intermediate.original_is_untouched(){size_of::<Option<T>>()}else{0};
  let demand=if let Some(child)=frame.pending.as_ref(){semio_framework_value::factory_ticket_demands(child,grant.maximum_copy_bytes)?}else{semio_framework_value::RetirementDemand{copy_bytes:frame.intermediate.next_copy_byte_demand()?.checked_add(header).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot retirement copy overflow"))?,capacity_bytes:frame.intermediate.next_capacity_byte_demand(grant.maximum_copy_bytes)?,release_bytes:frame.intermediate.next_release_byte_demand()?,depth:frame.intermediate.next_depth_demand()?}};
  if demand.depth>grant.maximum_depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"snapshot retirement exceeds original depth grant").into())}
  if demand.copy_bytes>grant.maximum_copy_bytes||demand.capacity_bytes>grant.maximum_capacity_bytes||demand.release_bytes>grant.maximum_release_bytes{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot retirement exceeds original physical grant").into())}
  native.checkpoint()?;native.charge(demand.capacity_bytes)?;
  let turn=if closing_child{semio_framework_value::close_factory_ticket(&mut frame.pending,grant)}else{let result=frame.intermediate.step(grant).map_err(Into::into);RetainedWorkOutcome{result,work:frame.intermediate.last_turn_work()}};
  let mut progress=turn.result.as_ref().map(|step|step.progress()).unwrap_or_else(|error|error.retained_progress());
  if header>0&&!frame.intermediate.original_is_untouched(){progress.copied_bytes=progress.copied_bytes.checked_add(header).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot retirement header copy overflow"))?;}
  let mut step=None;let result=turn.result.map(|value|{step=Some(value)}).map_err(Into::into);
  let result=report_work(result,work,turn.work);post(result,driver,performed,progress)?;
  semio_framework_value::retained_clone::admit_retained_clone_close(grant,step.unwrap(),if closing_child{frame.pending.is_none()}else{frame.intermediate.terminal_is_empty()},"snapshot receiving original")?;
  if progress==RetainedCloneProgress::default()&&turn.work==0&&(frame.pending.is_some()||!frame.intermediate.terminal_is_empty()){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot retirement requires a larger original grant").into())}
 }
 Ok(())
}
enum FrameExit{Output,Unwind(Box<dyn std::any::Any+Send>)}
pub(super) fn receive<T:RetireOwned,O:RetireOwned,N:SnapshotAdmission,E:Into<super::NativeSnapshotFailure>>(native:&mut N,original_grant:RetainedCloneGrant,performed:&mut RetainedCloneProgress,mut parent:Option<&mut Option<Box<dyn ErasedSnapshotRetirement>>>,output:&mut Option<O>,operation:&mut impl FnMut(&mut Option<T>,&mut Option<O>,&mut super::NativeSnapshotEffectProgress,&mut Option<NativeSnapshotFailure>,&mut N,&mut NativeSnapshotBodyWallet,&mut Option<Box<dyn ErasedSnapshotRetirement>>)->RetainedWorkOutcome<(),E>)->RetainedWorkOutcome<(),super::NativeSnapshotFailure>{
 let mut work=0_u64;let mut driver=NativeSnapshotBodyWallet{grant:original_grant,progress:*performed};
 let result=(||{
  if output.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"snapshot caller output slot must be empty").into())}
  if let Some(pending)=parent.as_deref_mut(){release_empty_pending(native,pending,&mut driver,performed,&mut work)?;}
  let frame_bytes=admission::<T,O>(driver.remaining_grant(),parent.as_deref())?;
  let local=driver.remaining_grant();let demand=semio_framework_value::RetirementDemand{capacity_bytes:frame_bytes,depth:1,..Default::default()};
  let frame_grant=match native.quote_frame(demand)?{Some(original)=>original,None=>local};
  if frame_grant.maximum_items==0||!semio_framework_value::retained_clone::retained_clone_grant_admits(frame_grant,demand){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot frame exceeds original recipient quote").into())}
  native.checkpoint()?;if !native.prepare_action()?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"snapshot frame requires original work authority").into())}
  let mut born=false;
  let mut construct=|native:&mut N|{
   let mut frame=Box::new(NativeSnapshotReceiving{pending:std::mem::ManuallyDrop::new(None),intermediate:ControlledRetirement::new(None::<T>).ok().unwrap(),action_marker:Default::default(),action_failure:None,unwind_failure:None,output:ControlledRetirement::new(None::<O>).ok().unwrap()});
   born=true;work=work.checked_add(1).expect("one admitted frame action fits original work");
   let result:Result<FrameExit,super::NativeSnapshotFailure>=(||{
    post_frame_birth(native,&mut driver,performed,frame_grant,frame_bytes)?;
    let mut grant=driver.remaining_grant();grant.maximum_depth-=1;let mut body=NativeSnapshotBodyWallet::new(grant);
    let turn=match std::panic::catch_unwind(std::panic::AssertUnwindSafe(||operation(frame.intermediate.original_mut().unwrap(),frame.output.original_mut().unwrap(),&mut frame.action_marker,&mut frame.action_failure,native,&mut body,&mut frame.pending))){
     Ok(turn)=>turn,
     Err(payload)=>{if let Err(error)=post(Ok(()),&mut driver,performed,body.progress()){frame.unwind_failure=Some(ControlledRetirement::retain(error));}return Ok(FrameExit::Unwind(payload))}
    };
    let result=turn.result.map_err(Into::into);
    let result=report_work(result,&mut work,turn.work);post(result,&mut driver,performed,body.progress())?;
    retire_inputs(&mut frame,native,&mut driver,performed,&mut work)?;
    publish_output(&mut frame,output,native,&mut driver,performed,&mut work)?;
    Ok(FrameExit::Output)
   })();
   match result{Ok(FrameExit::Output)=>(Ok(FrameExit::Output),Some(frame as Box<dyn ErasedSnapshotRetirement>)),Ok(FrameExit::Unwind(payload))=>(Ok(FrameExit::Unwind(payload)),Some(frame as Box<dyn ErasedSnapshotRetirement>)),Err(error)=>(Err(error),Some(frame as Box<dyn ErasedSnapshotRetirement>))}
  };
  let result:Result<FrameExit,super::NativeSnapshotFailure>=if let Some(parent)=parent{match native.checkpoint().and_then(|_|native.charge(frame_bytes)){Ok(())=>{let(result,pending)=construct(native);*parent=pending;result},Err(error)=>{drop(construct);Err(error.into())}}}else{native.with_retirement_owner(frame_bytes,construct)};
  if !born{if let Err(cancel)=native.cancel_action(){return match result{Ok(_)=>Err(cancel.into()),Err(error)=>Err(error.with_context_refusal(cancel))}}}
  result
 })();
 let result=match result{Ok(FrameExit::Output)=>Ok(()),Ok(FrameExit::Unwind(payload))=>std::panic::resume_unwind(payload),Err(error)=>Err(error)};
 RetainedWorkOutcome{result:result.map_err(|error:super::NativeSnapshotFailure|error.with_retained_progress(*performed)),work}
}
/// 🖱️ Original body work is independent of physical items, and every refusal returns its exact original cause.
pub(super) fn drive_cursor<T:RetireOwned,O:RetireOwned,N:SnapshotAdmission,E:Into<super::NativeSnapshotFailure>>(native:&mut N,original_grant:RetainedCloneGrant,performed:&mut RetainedCloneProgress,mut parent:Option<&mut Option<Box<dyn ErasedSnapshotRetirement>>>,cursor:&mut Option<T>,output:&mut Option<O>,advance:&mut impl FnMut(&mut T,&mut Option<O>,&mut super::NativeSnapshotEffectProgress,&mut Option<super::NativeSnapshotFailure>,&mut N,&mut NativeSnapshotBodyWallet)->RetainedWorkOutcome<(),E>)->RetainedWorkOutcome<(),super::NativeSnapshotFailure>{
 let mut work=0_u64;let mut driver=NativeSnapshotBodyWallet{grant:original_grant,progress:*performed};
 let result=(||{
  if output.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"snapshot caller output slot must be empty").into())}
  if cursor.is_none(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"snapshot cursor original is absent").into())}
  if let Some(pending)=parent.as_deref_mut(){release_empty_pending(native,pending,&mut driver,performed,&mut work)?;}
  let frame_bytes=admission::<T,O>(driver.remaining_grant(),parent.as_deref())?;
  let local=driver.remaining_grant();let demand=semio_framework_value::RetirementDemand{capacity_bytes:frame_bytes,depth:1,..Default::default()};
  let frame_grant=match native.quote_frame(demand)?{Some(original)=>original,None=>local};
  if frame_grant.maximum_items==0||!semio_framework_value::retained_clone::retained_clone_grant_admits(frame_grant,demand){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot frame exceeds original recipient quote").into())}
  native.checkpoint()?;if !native.prepare_action()?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"snapshot frame requires original work authority").into())}
  let mut born=false;
  let mut construct=|native:&mut N|{
   let mut frame=Box::new(NativeSnapshotReceiving{pending:std::mem::ManuallyDrop::new(None),intermediate:ControlledRetirement::new(None::<T>).ok().unwrap(),action_marker:Default::default(),action_failure:None,unwind_failure:None,output:ControlledRetirement::new(None::<O>).ok().unwrap()});
   born=true;work=work.checked_add(1).expect("one admitted frame action fits original work");
   let result:Result<FrameExit,super::NativeSnapshotFailure>=(||{
    post_frame_birth(native,&mut driver,performed,frame_grant,frame_bytes)?;
    let copy=size_of::<Option<T>>();let local=driver.remaining_grant();let demand=semio_framework_value::RetirementDemand{copy_bytes:copy,depth:1,..Default::default()};
    if local.maximum_items==0||!semio_framework_value::retained_clone::retained_clone_grant_admits(local,demand){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot cursor handoff exceeds original wallet").into())}
    let quote=native.quote_frame(demand)?.unwrap_or(local);
    if quote.maximum_items==0||!semio_framework_value::retained_clone::retained_clone_grant_admits(quote,demand){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot cursor handoff exceeds original recipient quote").into())}
    if frame.intermediate.original().is_none_or(Option::is_some)||cursor.is_none(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"snapshot cursor handoff requires its original and empty frame slot").into())}
    native.checkpoint()?;let next=work.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"snapshot cursor handoff work overflow"))?;
    if !native.prepare_action()?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"snapshot cursor handoff requires original work").into())}
    *frame.intermediate.original_mut().unwrap()=cursor.take();work=next;
    let physical=RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()};let action=native.commit_action();let local=driver.record_progress(physical);*performed=driver.progress();let receipt=native.submit_frame(quote,physical);
    let mut failure=action.err().map(super::NativeSnapshotFailure::work_post);
    if let Err(error)=local{failure=Some(match failure{Some(failure)=>failure.with_context_refusal(error),None=>super::NativeSnapshotFailure::context(error)})}
    if let Err(error)=receipt{failure=Some(match failure{Some(failure)=>failure.with_receipt_refusal(error),None=>super::NativeSnapshotFailure::receipt(error)})}
    if let Some(failure)=failure{return Err(failure)}
    loop{
     let mut child=driver.remaining_grant();child.maximum_depth-=1;
     if child.maximum_items==0{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"snapshot cursor exhausted original item grant").into())}
     native.checkpoint()?;
     let mut body=NativeSnapshotBodyWallet::new(child);
     let turn=match std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance(frame.intermediate.original_mut().unwrap().as_mut().unwrap(),frame.output.original_mut().unwrap(),&mut frame.action_marker,&mut frame.action_failure,native,&mut body))){
      Ok(turn)=>turn,
      Err(payload)=>{if let Err(error)=post(Ok(()),&mut driver,performed,body.progress()){frame.unwind_failure=Some(ControlledRetirement::retain(error));}return Ok(FrameExit::Unwind(payload))}
     };
     let ready=frame.output.original().is_some_and(Option::is_some);let progress=body.progress();
     let result=report_work(turn.result.map_err(Into::into),&mut work,turn.work);post(result,&mut driver,performed,progress)?;frame.action_marker=Default::default();
     if ready{break}
     if progress==RetainedCloneProgress::default()&&turn.work==0{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot cursor original grant cannot admit its frontier").into())}
    }
    retire_inputs(&mut frame,native,&mut driver,performed,&mut work)?;
    publish_output(&mut frame,output,native,&mut driver,performed,&mut work)?;
    Ok(FrameExit::Output)
   })();
   match result{Ok(FrameExit::Output)=>(Ok(FrameExit::Output),Some(frame as Box<dyn ErasedSnapshotRetirement>)),Ok(FrameExit::Unwind(payload))=>(Ok(FrameExit::Unwind(payload)),Some(frame as Box<dyn ErasedSnapshotRetirement>)),Err(error)=>(Err(error),Some(frame as Box<dyn ErasedSnapshotRetirement>))}
  };
  let result:Result<FrameExit,super::NativeSnapshotFailure>=if let Some(parent)=parent{match native.checkpoint().and_then(|_|native.charge(frame_bytes)){Ok(())=>{let(result,pending)=construct(native);*parent=pending;result},Err(error)=>{drop(construct);Err(error.into())}}}else{native.with_retirement_owner(frame_bytes,construct)};
  if !born{if let Err(cancel)=native.cancel_action(){return match result{Ok(_)=>Err(cancel.into()),Err(error)=>Err(error.with_context_refusal(cancel))}}}
  result
 })();
 let result=match result{Ok(FrameExit::Output)=>Ok(()),Ok(FrameExit::Unwind(payload))=>std::panic::resume_unwind(payload),Err(error)=>Err(error)};
 RetainedWorkOutcome{result:result.map_err(|error:super::NativeSnapshotFailure|error.with_retained_progress(*performed)),work}
}
