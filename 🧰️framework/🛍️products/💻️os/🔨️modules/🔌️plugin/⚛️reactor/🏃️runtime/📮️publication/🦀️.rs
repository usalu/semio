//! 📮️ Reactor publication retains eight original page loans through funded receiving and cancellation.
use super::*;
use semio_framework_actor::typed_operation_page::{ResultPageDelivery,ResultPageWireReceiver};
use semio_framework_value::{ValueError,ValueRefusalKind};
use semio_framework_value::retirement::controlled::ControlledRetirement;
use std::mem::{size_of,ManuallyDrop};

struct PageSlot{page:Option<TypedOperationResultPage>,delivery:Option<ResultPageDelivery>,receiver:Option<ResultPageWireReceiver>,publication:ManuallyDrop<Option<Effect>>,refused:Option<ControlledRetirement<(String,Vec<u8>)>>}
impl PageSlot{
 fn empty(&self)->bool{self.page.is_none()&&self.delivery.is_none()&&self.receiver.is_none()&&self.publication.is_none()&&self.refused.is_none()}
 fn transfer_bytes()->usize{size_of::<Option<ResultPageDelivery>>()+size_of::<Option<ResultPageWireReceiver>>()}
 fn advance(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
  if let Some(receiver)=self.receiver.as_mut(){return receiver.advance(grant).map(RetainedCloneStep::progress)}
  if let Some(delivery)=self.delivery.as_mut(){if !delivery.ready(){return delivery.advance(grant).map(RetainedCloneStep::progress)}}
  let parent=Self::transfer_bytes();if grant.maximum_items==0||grant.maximum_copy_bytes<parent||grant.maximum_depth==0{return Ok(Default::default())}
  let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-parent,..grant};
  let progress=if self.delivery.is_some(){match ResultPageWireReceiver::capture(&mut self.delivery,child)?{Some((receiver,p))=>{self.receiver=Some(receiver);p},None=>return Ok(Default::default())}}else if self.page.is_some(){match ResultPageDelivery::capture(&mut self.page,child)?{Some((delivery,p))=>{self.delivery=Some(delivery);p},None=>return Ok(Default::default())}}else{return Ok(Default::default())};
  let progress=RetainedCloneProgress{copied_items:progress.copied_items.max(1),copied_bytes:progress.copied_bytes+parent,..progress};semio_framework_value::retained_clone::admit_retained_clone_progress(grant,progress,"original Reactor page handoff")
 }
 fn ready(&self)->bool{self.receiver.as_ref().is_some_and(ResultPageWireReceiver::ready)}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{let mut d=self.receiver.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original Reactor receiving cursor is absent"))?.take_demand()?;d.copy_bytes+=size_of::<Option<ResultPageWireReceiver>>();Ok(d)}
 fn take(&mut self,grant:RetainedCloneGrant)->Result<Option<(Vec<u8>,RetainedCloneProgress)>,ValueError>{let d=self.take_demand()?;if !fits(d,grant){return Ok(None)}let parent=size_of::<Option<ResultPageWireReceiver>>();let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-parent,..grant};let Some((wire,mut p))=self.receiver.as_mut().unwrap().take_wire(child)?else{return Ok(None)};self.receiver=None;p.copied_bytes+=parent;Ok(Some((wire,p)))}
 fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
  let parent=Self::transfer_bytes()+size_of::<Option<Effect>>();if grant.maximum_items==0||grant.maximum_copy_bytes<parent||grant.maximum_depth==0{return Ok(Default::default())}let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-parent,..grant};
  if self.publication.is_some(){let Effect::SendMessage{target:MessageEndpoint::Shell{instance},payload}=self.publication.take().unwrap()else{unreachable!("original receiving publishes only a typed Shell destination")};self.refused=Some(ControlledRetirement::new((instance.0,payload)).unwrap_or_else(|_|unreachable!("original native destination fields declare controlled retirement")));return Ok(RetainedCloneProgress{copied_items:1,copied_bytes:parent,..Default::default()})}
  if let Some(owner)=self.refused.as_mut(){let p=owner.step(child)?.progress();if owner.terminal_is_empty(){self.refused=None;return Ok(RetainedCloneProgress{copied_items:p.copied_items.max(1),copied_bytes:p.copied_bytes+parent,..p})}return Ok(p)}
  let(p,cleared)=if let Some(owner)=self.receiver.as_mut(){owner.begin_close();let p=owner.close_step(child)?.progress();let empty=owner.terminal_is_empty();if empty{self.receiver=None;}(p,empty)}else if let Some(owner)=self.delivery.as_mut(){owner.begin_close();let p=owner.close_step(child)?.progress();let empty=owner.terminal_is_empty();if empty{self.delivery=None;}(p,empty)}else if let Some(owner)=self.page.as_mut(){let p=owner.close_step(child)?.progress();let empty=owner.terminal_is_empty();if empty{self.page=None;}(p,empty)}else{return Ok(Default::default())};
  Ok(if cleared{RetainedCloneProgress{copied_items:p.copied_items.max(1),copied_bytes:p.copied_bytes+parent,..p}}else{p})
 }
}
impl Drop for PageSlot{fn drop(&mut self){assert!(std::thread::panicking()||self.empty(),"original Reactor publication reached Drop before receiving and refused output closed");if self.empty(){unsafe{ManuallyDrop::drop(&mut self.publication);}}}}
fn fits(d:RetirementDemand,g:RetainedCloneGrant)->bool{g.maximum_items>0&&g.maximum_copy_bytes>=d.copy_bytes&&g.maximum_capacity_bytes>=d.capacity_bytes&&g.maximum_release_bytes>=d.release_bytes&&g.maximum_depth>=d.depth}
pub(super) struct NativeResultPages{slots:[PageSlot;TYPED_OPERATION_PAGES_PER_TURN_MAXIMUM]}
impl NativeResultPages{
 pub(super) fn new()->Self{Self{slots:[const{PageSlot{page:None,delivery:None,receiver:None,publication:ManuallyDrop::new(None),refused:None}};TYPED_OPERATION_PAGES_PER_TURN_MAXIMUM]}}
 pub(super) fn terminal_is_empty(&self)->bool{self.slots.iter().all(PageSlot::empty)}
 pub(super) fn capture<PA:PluginApp>(&mut self,app:&mut PA,instance:u32,cx:&mut semio_framework_job::StepContext<'_>)->Result<usize,Fault>{
  let mut admitted=0;
  for slot in self.slots.iter_mut().filter(|slot|slot.empty()){
   for _ in 0..2{
    let parent=size_of::<Option<TypedOperationResultPage>>();let grant=cx.retained_grant();let mut demand=app.typed_operation_result_page_demands(instance)?;if demand==RetirementDemand::default(){return Ok(admitted)}demand.copy_bytes=demand.copy_bytes.checked_add(parent).ok_or_else(||plugin_internal_fault("original page handoff quote overflows"))?;if !fits(demand,grant){return Ok(admitted)}let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-parent,..grant};
    let(page,mut progress)=app.take_typed_operation_result_page(instance,child)?;
    if let Some(page)=page{slot.page=Some(page);progress.copied_bytes+=parent;progress.copied_items=progress.copied_items.max(1);admitted+=1;cx.consume_retained(progress).map_err(|e|plugin_internal_fault(e.into_message()))?;break}
    cx.consume_retained(progress).map_err(|e|plugin_internal_fault(e.into_message()))?;if progress==RetainedCloneProgress::default(){return Ok(admitted)}
   }
  }
  Ok(admitted)
 }
 pub(super) fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{match self.slots.iter_mut().find(|slot|!slot.empty()){Some(slot)=>slot.close_step(grant),None=>Ok(Default::default())}}
}

