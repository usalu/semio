//! 📦️ A contiguous compiler input retains its genuine supplier loan until exact original backing retirement.
use super::*;

/// 🧱️ This sole contiguous backing is born and filled only through its actual original receiving context.
pub struct OriginalContiguousJobBytes{bytes:Option<Vec<u8>>,source:Option<OriginalCallerSourceLoan>,binding:OriginalSourceBinding,capacity:usize,active:bool}
impl OriginalContiguousJobBytes{
 pub fn admit(length:usize,output:&mut Option<Self>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  let demand=OriginalConstructionDemand{items:1,copy_bytes:0,capacity_bytes:length,release_bytes:0,depth:1};if cx.should_yield()||cx.is_cancelled()||!demand.fits(cx.retained_grant()){return Ok(false)}
  let parent=cx.original_source_parent()?;parent.validate_borrowed_context(cx)?;
  if output.is_none(){*output=Some(Self{bytes:None,source:None,binding:OriginalSourceBinding::borrow(cx),capacity:length,active:true});registry_control(cx,false)?;return Ok(false)}
  let owner=output.as_mut().unwrap();if !owner.active||!owner.binding.matches(cx)||owner.capacity!=length{return Err(refusal("contiguous birth changed its original source or exact extent").into())}
  if owner.source.is_none(){parent.admit_loan(&mut owner.source,cx)?;return Ok(false)}
  owner.source.as_ref().unwrap().validate_context(cx)?;
  admit_original_construction(cx,&mut owner.bytes,demand,|_|{let mut bytes=Vec::new();let result=bytes.try_reserve_exact(length).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original contiguous backing allocation refused")).map(|()|{let capacity=bytes.capacity();(bytes,RetainedCloneProgress{copied_items:1,retained_capacity_bytes:capacity,..Default::default()})});semio_framework_value_retained_clone::RetainedWorkOutcome{result,work:1}})
 }
 pub fn borrowed_bytes(&self)->Option<&[u8]>{self.bytes.as_deref()}
 pub fn retained_length(&self)->usize{self.bytes.as_ref().map_or(0,Vec::len)}
 pub fn ready(&self)->bool{self.bytes.is_some()&&self.source.is_some()}
 pub fn complete(&self)->bool{self.ready()&&self.retained_length()==self.capacity}
 pub fn terminal_is_empty(&self)->bool{!self.active&&self.bytes.is_none()&&self.source.is_none()}
 pub fn append(&mut self,bytes:&[u8],cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if !self.binding.matches(cx){return Err(refusal("contiguous append refused foreign original source").into())}self.source.as_ref().ok_or_else(||refusal("contiguous append lost its original source loan"))?.validate_context(cx)?;let output=self.bytes.as_mut().ok_or_else(||refusal("contiguous backing already retired"))?;
  if output.len()>self.capacity||bytes.len()>self.capacity-output.len(){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"contiguous append exceeds its exact admitted backing").into())}
  if bytes.is_empty(){return Ok(true)}let mut candidate=None;admit_original_construction(cx,&mut candidate,OriginalConstructionDemand{items:1,copy_bytes:bytes.len(),capacity_bytes:0,release_bytes:0,depth:1},|_|{output.extend_from_slice(bytes);semio_framework_value_retained_clone::RetainedWorkOutcome::<_,ValueError>{result:Ok(((),RetainedCloneProgress{copied_items:1,copied_bytes:bytes.len(),..Default::default()})),work:1}})
 }
 fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
  if self.terminal_is_empty(){return Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})}if !self.binding.matches(cx){return Err(refusal("contiguous close refused foreign original source").into())}cx.original_source_parent()?.validate_borrowed_context(cx)?;
  if let Some(source)=self.source.as_ref(){if !source.terminal_is_empty(){source.validate_retirement_context(cx)?;}}
  if let Some(bytes)=self.bytes.as_ref(){let demand=RetirementDemand{release_bytes:bytes.capacity(),depth:1,..Default::default()};return close_original_retained_step(cx,demand,|_|{drop(self.bytes.take());semio_framework_value_retained_clone::RetainedWorkOutcome::<_,ValueError>{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..Default::default()})),work:1}})}
  if let Some(source)=self.source.as_mut(){if !source.terminal_is_empty(){return source.close_step(cx.original_source_parent()?,cx)}}
  close_original_retained_step(cx,RetirementDemand{depth:1,..Default::default()},|_|{self.source.take();self.active=false;semio_framework_value_retained_clone::RetainedWorkOutcome::<_,ValueError>{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}})
 }
 pub fn retire_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{if !self.terminal_is_empty(){self.close_step(cx)?;}Ok(self.terminal_is_empty())}
}
impl Drop for OriginalContiguousJobBytes{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original contiguous backing requires its actual source paid close");}}

struct BytesOwners{source:Option<OriginalCallerSourceLoan>,bytes:Option<OriginalContiguousJobBytes>,binding:Option<OriginalSourceBinding>,credit:OriginalCallerReturnPlace}
struct BytesSlot{state:AtomicU8,generation:AtomicU64,identity:AtomicUsize,owners:UnsafeCell<BytesOwners>}
/// 🔐️ Acquire checkout exclusively borrows the retained source and contiguous owner until Release publication.
unsafe impl Sync for BytesSlot{}
impl BytesSlot{
 const fn empty()->Self{Self{state:AtomicU8::new(VACANT),generation:AtomicU64::new(0),identity:AtomicUsize::new(0),owners:UnsafeCell::new(BytesOwners{source:None,bytes:None,binding:None,credit:OriginalCallerReturnPlace::empty()})}}
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
 pub fn reserve(&'static self,output:&mut Option<OriginalJobBytesReservation<N>>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  fn require_send<T:Send>(){}require_send::<BytesOwners>();
  let parent=cx.original_source_parent()?;let identity=parent.source_identity()?;parent.validate_borrowed_context(cx)?;
  if let Some(owner)=output.as_mut(){
   if !std::ptr::eq(owner.registry,self)||!owner.active{return Err(refusal("original bytes reservation output belongs to another registry").into())}
   let slot=&self.slots[owner.index];if slot.identity.load(Ordering::Acquire)!=identity||slot.generation.load(Ordering::Acquire)!=owner.generation{return Err(refusal("original bytes reservation output changed source or generation").into())}
   let Some(mut checked)=slot.checkout(if owner.ready{RESERVED}else{CLAIMING})else{return Ok(false)};
   if !checked.owners().binding.is_some_and(|binding|binding.matches(cx)){return Err(refusal("original bytes reservation changed its bound caller context").into())}
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
   if checked.owners().credit.is_admitted(){checked.owners().binding=Some(OriginalSourceBinding::borrow(cx));slot.identity.store(identity,Ordering::Relaxed);slot.generation.store(generation,Ordering::Relaxed);checked.restore=CLAIMING;*output=Some(OriginalJobBytesReservation{registry:self,index,generation,active:true,ready:false});}
   admission?;return Ok(false)
  }Ok(false)
 }
 pub fn retains_source(&self,identity:usize)->bool{self.slots.iter().any(|slot|matches!(slot.state.load(Ordering::Acquire),CHECKED|CLAIMING|RESERVED|ISSUED|RETURNED)&&slot.identity.load(Ordering::Acquire)==identity)}
 pub fn has_returned_source(&self,identity:usize)->bool{self.slots.iter().any(|slot|slot.state.load(Ordering::Acquire)==RETURNED&&slot.identity.load(Ordering::Acquire)==identity)}
 pub fn close_returned_step(&self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
  let parent=cx.original_source_parent()?;parent.validate_borrowed_context(cx)?;let identity=parent.source_identity()?;
  for slot in &self.slots{
   if slot.identity.load(Ordering::Acquire)!=identity{continue}let Some(mut checked)=slot.checkout(RETURNED)else{continue};let owners=checked.owners();
   if !owners.binding.is_some_and(|binding|binding.matches(cx)){return Err(refusal("original bytes return changed its bound caller context").into())}
   if let Some(source)=owners.source.as_ref(){if !source.terminal_is_empty(){source.validate_retirement_context(cx)?;}}
   if owners.credit.is_admitted(){let before=cx.retained_progress();owners.credit.settle_return(cx)?;let after=cx.retained_progress();return Ok(InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:after.copied_items-before.copied_items,copied_bytes:after.copied_bytes-before.copied_bytes,retained_capacity_bytes:after.retained_capacity_bytes-before.retained_capacity_bytes,released_bytes:after.released_bytes-before.released_bytes}})}

