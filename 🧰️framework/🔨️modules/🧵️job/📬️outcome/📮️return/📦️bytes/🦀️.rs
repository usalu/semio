//! 📦️ A contiguous compiler input retains its genuine supplier loan until exact original backing retirement.
use super::*;

/// 🧱️ This sole contiguous backing is born and filled only through its actual original receiving context.
pub struct OriginalContiguousJobBytes{bytes:Option<Vec<u8>>,source:Option<OriginalCallerSourceLoan>,binding:OriginalSourceBinding,capacity:usize}
impl OriginalContiguousJobBytes{
 pub fn admit(length:usize,output:&mut Option<Self>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  let demand=OriginalConstructionDemand{items:1,copy_bytes:0,capacity_bytes:length,release_bytes:0,depth:1};if cx.should_yield()||cx.is_cancelled()||!demand.fits(cx.retained_grant()){return Ok(false)}
  if output.is_none(){let Some(source)=cx.original_source_parent()?.admit_loan(cx)?else{return Ok(false)};*output=Some(Self{bytes:None,source:Some(source),binding:OriginalSourceBinding::borrow(cx),capacity:length});return Ok(false)}
  let owner=output.as_mut().unwrap();owner.source.as_ref().ok_or_else(||refusal("contiguous birth lost its original source loan"))?.validate_context(cx)?;if !owner.binding.matches(cx)||owner.capacity!=length{return Err(refusal("contiguous birth changed its original source or exact extent").into())}
  admit_original_construction(cx,&mut owner.bytes,demand,|_|{let mut bytes=Vec::new();bytes.try_reserve_exact(length).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original contiguous backing allocation refused"))?;let capacity=bytes.capacity();Ok((bytes,RetainedCloneProgress{copied_items:1,retained_capacity_bytes:capacity,..Default::default()}))})
 }
 pub fn borrowed_bytes(&self)->Option<&[u8]>{self.bytes.as_deref()}
 pub fn retained_length(&self)->usize{self.bytes.as_ref().map_or(0,Vec::len)}
 pub fn ready(&self)->bool{self.bytes.is_some()&&self.source.is_some()}
 pub fn complete(&self)->bool{self.ready()&&self.retained_length()==self.capacity}
 pub fn terminal_is_empty(&self)->bool{self.bytes.is_none()&&self.source.is_none()}
 pub fn append(&mut self,bytes:&[u8],cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if !self.binding.matches(cx){return Err(refusal("contiguous append refused foreign original source").into())}self.source.as_ref().ok_or_else(||refusal("contiguous append lost its original source loan"))?.validate_context(cx)?;let output=self.bytes.as_mut().ok_or_else(||refusal("contiguous backing already retired"))?;
  if output.len()>self.capacity||bytes.len()>self.capacity-output.len(){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"contiguous append exceeds its exact admitted backing").into())}
  let mut candidate=None;admit_original_construction(cx,&mut candidate,OriginalConstructionDemand{items:1,copy_bytes:bytes.len(),capacity_bytes:0,release_bytes:0,depth:1},|_|{output.extend_from_slice(bytes);Ok(((),RetainedCloneProgress{copied_items:1,copied_bytes:bytes.len(),..Default::default()}))})
 }
 fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
  if self.terminal_is_empty(){return Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})}if !self.binding.matches(cx){return Err(refusal("contiguous close refused foreign original source").into())}self.source.as_ref().unwrap().validate_retirement_context(cx)?;
  if let Some(bytes)=self.bytes.as_ref(){let demand=RetirementDemand{copy_bytes:size_of::<Option<Vec<u8>>>(),release_bytes:bytes.capacity(),depth:1,..Default::default()};return close_original_retained_step(cx,demand,|_|{drop(self.bytes.take());Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}))})}
  let source=self.source.as_mut().unwrap();let step=source.close_step(cx.original_source_parent()?,cx)?;if source.terminal_is_empty(){self.source.take();}Ok(if self.terminal_is_empty(){InteractiveJobCloseStep::Complete{progress:step.progress()}}else{InteractiveJobCloseStep::Pending{progress:step.progress()}})
 }
 pub fn retire_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{if !self.terminal_is_empty(){self.close_step(cx)?;}Ok(self.terminal_is_empty())}
}
impl Drop for OriginalContiguousJobBytes{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original contiguous backing requires its actual source paid close");}}

struct BytesOwners{source:Option<OriginalCallerSourceLoan>,bytes:Option<OriginalContiguousJobBytes>}
struct BytesSlot{state:AtomicU8,generation:AtomicU64,identity:AtomicUsize,owners:UnsafeCell<BytesOwners>}
/// 🔐️ Acquire checkout exclusively borrows the retained source and contiguous owner until Release publication.
unsafe impl Sync for BytesSlot{}
impl BytesSlot{
 const fn empty()->Self{Self{state:AtomicU8::new(VACANT),generation:AtomicU64::new(0),identity:AtomicUsize::new(0),owners:UnsafeCell::new(BytesOwners{source:None,bytes:None})}}
 fn checkout(&self,state:u8)->Option<BytesCheckout<'_>>{self.state.compare_exchange(state,CHECKED,Ordering::Acquire,Ordering::Relaxed).ok().map(|_|BytesCheckout{slot:self,restore:state})}
}
struct BytesCheckout<'a>{slot:&'a BytesSlot,restore:u8}
impl BytesCheckout<'_>{fn owners(&mut self)->&mut BytesOwners{unsafe{&mut*self.slot.owners.get()}}}
impl Drop for BytesCheckout<'_>{fn drop(&mut self){self.slot.state.store(self.restore,Ordering::Release)}}
trait BytesReturnTarget:Sync{fn identity(&self,index:usize)->usize;fn return_bytes(&self,index:usize,generation:u64,bytes:OriginalContiguousJobBytes);}