/// 🎛️ Each destination action consumes the same caller receipt before any renderer effect becomes visible.
pub(crate) fn drive<PA:PluginApp>(runtime:&PluginRuntime<PA>,effects:&mut Vec<Effect>,maximum_effects:usize,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,Fault>{
 let instances=runtime.instances.try_borrow().map_err(|_|plugin_internal_fault("runtime instance authority is busy"))?;let mut cursor=runtime.typed_continuation_cursor.get();let mut first=None;let mut pending=false;
 while let Some((index,id,cell))=instances.next_entry_from(cursor){
  if first==Some(index){break}first.get_or_insert(index);cursor=(index+1)%PLUGIN_RUNTIME_INSTANCE_SLOTS;
  if runtime.guest_lifetimes.try_borrow().map_err(|_|plugin_internal_fault("lifecycle authority is busy"))?.get(id).is_some_and(|slot|!slot.cell.is_live()){continue}
  let mut pages=match cell.result_pages.try_lock(){Ok(pages)=>pages,Err(std::sync::TryLockError::WouldBlock)=>{pending=true;continue},Err(std::sync::TryLockError::Poisoned(_))=>return Err(plugin_internal_fault("original receiving authority is poisoned"))};
  for slot in &mut pages.slots{
   for _ in 0..TYPED_OPERATION_UNITS_PER_TURN{
    if slot.empty(){break}if slot.publication.is_some()||slot.refused.is_some()||cx.should_yield(){pending=true;break}
    let result=if slot.ready(){publish(slot,id,effects,maximum_effects,cx)}else{let grant=cx.retained_grant();slot.advance(grant).map(|p|(false,p))};
    match result{Ok((published,p))=>{cx.consume_retained(p).map_err(|e|plugin_internal_fault(e.into_message()))?;if published{effects.push(slot.publication.take().expect("accepted original publication remains in its receiving owner"))}if p==RetainedCloneProgress::default(){pending=true;break}},Err(error)=>{cx.consume_retained(error.retained_progress()).map_err(|e|plugin_internal_fault(e.into_message()))?;return Err(plugin_internal_fault(error.into_message()))}}
   }
  }
  pending|=!pages.terminal_is_empty();
 }
 Ok(pending)
}
fn publish(slot:&mut PageSlot,instance:u32,effects:&mut Vec<Effect>,maximum:usize,cx:&semio_framework_job::StepContext<'_>)->Result<(bool,RetainedCloneProgress),ValueError>{
 if effects.len()>=maximum{return Ok((false,Default::default()))}let grant=cx.retained_grant();let mut digits=[0u8;10];let mut n=instance;let mut start=digits.len();loop{start-=1;digits[start]=b'0'+(n%10)as u8;n/=10;if n==0{break}}
 let name=&digits[start..];let capacity=if effects.len()==effects.capacity(){effects.len().checked_add(1).and_then(|len|len.checked_mul(size_of::<Effect>())).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"renderer effect capacity quote overflows"))?}else{0};let relocation=if capacity>0{effects.len()*size_of::<Effect>()}else{0};let parent=relocation+name.len()+size_of::<Effect>()+size_of::<Option<Effect>>()*2;let mut d=slot.take_demand()?;d.copy_bytes=d.copy_bytes.checked_add(parent).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"renderer effect copy quote overflows"))?;d.capacity_bytes=d.capacity_bytes.checked_add(capacity+name.len()).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"renderer effect capacity quote overflows"))?;if !fits(d,grant){return Ok((false,Default::default()))}
 let old_capacity=if capacity>0{effects.capacity()*size_of::<Effect>()}else{0};if grant.maximum_release_bytes<old_capacity{return Ok((false,Default::default()))}let mut retained=0;let mut released=0;if capacity>0{effects.try_reserve_exact(1).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"renderer effect destination allocation failed"))?;retained+=effects.capacity()*size_of::<Effect>();released+=old_capacity;}
 let mut target=String::new();if target.try_reserve_exact(name.len()).is_err(){return Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"renderer target allocation failed").with_retained_progress(RetainedCloneProgress{copied_items:1,copied_bytes:relocation,retained_capacity_bytes:retained,released_bytes:released}))}target.push_str(std::str::from_utf8(name).unwrap());retained+=target.capacity();
 let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-parent,maximum_capacity_bytes:grant.maximum_capacity_bytes-retained,maximum_release_bytes:grant.maximum_release_bytes-released,..grant};let Some((payload,mut p))=slot.take(child)?else{unreachable!("original receiving transfer was preflighted")};p.copied_items=p.copied_items.max(1);p.copied_bytes+=parent;p.retained_capacity_bytes+=retained;p.released_bytes+=released;*slot.publication=Some(Effect::SendMessage{target:MessageEndpoint::Shell{instance:semio_framework::kernel::PluginInstanceId(target)},payload});Ok((true,p))
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
