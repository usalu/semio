//! 🪡️ Original-bound field copying seals only the authored typed child and retains every partial owner for closure.
use semio_framework_value_native_decoding::native_decoding::{NativeDecodeControl,OriginalReceivingPrimary,NativeReceivingEffect,NativeSharedAdmission,construction::NativeRecoveryLoan};
use semio_framework_value_retirement::{OriginalRetirementFailure,OriginalRetirementOutcome};
use semio_framework_value_control_contract::RetainedWorkOutcome;
use semio_framework_value_control_contract::{RetirementDemand,ValueError,ValueRefusalKind,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,retained_clone_grant_admits};
use semio_framework_value_shared_issuance::{SealedShared,SharedIssuer};
use semio_framework_value_retained_clone::retained_clone::{RetainedClone,RetainedCloneBinding,RetainedCloneClose,RetainedCloneRef,RetainedFieldCursor,RetainedCloneCursor};
use semio_framework_value_native_shared_slots::NativeSharedSlots;
use semio_framework_value_native_receiving_contract::{native_work_limit,work_sum,selected_native_work};
use semio_framework_value_native_source::receiving_capacity;
use std::mem::ManuallyDrop;
pub struct NativeIntrinsicSharedField<T:RetainedClone+Send+Sync+'static>{binding:Option<RetainedCloneBinding>,cursor:RetainedFieldCursor<T>,raw:ManuallyDrop<Option<T>>,output:ManuallyDrop<Option<SealedShared<T>>>,slots:NativeSharedSlots<T,SealedShared<T>>,retirement:RetainedCloneClose,effect:NativeReceivingEffect,retirement_effect:NativeReceivingEffect<OriginalRetirementFailure>,phase:u8,closing:bool}
impl<T:RetainedClone+Send+Sync+'static> Default for NativeIntrinsicSharedField<T>{fn default()->Self{Self{binding:None,cursor:Default::default(),raw:ManuallyDrop::new(None),output:ManuallyDrop::new(None),slots:Default::default(),retirement:Default::default(),effect:Default::default(),retirement_effect:Default::default(),phase:0,closing:false}}}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"native field receiving has closed or transferred its original owner")}
fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
fn receipt(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
impl<T:RetainedClone+Send+Sync+'static> NativeIntrinsicSharedField<T>{
 pub fn shared_admission(&mut self)->&mut NativeSharedAdmission<T>{self.slots.admission()}
 pub fn capture_plan(&self)->Result<Option<semio_framework_value_native_decoding::NativeEffectCapturePlan>,ValueError>{
 if self.effect.has_pending_recovery(){return Ok(Some(self.effect.capture_plan()))}
 if self.retirement_effect.has_pending_recovery(){return Ok(Some(self.retirement_effect.capture_plan()))}
 if self.slots.has_pending_recovery(){return self.slots.capture_plan()}
 if self.closing||self.phase>=4{return Ok(None)}
 if self.binding.is_none(){return Ok(Some(self.effect.capture_plan()))}
 if self.phase==0&&!self.cursor.output_ready()||self.phase>=2&&!self.cursor.terminal_is_empty(){return Ok(Some(self.retirement_effect.capture_plan()))}
 if self.phase==1&&self.output.is_none()&&self.raw.is_none()&&!self.slots.output_ready(){return self.slots.capture_plan()}
 Ok(Some(self.effect.capture_plan()))
 }
 pub fn has_pending_recovery(&self)->bool{self.effect.has_pending_recovery()||self.retirement_effect.has_pending_recovery()||self.slots.has_pending_recovery()}
 pub fn pending_recovery(&mut self)->Option<NativeRecoveryLoan<'_>>{if self.effect.has_pending_recovery(){return self.effect.pending_recovery().map(NativeRecoveryLoan::Value)}if self.retirement_effect.has_pending_recovery(){return self.retirement_effect.pending_recovery().map(NativeRecoveryLoan::Retirement)}self.slots.admission().pending_recovery().map(NativeRecoveryLoan::Value)}
 pub fn resume_recovery(&mut self,control:&mut NativeDecodeControl<'_>)->RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{if self.effect.has_pending_recovery(){control.finish_receiving(&mut self.effect)}else if self.retirement_effect.has_pending_recovery(){control.finish_retirement_receiving(&mut self.retirement_effect)}else{self.slots.resume_recovery(control)}}
 pub fn ready(&self)->bool{!self.closing&&!self.has_pending_recovery()&&self.phase==4&&self.output.is_some()}
 pub fn demands(&self,original:RetainedCloneRef<'_,T>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{
  if self.closing||self.phase>=4{return Err(invalid())}if maximum_work_units==0{return Ok(Default::default())}
  if !original.check_binding(&self.binding)?{return Ok(fixed(original.binding_copy_bytes()))}
  if self.phase==0{return if self.cursor.output_ready(){let mut d=self.cursor.take_demand()?;d.copy_bytes=d.copy_bytes.checked_add(size_of::<Option<T>>()+size_of::<u8>()).ok_or_else(invalid)?;Ok(d)}else{self.cursor.advance_demands(original,body,maximum_work_units)}}
  if self.phase==1{return if self.output.is_some(){Ok(fixed(size_of::<u8>()+if self.cursor.begin_close_work_demand()?!=0{size_of::<bool>()}else{0}))}else if self.raw.is_some(){Ok(NativeSharedSlots::<T,SealedShared<T>>::capture_demand())}else if self.slots.output_ready(){Ok(NativeSharedSlots::<T,SealedShared<T>>::take_demand())}else{Ok(self.slots.demand())}}
  if self.cursor.terminal_is_empty(){return Ok(fixed(size_of::<u8>()))}self.cursor.close_demands(body)
 }
 pub fn advance_work_demand(&self,original:RetainedCloneRef<'_,T>,body:usize,maximum_work_units:u64)->Result<u64,ValueError>{
  self.demands(original,body,maximum_work_units)?;if maximum_work_units==0{return Ok(0)}
  let work=if !original.check_binding(&self.binding)?{original.binding_work_demand(&self.binding)?}else if self.phase==0{if self.cursor.output_ready(){work_sum(self.cursor.take_work_demand()?,2)?}else{self.cursor.advance_work_demand(original,body,maximum_work_units)?}}else if self.phase==1{if self.output.is_some(){work_sum(self.cursor.begin_close_work_demand()?,1)?}else if self.raw.is_some(){NativeSharedSlots::<T,SealedShared<T>>::capture_work_demand()}else if self.slots.output_ready(){NativeSharedSlots::<T,SealedShared<T>>::take_work_demand()}else{self.slots.advance_work_demand()?}}else if self.cursor.terminal_is_empty(){1}else{self.cursor.close_work_demand(body)?};
  Ok(selected_native_work(work,maximum_work_units))
 }
 pub fn advance(&mut self,original:RetainedCloneRef<'_,T>,control:&mut NativeDecodeControl<'_>,body:usize,maximum_work_units:u64,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>{let result=(||{
  let maximum=native_work_limit(control,maximum_work_units)?;let q=self.advance_work_demand(original,body,maximum)?;let d=self.demands(original,body,maximum)?;if q==0||!retained_clone_grant_admits(grant,d){return Ok(None)}control.checkpoint()?;receiving_capacity(control,d.capacity_bytes)?;
  let turn=if !original.check_binding(&self.binding)?{control.receive_allocation(d,&mut self.effect,false,q,|g,_|{let turn=original.bind(&mut self.binding,g);RetainedWorkOutcome{result:turn.result.map(|p|RetainedCloneStep::Progress(p.unwrap_or_default())),work:turn.work}})}
  else if self.phase==0{
   if !self.cursor.output_ready(){let cursor=&mut self.cursor;control.receive_retirement(d,&mut self.retirement_effect,d.capacity_bytes!=0,q,|g,actual|{let mut none=None;cursor.advance(original,body,maximum,g,actual.unwrap_or(&mut none))})}
   else{let cursor=&mut self.cursor;let raw=&mut self.raw;let phase=&mut self.phase;control.receive_allocation(d,&mut self.effect,false,q,|g,_|{let parent_copy=size_of::<Option<T>>()+size_of::<u8>();let turn=cursor.take(RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes.saturating_sub(parent_copy),..g});let mut placed=false;let result=turn.result.map(|v|{if let Some((value,mut p))=v{**raw=Some(value);*phase=1;placed=true;p.copied_bytes+=parent_copy;RetainedCloneStep::Progress(p)}else{RetainedCloneStep::Progress(Default::default())}});RetainedWorkOutcome{result,work:turn.work+2*u64::from(placed)}})}
  }else if self.phase==1{
   if self.output.is_some(){let cursor=&mut self.cursor;let phase=&mut self.phase;control.receive_allocation(d,&mut self.effect,false,q,|g,_|{if !retained_clone_grant_admits(g,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}let turn=cursor.begin_close_paid();RetainedWorkOutcome{result:turn.result.map(|()|{*phase=2;RetainedCloneStep::Progress(receipt(d))}),work:turn.work+1}})}
   else if self.raw.is_some(){let slots=&mut self.slots;let raw=&mut self.raw;control.receive_allocation(d,&mut self.effect,false,q,|g,_|slots.capture(raw,g))}
   else if self.slots.output_ready(){let slots=&mut self.slots;let output=&mut self.output;control.receive_allocation(d,&mut self.effect,false,q,|g,_|slots.take(output,g))}
   else{self.slots.advance(control,SharedIssuer::owned())}
  }else if self.cursor.terminal_is_empty(){let phase=&mut self.phase;control.receive_allocation(d,&mut self.effect,false,q,|g,_|{if !retained_clone_grant_admits(g,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}*phase=4;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(receipt(d))),work:1}})}
  else{let cursor=&mut self.cursor;control.receive_retirement(d,&mut self.retirement_effect,d.capacity_bytes!=0,q,|g,actual|{let mut none=None;cursor.close_step(g,actual.unwrap_or(&mut none))})};
  Ok(Some(turn))
 })();match result{Ok(Some(turn))=>RetainedWorkOutcome{result:turn.result.map(|p|if self.ready(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)}),work:turn.work},Ok(None)=>RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0},Err(error)=>RetainedWorkOutcome{result:Err(error),work:0}}}
 pub fn take_work_demand(&self)->Result<u64,ValueError>{self.take_demand()?;Ok(1)}
 pub fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.ready(){return Err(invalid());}Ok(fixed(0))}
 pub fn take(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome<Option<(SealedShared<T>,RetainedCloneProgress)>>{let mut work=0;let result=(||{let d=self.take_demand()?;if !retained_clone_grant_admits(grant,d){return Ok(None)}let output=self.output.take().ok_or_else(invalid)?;self.phase=5;work=1;Ok(Some((output,receipt(d))))})();RetainedWorkOutcome{result,work}}
 pub fn is_closing(&self)->bool{self.closing}
 pub fn begin_close_demand(&self)->Result<RetirementDemand,ValueError>{if self.has_pending_recovery(){return Err(invalid())}Ok(if self.closing{Default::default()}else{fixed(size_of::<bool>())})}
 pub fn begin_close_work_demand(&self)->Result<u64,ValueError>{self.begin_close_demand()?;Ok(u64::from(!self.closing))}
 pub fn begin_close_step(&mut self,g:RetainedCloneGrant,maximum:u64)->RetainedWorkOutcome{let result=(||{let d=self.begin_close_demand()?;if self.closing{return Ok(RetainedCloneStep::Complete(Default::default()))}if maximum<1||!retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}self.begin_close()?;Ok(RetainedCloneStep::Complete(receipt(d)))})();let work=u64::from(matches!(&result,Ok(RetainedCloneStep::Complete(p))if *p!=Default::default()));RetainedWorkOutcome{result,work}}
 pub fn begin_close(&mut self)->Result<(),ValueError>{if self.has_pending_recovery(){return Err(ValueError::invariant("original native receiving effect requires recovery before close"))}self.closing=true;Ok(())}
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.closing{return Err(invalid());}
  if !self.slots.terminal_is_empty(){return self.slots.close_demand(body);}
  if !self.cursor.terminal_is_empty(){return if self.cursor.begin_close_work_demand()?!=0{Ok(fixed(size_of::<bool>()))}else{self.cursor.close_demands(body)};}
  if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<T>(false,&None,body);}
  if self.raw.is_some(){return self.retirement.next_owner_demands::<T>(true,&None,body);}
  if self.output.is_some(){return self.retirement.next_owner_demands::<SealedShared<T>>(true,&None,body);}
  RetainedCloneBinding::close_demand(&self.binding,body)
 }
 pub fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{
  self.close_demands(body)?;if !self.slots.terminal_is_empty(){return self.slots.close_work_demand(body)}if !self.cursor.terminal_is_empty(){let q=self.cursor.begin_close_work_demand()?;return if q!=0{Ok(q)}else{self.cursor.close_work_demand(body)}}if !self.retirement.is_empty(){return self.retirement.next_owner_work::<T>(false,&None,body)}if self.raw.is_some(){return self.retirement.next_owner_work::<T>(true,&None,body)}if self.output.is_some(){return self.retirement.next_owner_work::<SealedShared<T>>(true,&None,body)}RetainedCloneBinding::close_work_demand(&self.binding,body)
 }
 pub fn close_step(&mut self,grant:RetainedCloneGrant,actual:&mut Option<usize>)->OriginalRetirementOutcome{let mut work=0;let result:Result<RetainedCloneStep,OriginalRetirementFailure>=(||{
  let d=self.close_demands(grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()));}
  if !self.slots.terminal_is_empty(){let turn=self.slots.close_step(grant,actual);work=turn.work;return turn.result.map(|step|RetainedCloneStep::Progress(step.progress()));}
  if !self.cursor.terminal_is_empty(){if self.cursor.begin_close_work_demand()?!=0{let turn=self.cursor.begin_close_paid();work=turn.work;return turn.result.map(|()|RetainedCloneStep::Progress(receipt(d))).map_err(Into::into)}let turn=self.cursor.close_step(grant,actual);work=turn.work;return turn.result.map(|s|RetainedCloneStep::Progress(s.progress())).map_err(Into::into);}
  if !self.retirement.is_empty(){let turn=self.retirement.step_granted(grant,actual);work=turn.work;return turn.result.map(|s|RetainedCloneStep::Progress(s.progress()));}
  if self.raw.is_some(){let turn=self.retirement.begin_granted(&mut self.raw,grant,actual);work=turn.work;return turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into)}
  if self.output.is_some(){let turn=self.retirement.begin_granted(&mut self.output,grant,actual);work=turn.work;return turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into)}
  {let turn=RetainedCloneBinding::close_one(&mut self.binding,grant,actual);work=turn.work;turn.result.map(RetainedCloneStep::Progress).map_err(Into::into)}
 })();RetainedWorkOutcome{result,work}}
 pub fn terminal_is_empty(&self)->bool{!self.effect.has_pending_recovery()&&!self.retirement_effect.has_pending_recovery()&&self.slots.terminal_is_empty()&&self.binding.is_none()&&self.raw.is_none()&&self.output.is_none()&&self.retirement.is_empty()&&self.cursor.terminal_is_empty()}
}
impl<T:RetainedClone+Send+Sync+'static> Drop for NativeIntrinsicSharedField<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"native shared field retains original physical ownership");}}
