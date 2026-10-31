//! 🪡️ Original-bound field copying seals only the authored typed child and retains every partial owner for closure.
use crate::{NativeDecodeControl,RetirementDemand,ValueError,ValueRefusalKind,shared_issuance::{SealedShared,SharedIssuer},retained_clone::{RetainedClone,RetainedCloneBinding,RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneStep,RetainedFieldCursor,RetainedCloneCursor,retained_clone_grant_admits}};
use std::mem::ManuallyDrop;
/// 🧵️ Intrinsic field payload authority is explicit; enclosing native rows choose their own independent issuer.
pub struct NativeIntrinsicSharedField<T:RetainedClone+Send+Sync+'static>{binding:Option<RetainedCloneBinding>,cursor:RetainedFieldCursor<T>,raw:ManuallyDrop<Option<T>>,output:ManuallyDrop<Option<SealedShared<T>>>,retirement:RetainedCloneClose,phase:u8,closing:bool}
impl<T:RetainedClone+Send+Sync+'static> Default for NativeIntrinsicSharedField<T>{fn default()->Self{Self{binding:None,cursor:Default::default(),raw:ManuallyDrop::new(None),output:ManuallyDrop::new(None),retirement:Default::default(),phase:0,closing:false}}}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"native field receiving has closed or transferred its original owner")}
fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
fn receipt(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
impl<T:RetainedClone+Send+Sync+'static> NativeIntrinsicSharedField<T>{
 pub fn ready(&self)->bool{!self.closing&&self.phase==4&&self.output.is_some()}
 pub fn demands(&self,original:RetainedCloneRef<'_,T>,body:usize)->Result<RetirementDemand,ValueError>{
  if self.closing||self.phase>=4{return Err(invalid());}
  if !original.check_binding(&self.binding)?{return Ok(fixed(original.binding_copy_bytes()));}
  if self.phase==0{return if self.cursor.output_ready(){self.cursor.take_demand()}else{self.cursor.advance_demands(original,body)};}
  if self.phase==1{return if self.output.is_some(){Ok(fixed(0))}else{Ok(semio_framework_value_native_decoding::native_decoding::construction::NativeOwnedDecodeControl::shared_birth_demand::<T>())}}
  if self.cursor.terminal_is_empty(){return Ok(fixed(0));}
  self.cursor.close_demands(body)
 }
 pub fn advance(&mut self,original:RetainedCloneRef<'_,T>,control:&mut NativeDecodeControl<'_>,body:usize,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<RetainedCloneStep,crate::OriginalReceivingPrimary>{let mut work=0;let result:Result<RetainedCloneStep,crate::OriginalReceivingPrimary>=(||{
  let d=self.demands(original,body)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()))}control.checkpoint()?;crate::owned::receiving_capacity(control,d.capacity_bytes)?;let turn=original.bind(&mut self.binding,grant);work=turn.work;if let Some(p)=turn.result?{control.receive_retained_work(grant,crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(p)),work:0}).result?;return Ok(RetainedCloneStep::Progress(p))}
  if self.phase==0{if !self.cursor.output_ready(){let turn=self.cursor.advance(original,body,grant);let turn=control.receive_retained_work(grant,turn);work=turn.work;return turn.result.map(|s|RetainedCloneStep::Progress(s.progress())).map_err(Into::into)}let turn=self.cursor.take(grant);work=turn.work;let result=turn.result.map(|output|{if let Some((value,p))=output{*self.raw=Some(value);self.phase=1;RetainedCloneStep::Progress(p)}else{RetainedCloneStep::Progress(Default::default())}});let turn=control.receive_retained_work(grant,crate::RetainedWorkOutcome{result,work});work=turn.work;return turn.result.map_err(Into::into)}
  if self.phase==1{if self.output.is_some(){work=1;self.cursor.begin_close();self.phase=2;return Ok(RetainedCloneStep::Progress(receipt(d)))}let turn=control.receive_shared(&mut self.raw,&mut self.output,SharedIssuer::owned());work=turn.work;let p=turn.result?;if !p.fits(grant){return Err(invalid().with_retained_progress(p).into())}return Ok(RetainedCloneStep::Progress(p))}
  if self.cursor.terminal_is_empty(){work=1;self.phase=4;return Ok(RetainedCloneStep::Complete(receipt(d)))}let turn=self.cursor.close_step(grant);let turn=control.receive_retained_work(grant,turn);work=turn.work;turn.result.map(|s|RetainedCloneStep::Progress(s.progress())).map_err(Into::into)
 })();crate::RetainedWorkOutcome{result,work}}
 pub fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.ready(){return Err(invalid());}Ok(fixed(0))}
 pub fn take(&mut self,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<Option<(SealedShared<T>,RetainedCloneProgress)>>{let mut work=0;let result=(||{let d=self.take_demand()?;if !retained_clone_grant_admits(grant,d){return Ok(None)}let output=self.output.take().ok_or_else(invalid)?;self.phase=5;work=1;Ok(Some((output,receipt(d))))})();crate::RetainedWorkOutcome{result,work}}
 pub fn begin_close(&mut self){self.closing=true;self.cursor.begin_close();}
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.closing{return Err(invalid());}
  if !self.cursor.terminal_is_empty(){return self.cursor.close_demands(body);}
  if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<T>(false,&None,body);}
  if self.raw.is_some(){return self.retirement.next_owner_demands::<T>(true,&None,body);}
  if self.output.is_some(){return self.retirement.next_owner_demands::<SealedShared<T>>(true,&None,body);}
  RetainedCloneBinding::close_demand(&self.binding,body)
 }
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome{let mut work=0;let result=(||{
  let d=self.close_demands(grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()));}
  if !self.cursor.terminal_is_empty(){let turn=self.cursor.close_step(grant);work=turn.work;return turn.result.map(|s|RetainedCloneStep::Progress(s.progress()));}
  if !self.retirement.is_empty(){let turn=self.retirement.step_granted(grant);work=turn.work;return turn.result.map(|s|RetainedCloneStep::Progress(s.progress()));}
  if self.raw.is_some(){let turn=self.retirement.begin_granted(&mut self.raw,grant);work=turn.work;return turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default())))}
  if self.output.is_some(){let turn=self.retirement.begin_granted(&mut self.output,grant);work=turn.work;return turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default())))}
  {let turn=RetainedCloneBinding::close_one(&mut self.binding,grant);work=turn.work;turn.result}
 })();crate::RetainedWorkOutcome{result,work}}
 pub fn terminal_is_empty(&self)->bool{self.binding.is_none()&&self.raw.is_none()&&self.output.is_none()&&self.retirement.is_empty()&&self.cursor.terminal_is_empty()}
}
impl<T:RetainedClone+Send+Sync+'static> Drop for NativeIntrinsicSharedField<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"native shared field retains original physical ownership");}}
