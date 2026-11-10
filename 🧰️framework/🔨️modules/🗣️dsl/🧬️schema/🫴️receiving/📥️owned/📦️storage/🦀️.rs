//! 🪡️ Real typed child backing is admitted before its pure header and freed only after its genuine frontier is empty.
use super::*;
use semio_framework_value_native_storage::NativeOwnedStorage;
pub(crate) fn header_release_demand<C>()->RetirementDemand{RetirementDemand{release_bytes:std::alloc::Layout::new::<C>().size(),depth:1,..Default::default()}}
/// 🧹️ The defining caller proves C's real frontier empty before this fixed backing release; C::Drop is never invoked.
pub(crate) unsafe fn release_empty_header<C>(header:&mut Option<Box<C>>,effect:&mut RetainedAllocationEffect<'_>)->OwnedClose{
 if header.is_none(){return idle(effect)}let d=header_release_demand::<C>();if !admits(effect,d){return idle(effect)}
 if let Err(error)=perform(effect){return done(effect,Err(error.into()))}
 let layout=std::alloc::Layout::new::<C>();let pointer=Box::into_raw(match header.take(){Some(v)=>v,None=>return done(effect,Err(ValueError::invariant("header absent").into()))});
 if layout.size()!=0{unsafe{std::alloc::dealloc(pointer.cast(),layout)}}done(effect,Ok(receipt(d)))
}
/// 🎛️ The actual cursor and its completed header remain in the same owner through every denied or failed call.
pub struct OwnedReceivingSlot<T:DslOwnedReceivingField>{backing:NativeOwnedStorage<T::Cursor>,retired:ManuallyDrop<Option<Box<T::Cursor>>>,closing:bool}
impl<T:DslOwnedReceivingField>OwnedReceivingSlot<T>{
 pub const fn new()->Self{Self{backing:NativeOwnedStorage::empty(),retired:ManuallyDrop::new(None),closing:false}}
 pub fn is_ready(&self)->bool{!self.closing&&self.backing.get().is_some_and(|c|c.is_ready())}
 pub fn demands(&self,source:&Option<FieldValue>,body:usize)->Result<RetirementDemand,ValueError>{if self.closing{return Err(invariant())}if !self.backing.reserved(){return Ok(NativeOwnedStorage::<T::Cursor>::reserve_demand())}if !self.backing.initialized(){return Ok(NativeOwnedStorage::<T::Cursor>::seed_demand())}self.backing.get().ok_or_else(invariant)?.demands(source,body).and_then(nested)}
 pub fn select_advance(&self,source:&Option<FieldValue>,body:usize,maximum_work_units:u64)->Result<OwnedAdvanceSelection,ValueError>{if self.closing{return Err(invariant())}if !self.backing.reserved(){return Ok(OwnedAdvanceSelection::Local{work:self.backing.reserve_work_demand()})}if !self.backing.initialized(){return Ok(OwnedAdvanceSelection::Local{work:self.backing.seed_work_demand()})}self.backing.get().ok_or_else(invariant)?.select_advance(source,body,maximum_work_units)}
 pub fn advance_nested(&mut self,source:&mut Option<FieldValue>,native:&mut NativeDecodeControl<'_>,witness:OriginalSourceTurnWitness<'_>,body:usize,maximum_work_units:u64,g:RetainedCloneGrant)->OwnedPoll{match self.backing.get_mut(){Some(child)=>child.advance_nested(source,native,witness,body,maximum_work_units,child_grant(g)),None=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invariant().into()),work:0})}}
 pub fn advance_local(&mut self,source:&mut Option<FieldValue>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OwnedPoll{
  let d=match self.demands(source,body){Ok(d)=>d,Err(error)=>return done(effect,Err(error.into()))};if !admits(effect,d){return idle(effect)}
  if !self.backing.reserved(){let poll=normal(physical(self.backing.reserve_empty(effect)));return settle(effect,poll)}
  if !self.backing.initialized(){let poll=normal(physical(self.backing.initialize_seed(effect,||Ok(T::receiving_cursor()))));return settle(effect,poll)}
  let g=effect.grant();match self.backing.get_mut(){Some(child)=>{let poll=child.advance_local(source,body,&mut effect.nested(child_grant(g)));settle(effect,poll)},None=>done(effect,Err(ValueError::invariant("get_mut target absent").into()))}
 }
 pub fn take_work_demand(&self)->Result<u64,ValueError>{self.backing.get().ok_or_else(invariant)?.take_work_demand()}
 pub fn take_into(&mut self,output:&mut Option<T>,effect:&mut RetainedAllocationEffect<'_>)->OwnedTake{if self.closing{return done(effect,Err(invariant()))}let g=effect.grant();let Some(child)=self.backing.get_mut()else{return done(effect,Err(invariant()))};let poll=child.take_into(output,&mut effect.nested(child_grant(g)));settle(effect,poll)}
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if self.terminal_is_empty(){return Ok(Default::default())}if !self.closing{return Ok(metadata::<()>())}if let Some(header)=self.retired.as_ref(){if !header.terminal_is_empty(){return Err(invariant())}return Ok(header_release_demand::<T::Cursor>())}if let Some(child)=self.backing.get(){return if child.terminal_is_empty(){Ok(NativeOwnedStorage::<T::Cursor>::box_demand())}else{child.close_demands(body).and_then(nested)}}Ok(self.backing.release_demand())}
 pub fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if self.terminal_is_empty(){return Ok(0)}if !self.closing||self.retired.is_some(){return Ok(1)}if let Some(child)=self.backing.get(){return if child.terminal_is_empty(){Ok(self.backing.box_work_demand())}else{child.close_work_demand(body)}}Ok(self.backing.release_work_demand())}
 pub fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OwnedClose{
  if self.terminal_is_empty(){return idle(effect)}
  let g=effect.grant();let d=match self.close_demands(g.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return done(effect,Err(error.into()))};if !admits(effect,d){return idle(effect)}
  if !self.closing{return local_close(effect,d,||{self.closing=true;Ok(())})}
  if let Some(header)=self.retired.as_ref(){if !header.terminal_is_empty(){return done(effect,Err(invariant().into()))}return unsafe{release_empty_header(&mut self.retired,effect)}}
  if let Some(child)=self.backing.get_mut(){if !child.terminal_is_empty(){let poll=child.close_step(&mut effect.nested(child_grant(g)));return settle(effect,poll)}let poll=closing(physical(self.backing.take_box_into(&mut self.retired,effect)));return settle(effect,poll)}
  let poll=RetainedWorkPoll::Complete(self.backing.release(effect));settle(effect,closing(physical(poll)))
 }
 pub fn terminal_is_empty(&self)->bool{self.backing.terminal_is_empty()&&self.retired.is_none()}
}
impl<T:DslOwnedReceivingField>Drop for OwnedReceivingSlot<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"owned receiving child needs paid terminal closure");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.retired)}}}}
