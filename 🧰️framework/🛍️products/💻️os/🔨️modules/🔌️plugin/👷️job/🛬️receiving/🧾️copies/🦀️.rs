//! 🧾️ An original typed value supplies a bounded independent metadata owner per declared recipient.
use semio_framework_value::retained_clone::RetainedClone;
use semio_framework_job::{OriginalConstructionAdmission,OriginalSourceBinding,OriginalTurnFailure,StepContext};
use semio_framework_value::{OriginalRetirementFailure,RetainedCloneGrant,RetainedCloneStep,RetainedWorkPoll,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{NativeAllocationIdentity,RetainedCloneSource,RetainedCloneOwner,RetainedCloneClose}};
use crate::original_steps::{settled_grant,admission_demand,admission_return_demand,admission_settle,admission_step,adopted,begun,built,built_with,construction,fixed,funded,receipt,replied,retired,retired_native,retired_poll,scoped,unbuilt,unit};
use std::{collections::VecDeque,mem::{ManuallyDrop,size_of}};
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original typed metadata copies source changed")}
/// 🪡️ Each resulting origin owns its actual bounded text copies and preserves the complete original metadata.
pub struct OriginalValueCopies<T:RetainedClone>{binding:OriginalSourceBinding,decoder:(NativeAllocationIdentity,u64),body:usize,total:usize,original:ManuallyDrop<Option<T>>,source:ManuallyDrop<Option<RetainedCloneSource<T>>>,admission:Option<semio_framework_value::retained_clone::RetainedCloneSourceAdmission<T,T,()>>,cursor:ManuallyDrop<Option<RetainedCloneOwner<T>>>,values:ManuallyDrop<Option<VecDeque<T>>>,retirement:RetainedCloneClose,construction:OriginalConstructionAdmission,unit:Option<()>,phase:u8,closing:bool}
impl<T:RetainedClone> OriginalValueCopies<T>{
 /// 🧮️ Quotes the actual boxed copies frame the caller must fund before the original transfers.
 pub fn admission_demands()->RetirementDemand{RetirementDemand{capacity_bytes:size_of::<Self>(),..fixed::<Self>()}}
 pub fn admit(original:&mut Option<T>,count:usize,cx:&mut StepContext<'_>,admission:&mut OriginalConstructionAdmission,output:&mut Option<Box<Self>>)->Result<bool,OriginalTurnFailure>{if original.is_none()&&output.is_none(){return Err(invalid().into())}let binding=OriginalSourceBinding::borrow(cx);let body=cx.retained_grant().maximum_copy_bytes;let d=RetirementDemand{capacity_bytes:size_of::<Self>(),..fixed::<Self>()};let settled=built_with(cx,admission,output,construction(d),|control|Ok((Box::new(Self{binding,decoder:(control.ledger_identity(),control.retirement_recipient_identity().ok_or_else(invalid)?),body,total:count,original:ManuallyDrop::new(original.take()),source:ManuallyDrop::new(None),admission:None,cursor:ManuallyDrop::new(None),values:ManuallyDrop::new(None),retirement:Default::default(),construction:Default::default(),unit:None,phase:0,closing:false}),receipt(d))))?;Ok(adopted(settled,admission,output,|owner|&mut owner.construction))}
 fn next_phase(&mut self,phase:u8,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{unit(cx,&mut self.construction,&mut self.unit,fixed::<u8>(),||{self.phase=phase;Ok(())})?;Ok(())}
 fn validate(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{if !self.binding.matches(cx)||cx.native_receiving_identity()!=Some(self.decoder){return Err(invalid().into())}Ok(())}
 pub fn advance(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.validate(cx)?;if self.closing{return Err(invalid().into())}if cx.is_cancelled(){return Err(ValueError::literal(ValueRefusalKind::Canceled,"original typed metadata copies was cancelled").into())}if cx.should_yield(){return Ok(false)}match self.phase{
  0=>{if self.admission.is_none(){self.admission=Some(RetainedCloneSource::admission_owned(self.original.take().ok_or_else(invalid)?,()));return Ok(false)}let admission=self.admission.as_ref().unwrap();let d=if admission.phase()=="ready"{admission.take_output_demand()}else{admission_demand(admission)?};funded(cx,d,|effect|{let admission=self.admission.as_mut().unwrap();if admission.phase()!="ready"{return admission_step(admission,effect)}let turn=admission.take_output(&mut self.source,effect);if admission.terminal_is_empty(){self.admission=None;}RetainedWorkPoll::Complete(turn)})?;if self.admission.is_none(){self.phase=1}},
  1=>{let d=RetirementDemand{capacity_bytes:self.total.checked_mul(size_of::<T>()).ok_or_else(invalid)?,..fixed::<VecDeque<T>>()};built(cx,&mut self.construction,&mut self.values,construction(d),||{let mut values=VecDeque::new();values.try_reserve_exact(self.total).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original typed metadata copies rows could not be reserved"))?;let mut p=receipt(d);p.retained_capacity_bytes=values.capacity()*size_of::<T>();Ok((values,p))})?;if self.values.is_some(){self.phase=2}},
  2=>{if self.values.as_ref().ok_or_else(invalid)?.len()==self.total{self.next_phase(6,cx)?;return Ok(false)}let d=fixed::<RetainedCloneOwner<T>>();built(cx,&mut self.construction,&mut self.cursor,construction(d),||Ok((RetainedCloneOwner::default(),receipt(d))))?;if self.cursor.is_some(){self.phase=3}},
  3=>{let source=self.source.as_ref().ok_or_else(invalid)?.try_borrow()?;let body=self.body;let cursor=self.cursor.as_mut().ok_or_else(invalid)?;let maximum=cx.fuel_remaining();let d=cursor.advance_demands(source,body,maximum)?;scoped(cx,d,|effect|replied::<OriginalRetirementFailure>(effect,|effect,reply|cursor.advance_into(source,body,maximum,effect,reply).map(drop)))?;if cursor.output_ready(){self.phase=4}},
  4=>{let cursor=self.cursor.as_mut().ok_or_else(invalid)?;let d=cursor.take_demand()?;let mut value=None;scoped(cx,d,|effect|replied::<ValueError>(effect,|effect,reply|cursor.take(&mut value,effect,reply).map(drop)))?;if let Some(value)=value{self.values.as_mut().ok_or_else(invalid)?.push_back(value);self.phase=5}},
  5=>{if self.close_cursor(cx)?{self.next_phase(2,cx)?}},
  6=>{if self.close_source(cx)?{self.next_phase(7,cx)?}},
  7=>return Ok(true),_=>unreachable!(),
 }Ok(self.phase==7)}
 fn close_cursor(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{let Some(cursor)=self.cursor.as_mut()else{return Ok(true)};if !cursor.terminal_is_empty(){let d=cursor.selected_close_demand(self.body)?;let body=self.body;retired_native(cx,d,|effect|replied::<OriginalRetirementFailure>(effect,|effect,reply|cursor.close_into(body,effect,reply).map(drop)))?;return Ok(false)}let d=fixed::<Option<RetainedCloneOwner<T>>>();retired(cx,d,|_|{self.cursor.take();Ok(RetainedCloneStep::Complete(receipt(d)))})?;Ok(false)}
 fn close_source(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{if let Some(admission)=self.admission.as_ref(){let d=if admission.phase()=="ready"{admission.take_output_demand()}else{admission_return_demand(admission)?};retired(cx,d,|effect|{let admission=self.admission.as_mut().unwrap();if admission.phase()=="ready"{let step=admission.take_output(&mut self.source,effect).result?;if admission.terminal_is_empty(){self.admission=None;}return Ok(RetainedCloneStep::Progress(step.progress()))}if let Some(step)=admission_settle(admission,effect)?{return Ok(step)}let mut back=None;let step=admission.return_inputs(&mut back,effect).result?;if let Some((value,()))=back{*self.original=Some(value);self.admission=None;}Ok(RetainedCloneStep::Progress(step.progress()))})?;return Ok(false)}let Some(source)=self.source.as_mut()else{return Ok(true)};if !source.terminal_is_empty(){let d=source.next_close_demand(self.body)?;let body=self.body;retired_poll(cx,d,|effect|source.close_step(body,effect))?;return Ok(false)}let d=fixed::<Option<RetainedCloneSource<T>>>();retired(cx,d,|_|{self.source.take();Ok(RetainedCloneStep::Complete(receipt(d)))})?;Ok(false)}
 pub fn take(&mut self,cx:&mut StepContext<'_>,output:&mut Option<VecDeque<T>>)->Result<bool,OriginalTurnFailure>{self.validate(cx)?;if output.is_none()&&(self.phase!=7||self.closing||self.values.is_none()){return Err(invalid().into())}let d=Self::take_demands();built(cx,&mut self.construction,output,construction(d),||Ok((self.values.take().unwrap(),receipt(d))))}
 pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.validate(cx)?;self.closing=true;if !self.close_cursor(cx)?||!self.close_source(cx)?{return Ok(false)}if self.original.is_some(){let d=self.retirement.next_owner_demands::<T>(true,&None,self.body)?;begun(cx,d,|effect|self.retirement.begin_granted(&mut self.original,effect))?;return Ok(false)}if !self.retirement.is_empty(){let d=self.retirement.next_owner_demands::<T>(false,&None,self.body)?;let body=self.body;retired_poll(cx,d,|effect|self.retirement.step_granted(body,effect))?;return Ok(false)}if self.values.is_some(){let d=self.retirement.next_owner_demands::<VecDeque<T>>(true,&None,self.body)?;begun(cx,d,|effect|self.retirement.begin_granted(&mut self.values,effect))?;return Ok(false)}if !unbuilt(cx,&mut self.construction)?{return Ok(false)}Ok(self.terminal_is_empty())}
 /// 📐️ Quotes the physical demand of the next advance, so a caller funds exactly the action this owner takes next; `fuel` is the caller's remaining work.
 fn advance_demands(&self,fuel:u64)->Result<RetirementDemand,ValueError>{
  Ok(match self.phase{
   0=>match self.admission.as_ref(){None=>Default::default(),Some(admission)=>if admission.phase()=="ready"{admission.take_output_demand()}else{admission_demand(admission)?}},
   1=>RetirementDemand{capacity_bytes:self.total.checked_mul(size_of::<T>()).ok_or_else(invalid)?,..fixed::<VecDeque<T>>()},
   2=>if self.values.as_ref().ok_or_else(invalid)?.len()==self.total{fixed::<u8>()}else{fixed::<RetainedCloneOwner<T>>()},
   3=>self.cursor.as_ref().ok_or_else(invalid)?.advance_demands(self.source.as_ref().ok_or_else(invalid)?.try_borrow()?,self.body,fuel)?,
   4=>self.cursor.as_ref().ok_or_else(invalid)?.take_demand()?,
   5=>self.cursor_close_demands()?,
   6=>self.source_close_demands()?,
   _=>fixed::<u8>(),
  })
 }
 fn cursor_close_demands(&self)->Result<RetirementDemand,ValueError>{Ok(match self.cursor.as_ref(){None=>fixed::<u8>(),Some(cursor)=>if cursor.terminal_is_empty(){fixed::<Option<RetainedCloneOwner<T>>>()}else{cursor.selected_close_demand(self.body)?}})}
 fn source_close_demands(&self)->Result<RetirementDemand,ValueError>{if let Some(admission)=self.admission.as_ref(){return if admission.phase()=="ready"{Ok(admission.take_output_demand())}else{admission_return_demand(admission)}}Ok(match self.source.as_ref(){None=>fixed::<u8>(),Some(source)=>if source.terminal_is_empty(){fixed::<Option<RetainedCloneSource<T>>>()}else{source.next_close_demand(self.body)?}})}
 fn take_demands()->RetirementDemand{fixed::<(Option<VecDeque<T>>,Option<VecDeque<T>>)>()}
 /// 🎫️ The grant of the unique handoff of the copied rows.
 pub fn take_grant()->Result<RetainedCloneGrant,ValueError>{settled_grant::<VecDeque<T>>(Self::take_demands())}
 /// 🎫️ The grant of the next advance; `fuel` is the caller's remaining work.
 pub fn advance_grant(&self,fuel:u64)->Result<RetainedCloneGrant,ValueError>{let demand=self.advance_demands(fuel)?;match self.phase{1=>settled_grant::<VecDeque<T>>(demand),2 if self.values.as_ref().ok_or_else(invalid)?.len()!=self.total=>settled_grant::<RetainedCloneOwner<T>>(demand),_=>settled_grant::<()>(demand)}}
 /// 🎫️ The grant of the next close step.
 pub fn close_grant(&self)->Result<RetainedCloneGrant,ValueError>{settled_grant::<()>(self.close_demands()?)}
 fn close_demands(&self)->Result<RetirementDemand,ValueError>{
  if self.cursor.is_some(){return self.cursor_close_demands()}
  if self.admission.is_some()||self.source.is_some(){return self.source_close_demands()}
  if self.original.is_some(){return self.retirement.next_owner_demands::<T>(true,&None,self.body)}
  if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<T>(false,&None,self.body)}
  if self.values.is_some(){return self.retirement.next_owner_demands::<VecDeque<T>>(true,&None,self.body)}
  Ok(fixed::<u8>())
 }
 pub fn terminal_is_empty(&self)->bool{self.original.is_none()&&self.admission.is_none()&&self.source.is_none()&&self.cursor.is_none()&&self.values.is_none()&&self.retirement.is_empty()&&self.construction.terminal_is_empty()}
 pub fn release_demands()->RetirementDemand{RetirementDemand{release_bytes:size_of::<Self>(),depth:1,..Default::default()}}
}
impl<T:RetainedClone> Drop for OriginalValueCopies<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original typed metadata copies requires funded closure")}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
