//! 👁️ Cached view reads pin the actual registry owner; typed captures share the same source lease.
use super::*;
use std::mem::size_of;
use semio_framework_value::{RetainedWorkOutcome,retained_clone::RetainedCloneRef};
fn construction(d:RetirementDemand)->OriginalConstructionDemand{OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:d.capacity_bytes,release_bytes:d.release_bytes,depth:d.depth}}
fn construct<O>(cx:&mut StepContext<'_>,output:&mut Option<O>,d:RetirementDemand,run:impl FnOnce(&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->RetainedWorkOutcome<(O,RetainedCloneProgress)>)->Result<bool,OriginalTurnFailure>{semio_framework_job::admit_original_construction(cx,output,construction(d),run)}
fn event(cx:&mut StepContext<'_>,d:RetirementDemand,run:impl FnOnce(RetainedCloneGrant)->RetainedWorkOutcome)->Result<bool,OriginalTurnFailure>{
 let g=cx.retained_grant();let mut accepted=None;
 let mut quote=construction(d);quote.items=g.maximum_items;semio_framework_job::admit_original_construction(cx,&mut accepted,quote,|_|{let turn=run(g);RetainedWorkOutcome{result:turn.result.map(|step|((),step.progress())),work:turn.work}})
}
fn metadata(cx:&mut StepContext<'_>,d:RetirementDemand,run:impl FnOnce())->Result<bool,OriginalTurnFailure>{event(cx,d,|_|{run();RetainedWorkOutcome{result:Ok(step(d)),work:1}})}
pub(super) fn retire(cx:&mut StepContext<'_>,d:RetirementDemand,run:impl FnOnce(RetainedCloneGrant)->RetainedWorkOutcome)->Result<semio_framework_job::InteractiveJobCloseStep,OriginalTurnFailure>{cx.with_original_retirement(|cx|semio_framework_job::close_original_retained_step(cx,d,run))}
pub(super) fn retire_metadata(cx:&mut StepContext<'_>,d:RetirementDemand,run:impl FnOnce())->Result<bool,OriginalTurnFailure>{
 let mut advanced=false;
 retire(cx,d,|g|{if !retained_clone_grant_admits(g,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}run();advanced=true;RetainedWorkOutcome{result:Ok(step(d)),work:1}})?;Ok(advanced)
}
/// 🧷️ A boxed read or embedded capture pin keeps the exact registry owner immutable across ready frames.
pub struct OriginalSurfaceViewRead{binding:OriginalSourceBinding,decoder:(u64,u64),body:usize,registry:Option<usize>,admission:Option<RetainedCloneSourceAdmission<ViewModel,ViewModel,()>>,phase:u8,closing:bool,locked:bool,selection:Option<(usize,usize)>,scan:usize,offset:usize,selected:bool}
impl OriginalSurfaceViewRead{
 fn new(binding:OriginalSourceBinding,decoder:(u64,u64),body:usize)->Self{Self{binding,decoder,body,registry:None,admission:None,phase:0,closing:false,locked:false,selection:None,scan:0,offset:0,selected:false}}
 pub fn birth_demand()->RetirementDemand{RetirementDemand{capacity_bytes:size_of::<Self>(),..demand::<Self>()}}
 pub fn admit(cx:&mut StepContext<'_>,slot:&mut Option<Box<Self>>)->Result<bool,OriginalTurnFailure>{
  let d=Self::birth_demand();let binding=OriginalSourceBinding::borrow(cx);let body=cx.retained_grant().maximum_copy_bytes;
  construct(cx,slot,d,|control|{let result=control.retirement_recipient_identity().ok_or_else(invalid).map(|recipient|(Box::new(Self::new(binding,(control.ledger_identity(),recipient),body)),step(d).progress()));RetainedWorkOutcome{result,work:1}})
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
    metadata(cx,demand::<Self>(),||{self.registry=Some(registry as*const _ as usize);registry.cached_capture=Some(self as*const _ as usize);registry.captured_slot=None;self.selection=selection;self.selected=surface.is_none();self.locked=true;self.phase=1})?;
   },
   1=>{
    if self.selected{metadata(cx,demand::<u8>(),||self.phase=2)?;return Ok(false)}
    let surface=surface.ok_or_else(invalid)?;
    if self.scan==registry.slots.len(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"selected surface has no original host context").into())}
    let candidate=registry.slots[self.scan].as_ref().map(|binding|binding.surface.as_str());
    let count=candidate.map_or(0,|candidate|if candidate.len()==surface.len(){64.min(surface.len().saturating_sub(self.offset))}else{0});
    let d=RetirementDemand{copy_bytes:count.saturating_mul(2)+size_of::<(usize,usize,bool,Option<usize>)>(),depth:1,..Default::default()};
    metadata(cx,d,||{
     let equal=candidate.is_some_and(|candidate|candidate.len()==surface.len()&&candidate.as_bytes()[self.offset..self.offset+count].iter().zip(&surface.as_bytes()[self.offset..self.offset+count]).fold(true,|equal,(a,b)|equal&(*a==*b)));
     if equal{self.offset+=count;if self.offset==surface.len(){registry.captured_slot=Some(self.scan);self.selected=true}}else{self.scan+=1;self.offset=0}
    })?;
   },
   2=>{
    let present=if let Some(index)=registry.captured_slot{registry.slots[index].as_ref().and_then(|binding|binding.view_state.as_ref()).is_some()}else{registry.view_state.is_some()};
    if !present{metadata(cx,demand::<u8>(),||self.phase=4)?;return Ok(self.ready())}
    metadata(cx,demand::<Option<RetainedCloneSourceAdmission<ViewModel,ViewModel,()>>>(),||{
     let view=if let Some(index)=registry.captured_slot{&mut registry.slots[index].as_mut().unwrap().view_state}else{&mut registry.view_state};
     self.admission=Some(RetainedCloneSource::admission_owned(view.take().unwrap(),()));self.phase=3;
    })?;
   },
   3=>{
    let admission=self.admission.as_ref().ok_or_else(invalid)?;let d=admission.next_demand()?;
    event(cx,d,|g|{
     let admission=self.admission.as_mut().unwrap();
     let turn=if admission.phase()=="ready"{admission.take_output(&mut registry.cached_source,g)}else{admission.advance(g)};
     if admission.terminal_is_empty(){self.admission=None}
     if registry.cached_source.is_some(){self.phase=4}
     turn
    })?;
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
   retire(cx,d,|g|{
    let admission=self.admission.as_mut().unwrap();
    let turn=if published{admission.take_output(&mut registry.cached_source,g)}else{let mut inputs=None;let turn=admission.return_inputs(&mut inputs,g);if let Some((view,()))=inputs{Self::restore(registry,view)}turn};
    if admission.terminal_is_empty(){self.admission=None}turn
   })?;return Ok(false)
  }
  if self.locked&&registry.cached_source.is_some(){
   Self::vacant(registry)?;let d=registry.cached_source.as_ref().unwrap().next_take_demand(self.body)?;
   retire(cx,d,|g|{
    let turn=registry.cached_source.as_mut().unwrap().take_authority(g);
    let result=turn.result.map(|taken|match taken{RetainedCloneSourceTake::Pending(p)=>RetainedCloneStep::Progress(p),RetainedCloneSourceTake::Ready(view,p)=>{Self::restore(registry,view);registry.cached_source.take();RetainedCloneStep::Progress(p)}});
    RetainedWorkOutcome{result,work:turn.work}
   })?;return Ok(false)
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
 pub fn admit(cx:&mut StepContext<'_>,slot:&mut Option<Box<Self>>)->Result<bool,OriginalTurnFailure>{
  let d=Self::birth_demand();let binding=OriginalSourceBinding::borrow(cx);let body=cx.retained_grant().maximum_copy_bytes;
  construct(cx,slot,d,|control|{
   let result=control.retirement_recipient_identity().ok_or_else(invalid).map(|recipient|(Box::new(Self{read:OriginalSurfaceViewRead::new(binding,(control.ledger_identity(),recipient),body),cursor:Default::default(),output:ManuallyDrop::new(None),retirement:Default::default(),phase:0,closing:false}),step(d).progress()));
   RetainedWorkOutcome{result,work:1}
  })
 }
 pub fn advance(&mut self,registry:&mut SurfaceContexts,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.advance_selected(registry,None,cx)}
 pub fn advance_surface(&mut self,registry:&mut SurfaceContexts,surface:&str,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.advance_selected(registry,Some(surface),cx)}
 fn advance_selected(&mut self,registry:&mut SurfaceContexts,surface:Option<&str>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.read.validate(cx)?;self.read.registry(registry)?;if self.closing{return Err(invalid().into())}if cx.is_cancelled(){return Err(ValueError::literal(ValueRefusalKind::Canceled,"original view capture cancelled").into())}
  if self.phase>0&&self.read.selection!=surface.map(|surface|(surface.as_ptr()as usize,surface.len())){return Err(invalid().into())}
  match self.phase{
   0=>{if self.read.ready(){metadata(cx,demand::<u8>(),||{self.phase=if registry.cached_source.is_some(){1}else{self.cursor.begin_close();2}})?;}else{self.read.advance_selected(registry,surface,cx)?;}},
   1=>{
    let source=self.read.borrow(registry,cx)?.ok_or_else(invalid)?;
    if self.cursor.output_ready(){
     let d=self.cursor.take_demand()?;let g=cx.retained_grant();
     if construct(cx,&mut *self.output,d,|_|{let turn=self.cursor.take(g);RetainedWorkOutcome{result:turn.result.and_then(|value|value.ok_or_else(invalid)),work:turn.work}})?{self.cursor.begin_close();self.phase=2}
    }else{let d=self.cursor.advance_demands(source,self.read.body)?;event(cx,d,|g|self.cursor.advance(source,self.read.body,g))?;}
   },
   2=>{if self.cursor.terminal_is_empty(){metadata(cx,demand::<u8>(),||self.phase=3)?;}else{let d=self.cursor.close_demands(self.read.body)?;retire(cx,d,|g|self.cursor.close_step(g))?;}},
   3=>{if self.read.terminal_is_empty(){metadata(cx,demand::<u8>(),||self.phase=4)?;}else{self.read.close_step(registry,cx)?;}},
   4|5=>return Ok(true),
   _=>return Err(invalid().into()),
  }Ok(self.phase>=4)
 }
 pub fn take_into(&mut self,output:&mut Option<ViewModel>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.read.validate(cx)?;if self.closing||self.phase!=4||output.is_some(){return Err(invalid().into())}
  metadata(cx,demand::<Option<ViewModel>>(),||{*output=self.output.take();self.phase=5})
 }
 pub fn close_step(&mut self,registry:&mut SurfaceContexts,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.read.validate(cx)?;self.read.registry(registry)?;
  if !self.closing{retire_metadata(cx,demand::<bool>(),||{self.closing=true;self.cursor.begin_close();})?;return Ok(false)}
  if !self.cursor.terminal_is_empty(){let d=self.cursor.close_demands(self.read.body)?;retire(cx,d,|g|self.cursor.close_step(g))?;return Ok(false)}
  if !self.read.terminal_is_empty(){self.read.close_step(registry,cx)?;return Ok(false)}
  if !self.retirement.is_empty(){let d=self.retirement.demands(self.read.body)?;retire(cx,d,|g|self.retirement.step_granted(g))?;return Ok(false)}
  if self.output.is_some(){
   let d=self.retirement.next_owner_demands::<ViewModel>(true,&None,self.read.body)?;
   retire(cx,d,|g|{let turn=self.retirement.begin_granted(&mut self.output,g);RetainedWorkOutcome{result:turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Complete(Default::default()))),work:turn.work}})?;return Ok(false)
  }Ok(self.terminal_is_empty())
 }
 pub fn close_demands(&self,registry:&SurfaceContexts)->Result<RetirementDemand,ValueError>{
  self.read.registry(registry)?;if !self.closing{return Ok(demand::<bool>())}
  if !self.cursor.terminal_is_empty(){return self.cursor.close_demands(self.read.body)}
  if !self.read.terminal_is_empty(){return self.read.close_demands(registry)}
  if !self.retirement.is_empty(){return self.retirement.demands(self.read.body)}
  if self.output.is_some(){return self.retirement.next_owner_demands::<ViewModel>(true,&None,self.read.body)}Ok(Default::default())
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.read.terminal_is_empty()&&self.output.is_none()&&self.cursor.terminal_is_empty()&&self.retirement.is_empty()}
 pub fn release_demand()->RetirementDemand{RetirementDemand{release_bytes:size_of::<Self>(),depth:1,..Default::default()}}
}
impl Drop for OriginalSurfaceViewCapture{fn drop(&mut self){assert!(self.terminal_is_empty(),"original surface view capture requires cache restoration and paid closure")}}