/// 🗄️ Fixed preowned slots retain authentic source loans across arbitrary read-only consumer awaits.
pub struct OriginalJobBytesReturnRegistry<const N:usize>{slots:[BytesSlot;N],wake:fn()}
impl<const N:usize> OriginalJobBytesReturnRegistry<N>{
 pub const fn with_return_wake(wake:fn())->Self{Self{slots:[const{BytesSlot::empty()};N],wake}}
 pub fn reserve(&'static self,cx:&mut StepContext<'_>)->Result<Option<OriginalJobBytesReservation<N>>,OriginalTurnFailure>{
  fn require_send<T:Send>(){}require_send::<BytesOwners>();
  let identity=cx.original_source_parent()?.source_identity()?;
  for(index,slot)in self.slots.iter().enumerate(){
   let Some(mut checked)=slot.checkout(VACANT)else{continue};
   let Some(generation)=slot.generation.load(Ordering::Relaxed).checked_add(1)else{checked.restore=EXHAUSTED;continue};
   let Some(source)=cx.original_source_parent()?.admit_loan(cx)?else{return Ok(None)};
   checked.owners().source=Some(source);slot.identity.store(identity,Ordering::Relaxed);slot.generation.store(generation,Ordering::Relaxed);checked.restore=RESERVED;
   return Ok(Some(OriginalJobBytesReservation{registry:self,index,generation,active:true}))
  }Ok(None)
 }
 pub fn retains_source(&self,identity:usize)->bool{self.slots.iter().any(|slot|matches!(slot.state.load(Ordering::Acquire),CHECKED|RESERVED|ISSUED|RETURNED)&&slot.identity.load(Ordering::Acquire)==identity)}
 pub fn has_returned_source(&self,identity:usize)->bool{self.slots.iter().any(|slot|slot.state.load(Ordering::Acquire)==RETURNED&&slot.identity.load(Ordering::Acquire)==identity)}
 pub fn close_returned_step(&self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
  let identity=cx.original_source_parent()?.source_identity()?;
  for slot in &self.slots{
   if slot.identity.load(Ordering::Acquire)!=identity{continue}let Some(mut checked)=slot.checkout(RETURNED)else{continue};
   let owners=checked.owners();if let Some(source)=owners.source.as_ref(){source.validate_retirement_context(cx)?;}
   if let Some(bytes)=owners.bytes.as_mut(){if !bytes.terminal_is_empty(){let step=bytes.close_step(cx)?;return Ok(match step{InteractiveJobCloseStep::Blocked=>InteractiveJobCloseStep::Blocked,_=>InteractiveJobCloseStep::Pending{progress:step.progress()}})}if !metadata(cx,size_of::<Option<OriginalContiguousJobBytes>>(),true)?{return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}owners.bytes.take();return Ok(InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Option<OriginalContiguousJobBytes>>(),..Default::default()}})}
   if let Some(source)=owners.source.as_mut(){let step=source.close_step(cx.original_source_parent()?,cx)?;if source.terminal_is_empty(){owners.source.take();}return Ok(InteractiveJobCloseStep::Pending{progress:step.progress()})}
   if !metadata(cx,size_of::<bool>(),true)?{return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}checked.restore=VACANT;return Ok(InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<bool>(),..Default::default()}})
  }Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})
 }
}
impl<const N:usize> BytesReturnTarget for OriginalJobBytesReturnRegistry<N>{
 fn identity(&self,index:usize)->usize{self.slots[index].identity.load(Ordering::Acquire)}
 fn return_bytes(&self,index:usize,generation:u64,bytes:OriginalContiguousJobBytes){let slot=&self.slots[index];let mut checked=slot.checkout(ISSUED).expect("contiguous bytes retain their original reserved return");assert_eq!(slot.generation.load(Ordering::Relaxed),generation);checked.owners().bytes=Some(bytes);checked.restore=RETURNED;drop(checked);(self.wake)();}
}

