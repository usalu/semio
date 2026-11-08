//! 🪡️ Original-bound field copying seals only the authored typed child and retains every partial owner for closure.
use semio_framework_value_native_decoding::native_decoding::{NativeDecodeControl,OriginalReceivingPrimary,NativeSharedAdmission};
use semio_framework_value_retirement::{OriginalRetirementFailure,OriginalRetirementOutcome,OriginalRetirementPoll,RetirementPollExit};
use semio_framework_value_control_contract::{RetainedWorkOutcome,RetainedWorkPoll};
use semio_framework_value_control_contract::{RetirementDemand,ValueError,ValueRefusalKind,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,retained_clone_grant_admits};
use semio_framework_value_shared_issuance::{SealedShared,SharedIssuer};
use semio_framework_value_retained_clone::retained_clone::{RetainedClone,RetainedCloneBinding,RetainedCloneClose,RetainedCloneRef,RetainedFieldCursor,RetainedCloneCursor};
use semio_framework_value_native_shared_slots::NativeSharedSlots;
use semio_framework_value_native_receiving_contract::{work_sum,selected_native_work};
use semio_framework_value_native_source::receiving_capacity;
use semio_framework_value_native_receiver_protocol::{NativeReceivingChild,NativeLocalProducer,NativeLocalOperation,NativeLocalAuthority,native_receive_local};
use std::mem::ManuallyDrop;
type Retiring=OriginalRetirementFailure;
pub struct NativeIntrinsicSharedField<T:RetainedClone+Send+Sync+'static>{binding:Option<RetainedCloneBinding>,cursor:RetainedFieldCursor<T>,raw:ManuallyDrop<Option<T>>,output:ManuallyDrop<Option<SealedShared<T>>>,slots:NativeSharedSlots<T,SealedShared<T>>,retirement:RetainedCloneClose,value_action:NativeReceivingChild<ValueError>,retire_action:NativeReceivingChild<Retiring>,value_authority:ManuallyDrop<Option<NativeLocalAuthority<ValueError>>>,retire_authority:ManuallyDrop<Option<NativeLocalAuthority<Retiring>>>,phase:u8,closing:bool}
impl<T:RetainedClone+Send+Sync+'static> Default for NativeIntrinsicSharedField<T>{fn default()->Self{Self{binding:None,cursor:Default::default(),raw:ManuallyDrop::new(None),output:ManuallyDrop::new(None),slots:Default::default(),retirement:Default::default(),value_action:Default::default(),retire_action:Default::default(),value_authority:ManuallyDrop::new(Some(semio_framework_value_native_decoding::native_decoding::OriginalEffectAuthority::new(NativeLocalProducer::new()))),retire_authority:ManuallyDrop::new(Some(semio_framework_value_native_decoding::native_decoding::OriginalEffectAuthority::new(NativeLocalProducer::new()))),phase:0,closing:false}}}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"native field receiving has closed or transferred its original owner")}
fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
fn receipt(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
impl<T:RetainedClone+Send+Sync+'static> NativeIntrinsicSharedField<T>{
 pub fn shared_admission(&mut self)->&mut NativeSharedAdmission<T>{self.slots.admission()}
 pub fn has_pending_recovery(&self)->bool{self.value_action.pending()||self.retire_action.pending()||self.slots.has_pending_recovery()}
 pub fn ready(&self)->bool{!self.closing&&!self.has_pending_recovery()&&self.phase==4&&self.output.is_some()}
 pub fn demands(&self,original:RetainedCloneRef<'_,T>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{
  if self.closing||self.phase>=4{return Err(invalid())}if maximum_work_units==0{return Ok(Default::default())}
  if let Some(d)=self.value_action.demand.or(self.retire_action.demand){return Ok(d)}
  if !original.check_binding(&self.binding)?{return Ok(fixed(original.binding_copy_bytes()))}
  if self.phase==0{return if self.cursor.output_ready(){let mut d=self.cursor.take_demand()?;d.copy_bytes=d.copy_bytes.checked_add(size_of::<Option<T>>()+size_of::<u8>()).ok_or_else(invalid)?;Ok(d)}else{self.cursor.advance_demands(original,body,maximum_work_units)}}
  if self.phase==1{return if self.output.is_some(){Ok(fixed(size_of::<u8>()+if self.cursor.begin_close_work_demand()?!=0{size_of::<bool>()}else{0}))}else if self.raw.is_some(){Ok(NativeSharedSlots::<T,SealedShared<T>>::capture_demand())}else if self.slots.output_ready(){Ok(NativeSharedSlots::<T,SealedShared<T>>::take_demand())}else{Ok(self.slots.demand())}}
  if self.cursor.terminal_is_empty(){return Ok(fixed(size_of::<u8>()))}self.cursor.close_demands(body)
 }
 pub fn advance_work_demand(&self,original:RetainedCloneRef<'_,T>,body:usize,maximum_work_units:u64)->Result<u64,ValueError>{
  self.demands(original,body,maximum_work_units)?;if maximum_work_units==0{return Ok(0)}
  let work=if !original.check_binding(&self.binding)?{original.binding_work_demand(&self.binding)?}else if self.phase==0{if self.cursor.output_ready(){work_sum(self.cursor.take_work_demand()?,2)?}else{self.cursor.advance_work_demand(original,body,maximum_work_units)?}}else if self.phase==1{if self.output.is_some(){work_sum(self.cursor.begin_close_work_demand()?,1)?}else if self.raw.is_some(){self.slots.capture_work_demand(&self.raw)?}else if self.slots.output_ready(){self.slots.take_work_demand(&self.output)?}else{self.slots.advance_work_demand()?}}else if self.cursor.terminal_is_empty(){1}else{self.cursor.close_work_demand(body)?};
  Ok(selected_native_work(work,maximum_work_units))
 }

 pub fn advance(&mut self,original:RetainedCloneRef<'_,T>,control:&mut NativeDecodeControl<'_>,body:usize,maximum_work_units:u64,grant:RetainedCloneGrant)->RetainedWorkPoll<RetainedCloneStep,OriginalReceivingPrimary>{
  use RetainedWorkPoll::{Complete,Pending};
  #[derive(Clone,Copy,PartialEq,Eq)]enum Taken{No,Cursor,CloseStart}
  let in_flight=self.value_action.demand.is_some()||self.retire_action.demand.is_some();
  let idle=|in_flight:bool|if in_flight{Pending{work:0}}else{Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})};
  let result:Result<RetainedWorkPoll<RetainedCloneStep,OriginalReceivingPrimary>,OriginalReceivingPrimary>=(||{
   let q=if in_flight{self.value_action.work.or(self.retire_action.work).unwrap_or(1)}else{self.advance_work_demand(original,body,maximum_work_units)?};let d=self.demands(original,body,maximum_work_units)?;if q==0||!retained_clone_grant_admits(grant,d){return Ok(idle(in_flight))}
   if !in_flight{control.checkpoint()?;receiving_capacity(control,d.capacity_bytes)?;}
   let mut taken=Taken::No;
   let poll=if !original.check_binding(&self.binding)?||self.value_action.tag==1{self.value_action.tag=1;let authority=self.value_authority.as_mut().ok_or_else(invalid)?;let binding=&mut self.binding;self.value_action.poll(control,d,grant,q,maximum_work_units,authority,|_,effect,authority,action|authority.drive(&mut NativeLocalOperation::Poll(&mut |effect|{let turn=original.bind(binding,effect);RetainedWorkPoll::Complete(RetainedWorkOutcome{result:turn.result.map(|progress|RetainedCloneStep::Progress(progress.unwrap_or_default())),work:turn.work})}),effect,action))}
   else if self.phase==0{
    if !self.cursor.output_ready()||self.retire_action.tag==2{self.retire_action.tag=2;let authority=self.retire_authority.as_mut().ok_or_else(invalid)?;let cursor=&mut self.cursor;self.retire_action.poll(control,d,grant,q,maximum_work_units,authority,|_,effect,authority,action|authority.drive(&mut NativeLocalOperation::Poll(&mut |effect|RetainedWorkPoll::Complete(cursor.advance(original,body,maximum_work_units,effect))),effect,action))}
    else{self.value_action.tag=3;taken=Taken::Cursor;let authority=self.value_authority.as_mut().ok_or_else(invalid)?;let cursor=&mut self.cursor;let raw=&mut self.raw;self.value_action.poll(control,d,grant,q,maximum_work_units,authority,|_,effect,authority,action|authority.drive(&mut NativeLocalOperation::Reply(&mut |effect,reply|cursor.take(raw,effect,reply)),effect,action))}
   }else if self.phase==1{
    if self.output.is_some()||self.value_action.tag==4{self.value_action.tag=4;taken=Taken::CloseStart;let authority=self.value_authority.as_mut().ok_or_else(invalid)?;let cursor=&mut self.cursor;self.value_action.poll(control,d,grant,q,maximum_work_units,authority,|_,effect,authority,action|authority.drive(&mut NativeLocalOperation::Poll(&mut |effect|{let turn=cursor.begin_close_paid(effect);RetainedWorkPoll::Complete(RetainedWorkOutcome{result:turn.result.map(RetainedCloneStep::Progress),work:turn.work})}),effect,action))}
    else if self.raw.is_some()||self.value_action.tag==5{self.value_action.tag=5;let authority=self.value_authority.as_mut().ok_or_else(invalid)?;let slots=&mut self.slots;let raw=&mut self.raw;self.value_action.poll(control,d,grant,q,maximum_work_units,authority,|_,effect,authority,action|authority.drive(&mut NativeLocalOperation::Poll(&mut |effect|RetainedWorkPoll::Complete(slots.capture(raw,effect))),effect,action))}
    else if self.slots.output_ready()||self.value_action.tag==6{self.value_action.tag=6;let authority=self.value_authority.as_mut().ok_or_else(invalid)?;let slots=&mut self.slots;let output=&mut self.output;self.value_action.poll(control,d,grant,q,maximum_work_units,authority,|_,effect,authority,action|authority.drive(&mut NativeLocalOperation::Poll(&mut |effect|RetainedWorkPoll::Complete(slots.take(output,effect))),effect,action))}
    else{let turn=self.slots.advance(control,SharedIssuer::owned());return Ok(Complete(RetainedWorkOutcome{result:turn.result.map(RetainedCloneStep::Progress),work:turn.work}))}
   }else if self.cursor.terminal_is_empty(){return Ok(Complete(native_receive_local(control,1,||{self.phase=4;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(receipt(d))),work:1}})))}
   else{self.retire_action.tag=7;let authority=self.retire_authority.as_mut().ok_or_else(invalid)?;let cursor=&mut self.cursor;self.retire_action.poll(control,d,grant,q,maximum_work_units,authority,|_,effect,authority,action|authority.drive(&mut NativeLocalOperation::Poll(&mut |effect|cursor.close_step(effect)),effect,action))};
   if let Complete(turn)=&poll{if turn.result.is_ok(){match taken{Taken::Cursor if self.raw.is_some()=>self.phase=1,Taken::CloseStart if self.cursor.begin_close_work_demand().ok()==Some(0)=>self.phase=2,_=>{}}}}
   Ok(poll.map(|step|if self.ready(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())}))
  })();match result{Ok(turn)=>turn,Err(error)=>Complete(RetainedWorkOutcome{result:Err(error),work:0})}
 }
 pub fn take_work_demand(&self)->Result<u64,ValueError>{self.take_demand()?;Ok(1)}
 pub fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.ready(){return Err(invalid());}Ok(fixed(0))}
 pub fn take(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome<Option<(SealedShared<T>,RetainedCloneProgress)>>{let mut work=0;let result=(||{let d=self.take_demand()?;if !retained_clone_grant_admits(grant,d){return Ok(None)}let output=self.output.take().ok_or_else(invalid)?;self.phase=5;work=1;Ok(Some((output,receipt(d))))})();RetainedWorkOutcome{result,work}}
 pub fn is_closing(&self)->bool{self.closing}
 pub fn begin_close_demand(&self)->Result<RetirementDemand,ValueError>{if self.has_pending_recovery(){return Err(invalid())}Ok(if self.closing{Default::default()}else{fixed(size_of::<bool>())})}
 pub fn begin_close_work_demand(&self)->Result<u64,ValueError>{self.begin_close_demand()?;Ok(u64::from(!self.closing))}
 pub fn begin_close_step(&mut self,g:RetainedCloneGrant,maximum:u64)->RetainedWorkOutcome{let result=(||{let d=self.begin_close_demand()?;if self.closing{return Ok(RetainedCloneStep::Complete(Default::default()))}if maximum<1||!retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}self.begin_close()?;Ok(RetainedCloneStep::Complete(receipt(d)))})();let work=u128::from(matches!(&result,Ok(RetainedCloneStep::Complete(p))if *p!=Default::default()));RetainedWorkOutcome{result,work}}
 pub fn begin_close(&mut self)->Result<(),ValueError>{if self.has_pending_recovery(){return Err(ValueError::invariant("original native receiving effect requires recovery before close"))}self.closing=true;Ok(())}
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.closing{return Err(invalid());}
  if !self.slots.terminal_is_empty(){return self.slots.close_demand(body);}
  if !self.cursor.terminal_is_empty(){return if self.cursor.begin_close_work_demand()?!=0{Ok(fixed(size_of::<bool>()))}else{self.cursor.close_demands(body)};}
  if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<T>(false,&None,body);}
  if self.value_action.failure.is_some(){return self.value_action.failure_demand(&self.retirement,body);}
  if self.retire_action.failure.is_some(){return self.retire_action.failure_demand(&self.retirement,body);}
  if self.raw.is_some(){return self.retirement.next_owner_demands::<T>(true,&None,body);}
  if self.output.is_some(){return self.retirement.next_owner_demands::<SealedShared<T>>(true,&None,body);}
  if self.value_authority.is_some(){return self.retirement.next_owner_demands::<NativeLocalAuthority<ValueError>>(true,&None,body);}
  if self.retire_authority.is_some(){return self.retirement.next_owner_demands::<NativeLocalAuthority<Retiring>>(true,&None,body);}
  RetainedCloneBinding::close_demand(&self.binding,body)
 }
 pub fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{
  self.close_demands(body)?;if !self.slots.terminal_is_empty(){return self.slots.close_work_demand(body)}if !self.cursor.terminal_is_empty(){let q=self.cursor.begin_close_work_demand()?;return if q!=0{Ok(q)}else{self.cursor.close_work_demand(body)}}if !self.retirement.is_empty(){return self.retirement.next_owner_work::<T>(false,&None,body)}if self.value_action.failure.is_some(){return self.value_action.failure_work(&self.retirement,body)}if self.retire_action.failure.is_some(){return self.retire_action.failure_work(&self.retirement,body)}if self.raw.is_some(){return self.retirement.next_owner_work::<T>(true,&None,body)}if self.output.is_some(){return self.retirement.next_owner_work::<SealedShared<T>>(true,&None,body)}if self.value_authority.is_some(){return self.retirement.next_owner_work::<NativeLocalAuthority<ValueError>>(true,&None,body)}if self.retire_authority.is_some(){return self.retirement.next_owner_work::<NativeLocalAuthority<Retiring>>(true,&None,body)}RetainedCloneBinding::close_work_demand(&self.binding,body)
 }
 pub fn close_step(&mut self,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>)->OriginalRetirementPoll{let grant=effect.grant();let mut work=0;let result:Result<RetainedCloneStep,RetirementPollExit>=(||{
  let d=self.close_demands(grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()));}
  if !self.slots.terminal_is_empty(){let turn=RetirementPollExit::settled(self.slots.close_step(effect))?;work=turn.work;return turn.result.map(|step|RetainedCloneStep::Progress(step.progress())).map_err(Into::into);}
  if !self.cursor.terminal_is_empty(){if self.cursor.begin_close_work_demand()?!=0{let turn=self.cursor.begin_close_paid(effect);work=turn.work;return turn.result.map(|progress|RetainedCloneStep::Progress(progress)).map_err(Into::into)}let turn=RetirementPollExit::settled(self.cursor.close_step(effect))?;work=turn.work;return turn.result.map(|s|RetainedCloneStep::Progress(s.progress())).map_err(Into::into);}
  if !self.retirement.is_empty(){let turn=RetirementPollExit::settled(self.retirement.step_granted(effect))?;work=turn.work;return turn.result.map(|s|RetainedCloneStep::Progress(s.progress())).map_err(Into::into);}
  if self.value_action.failure.is_some(){let turn=self.value_action.close_failure(&mut self.retirement,effect);work=turn.work;return turn.result.map_err(Into::into)}
  if self.retire_action.failure.is_some(){let turn=self.retire_action.close_failure(&mut self.retirement,effect);work=turn.work;return turn.result.map_err(Into::into)}
  if self.raw.is_some(){let turn=self.retirement.begin_granted(&mut self.raw,effect);work=turn.work;return turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into)}
  if self.output.is_some(){let turn=self.retirement.begin_granted(&mut self.output,effect);work=turn.work;return turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into)}
  if self.value_authority.is_some(){let turn=self.retirement.begin_granted(&mut self.value_authority,effect);work=turn.work;return turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into)}
  if self.retire_authority.is_some(){let turn=self.retirement.begin_granted(&mut self.retire_authority,effect);work=turn.work;return turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into)}
  let turn=RetirementPollExit::settled(RetainedCloneBinding::close_one(&mut self.binding,effect))?;work=turn.work;turn.result.map_err(Into::into)
 })();RetirementPollExit::finish(result,work)}
 pub fn terminal_is_empty(&self)->bool{!self.has_pending_recovery()&&self.slots.terminal_is_empty()&&self.binding.is_none()&&self.raw.is_none()&&self.output.is_none()&&self.retirement.is_empty()&&self.cursor.terminal_is_empty()&&self.value_authority.is_none()&&self.retire_authority.is_none()&&self.value_action.terminal_is_empty()&&self.retire_action.terminal_is_empty()}
}
impl<T:RetainedClone+Send+Sync+'static> Drop for NativeIntrinsicSharedField<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"native shared field retains original physical ownership");}}
