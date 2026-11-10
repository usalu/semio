//! 📮️ Pre-reserved page returns keep the genuine supplier source live without copying payload backing.
use super::*;
use std::cell::UnsafeCell;
use semio_framework_job_payload::RetainedJobPageTransfer;
use semio_framework_job_context::OriginalCallerReturnPlace;

#[path="📦️bytes/🦀️.rs"]
mod original_bytes;
pub use original_bytes::{OriginalContiguousJobBytes,OriginalJobBytesReturnRegistry,OriginalJobBytesReservation,OriginalSuppliedJobBytes};

const VACANT:u8=0;
const CHECKED:u8=1;
const RESERVED:u8=2;
const ISSUED:u8=3;
const RETURNED:u8=4;
const CLAIMING:u8=5;

struct ReturnOwners{source:Option<OriginalCallerSourceLoan>,returned:Option<RetainedJobPageTransfer>,pending:Option<RetainedJobPageTransfer>,binding:Option<OriginalSourceBinding>,credit:OriginalCallerReturnPlace}
trait PageReturnTarget:Sync{fn identity(&self,index:usize)->usize;fn return_page(&self,index:usize,generation:u64,page:RetainedJobPageTransfer);}
struct ReturnSlot{state:AtomicU8,generation:AtomicU64,identity:AtomicUsize,owners:UnsafeCell<ReturnOwners>}
/// 🔒️ Acquire checkout exclusively borrows owners; Release publishes their exact return before another checkout.
unsafe impl Sync for ReturnSlot{}
impl ReturnSlot{
 const fn vacant()->Self{Self{state:AtomicU8::new(VACANT),generation:AtomicU64::new(0),identity:AtomicUsize::new(0),owners:UnsafeCell::new(ReturnOwners{source:None,returned:None,pending:None,binding:None,credit:OriginalCallerReturnPlace::empty()})}}
 fn checkout(&self,state:u8)->Option<ReturnCheckout<'_>>{self.state.compare_exchange(state,CHECKED,Ordering::Acquire,Ordering::Relaxed).ok().map(|_|ReturnCheckout{slot:self,restore:state})}
}
struct ReturnCheckout<'a>{slot:&'a ReturnSlot,restore:u8}
impl ReturnCheckout<'_>{fn owners(&mut self)->&mut ReturnOwners{unsafe{&mut*self.slot.owners.get()}}}
impl Drop for ReturnCheckout<'_>{fn drop(&mut self){self.slot.state.store(self.restore,Ordering::Release)}}