   if let Some(bytes)=owners.bytes.as_mut(){if !bytes.terminal_is_empty(){return bytes.close_step(cx)}
    return close_original_retained_step(cx,RetirementDemand{depth:1,..Default::default()},|_|{owners.bytes.take();semio_framework_value_retained_clone::RetainedWorkOutcome::<_,ValueError>{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}})
   }
   if let Some(source)=owners.source.as_mut(){if !source.terminal_is_empty(){return source.close_step(parent,cx)}
    return close_original_retained_step(cx,RetirementDemand{depth:1,..Default::default()},|_|{owners.source.take();semio_framework_value_retained_clone::RetainedWorkOutcome::<_,ValueError>{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}})
   }
   let result=close_original_retained_step(cx,RetirementDemand{depth:1,..Default::default()},|_|{owners.binding=None;semio_framework_value_retained_clone::RetainedWorkOutcome::<_,ValueError>{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}});
   if owners.binding.is_none(){checked.restore=VACANT}return result
  }Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})
 }

}
impl<const N:usize> BytesReturnTarget for OriginalJobBytesReturnRegistry<N>{
 fn identity(&self,index:usize)->usize{self.slots[index].identity.load(Ordering::Acquire)}
 fn return_bytes(&self,index:usize,generation:u64,bytes:OriginalContiguousJobBytes){let slot=&self.slots[index];let mut checked=slot.checkout(ISSUED).expect("contiguous bytes retain their original reserved return");assert_eq!(slot.generation.load(Ordering::Relaxed),generation);checked.owners().bytes=Some(bytes);checked.owners().credit.record_return().expect("original bytes retain their one-shot held source return");checked.restore=RETURNED;drop(checked);(self.wake)();}
}