/// 🎟️ Reservation cancellation returns its authentic loan without dropping any supplier root.
pub struct OriginalJobBytesReservation<const N:usize>{registry:&'static OriginalJobBytesReturnRegistry<N>,index:usize,generation:u64,active:bool}
impl<const N:usize> OriginalJobBytesReservation<N>{
 pub fn publish(&mut self,bytes:&mut Option<OriginalContiguousJobBytes>,output:&mut Option<OriginalSuppliedJobBytes>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if !self.active||!bytes.as_ref().is_some_and(OriginalContiguousJobBytes::complete)||output.is_some(){return Err(refusal("contiguous publication requires complete original bytes, active reservation and empty candidate").into())}
  let slot=&self.registry.slots[self.index];let Some(mut checked)=slot.checkout(RESERVED)else{return Ok(false)};
  if slot.generation.load(Ordering::Relaxed)!=self.generation{return Err(refusal("contiguous return generation changed").into())}
  checked.owners().source.as_ref().unwrap().validate_context(cx)?;
  if !bytes.as_ref().unwrap().binding.matches(cx){return Err(refusal("contiguous publication refused a foreign original backing").into())}
  if !metadata(cx,size_of::<Option<OriginalContiguousJobBytes>>()+size_of::<OriginalSuppliedJobBytes>()+size_of::<bool>(),false)?{return Ok(false)}
  *output=Some(OriginalSuppliedJobBytes{target:self.registry,index:self.index,generation:self.generation,bytes:bytes.take()});self.active=false;checked.restore=ISSUED;Ok(true)
 }
}
impl<const N:usize> Drop for OriginalJobBytesReservation<N>{fn drop(&mut self){if !self.active{return}let slot=&self.registry.slots[self.index];let mut checked=slot.checkout(RESERVED).expect("contiguous reservation retains exclusive source custody");assert_eq!(slot.generation.load(Ordering::Relaxed),self.generation);checked.restore=RETURNED;drop(checked);(self.registry.wake)();}}

/// 📖️ Read-only contiguous bytes return the exact original Vec; no raw owning extraction or clone is available.
pub struct OriginalSuppliedJobBytes{target:&'static dyn BytesReturnTarget,index:usize,generation:u64,bytes:Option<OriginalContiguousJobBytes>}
impl OriginalSuppliedJobBytes{pub fn bytes(&self)->&[u8]{self.bytes.as_ref().unwrap().borrowed_bytes().unwrap()}pub fn source_identity(&self)->usize{self.target.identity(self.index)}}
impl std::fmt::Debug for OriginalSuppliedJobBytes{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{f.debug_struct("OriginalSuppliedJobBytes").field("length",&self.bytes().len()).finish()}}
impl Drop for OriginalSuppliedJobBytes{fn drop(&mut self){self.target.return_bytes(self.index,self.generation,self.bytes.take().unwrap())}}