fn refusal(reason:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::UnsupportedOwner,reason)}
fn registry_control_ready(cx:&StepContext<'_>)->bool{let grant=cx.retained_grant();!cx.should_yield()&&!cx.is_cancelled()&&grant.maximum_items>0&&grant.maximum_depth>0}
fn registry_control(cx:&mut StepContext<'_>,handoff:bool)->Result<(),OriginalTurnFailure>{
 #[cfg(test)]if handoff{admission_work_tests::narrow_handoff(cx);}
 #[cfg(not(test))]let _=handoff;
 receive_original_retained_work(cx,semio_framework_value_retained_clone::RetainedWorkOutcome::<_,ValueError>{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}).map(|_|())
}

/// 🗄️ The program owns a fixed inline return table before issuing any payload or source loan.
pub struct OriginalJobPageReturnRegistry<const N:usize>{slots:[ReturnSlot;N],return_wake:Option<fn()>}
impl<const N:usize> PageReturnTarget for OriginalJobPageReturnRegistry<N>{
 fn identity(&self,index:usize)->usize{self.slots[index].identity.load(Ordering::Acquire)}
 fn return_page(&self,index:usize,generation:u64,page:RetainedJobPageTransfer){let slot=&self.slots[index];let mut checked=slot.checkout(ISSUED).expect("supplied page retains its exclusive reserved return");assert_eq!(slot.generation.load(Ordering::Relaxed),generation);checked.owners().returned=Some(page);checked.owners().credit.record_return().expect("original page retains its one-shot held source return");checked.restore=RETURNED;drop(checked);if let Some(wake)=self.return_wake{wake();}}
}
impl<const N:usize> OriginalJobPageReturnRegistry<N>{
 pub const fn new()->Self{Self{slots:[const{ReturnSlot::vacant()};N],return_wake:None}}
 /// 🔔️ A preowned function observes a published return after exclusive slot checkout ends, without owning another closure.
 pub const fn with_return_wake(wake:fn())->Self{Self{slots:[const{ReturnSlot::vacant()};N],return_wake:Some(wake)}}
 /// 🎟️ A successful reservation retains one genuine source loan before any page leaves its supplier.
 pub fn reserve(&'static self,output:&mut Option<OriginalJobPageReservation<N>>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  fn require_send<T:Send>(){}require_send::<ReturnOwners>();
  let parent=cx.original_source_parent()?;let identity=parent.source_identity()?;parent.validate_borrowed_context(cx)?;
  if let Some(owner)=output.as_mut(){
   if !std::ptr::eq(owner.registry,self)||!owner.active{return Err(refusal("original page reservation output belongs to another registry").into())}
   let slot=&self.slots[owner.index];if slot.identity.load(Ordering::Acquire)!=identity||slot.generation.load(Ordering::Acquire)!=owner.generation{return Err(refusal("original page reservation output changed source or generation").into())}
   let Some(mut checked)=slot.checkout(if owner.ready{RESERVED}else{CLAIMING})else{return Ok(false)};
   if !checked.owners().binding.is_some_and(|binding|binding.matches(cx)){return Err(refusal("original page reservation changed its bound caller context").into())}
   if let Some(source)=checked.owners().source.as_ref(){source.validate_context(cx)?;}
   if owner.ready{return Ok(true)}if !registry_control_ready(cx){return Ok(false)}
   if checked.owners().source.is_none(){parent.admit_loan(&mut checked.owners().source,cx)?;return Ok(false)}
   owner.ready=true;checked.restore=RESERVED;registry_control(cx,true)?;return Ok(true)
  }
  if !registry_control_ready(cx){return Ok(false)}
  for(index,slot)in self.slots.iter().enumerate(){
   let Some(mut checked)=slot.checkout(VACANT)else{continue};
   let Some(generation)=slot.generation.load(Ordering::Relaxed).checked_add(1)else{continue};
   let admission=checked.owners().credit.admit(cx);
   if checked.owners().credit.is_admitted(){checked.owners().binding=Some(OriginalSourceBinding::borrow(cx));slot.identity.store(identity,Ordering::Relaxed);slot.generation.store(generation,Ordering::Relaxed);checked.restore=CLAIMING;*output=Some(OriginalJobPageReservation{registry:self,index,generation,active:true,ready:false});}
   admission?;return Ok(false)
  }Ok(false)
 }
 /// 🔗️ The original source cannot finish while a reserved, supplied or returned page still retains it.
 pub fn retains_source(&self,identity:usize)->bool{self.slots.iter().any(|slot|matches!(slot.state.load(Ordering::Acquire),CHECKED|CLAIMING|RESERVED|ISSUED|RETURNED)&&slot.identity.load(Ordering::Acquire)==identity)}
 /// 📣️ Observes only already-published returns; callback notification never exposes an exclusively checked-out slot.
 pub fn has_returned_source(&self,identity:usize)->bool{self.slots.iter().any(|slot|slot.state.load(Ordering::Acquire)==RETURNED&&slot.identity.load(Ordering::Acquire)==identity)}
 /// ♻️ The exact original caller pays page backing, ledger, loan return and final slot removal separately.
 pub fn close_returned_step(&self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
  let parent=cx.original_source_parent()?;parent.validate_borrowed_context(cx)?;let identity=parent.source_identity()?;
  for slot in &self.slots{
   if slot.identity.load(Ordering::Acquire)!=identity{continue}
   let Some(mut checked)=slot.checkout(RETURNED)else{continue};let owners=checked.owners();
   if !owners.binding.is_some_and(|binding|binding.matches(cx)){return Err(refusal("original page return changed its bound caller context").into())}
   if let Some(source)=owners.source.as_ref(){if !source.terminal_is_empty(){source.validate_retirement_context(cx)?;}}
   if owners.credit.is_admitted(){let before=cx.retained_progress();owners.credit.settle_return(cx)?;let after=cx.retained_progress();return Ok(InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:after.copied_items-before.copied_items,copied_bytes:after.copied_bytes-before.copied_bytes,retained_capacity_bytes:after.retained_capacity_bytes-before.retained_capacity_bytes,released_bytes:after.released_bytes-before.released_bytes}})}

   let pending=owners.pending.is_some();
   if let Some(owner)=if pending{owners.pending.as_mut()}else{owners.returned.as_mut()}{
    if !owner.terminal_is_empty(){let mut demand=owner.retirement_demands();demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"returned original page parent depth overflow"))?;return close_original_retained_effect_step(cx,demand,|effect|{let g=effect.grant();owner.close_step(&mut effect.nested(RetainedCloneGrant{maximum_depth:g.maximum_depth-1,..g})).result})}
    return close_original_retained_step(cx,RetirementDemand{depth:1,..Default::default()},|_|{if pending{owners.pending.take();}else{owners.returned.take();}semio_framework_value_retained_clone::RetainedWorkOutcome::<_,ValueError>{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}})
   }
   if let Some(source)=owners.source.as_mut(){if !source.terminal_is_empty(){return source.close_step(parent,cx)}
    return close_original_retained_step(cx,RetirementDemand{depth:1,..Default::default()},|_|{owners.source.take();semio_framework_value_retained_clone::RetainedWorkOutcome::<_,ValueError>{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}})
   }
   let result=close_original_retained_step(cx,RetirementDemand{depth:1,..Default::default()},|_|{owners.binding=None;semio_framework_value_retained_clone::RetainedWorkOutcome::<_,ValueError>{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}});
   if owners.binding.is_none(){checked.restore=VACANT}return result
  }Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})
 }

}