/// 🎟️ Reservation cancellation returns its authentic loan without dropping any supplier root.
pub struct OriginalJobBytesReservation<const N:usize>{registry:&'static OriginalJobBytesReturnRegistry<N>,index:usize,generation:u64,active:bool,ready:bool}
impl<const N:usize> OriginalJobBytesReservation<N>{
 pub fn ready(&self)->bool{self.active&&self.ready}
 pub fn publish(&mut self,bytes:&mut Option<OriginalContiguousJobBytes>,output:&mut Option<OriginalSuppliedJobBytes>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if !self.ready()||!bytes.as_ref().is_some_and(OriginalContiguousJobBytes::complete)||output.is_some(){return Err(refusal("contiguous publication requires complete original bytes, active reservation and empty candidate").into())}
  let slot=&self.registry.slots[self.index];let Some(mut checked)=slot.checkout(RESERVED)else{return Ok(false)};
  if slot.generation.load(Ordering::Relaxed)!=self.generation{return Err(refusal("contiguous return generation changed").into())}
  checked.owners().source.as_ref().unwrap().validate_context(cx)?;
  if !bytes.as_ref().unwrap().binding.matches(cx){return Err(refusal("contiguous publication refused a foreign original backing").into())}
  if !registry_control_ready(cx){return Ok(false)}
  *output=Some(OriginalSuppliedJobBytes{target:self.registry,index:self.index,generation:self.generation,bytes:bytes.take()});self.active=false;checked.restore=ISSUED;registry_control(cx,false)?;Ok(true)
 }
}
impl<const N:usize> Drop for OriginalJobBytesReservation<N>{fn drop(&mut self){if !self.active{return}let slot=&self.registry.slots[self.index];let mut checked=slot.checkout(if self.ready{RESERVED}else{CLAIMING}).expect("contiguous reservation retains exclusive source custody");assert_eq!(slot.generation.load(Ordering::Relaxed),self.generation);checked.owners().credit.record_return().expect("cancelled original reservation retains its held source return");checked.restore=RETURNED;drop(checked);(self.registry.wake)();}}

/// 📖️ Read-only contiguous bytes return the exact original Vec; no raw owning extraction or clone is available.
pub struct OriginalSuppliedJobBytes{target:&'static dyn BytesReturnTarget,index:usize,generation:u64,bytes:Option<OriginalContiguousJobBytes>}
impl OriginalSuppliedJobBytes{pub fn bytes(&self)->&[u8]{self.bytes.as_ref().unwrap().borrowed_bytes().unwrap()}pub fn source_identity(&self)->usize{self.target.identity(self.index)}}
impl std::fmt::Debug for OriginalSuppliedJobBytes{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{f.debug_struct("OriginalSuppliedJobBytes").field("length",&self.bytes().len()).finish()}}
impl Drop for OriginalSuppliedJobBytes{fn drop(&mut self){self.target.return_bytes(self.index,self.generation,self.bytes.take().unwrap())}}
