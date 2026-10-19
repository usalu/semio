//! 📮️ Pre-reserved page returns keep the genuine supplier source live without copying payload backing.
use super::*;
use std::cell::UnsafeCell;

#[path="📦️bytes/🦀️.rs"]
mod original_bytes;
pub use original_bytes::{OriginalContiguousJobBytes,OriginalJobBytesReturnRegistry,OriginalJobBytesReservation,OriginalSuppliedJobBytes};

const VACANT:u8=0;
const CHECKED:u8=1;
const RESERVED:u8=2;
const ISSUED:u8=3;
const RETURNED:u8=4;
const EXHAUSTED:u8=5;

struct ReturnedPage{page:Option<JobPayloadPage>,ledger:Option<Arc<JobPayloadOperationLedger>>,stream:JobPayloadStream,original_index:usize}
struct ReturnOwners{source:Option<OriginalCallerSourceLoan>,returned:Option<ReturnedPage>}
trait PageReturnTarget:Sync{fn identity(&self,index:usize)->usize;fn return_page(&self,index:usize,generation:u64,page:ReturnedPage);}
struct ReturnSlot{state:AtomicU8,generation:AtomicU64,identity:AtomicUsize,owners:UnsafeCell<ReturnOwners>}
/// 🔒️ Acquire checkout exclusively borrows owners; Release publishes their exact return before another checkout.
unsafe impl Sync for ReturnSlot{}
impl ReturnSlot{
 const fn vacant()->Self{Self{state:AtomicU8::new(VACANT),generation:AtomicU64::new(0),identity:AtomicUsize::new(0),owners:UnsafeCell::new(ReturnOwners{source:None,returned:None})}}
 fn checkout(&self,state:u8)->Option<ReturnCheckout<'_>>{self.state.compare_exchange(state,CHECKED,Ordering::Acquire,Ordering::Relaxed).ok().map(|_|ReturnCheckout{slot:self,restore:state})}
}
struct ReturnCheckout<'a>{slot:&'a ReturnSlot,restore:u8}
impl ReturnCheckout<'_>{fn owners(&mut self)->&mut ReturnOwners{unsafe{&mut*self.slot.owners.get()}}}
impl Drop for ReturnCheckout<'_>{fn drop(&mut self){self.slot.state.store(self.restore,Ordering::Release)}}

fn refusal(reason:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::UnsupportedOwner,reason)}
fn metadata(cx:&mut StepContext<'_>,copy_bytes:usize,closing:bool)->Result<bool,OriginalTurnFailure>{let grant=cx.retained_grant();if (if closing{cx.retirement_should_yield()}else{cx.should_yield()||cx.is_cancelled()})||grant.maximum_items==0||grant.maximum_copy_bytes<copy_bytes||grant.maximum_depth==0{return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:copy_bytes,..Default::default()})?;cx.consume_fuel(1);Ok(true)}

/// 🗄️ The program owns a fixed inline return table before issuing any payload or source loan.
pub struct OriginalJobPageReturnRegistry<const N:usize>{slots:[ReturnSlot;N],return_wake:Option<fn()>}
impl<const N:usize> PageReturnTarget for OriginalJobPageReturnRegistry<N>{
 fn identity(&self,index:usize)->usize{self.slots[index].identity.load(Ordering::Acquire)}
 fn return_page(&self,index:usize,generation:u64,page:ReturnedPage){let slot=&self.slots[index];let mut checked=slot.checkout(ISSUED).expect("supplied page retains its exclusive reserved return");assert_eq!(slot.generation.load(Ordering::Relaxed),generation);checked.owners().returned=Some(page);checked.restore=RETURNED;drop(checked);if let Some(wake)=self.return_wake{wake();}}
}
impl<const N:usize> OriginalJobPageReturnRegistry<N>{
 pub const fn new()->Self{Self{slots:[const{ReturnSlot::vacant()};N],return_wake:None}}
 /// 🔔️ A preowned function observes a published return after exclusive slot checkout ends, without owning another closure.
 pub const fn with_return_wake(wake:fn())->Self{Self{slots:[const{ReturnSlot::vacant()};N],return_wake:Some(wake)}}
 /// 🎟️ A successful reservation retains one genuine source loan before any page leaves its supplier.
 pub fn reserve(&'static self,cx:&mut StepContext<'_>)->Result<Option<OriginalJobPageReservation<N>>,OriginalTurnFailure>{
  fn require_send<T:Send>(){}require_send::<ReturnOwners>();
  let identity=cx.original_source_parent()?.source_identity()?;
  for(index,slot)in self.slots.iter().enumerate(){
   let Some(mut checked)=slot.checkout(VACANT)else{continue};
   let Some(generation)=slot.generation.load(Ordering::Relaxed).checked_add(1)else{checked.restore=EXHAUSTED;continue};
   let Some(source)=cx.original_source_parent()?.admit_loan(cx)?else{return Ok(None)};
   checked.owners().source=Some(source);slot.identity.store(identity,Ordering::Relaxed);slot.generation.store(generation,Ordering::Relaxed);checked.restore=RESERVED;
   return Ok(Some(OriginalJobPageReservation{registry:self,index,generation,active:true}))
  }
  Ok(None)
 }
 /// 🔗️ The original source cannot finish while a reserved, supplied or returned page still retains it.
 pub fn retains_source(&self,identity:usize)->bool{self.slots.iter().any(|slot|matches!(slot.state.load(Ordering::Acquire),CHECKED|RESERVED|ISSUED|RETURNED)&&slot.identity.load(Ordering::Acquire)==identity)}
 /// 📣️ Observes only already-published returns; callback notification never exposes an exclusively checked-out slot.
 pub fn has_returned_source(&self,identity:usize)->bool{self.slots.iter().any(|slot|slot.state.load(Ordering::Acquire)==RETURNED&&slot.identity.load(Ordering::Acquire)==identity)}
 /// ♻️ The exact original caller pays page backing, ledger, loan return and final slot removal separately.
 pub fn close_returned_step(&self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
  let identity=cx.original_source_parent()?.source_identity()?;
  for slot in &self.slots{
   if slot.identity.load(Ordering::Acquire)!=identity{continue}
   let Some(mut checked)=slot.checkout(RETURNED)else{continue};
   let owners=checked.owners();
   if let Some(source)=owners.source.as_ref(){source.validate_retirement_context(cx)?;}
   if let Some(returned)=owners.returned.as_mut(){
    if let Some(page)=returned.page.as_ref(){
     let extent=page.source.allocated_capacity_bytes();let demand=RetirementDemand{copy_bytes:size_of::<Option<JobPayloadPage>>()+JobPayloadOperationLedger::reservation_copy_bytes(),release_bytes:extent,depth:2,..Default::default()};
     return close_original_retained_step(cx,demand,|_|{let page=returned.page.take().unwrap();returned.ledger.as_ref().expect("supplied page retains its original ledger").release(returned.stream,extent);drop(page);Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:extent,..Default::default()}))})
    }
    if returned.ledger.is_some(){let demand=payload_retirement::ledger_demands(&returned.ledger);return close_original_retained_step(cx,demand,|grant|payload_retirement::close_ledger(&mut returned.ledger,grant))}
    if !metadata(cx,size_of::<bool>(),true)?{return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}owners.returned.take();return Ok(InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<bool>(),..Default::default()}})
   }
   if let Some(source)=owners.source.as_mut(){let result=source.close_step(cx.original_source_parent()?,cx)?;if source.terminal_is_empty(){owners.source.take();}return Ok(result)}
   if !metadata(cx,size_of::<bool>(),true)?{return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}
   checked.restore=VACANT;return Ok(InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<bool>(),..Default::default()}})
  }
  Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})
 }
}

