//! 👁️ Cached view reads pin the actual registry owner; typed captures share the same source lease.
use super::*;
use std::mem::size_of;
use semio_framework_value::retained_clone::{RetainedCloneRef,RetainedClone};
/// 🧱️ A resident construction admission and unit marker fund every one-unit state change of its owner.
#[derive(Default)]
pub(super) struct Unit{pub(super) admission:OriginalConstructionAdmission,marker:Option<()>}
impl Unit{
 pub(super) fn terminal_is_empty(&self)->bool{self.admission.terminal_is_empty()&&self.marker.is_none()}
 pub(super) fn step(&mut self,cx:&mut StepContext<'_>,d:RetirementDemand,run:impl FnOnce())->Result<bool,OriginalTurnFailure>{unit(cx,&mut self.admission,&mut self.marker,d,||{run();Ok(())})}
}
/// 🪡️ One funded cursor advance over the borrowed original source.
pub(super) fn advance_cursor<T:RetainedClone>(cx:&mut StepContext<'_>,cursor:&mut RetainedCloneOwner<T>,source:RetainedCloneRef<'_,T>,body:usize)->Result<(),OriginalTurnFailure>{
 let maximum=cx.fuel_remaining();let d=cursor.advance_demands(source,body,maximum)?;
 funded(cx,d,|effect|replied::<OriginalRetirementFailure>(effect,|effect,reply|cursor.advance_into(source,body,maximum,effect,reply).map(drop)))?;Ok(())
}
/// 📤️ One funded cursor take of its finished output.
pub(super) fn take_cursor<T:RetainedClone>(cx:&mut StepContext<'_>,cursor:&mut RetainedCloneOwner<T>,output:&mut Option<T>)->Result<(),OriginalTurnFailure>{
 let d=cursor.take_demand()?;funded(cx,d,|effect|replied::<ValueError>(effect,|effect,reply|cursor.take(output,effect,reply).map(drop)))?;Ok(())
}
/// 🪦️ One funded cursor close step.
pub(super) fn close_cursor<T:RetainedClone>(cx:&mut StepContext<'_>,cursor:&mut RetainedCloneOwner<T>,body:usize)->Result<(),OriginalTurnFailure>{
 let d=cursor.selected_close_demand(body)?;
 retired_poll(cx,d,|effect|replied::<OriginalRetirementFailure>(effect,|effect,reply|cursor.close_into(body,effect,reply).map(drop)))?;Ok(())
}
pub(super) fn retire_metadata(cx:&mut StepContext<'_>,d:RetirementDemand,run:impl FnOnce())->Result<bool,OriginalTurnFailure>{
 let mut advanced=false;
 retired(cx,d,|_|{run();advanced=true;Ok(step(d))})?;Ok(advanced)
}
/// 🧷️ A boxed read or embedded capture pin keeps the exact registry owner immutable across ready frames.
pub struct OriginalSurfaceViewRead{binding:OriginalSourceBinding,decoder:(NativeAllocationIdentity,u64),body:usize,registry:Option<usize>,admission:SourceAdmission<ViewModel,ViewModel,()>,unit:Unit,phase:u8,closing:bool,locked:bool,selection:Option<(usize,usize)>,scan:usize,offset:usize,selected:bool}
impl OriginalSurfaceViewRead{
 fn new(binding:OriginalSourceBinding,decoder:(NativeAllocationIdentity,u64),body:usize)->Self{Self{binding,decoder,body,registry:None,admission:Default::default(),unit:Default::default(),phase:0,closing:false,locked:false,selection:None,scan:0,offset:0,selected:false}}
 pub fn birth_demand()->RetirementDemand{RetirementDemand{capacity_bytes:size_of::<Self>(),..demand::<Self>()}}
 pub fn admit(cx:&mut StepContext<'_>,admission:&mut OriginalConstructionAdmission,slot:&mut Option<Box<Self>>)->Result<bool,OriginalTurnFailure>{
  let d=Self::birth_demand();let binding=OriginalSourceBinding::borrow(cx);let body=cx.retained_grant().maximum_copy_bytes;
  let settled=built_with(cx,admission,slot,construction(d),|control|{let recipient=control.retirement_recipient_identity().ok_or_else(invalid)?;Ok((Box::new(Self::new(binding,(control.ledger_identity(),recipient),body)),step(d).progress()))})?;
  Ok(adopted(settled,admission,slot,|owner|&mut owner.unit.admission))
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
    let me=self as*const Self as usize;
    self.unit.step(cx,demand::<Self>(),||{self.registry=Some(registry as*const _ as usize);registry.cached_capture=Some(me);registry.captured_slot=None;self.selection=selection;self.selected=surface.is_none();self.locked=true;self.phase=1})?;
   },
   1=>{
    if self.selected{self.unit.step(cx,demand::<u8>(),||self.phase=2)?;return Ok(false)}
    let surface=surface.ok_or_else(invalid)?;
    if self.scan==registry.slots.len(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"selected surface has no original host context").into())}
    let candidate=registry.slots[self.scan].as_ref().map(|binding|binding.surface.as_str());
    let count=candidate.map_or(0,|candidate|if candidate.len()==surface.len(){64.min(surface.len().saturating_sub(self.offset))}else{0});
    let d=RetirementDemand{copy_bytes:count.saturating_mul(2)+size_of::<(usize,usize,bool,Option<usize>)>(),depth:1,..Default::default()};
    self.unit.step(cx,d,||{
     let equal=candidate.is_some_and(|candidate|candidate.len()==surface.len()&&candidate.as_bytes()[self.offset..self.offset+count].iter().zip(&surface.as_bytes()[self.offset..self.offset+count]).fold(true,|equal,(a,b)|equal&(*a==*b)));
     if equal{self.offset+=count;if self.offset==surface.len(){registry.captured_slot=Some(self.scan);self.selected=true}}else{self.scan+=1;self.offset=0}
    })?;
   },
   2=>{
    let present=if let Some(index)=registry.captured_slot{registry.slots[index].as_ref().and_then(|binding|binding.view_state.as_ref()).is_some()}else{registry.view_state.is_some()};
    if !present{self.unit.step(cx,demand::<u8>(),||self.phase=4)?;return Ok(self.ready())}
    self.unit.step(cx,demand::<SourceAdmission<ViewModel,ViewModel,()>>(),||{
     let view=if let Some(index)=registry.captured_slot{&mut registry.slots[index].as_mut().unwrap().view_state}else{&mut registry.view_state};
     self.admission.set(RetainedCloneSource::admission_owned(view.take().unwrap(),()));self.phase=3;
    })?;
   },
   3=>{
    self.admission.drive(cx,&mut registry.cached_source)?;
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
  if !self.admission.terminal_is_empty(){
   Self::vacant(registry)?;let mut inputs=None;self.admission.close(cx,&mut registry.cached_source,&mut inputs)?;
   if let Some((view,()))=inputs{Self::restore(registry,view)}return Ok(false)
  }
  if self.locked&&registry.cached_source.is_some(){
   let vacant=Self::vacant(registry).is_ok();let body=self.body;let source=registry.cached_source.as_mut().unwrap();
   let mut taken=None;
   if vacant{let d=source.next_take_demand(body)?;retired_poll(cx,d,|effect|replied::<OriginalRetirementFailure>(effect,|effect,reply|source.take_authority_into(&mut taken,body,effect,reply).map(drop)))?;}
   else{let d=source.next_close_demand(body)?;retired_poll(cx,d,|effect|source.close_step(body,effect))?;}
   if let Some(view)=taken{Self::restore(registry,view)}
   if registry.cached_source.as_ref().is_some_and(RetainedCloneSource::terminal_is_empty){registry.cached_source.take();}
   return Ok(false)
  }
  if self.locked{retire_metadata(cx,demand::<(Option<usize>,Option<usize>,bool)>(),||{registry.cached_capture=None;registry.captured_slot=None;self.locked=false})?;return Ok(false)}
  if !unbuilt(cx,&mut self.unit.admission)?{return Ok(false)}
  Ok(self.terminal_is_empty())
 }
 pub fn close_demands(&self,registry:&SurfaceContexts)->Result<RetirementDemand,ValueError>{
  self.registry(registry)?;if !self.closing{return Ok(demand::<bool>())}
  if !self.admission.terminal_is_empty(){Self::vacant(registry)?;return self.admission.close_quote()}
  if self.locked{if let Some(source)=registry.cached_source.as_ref(){return if Self::vacant(registry).is_ok(){source.next_take_demand(self.body)}else{source.next_close_demand(self.body)}}}
  if self.locked{return Ok(demand::<(Option<usize>,Option<usize>,bool)>())}Ok(Default::default())
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&!self.locked&&self.admission.terminal_is_empty()&&self.unit.terminal_is_empty()}
 pub fn release_demand()->RetirementDemand{RetirementDemand{release_bytes:size_of::<Self>(),depth:1,..Default::default()}}
}
impl Drop for OriginalSurfaceViewRead{fn drop(&mut self){assert!(self.terminal_is_empty(),"original surface view read requires exact source restoration and paid closure")}}
/// 🎟️ The existing typed clone capture embeds the same stable source pin.
pub struct OriginalSurfaceViewCapture{read:OriginalSurfaceViewRead,cursor:RetainedCloneOwner<ViewModel>,output:ManuallyDrop<Option<ViewModel>>,retirement:RetainedCloneClose,unit:Unit,phase:u8,closing:bool}
impl OriginalSurfaceViewCapture{
 pub fn birth_demand()->RetirementDemand{RetirementDemand{capacity_bytes:size_of::<Self>(),..demand::<Self>()}}
 pub fn admit(cx:&mut StepContext<'_>,admission:&mut OriginalConstructionAdmission,slot:&mut Option<Box<Self>>)->Result<bool,OriginalTurnFailure>{
  let d=Self::birth_demand();let binding=OriginalSourceBinding::borrow(cx);let body=cx.retained_grant().maximum_copy_bytes;
  let settled=built_with(cx,admission,slot,construction(d),|control|{
   let recipient=control.retirement_recipient_identity().ok_or_else(invalid)?;
   Ok((Box::new(Self{read:OriginalSurfaceViewRead::new(binding,(control.ledger_identity(),recipient),body),cursor:Default::default(),output:ManuallyDrop::new(None),retirement:Default::default(),unit:Default::default(),phase:0,closing:false}),step(d).progress()))
  })?;
  Ok(adopted(settled,admission,slot,|owner|&mut owner.unit.admission))
 }
 pub fn advance(&mut self,registry:&mut SurfaceContexts,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.advance_selected(registry,None,cx)}
 pub fn advance_surface(&mut self,registry:&mut SurfaceContexts,surface:&str,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.advance_selected(registry,Some(surface),cx)}
 fn advance_selected(&mut self,registry:&mut SurfaceContexts,surface:Option<&str>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.read.validate(cx)?;self.read.registry(registry)?;if self.closing{return Err(invalid().into())}if cx.is_cancelled(){return Err(ValueError::literal(ValueRefusalKind::Canceled,"original view capture cancelled").into())}
  if self.phase>0&&self.read.selection!=surface.map(|surface|(surface.as_ptr()as usize,surface.len())){return Err(invalid().into())}
  match self.phase{
   0=>{if self.read.ready(){self.unit.step(cx,demand::<u8>(),||{self.phase=if registry.cached_source.is_some(){1}else{2}})?;}else{self.read.advance_selected(registry,surface,cx)?;}},
   1=>{
    let source=self.read.borrow(registry,cx)?.ok_or_else(invalid)?;
    if self.cursor.output_ready(){take_cursor(cx,&mut self.cursor,&mut *self.output)?;if self.output.is_some(){self.phase=2}}
    else{advance_cursor(cx,&mut self.cursor,source,self.read.body)?}
   },
   2=>{if self.cursor.terminal_is_empty(){self.unit.step(cx,demand::<u8>(),||self.phase=3)?;}else{close_cursor(cx,&mut self.cursor,self.read.body)?}},
   3=>{if self.read.terminal_is_empty(){self.unit.step(cx,demand::<u8>(),||self.phase=4)?;}else{self.read.close_step(registry,cx)?;}},
   4|5=>return Ok(true),
   _=>return Err(invalid().into()),
  }Ok(self.phase>=4)
 }
 pub fn take_into(&mut self,output:&mut Option<ViewModel>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.read.validate(cx)?;if self.closing||self.phase!=4||output.is_some(){return Err(invalid().into())}
  self.unit.step(cx,demand::<Option<ViewModel>>(),||{*output=self.output.take();self.phase=5})
 }
 pub fn close_step(&mut self,registry:&mut SurfaceContexts,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.read.validate(cx)?;self.read.registry(registry)?;
  if !self.closing{retire_metadata(cx,demand::<bool>(),||self.closing=true)?;return Ok(false)}
  if !self.cursor.terminal_is_empty(){close_cursor(cx,&mut self.cursor,self.read.body)?;return Ok(false)}
  if !self.read.terminal_is_empty(){self.read.close_step(registry,cx)?;return Ok(false)}
  if !self.retirement.is_empty(){let d=self.retirement.demands(self.read.body)?;let body=self.read.body;retired_poll(cx,d,|effect|self.retirement.step_granted(body,effect))?;return Ok(false)}
  if self.output.is_some(){
   let d=self.retirement.next_owner_demands::<ViewModel>(true,&None,self.read.body)?;
   begun(cx,d,|effect|self.retirement.begin_granted(&mut self.output,effect))?;return Ok(false)
  }
  if !unbuilt(cx,&mut self.unit.admission)?{return Ok(false)}
  Ok(self.terminal_is_empty())
 }
 pub fn close_demands(&self,registry:&SurfaceContexts)->Result<RetirementDemand,ValueError>{
  self.read.registry(registry)?;if !self.closing{return Ok(demand::<bool>())}
  if !self.cursor.terminal_is_empty(){return self.cursor.selected_close_demand(self.read.body)}
  if !self.read.terminal_is_empty(){return self.read.close_demands(registry)}
  if !self.retirement.is_empty(){return self.retirement.demands(self.read.body)}
  if self.output.is_some(){return self.retirement.next_owner_demands::<ViewModel>(true,&None,self.read.body)}Ok(Default::default())
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.read.terminal_is_empty()&&self.output.is_none()&&self.cursor.terminal_is_empty()&&self.retirement.is_empty()&&self.unit.terminal_is_empty()}
 pub fn release_demand()->RetirementDemand{RetirementDemand{release_bytes:size_of::<Self>(),depth:1,..Default::default()}}
}
impl Drop for OriginalSurfaceViewCapture{fn drop(&mut self){assert!(self.terminal_is_empty(),"original surface view capture requires cache restoration and paid closure")}}
