//! 🧾️ An original typed value supplies a bounded independent metadata owner per declared recipient.
use semio_framework_value::retained_clone::RetainedClone;
use semio_framework_job::{OriginalConstructionAdmission,OriginalConstructionDemand,OriginalConstructionFailure,OriginalConstructionStatus,OriginalSourceBinding,OriginalTurnFailure,StepContext,admit_original_construction};
use semio_framework_value::{OriginalRetirementFailure,RetirementDemand,RetainedCloneStep,RetainedWorkPoll,ValueError,ValueRefusalKind,retained_clone::{NativeAllocationIdentity,RetainedCloneSource,RetainedCloneOwner,RetainedCloneClose,RetainedCloneSourceAdmission}};
use super::original_steps::{begin,fixed,forgotten,forward,funds,moved,paid,receipt,replied,retired,settle};
use std::{collections::VecDeque,mem::{ManuallyDrop,size_of}};
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original typed metadata copies source changed")}
fn construction(d:RetirementDemand)->OriginalConstructionDemand{OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:d.capacity_bytes,release_bytes:d.release_bytes,depth:d.depth}}
/// 🪡️ Each resulting origin owns its actual bounded text copies and preserves the complete original metadata.
pub struct OriginalValueCopies<T:RetainedClone>{binding:OriginalSourceBinding,decoder:(NativeAllocationIdentity,u64),body:usize,total:usize,original:ManuallyDrop<Option<T>>,source:ManuallyDrop<Option<RetainedCloneSource<T>>>,admission:Option<RetainedCloneSourceAdmission<T,T,()>>,cursor:ManuallyDrop<Option<RetainedCloneOwner<T>>>,values:ManuallyDrop<Option<VecDeque<T>>>,retirement:RetainedCloneClose,phase:u8,closing:bool}
impl<T:RetainedClone> OriginalValueCopies<T>{
 /// 🧮️ Quotes the actual boxed copies frame the caller must fund before the original transfers.
 pub fn admission_demands()->RetirementDemand{RetirementDemand{capacity_bytes:size_of::<Self>(),..fixed::<Self>()}}
 pub fn admit(original:&mut Option<T>,count:usize,cx:&mut StepContext<'_>,admission:&mut OriginalConstructionAdmission,failure:&mut Option<OriginalConstructionFailure>,output:&mut Option<Box<Self>>)->Result<bool,OriginalTurnFailure>{
  cx.original_source_parent()?;
  if output.is_some(){return if admission.terminal_is_empty(){Ok(true)}else{admission.reset_step(cx)}}
  if failure.is_some()||original.is_none(){return Err(invalid().into())}
  let binding=OriginalSourceBinding::borrow(cx);let body=cx.retained_grant().maximum_copy_bytes;let d=Self::admission_demands();
  let status=admit_original_construction(cx,output,admission,construction(d),1,|native,_|{let result=(||{let decoder=(native.ledger_identity(),native.retirement_recipient_identity().ok_or_else(invalid)?);Ok((Box::new(Self{binding,decoder,body,total:count,original:ManuallyDrop::new(original.take()),source:ManuallyDrop::new(None),admission:None,cursor:ManuallyDrop::new(None),values:ManuallyDrop::new(None),retirement:Default::default(),phase:0,closing:false}),receipt(d)))})();semio_framework_value::RetainedWorkOutcome{work:u128::from(result.is_ok()),result}})?;
  match status{OriginalConstructionStatus::Ready=>admission.reset_step(cx),OriginalConstructionStatus::Pending=>Ok(false),OriginalConstructionStatus::Failed=>{if !admission.take_failure_into_step(failure,cx)?{return Ok(false)}Err(invalid().into())}}
 }
 fn validate(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{if !self.binding.matches(cx)||cx.native_receiving_identity()!=Some(self.decoder){return Err(invalid().into())}Ok(())}
 pub fn advance(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.validate(cx)?;if self.closing{return Err(invalid().into())}if cx.is_cancelled(){return Err(ValueError::literal(ValueRefusalKind::Canceled,"original typed metadata copies was cancelled").into())}if cx.should_yield(){return Ok(false)}match self.phase{
  0=>{
   if self.admission.is_none()&&self.source.is_none(){if self.original.is_none(){return Err(invalid().into())}moved(cx,fixed::<Option<RetainedCloneSourceAdmission<T,T,()>>>(),||{let value=self.original.take().expect("copies hold their original");self.admission=Some(RetainedCloneSource::admission_owned(value,()));})?;return Ok(false)}
   if let Some(admission)=self.admission.as_ref(){let ready=admission.phase()=="ready";let d=if ready{fixed::<Option<RetainedCloneSource<T>>>()}else{admission.next_demand()?};
    forward(cx,d,|effect|{let admission=self.admission.as_mut().expect("admission checked present");if !ready{let turn=admission.advance(effect);return settle(effect,turn.result)}let mut taken=None;let turn=admission.take_output(&mut taken,effect);let result=turn.result.map(|step|{if taken.is_some(){*self.source=taken;self.admission=None;}step});settle(effect,result)})?;}
   if self.source.is_some(){self.phase=1}
  },
  1=>{let d=RetirementDemand{capacity_bytes:self.total.checked_mul(size_of::<T>()).ok_or_else(invalid)?,..fixed::<VecDeque<T>>()};if funds(cx,d){let mut values=VecDeque::new();values.try_reserve_exact(self.total).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original typed metadata copies rows could not be reserved"))?;let mut p=receipt(d);p.retained_capacity_bytes=values.capacity()*size_of::<T>();*self.values=Some(values);self.phase=2;cx.consume_retained(p)?;cx.consume_fuel(1)}},
  2=>{if self.values.as_ref().ok_or_else(invalid)?.len()==self.total{if moved(cx,fixed::<u8>(),||{})?{self.phase=6}return Ok(false)}let d=fixed::<RetainedCloneOwner<T>>();if moved(cx,d,||{*self.cursor=Some(RetainedCloneOwner::default());})?{self.phase=3}},
  3=>{let source=self.source.as_ref().ok_or_else(invalid)?.try_borrow()?;let cursor=self.cursor.as_mut().ok_or_else(invalid)?;let units=cx.fuel_remaining();let d=cursor.advance_demands(source,self.body,units)?;let body=self.body;let mut finished=false;
   forward(cx,d,|effect|{let poll=replied::<OriginalRetirementFailure>(effect,|effect,reply|cursor.advance_into(source,body,units,effect,reply));if let RetainedWorkPoll::Complete(turn)=&poll{finished=matches!(turn.result,Ok(RetainedCloneStep::Complete(_)))}poll})?;if finished{self.phase=4}},
  4=>{let cursor=self.cursor.as_mut().ok_or_else(invalid)?;let d=cursor.take_demand()?;let values=&mut self.values;let mut taken=false;
   forward(cx,d,|effect|replied::<ValueError>(effect,|effect,reply|{let mut value=None;let done=cursor.take(&mut value,effect,reply)?;if let Some(value)=value{values.as_mut().expect("values reserved").push_back(value);taken=true}Ok(done)}))?;if taken{self.phase=5}},
  5=>{if self.close_cursor(cx)?&&moved(cx,fixed::<u8>(),||{})?{self.phase=2}},
  6=>{if self.close_source(cx)?&&moved(cx,fixed::<u8>(),||{})?{self.phase=7}},
  7=>return Ok(true),_=>unreachable!(),
 }Ok(self.phase==7)}
 fn continue_retirement(&mut self,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{let body=self.body;let d=self.retirement.next_owner_demands::<()>(false,&None,body)?;retired(cx,d,|effect|self.retirement.step_granted(effect.grant().maximum_copy_bytes,effect))?;Ok(())}
 fn close_cursor(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if !self.retirement.is_empty(){self.continue_retirement(cx)?;return Ok(false)}
  if self.cursor.is_some(){let d=self.retirement.next_owner_demands::<RetainedCloneOwner<T>>(true,&None,self.body)?;retired(cx,d,|effect|begin(&mut self.retirement,&mut self.cursor,effect))?;return Ok(false)}
  Ok(true)
 }
 fn close_source(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  let body=self.body;
  if !self.retirement.is_empty(){self.continue_retirement(cx)?;return Ok(false)}
  if let Some(admission)=self.admission.as_ref(){let ready=admission.phase()=="ready";let d=if ready{fixed::<Option<RetainedCloneSource<T>>>()}else{admission.next_return_demand()?};
   retired(cx,d,|effect|{let admission=self.admission.as_mut().expect("admission checked present");if ready{let mut taken=None;let turn=admission.take_output(&mut taken,effect);let result=turn.result.map(|step|{if taken.is_some(){*self.source=taken;self.admission=None;}RetainedCloneStep::Progress(step.progress())});return settle(effect,result)}let mut back=None;let turn=admission.return_inputs(&mut back,effect);let result=turn.result.map(|step|{if let Some((value,()))=back{*self.original=Some(value);self.admission=None;}RetainedCloneStep::Progress(step.progress())});settle(effect,result)})?;return Ok(false)}
  if let Some(source)=self.source.as_mut(){if !source.terminal_is_empty(){let d=source.next_close_demand(body)?;retired(cx,d,|effect|source.close_step(body,effect))?;return Ok(false)}forgotten(cx,fixed::<Option<RetainedCloneSource<T>>>(),||{self.source.take();})?;return Ok(false)}
  Ok(true)
 }
 pub fn take(&mut self,cx:&mut StepContext<'_>,output:&mut Option<VecDeque<T>>)->Result<bool,OriginalTurnFailure>{self.validate(cx)?;if self.phase!=7||self.closing||self.values.is_none()||output.is_some(){return Err(invalid().into())}moved(cx,fixed::<(Option<VecDeque<T>>,Option<VecDeque<T>>)>(),||{*output=self.values.take();})}
 pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate(cx)?;self.closing=true;let body=self.body;
  if !self.close_cursor(cx)?||!self.close_source(cx)?{return Ok(false)}
  if self.original.is_some(){let d=self.retirement.next_owner_demands::<T>(true,&None,body)?;retired(cx,d,|effect|begin(&mut self.retirement,&mut self.original,effect))?;return Ok(false)}
  if self.values.is_some(){let d=self.retirement.next_owner_demands::<VecDeque<T>>(true,&None,body)?;retired(cx,d,|effect|begin(&mut self.retirement,&mut self.values,effect))?;return Ok(false)}
  Ok(self.terminal_is_empty())
 }
 pub fn terminal_is_empty(&self)->bool{self.original.is_none()&&self.admission.is_none()&&self.source.is_none()&&self.cursor.is_none()&&self.values.is_none()&&self.retirement.is_empty()}
 pub fn release_demands()->RetirementDemand{RetirementDemand{release_bytes:size_of::<Self>(),depth:1,..Default::default()}}
}
impl<T:RetainedClone> Drop for OriginalValueCopies<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original typed metadata copies requires funded closure")}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