/// 🪪️ This non-clone reservation returns its still-live source when publication is cancelled.
pub struct OriginalJobPageReservation<const N:usize>{registry:&'static OriginalJobPageReturnRegistry<N>,index:usize,generation:u64,active:bool}
impl<const N:usize> OriginalJobPageReservation<N>{
 /// 📦️ Moves one original page and ledger into a small carrier after checking the authentic source.
 pub fn publish_last(&mut self,payload:&mut RetainedJobPayload,cx:&mut StepContext<'_>)->Result<Option<OriginalSuppliedJobPage>,OriginalTurnFailure>{
  if !self.active{return Err(refusal("original page reservation already transferred").into())}
  let slot=&self.registry.slots[self.index];let Some(mut checked)=slot.checkout(RESERVED)else{return Ok(None)};
  if slot.generation.load(Ordering::Relaxed)!=self.generation{return Err(refusal("original page reservation generation changed").into())}
  checked.owners().source.as_ref().expect("reserved page retains its source").validate_context(cx)?;
  if payload.page_count()==0||payload.payload_authority_identity()!=Some(cx.payload_authority_identity()){return Err(refusal("supplied page requires its actual live original payload source").into())}
  let copy_bytes=size_of::<ReturnedPage>()+size_of::<Option<JobPayloadPage>>()+2*size_of::<usize>()+size_of::<bool>();
  if !metadata(cx,copy_bytes,false)?{return Ok(None)}
  let original_index=payload.page_count-1;let page=payload.pages[original_index].take().unwrap();payload.page_count-=1;payload.length-=page.length;let ledger=if payload.page_count==0{payload.ledger.take()}else{Some(Arc::clone(payload.ledger.as_ref().unwrap()))};let page=ReturnedPage{page:Some(page),ledger,stream:payload.stream,original_index};self.active=false;checked.restore=ISSUED;
  Ok(Some(OriginalSuppliedJobPage{target:self.registry,index:self.index,generation:self.generation,page:Some(page)}))
 }
}
impl<const N:usize> Drop for OriginalJobPageReservation<N>{fn drop(&mut self){if !self.active{return}let slot=&self.registry.slots[self.index];let mut checked=slot.checkout(RESERVED).expect("original reservation has exclusive return custody");assert_eq!(slot.generation.load(Ordering::Relaxed),self.generation);checked.restore=RETURNED;drop(checked);if let Some(wake)=self.registry.return_wake{wake();}}}

/// 📖️ A consumer only reads the original bytes; dropping this carrier returns them to its reserved supplier slot.
pub struct OriginalSuppliedJobPage{target:&'static dyn PageReturnTarget,index:usize,generation:u64,page:Option<ReturnedPage>}
impl OriginalSuppliedJobPage{
 pub fn bytes(&self)->&[u8]{self.page.as_ref().unwrap().page.as_ref().unwrap().bytes()}
 pub fn source_identity(&self)->usize{self.target.identity(self.index)}
 pub fn original_page_index(&self)->usize{self.page.as_ref().unwrap().original_index}
}
impl std::fmt::Debug for OriginalSuppliedJobPage{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{f.debug_struct("OriginalSuppliedJobPage").field("length",&self.bytes().len()).finish()}}
impl Drop for OriginalSuppliedJobPage{fn drop(&mut self){self.target.return_page(self.index,self.generation,self.page.take().unwrap())}}