/// 🪪️ This non-clone reservation returns its still-live source when publication is cancelled.
pub struct OriginalJobPageReservation<const N:usize>{registry:&'static OriginalJobPageReturnRegistry<N>,index:usize,generation:u64,active:bool,ready:bool}
impl<const N:usize> OriginalJobPageReservation<N>{
 pub fn ready(&self)->bool{self.active&&self.ready}
 /// 📦️ Moves one original page and ledger into a small carrier after checking the authentic source.
 pub fn publish_last(&mut self,payload:&mut RetainedJobPayload,output:&mut Option<OriginalSuppliedJobPage>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  let parent=cx.original_source_parent()?;parent.validate_borrowed_context(cx)?;
  if let Some(owner)=output.as_ref(){let target:&'static dyn PageReturnTarget=self.registry;if !std::ptr::eq(owner.target,target)||owner.index!=self.index||owner.generation!=self.generation||self.active{return Err(refusal("supplied page output belongs to another original reservation").into())}let slot=&self.registry.slots[self.index];if slot.identity.load(Ordering::Acquire)!=parent.source_identity()?{return Err(refusal("supplied page cached output changed its original source").into())}let Some(mut checked)=slot.checkout(ISSUED)else{return Ok(false)};if !checked.owners().binding.is_some_and(|binding|binding.matches(cx)){return Err(refusal("supplied page cached output changed its original caller context").into())}return Ok(true)}
  if !self.ready(){return Err(refusal("original page reservation is not ready for transfer").into())}
  let slot=&self.registry.slots[self.index];let Some(mut checked)=slot.checkout(RESERVED)else{return Ok(false)};
  if slot.generation.load(Ordering::Relaxed)!=self.generation{return Err(refusal("original page reservation generation changed").into())}
  checked.owners().source.as_ref().expect("reserved page retains its source").validate_context(cx)?;
  if payload.page_count()==0&&checked.owners().pending.is_none(){return Ok(false)}
  if payload.payload_authority_identity()!=Some(cx.payload_authority_identity()){return Err(refusal("supplied page requires its actual live original payload source").into())}
  if !registry_control_ready(cx){return Ok(false)}
  let grant=cx.retained_grant();let Some((turn,_))=cx.with_original_effect(grant,|_,effect|payload.transfer_last_original_page(&mut checked.owners().pending,effect))?else{return Ok(false)};
  if checked.owners().pending.as_ref().is_some_and(RetainedJobPageTransfer::is_ready){
   *output=Some(OriginalSuppliedJobPage{target:self.registry,index:self.index,generation:self.generation,page:checked.owners().pending.take()});self.active=false;checked.restore=ISSUED;
  }
  #[cfg(test)]if output.is_some(){admission_work_tests::narrow_publication(cx);}
  turn.result?;Ok(output.is_some())

 }
}
impl<const N:usize> Drop for OriginalJobPageReservation<N>{fn drop(&mut self){if !self.active{return}let slot=&self.registry.slots[self.index];let mut checked=slot.checkout(if self.ready{RESERVED}else{CLAIMING}).expect("original reservation has exclusive return custody");assert_eq!(slot.generation.load(Ordering::Relaxed),self.generation);checked.owners().credit.record_return().expect("cancelled original reservation retains its held source return");checked.restore=RETURNED;drop(checked);if let Some(wake)=self.registry.return_wake{wake();}}}

/// 📖️ A consumer only reads the original bytes; dropping this carrier returns them to its reserved supplier slot.
pub struct OriginalSuppliedJobPage{target:&'static dyn PageReturnTarget,index:usize,generation:u64,page:Option<RetainedJobPageTransfer>}
impl OriginalSuppliedJobPage{
 pub fn bytes(&self)->&[u8]{self.page.as_ref().unwrap().bytes().expect("supplied original page is ready")}
 pub fn source_identity(&self)->usize{self.target.identity(self.index)}
 pub fn original_page_index(&self)->usize{self.page.as_ref().unwrap().original_index()}
}
impl std::fmt::Debug for OriginalSuppliedJobPage{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{f.debug_struct("OriginalSuppliedJobPage").field("length",&self.bytes().len()).finish()}}
impl Drop for OriginalSuppliedJobPage{fn drop(&mut self){self.target.return_page(self.index,self.generation,self.page.take().unwrap())}}

#[cfg(test)]
#[path="🎟️admission/🧪️tests/🦀️.rs"]
mod admission_work_tests;
