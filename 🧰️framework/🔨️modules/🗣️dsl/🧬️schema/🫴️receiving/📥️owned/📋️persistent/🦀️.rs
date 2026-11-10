//! 📋️ Owned array fields stream one genuine row into immutable pages without retaining a flat output.
use super::*;
use semio_framework_value::{persistent::{PersistentList,PersistentListBuilder},retained_clone::{OriginalEffectReply,OriginalEffectReplyComplete},shared_issuance::{SealedShared,SealedSharedBirth,SharedIssuer}};
pub struct PersistentReceiving<T:DslOwnedReceivingField+Sync>{source:OwnedSlot<FieldValue>,current:OwnedSlot<FieldValue>,row:OwnedSlot<T>,sealed:OwnedSlot<SealedShared<T>>,output:OwnedSlot<PersistentList<T>>,birth:ManuallyDrop<Option<SealedSharedBirth<T>>>,issuer:Option<SharedIssuer<T>>,child:OwnedReceivingSlot<T>,builder:ManuallyDrop<Option<PersistentListBuilder<T>>>,index:usize,length:usize,phase:u8,closing:bool,failed:bool}
impl<T:DslOwnedReceivingField+Sync>Default for PersistentReceiving<T>{fn default()->Self{Self{source:Default::default(),current:Default::default(),row:Default::default(),sealed:Default::default(),output:Default::default(),birth:ManuallyDrop::new(None),issuer:None,child:OwnedReceivingSlot::new(),builder:ManuallyDrop::new(None),index:0,length:0,phase:0,closing:false,failed:false}}}
impl<T:DslOwnedReceivingField+Sync>DslOwnedReceivingField for PersistentList<T>{type Cursor=PersistentReceiving<T>;fn receiving_cursor()->Self::Cursor{Self::Cursor::default()}}
/// 🪢️ Runs one reply-shaped builder action against a borrowed answer slot; an unanswered slot stays pending.
fn answered(call:impl for<'r> FnOnce(OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>)->Result<Option<Result<RetainedCloneStep,ValueError>>,ValueError>{let mut slot=None;drop(call(OriginalEffectReply::borrow(&mut slot)?)?);Ok(slot.take())}
/// 🧾️ Reports a builder answer as one owned turn, keeping an unanswered action pending.
fn answer_poll(effect:&RetainedAllocationEffect<'_>,answer:Result<Option<Result<RetainedCloneStep,ValueError>>,ValueError>)->(OwnedPoll,Option<RetainedCloneStep>){match answer{Err(error)=>(done(effect,Err(error.into())),None),Ok(None)=>(RetainedWorkPoll::Pending{work:effect.performed_work()},None),Ok(Some(Err(error)))=>(done(effect,Err(error.into())),None),Ok(Some(Ok(step)))=>(done(effect,Ok(step.progress())),Some(step))}}
/// ♻️ Pays the builder close start once, then settles its reply-shaped closure into an owned closing turn.
fn builder_close(builder:&mut PersistentListBuilder<impl RetireOwned+Send+Sync+'static>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OwnedClose{
 if !builder.is_closing(){let turn=builder.begin_close_paid(effect);return done(effect,turn.result.map(|step|step.progress()).map_err(Into::into))}
 let mut slot=None;let ran=OriginalEffectReply::borrow(&mut slot).and_then(|reply|builder.close_into(body,effect,reply).map(drop));
 match(ran,slot.take()){(Err(error),_)=>done(effect,Err(error.into())),(Ok(()),Some(settled))=>done(effect,settled.map(|step|step.progress())),(Ok(()),None)=>RetainedWorkPoll::Pending{work:effect.performed_work()}}
}
fn recorded<R>(effect:&mut RetainedAllocationEffect<'_>,turn:RetainedWorkOutcome<R>)->Result<R,ValueError>{let owed=owe(effect,turn.work);let value=turn.result?;owed?;Ok(value)}
impl<T:DslOwnedReceivingField+Sync>PersistentReceiving<T>{
 fn builder(&self)->Result<&PersistentListBuilder<T>,ValueError>{self.builder.as_ref().ok_or_else(invariant)}
 fn shared_demand(&self)->RetirementDemand{let mut demand=if self.birth.is_some(){SealedSharedBirth::<T>::publication_demand()}else{SealedShared::<T>::prepare_birth_demand().unwrap_or(RetirementDemand{capacity_bytes:usize::MAX,depth:1,..Default::default()})};demand.depth=demand.depth.max(1);demand}
 fn shared_work_demand(&self)->u64{if self.birth.is_some(){SealedSharedBirth::<T>::publication_work_demand()}else{SealedShared::<T>::prepare_birth_work_demand()}}
 fn birth_close_demand(&self)->Option<RetirementDemand>{self.birth.as_ref().map(|birth|if birth.terminal_is_empty(){RetirementDemand{copy_bytes:std::mem::size_of::<Option<SealedSharedBirth<T>>>()+std::mem::size_of::<Option<SharedIssuer<T>>>(),depth:1,..Default::default()}}else{birth.next_close_demand()})}
 fn seal(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OwnedPoll{
  if self.row.value.is_none()||self.sealed.value.is_some(){return done(effect,Err(ValueError::invariant("native sealed admission requires its original slots").into()))}
  let born=self.birth.is_some();let d=self.shared_demand();if !admits(effect,d)||effect.maximum_work_units()<self.shared_work_demand(){return idle(effect)}
  let result:Result<RetainedCloneProgress,ValueError>=(||{if !born{
    let mut birth=None;let turn=SealedShared::<T>::prepare_birth(&mut birth,effect);if let Some(birth)=birth{*self.birth=Some(birth);self.issuer=Some(SharedIssuer::owned())}recorded(effect,turn).map(|step|step.progress())
   }else{
    let birth=self.birth.as_mut().ok_or_else(||ValueError::invariant("sealed birth absent"))?;let authority=self.issuer.ok_or_else(||ValueError::invariant("original sealed issuer absent"))?;
    let turn=birth.publish(&mut self.row.value,authority,&mut self.sealed.value,effect);if self.sealed.value.is_some(){self.birth.take();self.issuer.take();}recorded(effect,turn).map(|step|step.progress())
   }})();
  done(effect,result.map_err(Into::into))
 }
 fn close_birth(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OwnedClose{
  let g=effect.grant();let Some(d)=self.birth_close_demand()else{return idle(effect)};if !admits(effect,d){return idle(effect)}
  let Some(birth)=self.birth.as_mut()else{return idle(effect)};
  if !birth.terminal_is_empty(){let turn=birth.close_step(g.maximum_copy_bytes,&mut effect.nested(g));let result=recorded(effect,turn).map(|step|step.progress()).map_err(Into::into);return done(effect,result)}
  local_close(effect,d,||{self.birth.take();self.issuer.take();Ok(())})
 }
 fn selected(&mut self,input:&mut Option<FieldValue>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OwnedPoll{
  if self.closing||self.failed{return done(effect,Err(invariant().into()))}if self.phase==11{return idle(effect)}
  let d=match self.demands(input,body){Ok(d)=>d,Err(error)=>return done(effect,Err(error.into()))};if !admits(effect,d){return idle(effect)}
  let g=effect.grant();
  match self.phase{
   1 if self.builder.as_ref().is_some_and(|builder|!builder.accepts_row())=>{let answer=match self.builder.as_mut(){Some(builder)=>answered(|reply|builder.advance_into(effect,reply)),None=>Err(invariant())};return answer_poll(effect,answer).0},
   1 if self.index==self.length=>{let answer=match self.builder.as_mut(){Some(builder)=>answered(|reply|builder.finish_into(effect,reply)),None=>Err(invariant())};let(poll,step)=answer_poll(effect,answer);if matches!(step,Some(RetainedCloneStep::Complete(_))){self.phase=7}return poll},
   2 if !self.child.is_ready()=>{let poll=self.child.advance_local(&mut self.current.value,body,&mut effect.nested(child_grant(g)));return settle(effect,poll)},
   3 if self.row.value.is_none()=>{let poll=self.child.take_into(&mut self.row.value,&mut effect.nested(child_grant(g)));return settle(effect,normal(poll))},
   4 if !self.child.terminal_is_empty()=>{let poll=self.child.close_step(&mut effect.nested(child_grant(g)));return settle(effect,receiving(poll))},
   5 if self.sealed.value.is_none()=>return self.seal(effect),
   6=>{let answer=match self.builder.as_mut(){Some(builder)=>answered(|reply|builder.adopt_into(&mut self.sealed.value,effect,reply)),None=>Err(invariant())};let(poll,step)=answer_poll(effect,answer);if matches!(step,Some(RetainedCloneStep::Complete(_))){self.index+=1;self.child=OwnedReceivingSlot::new();self.phase=1}return poll},
   7 if !self.builder.as_ref().is_some_and(PersistentListBuilder::output_ready)=>{let answer=match self.builder.as_mut(){Some(builder)=>answered(|reply|builder.advance_into(effect,reply)),None=>Err(invariant())};return answer_poll(effect,answer).0},
   8=>{let answer=match self.builder.as_mut(){Some(builder)=>answered(|reply|builder.take_into(&mut self.output.value,effect,reply)),None=>Err(invariant())};let(poll,step)=answer_poll(effect,answer);if matches!(step,Some(RetainedCloneStep::Complete(_))){self.phase=9}return poll},
   9 if self.builder.as_ref().is_some_and(|builder|!builder.terminal_is_empty())=>{let poll=match self.builder.as_mut(){Some(builder)=>builder_close(builder,body,effect),None=>return done(effect,Err(invariant().into()))};return settle(effect,receiving(poll))},
   10 if !self.source.terminal_is_empty()=>{let poll=self.source.close_step(effect);return settle(effect,receiving(poll))},
   _=>{}
  }
  local(effect,d,||{match self.phase{
   0=>{self.length=match input.as_ref(){Some(FieldValue::List(rows)|FieldValue::Tuple(rows))=>rows.len(),_=>return Err(invalid())};*self.source.value=input.take();*self.builder=Some(PersistentListBuilder::new(self.length));self.phase=1;},
   1=>{let Some(FieldValue::List(rows)|FieldValue::Tuple(rows))=self.source.value.as_mut()else{return Err(invariant())};*self.current.value=Some(std::mem::replace(rows.get_mut(self.index).ok_or_else(invariant)?,FieldValue::Absent));self.phase=2;},
   2=>self.phase=3,3=>self.phase=4,4=>self.phase=5,5=>self.phase=6,7=>self.phase=8,
   9=>{self.builder.take();self.phase=10;},10=>self.phase=11,_=>return Err(invariant())}Ok(())})
 }
}
impl<T:DslOwnedReceivingField+Sync>OwnedFieldReceiving<PersistentList<T>>for PersistentReceiving<T>{
 fn demands(&self,_:&Option<FieldValue>,body:usize)->Result<RetirementDemand,ValueError>{if self.closing||self.failed{return Err(invariant())}match self.phase{
  1 if !self.builder()?.accepts_row()=>self.builder()?.demand(),1 if self.index==self.length=>self.builder()?.finish_demand(),
  2 if !self.child.is_ready()=>self.child.demands(&self.current.value,body).and_then(nested),3 if self.row.value.is_none()=>nested(nested(metadata::<T>())?),4 if !self.child.terminal_is_empty()=>self.child.close_demands(body).and_then(nested),
  5 if self.sealed.value.is_none()=>Ok(self.shared_demand()),6=>self.builder()?.adopt_demand(),7 if !self.builder()?.output_ready()=>self.builder()?.demand(),8=>self.builder()?.take_demand(),9 if !self.builder()?.terminal_is_empty()=>{let builder=self.builder()?;if builder.is_closing(){builder.close_demand(body)}else{Ok(builder.begin_close_demand())}},10 if !self.source.terminal_is_empty()=>self.source.close_demands(body),11=>Ok(Default::default()),_=>Ok(metadata::<()>())}}
 fn select_advance(&self,_:&Option<FieldValue>,body:usize,maximum_work_units:u64)->Result<OwnedAdvanceSelection,ValueError>{if self.closing||self.failed{return Err(invariant())}let local=|work:u64|Ok(OwnedAdvanceSelection::Local{work});match self.phase{
  1 if !self.builder()?.accepts_row()=>local(self.builder()?.advance_work_demand()?),2 if !self.child.is_ready()=>self.child.select_advance(&self.current.value,body,maximum_work_units),3 if self.row.value.is_none()=>local(self.child.take_work_demand()?),4 if !self.child.terminal_is_empty()=>local(self.child.close_work_demand(body)?),
  5 if self.sealed.value.is_none()=>local(self.shared_work_demand()),7 if !self.builder()?.output_ready()=>local(self.builder()?.advance_work_demand()?),9 if !self.builder()?.terminal_is_empty()=>{let builder=self.builder()?;local(if builder.is_closing(){builder.close_work_demand(body)?}else{builder.begin_close_work_demand()})},10 if !self.source.terminal_is_empty()=>local(self.source.close_work_demand(body)?),11=>local(0),_=>local(1)}}
 fn advance_nested(&mut self,_:&mut Option<FieldValue>,native:&mut NativeDecodeControl<'_>,witness:OriginalSourceTurnWitness<'_>,body:usize,maximum_work_units:u64,g:RetainedCloneGrant)->OwnedPoll{if self.failed||self.closing||self.phase!=2||self.child.is_ready(){return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invariant().into()),work:0})}let poll=self.child.advance_nested(&mut self.current.value,native,witness,body,maximum_work_units,child_grant(g));poison(&mut self.failed,&poll);poll}
 fn advance_local(&mut self,input:&mut Option<FieldValue>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OwnedPoll{let poll=self.selected(input,body,effect);poison(&mut self.failed,&poll);poll}
 fn is_ready(&self)->bool{!self.closing&&!self.failed&&self.phase==11&&self.output.value.is_some()}
 fn take_into(&mut self,output:&mut Option<PersistentList<T>>,effect:&mut RetainedAllocationEffect<'_>)->OwnedTake{if !self.is_ready(){return done(effect,Err(invariant()))}move_into(&mut self.output.value,output,effect)}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if let Some(d)=self.birth_close_demand(){return Ok(d)}if self.terminal_is_empty(){return Ok(Default::default())}if !self.closing{return Ok(metadata::<()>())}if !self.child.terminal_is_empty(){return self.child.close_demands(body).and_then(nested)}if !self.current.terminal_is_empty(){return self.current.close_demands(body)}if !self.row.terminal_is_empty(){return self.row.close_demands(body)}if !self.sealed.terminal_is_empty(){return self.sealed.close_demands(body)}if let Some(builder)=self.builder.as_ref(){if !builder.terminal_is_empty(){return if builder.is_closing(){builder.close_demand(body)}else{Ok(builder.begin_close_demand())}}return Ok(metadata::<()>())}if !self.output.terminal_is_empty(){return self.output.close_demands(body)}self.source.close_demands(body)}
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if self.birth.is_some(){return Ok(1)}if self.terminal_is_empty(){return Ok(0)}if !self.closing{return Ok(1)}if !self.child.terminal_is_empty(){return self.child.close_work_demand(body)}if !self.current.terminal_is_empty(){return self.current.close_work_demand(body)}if !self.row.terminal_is_empty(){return self.row.close_work_demand(body)}if !self.sealed.terminal_is_empty(){return self.sealed.close_work_demand(body)}if let Some(builder)=self.builder.as_ref(){if !builder.terminal_is_empty(){return if builder.is_closing(){builder.close_work_demand(body)}else{Ok(builder.begin_close_work_demand())}}return Ok(1)}if !self.output.terminal_is_empty(){return self.output.close_work_demand(body)}self.source.close_work_demand(body)}
 fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OwnedClose{
  if self.birth.is_some(){return self.close_birth(effect)}if self.terminal_is_empty(){return idle(effect)}
  let g=effect.grant();let d=match self.close_demands(g.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return done(effect,Err(error.into()))};if !admits(effect,d){return idle(effect)}
  if !self.closing{return local_close(effect,d,||{self.closing=true;Ok(())})}
  if !self.child.terminal_is_empty(){let poll=self.child.close_step(&mut effect.nested(child_grant(g)));return settle(effect,poll)}
  if !self.current.terminal_is_empty(){return self.current.close_step(effect)}if !self.row.terminal_is_empty(){return self.row.close_step(effect)}if !self.sealed.terminal_is_empty(){return self.sealed.close_step(effect)}
  if let Some(builder)=self.builder.as_mut(){if !builder.terminal_is_empty(){let poll=builder_close(builder,g.maximum_copy_bytes,effect);return settle(effect,poll)}return local_close(effect,d,||{self.builder.take();Ok(())})}
  if !self.output.terminal_is_empty(){return self.output.close_step(effect)}self.source.close_step(effect)
 }
 fn terminal_is_empty(&self)->bool{self.birth.is_none()&&self.child.terminal_is_empty()&&self.current.terminal_is_empty()&&self.row.terminal_is_empty()&&self.sealed.terminal_is_empty()&&self.builder.is_none()&&self.output.terminal_is_empty()&&self.source.terminal_is_empty()}
}
impl<T:DslOwnedReceivingField+Sync>Drop for PersistentReceiving<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"persistent field receiving retains original row and page custody");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.builder);ManuallyDrop::drop(&mut self.birth)}}}}
#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
