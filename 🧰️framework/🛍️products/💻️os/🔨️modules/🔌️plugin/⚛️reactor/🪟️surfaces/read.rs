//! 👁️ Cached view reads pin the actual registry owner; typed captures share the same source lease.
use super::*;
use std::mem::size_of;
use semio_framework_value::{OriginalRetirementPoll,RetainedAllocationEffect,retained_clone::RetainedCloneRef};
use semio_framework_job::InteractiveJobCloseStep;
pub(super) fn retire(cx:&mut StepContext<'_>,d:RetirementDemand,run:impl FnOnce(&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{cx.with_original_retirement(|cx|retired(cx,d,run))}
pub(super) fn retire_metadata(cx:&mut StepContext<'_>,d:RetirementDemand,run:impl FnOnce())->Result<bool,OriginalTurnFailure>{cx.with_original_retirement(|cx|forgotten(cx,d,run))}
pub(super) fn close_cursor(cx:&mut StepContext<'_>,cursor:&mut RetainedFieldCursor<ViewModel>,body:usize)->Result<(),OriginalTurnFailure>{if cursor.close_started(){let d=cursor.close_demands(body)?;retired(cx,d,|effect|replied(effect,|effect,reply|cursor.close_into(body,effect,reply)))?;}else{let d=cursor.begin_close_demand()?;retired(cx,d,|effect|replied(effect,|effect,reply|cursor.begin_close_into(effect,reply)))?;}Ok(())}
pub(super) fn cursor_close_demands(cursor:&RetainedFieldCursor<ViewModel>,body:usize)->Result<RetirementDemand,ValueError>{if cursor.close_started(){cursor.close_demands(body)}else{cursor.begin_close_demand()}}
/// 🧷️ A boxed read or embedded capture pin keeps the exact registry owner immutable across ready frames.
pub struct OriginalSurfaceViewRead{binding:OriginalSourceBinding,decoder:(semio_framework_value_native_owned::NativeAllocationIdentity,u64),body:usize,registry:Option<usize>,admission:Option<RetainedCloneSourceAdmission<ViewModel,ViewModel,()>>,phase:u8,closing:bool,locked:bool,selection:Option<(usize,usize)>,scan:usize,offset:usize,selected:bool}
impl OriginalSurfaceViewRead{
 fn new(binding:OriginalSourceBinding,decoder:(semio_framework_value_native_owned::NativeAllocationIdentity,u64),body:usize)->Self{Self{binding,decoder,body,registry:None,admission:None,phase:0,closing:false,locked:false,selection:None,scan:0,offset:0,selected:false}}
 pub fn birth_demand()->RetirementDemand{RetirementDemand{capacity_bytes:size_of::<Self>(),..demand::<Self>()}}
 pub fn admit(cx:&mut StepContext<'_>,admission:&mut OriginalConstructionAdmission,failure:&mut Option<OriginalConstructionFailure>,slot:&mut Option<Box<Self>>)->Result<bool,OriginalTurnFailure>{
  let d=Self::birth_demand();let binding=OriginalSourceBinding::borrow(cx);let body=cx.retained_grant().maximum_copy_bytes;
  born(cx,slot,admission,failure,d,|native|{let recipient=native.retirement_recipient_identity().ok_or_else(invalid)?;Ok(Box::new(Self::new(binding,(native.ledger_identity(),recipient),body)))})
 }
 fn validate(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{if !self.binding.matches(cx)||cx.native_receiving_identity()!=Some(self.decoder){return Err(invalid().into())}cx.original_source_parent()?.validate_borrowed_context(cx)?;Ok(())}
 fn registry(&self,registry:&SurfaceContexts)->Result<(),ValueError>{
  if self.registry.is_some_and(|original|original!=registry as*const _ as usize){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original cached view registry changed"))}
  if self.locked&&registry.cached_capture!=Some(self as*const _ as usize){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original cached view read lost its registry lease"))}Ok(())
 }
 pub fn ready(&self)->bool{!self.closing&&self.phase==4}
 /// 🔎️ A pending admission is a control refusal; only a ready original absence yields None.
 pub fn borrow<'a>(&'a self,registry:&'a SurfaceContexts,cx:&StepContext<'_>)->Result<Option<RetainedCloneRef<'a,ViewModel>>,OriginalTurnFailure>{
  self.validate(cx)?;self.registry(registry)?;
  if !self.ready(){return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original cached view read is not ready").into())}
  if cx.is_cancelled(){return Err(ValueError::literal(ValueRefusalKind::Canceled,"original cached view read cancelled").into())}
  Ok(registry.cached_source.as_ref().map(RetainedCloneSource::borrow))
 }
 pub fn advance(&mut self,registry:&mut SurfaceContexts,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.advance_selected(registry,None,cx)}
 pub fn advance_surface(&mut self,registry:&mut SurfaceContexts,surface:&str,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.advance_selected(registry,Some(surface),cx)}
 fn advance_selected(&mut self,registry:&mut SurfaceContexts,surface:Option<&str>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate(cx)?;self.registry(registry)?;if self.closing{return Err(invalid().into())}
  if cx.is_cancelled(){return Err(ValueError::literal(ValueRefusalKind::Canceled,"original cached view read cancelled").into())}
  let selection=surface.map(|surface|(surface.as_ptr()as usize,surface.len()));
  if self.phase>0&&self.selection!=selection{return Err(invalid().into())}
  match self.phase{
   0=>{
    if registry.cached_capture.is_some(){return Ok(false)}
    moved(cx,demand::<Self>(),||{self.registry=Some(registry as*const _ as usize);registry.cached_capture=Some(self as*const _ as usize);registry.captured_slot=None;self.selection=selection;self.selected=surface.is_none();self.locked=true;self.phase=1})?;
   },
   1=>{
    if self.selected{moved(cx,demand::<u8>(),||self.phase=2)?;return Ok(false)}
    let surface=surface.ok_or_else(invalid)?;
    if self.scan==registry.slots.len(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"selected surface has no original host context").into())}
    let candidate=registry.slots[self.scan].as_ref().map(|binding|binding.surface.as_str());
    let count=candidate.map_or(0,|candidate|if candidate.len()==surface.len(){64.min(surface.len().saturating_sub(self.offset))}else{0});
    let d=RetirementDemand{copy_bytes:count.saturating_mul(2)+size_of::<(usize,usize,bool,Option<usize>)>(),depth:1,..Default::default()};
    moved(cx,d,||{
     let equal=candidate.is_some_and(|candidate|candidate.len()==surface.len()&&candidate.as_bytes()[self.offset..self.offset+count].iter().zip(&surface.as_bytes()[self.offset..self.offset+count]).fold(true,|equal,(a,b)|equal&(*a==*b)));
     if equal{self.offset+=count;if self.offset==surface.len(){registry.captured_slot=Some(self.scan);self.selected=true}}else{self.scan+=1;self.offset=0}
    })?;
   },
   2=>{
    let present=if let Some(index)=registry.captured_slot{registry.slots[index].as_ref().and_then(|binding|binding.view_state.as_ref()).is_some()}else{registry.view_state.is_some()};
    if !present{moved(cx,demand::<u8>(),||self.phase=4)?;return Ok(self.ready())}
    moved(cx,demand::<Option<RetainedCloneSourceAdmission<ViewModel,ViewModel,()>>>(),||{
     let view=if let Some(index)=registry.captured_slot{&mut registry.slots[index].as_mut().unwrap().view_state}else{&mut registry.view_state};
     self.admission=Some(RetainedCloneSource::admission_owned(view.take().unwrap(),()));self.phase=3;
    })?;
   },
   3=>{
    let admission=self.admission.as_ref().ok_or_else(invalid)?;let ready=admission.phase()=="ready";let d=admission.next_demand()?;
    forward(cx,d,|effect|{
     let admission=self.admission.as_mut().unwrap();
     let turn=if ready{admission.take_output(&mut registry.cached_source,effect)}else{admission.advance(effect)};
     settle(effect,turn.result)
    })?;
    if self.admission.as_ref().is_some_and(|admission|admission.terminal_is_empty()){self.admission=None}
    if registry.cached_source.is_some(){self.phase=4}
   },
   4=>return Ok(true),
   _=>return Err(invalid().into()),
  }Ok(self.ready())
 }
 fn vacant(registry:&SurfaceContexts)->Result<(),ValueError>{
  let occupied=if let Some(index)=registry.captured_slot{registry.slots.get(index).and_then(Option::as_ref).ok_or_else(invalid)?.view_state.is_some()}else{registry.view_state.is_some()};
  if occupied{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original view handback destination is occupied"))}Ok(())
 }
 fn restore(registry:&mut SurfaceContexts,view:ViewModel){let destination=if let Some(index)=registry.captured_slot{&mut registry.slots[index].as_mut().unwrap().view_state}else{&mut registry.view_state};*destination=Some(view)}
 pub fn close_step(&mut self,registry:&mut SurfaceContexts,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate(cx)?;self.registry(registry)?;
  if !self.closing{retire_metadata(cx,demand::<bool>(),||self.closing=true)?;return Ok(false)}
  if let Some(admission)=self.admission.as_ref(){
   Self::vacant(registry)?;let published=admission.phase()=="ready";let d=if published{admission.next_demand()?}else{admission.next_return_demand()?};
   retire(cx,d,|effect|{
    let admission=self.admission.as_mut().unwrap();
    let result=if published{admission.take_output(&mut registry.cached_source,effect).result}else{let mut inputs=None;let turn=admission.return_inputs(&mut inputs,effect);if let Some((view,()))=inputs{Self::restore(registry,view)}turn.result};
    if admission.terminal_is_empty(){self.admission=None}settle(effect,result)
   })?;return Ok(false)
  }
  if self.locked&&registry.cached_source.is_some(){
   Self::vacant(registry)?;let body=self.body;let d=registry.cached_source.as_ref().unwrap().next_take_demand(body)?;
   retire(cx,d,|effect|{
    let destination=match registry.captured_slot{Some(index)=>&mut registry.slots[index].as_mut().unwrap().view_state,None=>&mut registry.view_state};
    replied(effect,|effect,reply|registry.cached_source.as_mut().unwrap().take_authority_into(destination,body,effect,reply))
   })?;
   let restored=match registry.captured_slot{Some(index)=>registry.slots[index].as_ref().is_some_and(|binding|binding.view_state.is_some()),None=>registry.view_state.is_some()};
   if restored{registry.cached_source.take();}return Ok(false)
  }
  if self.locked{retire_metadata(cx,demand::<(Option<usize>,Option<usize>,bool)>(),||{registry.cached_capture=None;registry.captured_slot=None;self.locked=false})?;return Ok(false)}
  Ok(self.terminal_is_empty())
 }
 pub fn close_demands(&self,registry:&SurfaceContexts)->Result<RetirementDemand,ValueError>{
  self.registry(registry)?;if !self.closing{return Ok(demand::<bool>())}
  if let Some(admission)=self.admission.as_ref(){Self::vacant(registry)?;return if admission.phase()=="ready"{admission.next_demand()}else{admission.next_return_demand()}}
  if self.locked{if let Some(source)=registry.cached_source.as_ref(){Self::vacant(registry)?;return source.next_take_demand(self.body)}}
  if self.locked{return Ok(demand::<(Option<usize>,Option<usize>,bool)>())}Ok(Default::default())
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&!self.locked&&self.admission.is_none()}
 pub fn release_demand()->RetirementDemand{RetirementDemand{release_bytes:size_of::<Self>(),depth:1,..Default::default()}}
}
impl Drop for OriginalSurfaceViewRead{fn drop(&mut self){assert!(self.terminal_is_empty(),"original surface view read requires exact source restoration and paid closure")}}
/// 🎟️ The existing typed clone capture embeds the same stable source pin.
pub struct OriginalSurfaceViewCapture{read:OriginalSurfaceViewRead,cursor:RetainedFieldCursor<ViewModel>,output:ManuallyDrop<Option<ViewModel>>,retirement:RetainedCloneClose,phase:u8,closing:bool}
impl OriginalSurfaceViewCapture{
 pub fn birth_demand()->RetirementDemand{RetirementDemand{capacity_bytes:size_of::<Self>(),..demand::<Self>()}}
 pub fn admit(cx:&mut StepContext<'_>,admission:&mut OriginalConstructionAdmission,failure:&mut Option<OriginalConstructionFailure>,slot:&mut Option<Box<Self>>)->Result<bool,OriginalTurnFailure>{
  let d=Self::birth_demand();let binding=OriginalSourceBinding::borrow(cx);let body=cx.retained_grant().maximum_copy_bytes;
  born(cx,slot,admission,failure,d,|native|{let recipient=native.retirement_recipient_identity().ok_or_else(invalid)?;Ok(Box::new(Self{read:OriginalSurfaceViewRead::new(binding,(native.ledger_identity(),recipient),body),cursor:Default::default(),output:ManuallyDrop::new(None),retirement:Default::default(),phase:0,closing:false}))})
 }
 pub fn advance(&mut self,registry:&mut SurfaceContexts,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.advance_selected(registry,None,cx)}
 pub fn advance_surface(&mut self,registry:&mut SurfaceContexts,surface:&str,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.advance_selected(registry,Some(surface),cx)}
 fn advance_selected(&mut self,registry:&mut SurfaceContexts,surface:Option<&str>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.read.validate(cx)?;self.read.registry(registry)?;if self.closing{return Err(invalid().into())}if cx.is_cancelled(){return Err(ValueError::literal(ValueRefusalKind::Canceled,"original view capture cancelled").into())}
  if self.phase>0&&self.read.selection!=surface.map(|surface|(surface.as_ptr()as usize,surface.len())){return Err(invalid().into())}
  match self.phase{
   0=>{if self.read.ready(){moved(cx,demand::<u8>(),||{self.phase=if registry.cached_source.is_some(){1}else{2}})?;}else{self.read.advance_selected(registry,surface,cx)?;}},
   1=>{
    let source=self.read.borrow(registry,cx)?.ok_or_else(invalid)?;
    if self.cursor.output_ready(){
     let d=self.cursor.take_demand()?;forward(cx,d,|effect|replied(effect,|effect,reply|self.cursor.take(&mut self.output,effect,reply)))?;
     if self.output.is_some(){self.phase=2}
    }else{let d=self.cursor.advance_demands(source,self.read.body,cx.fuel_remaining())?;forward(cx,d,|effect|{let maximum=effect.maximum_work_units();replied(effect,|effect,reply|self.cursor.advance_into(source,self.read.body,maximum,effect,reply))})?;}
   },
   2=>{if self.cursor.terminal_is_empty(){moved(cx,demand::<u8>(),||self.phase=3)?;}else{let body=self.read.body;cx.with_original_retirement(|cx|close_cursor(cx,&mut self.cursor,body))?;}},
   3=>{if self.read.terminal_is_empty(){moved(cx,demand::<u8>(),||self.phase=4)?;}else{self.read.close_step(registry,cx)?;}},
   4|5=>return Ok(true),
   _=>return Err(invalid().into()),
  }Ok(self.phase>=4)
 }
 pub fn take_into(&mut self,output:&mut Option<ViewModel>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.read.validate(cx)?;if self.closing||self.phase!=4||output.is_some(){return Err(invalid().into())}
  moved(cx,demand::<Option<ViewModel>>(),||{*output=self.output.take();self.phase=5})
 }
 pub fn close_step(&mut self,registry:&mut SurfaceContexts,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.read.validate(cx)?;self.read.registry(registry)?;
  if !self.closing{retire_metadata(cx,demand::<bool>(),||self.closing=true)?;return Ok(false)}
  if !self.cursor.terminal_is_empty(){let body=self.read.body;cx.with_original_retirement(|cx|close_cursor(cx,&mut self.cursor,body))?;return Ok(false)}
  if !self.read.terminal_is_empty(){self.read.close_step(registry,cx)?;return Ok(false)}
  if !self.retirement.is_empty(){let body=self.read.body;let d=self.retirement.demands(body)?;retire(cx,d,|effect|self.retirement.step_granted(body,effect))?;return Ok(false)}
  if self.output.is_some(){
   let d=self.retirement.next_owner_demands::<ViewModel>(true,&None,self.read.body)?;
   retire(cx,d,|effect|begin(&mut self.retirement,&mut self.output,effect))?;return Ok(false)
  }Ok(self.terminal_is_empty())
 }
 pub fn close_demands(&self,registry:&SurfaceContexts)->Result<RetirementDemand,ValueError>{
  self.read.registry(registry)?;if !self.closing{return Ok(demand::<bool>())}
  if !self.cursor.terminal_is_empty(){return cursor_close_demands(&self.cursor,self.read.body)}
  if !self.read.terminal_is_empty(){return self.read.close_demands(registry)}
  if !self.retirement.is_empty(){return self.retirement.demands(self.read.body)}
  if self.output.is_some(){return self.retirement.next_owner_demands::<ViewModel>(true,&None,self.read.body)}Ok(Default::default())
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.read.terminal_is_empty()&&self.output.is_none()&&self.cursor.terminal_is_empty()&&self.retirement.is_empty()}
 pub fn release_demand()->RetirementDemand{RetirementDemand{release_bytes:size_of::<Self>(),depth:1,..Default::default()}}
}
impl Drop for OriginalSurfaceViewCapture{fn drop(&mut self){assert!(self.terminal_is_empty(),"original surface view capture requires cache restoration and paid closure")}}
