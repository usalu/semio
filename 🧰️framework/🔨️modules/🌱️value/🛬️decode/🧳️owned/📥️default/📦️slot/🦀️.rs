//! 📦️ Typed default cursor backing is retained before separate pure initialization and output handoff.
use super::*;
use semio_framework_value_native_storage::NativeOwnedStorage;
use crate::native_decoding::{OriginalEffectAuthority,OriginalEffectProducer,OriginalEffectReply,OriginalEffectReplyComplete};
use crate::RetainedWorkPoll::{Complete,Pending};
use semio_framework_value_native_receiver_protocol::NativeReceivingChild;
type Authority<T>=OriginalEffectAuthority<NativeDefaultSlotProducer<T>>;
/// 🎛️ Resident producer authenticates each original reserve, seed and publication effect of one default slot.
pub struct NativeDefaultSlotProducer<T:NativeDefault>{marker:std::marker::PhantomData<fn()->T>}
impl<T:NativeDefault> NativeDefaultSlotProducer<T>{fn new()->Self{Self{marker:std::marker::PhantomData}}}
/// 🧵️ Lends only the private caller-owned slots selected by the default slot.
pub enum NativeDefaultSlotOperation<'a,T:NativeDefault>{Reserve(&'a mut NativeOwnedStorage<T::Cursor>),Seed(&'a mut NativeOwnedStorage<T::Cursor>),Publish(&'a mut T::Cursor,&'a mut Option<T>)}
impl<T:NativeDefault> RetireOwned for NativeDefaultSlotProducer<T>{type RetirementCursor=<() as RetireOwned>::RetirementCursor;fn controlled_retirement_supported()->bool{<() as RetireOwned>::controlled_retirement_supported()}fn retirement_seed(self)->Self::RetirementCursor{().retirement_seed()}}
impl<T:NativeDefault> OriginalEffectProducer for NativeDefaultSlotProducer<T>{
 type Output=RetainedCloneStep;type Failure=OriginalReceivingPrimary;type Context<'turn,'source>=NativeDefaultSlotOperation<'turn,T> where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&Self::Context<'turn,'source>)->Result<(),ValueError>where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&Self::Context<'turn,'source>)->Option<crate::OriginalSourceTurnIdentity>where 'source:'turn{None}
 fn receive<'reply,'turn,'source>(&mut self,context:&mut Self::Context<'turn,'source>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>where 'source:'turn{
  let turn=match context{NativeDefaultSlotOperation::Reserve(storage)=>storage.reserve_empty(effect).map_err(Into::into),NativeDefaultSlotOperation::Seed(storage)=>storage.initialize_seed(effect,||Ok(T::default_cursor())).map_err(Into::into),NativeDefaultSlotOperation::Publish(cursor,output)=>return cursor.take(output,effect,reply)};
  Ok(match turn{Pending{..}=>reply.pending(),Complete(turn)=>reply.complete(turn.result)})
 }
 fn recover<'reply,'turn,'source>(&mut self,_:&mut Self::Context<'turn,'source>,_:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>where 'source:'turn{Ok(reply.pending())}
}
fn backing_demands<T:NativeDefault>(storage:&NativeOwnedStorage<T::Cursor>,retired:&Option<Box<T::Cursor>>,body:usize)->Result<RetirementDemand,ValueError>{
 if let Some(cursor)=retired.as_ref(){if !cursor.terminal_is_empty(){return Err(invalid())}return Ok(RetirementDemand{copy_bytes:size_of::<Option<Box<T::Cursor>>>(),release_bytes:std::alloc::Layout::new::<T::Cursor>().size(),depth:1,..Default::default()})}
 if let Some(cursor)=storage.get(){if !cursor.terminal_is_empty(){return nested(cursor.close_demands(body)?,0)}return Ok(NativeOwnedStorage::<T::Cursor>::box_demand())}
 Ok(storage.release_demand())
}
fn close_backing<T:NativeDefault>(storage:&mut NativeOwnedStorage<T::Cursor>,retired:&mut Option<Box<T::Cursor>>,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{
 let g=effect.grant();let d=match backing_demands::<T>(storage,retired,g.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})};if !retained_clone_grant_admits(g,d)||effect.maximum_work_units()==0{return Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}
 if retired.is_some(){match effect.performed(1){Ok(true)=>{},Ok(false)=>return Complete(RetainedWorkOutcome{result:Err(ValueError::literal(crate::ValueRefusalKind::WorkLimit,"native default release exceeded its work authority").into()),work:1}),Err(error)=>return Complete(RetainedWorkOutcome{result:Err(error.into()),work:1})}let layout=std::alloc::Layout::new::<T::Cursor>();let Some(owner)=retired.take()else{return Complete(RetainedWorkOutcome{result:Err(ValueError::invariant("retired_header absent").into()),work:1})};let pointer=Box::into_raw(owner);if layout.size()!=0{unsafe{std::alloc::dealloc(pointer.cast(),layout)}}return Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()})),work:1})}
 if let Some(cursor)=storage.get_mut(){if !cursor.terminal_is_empty(){return match cursor.close_step(&mut effect.nested(child_grant(g,0))){Pending{work}=>Pending{work},Complete(turn)=>Complete(RetainedWorkOutcome{result:turn.result.map(|step|RetainedCloneStep::Progress(step.progress())),work:turn.work})}}return match storage.take_box_into(retired,effect){Pending{work}=>Pending{work},Complete(turn)=>Complete(RetainedWorkOutcome{result:turn.result.map(|step|RetainedCloneStep::Progress(step.progress())).map_err(Into::into),work:turn.work})}}
 let turn=storage.release(effect);Complete(RetainedWorkOutcome{result:turn.result.map_err(Into::into),work:turn.work})
}
pub struct NativeDefaultSlot<T:NativeDefault>{cursor:NativeOwnedStorage<T::Cursor>,retired_header:Option<Box<T::Cursor>>,output:ManuallyDrop<Option<T>>,retirement:RetainedCloneClose,action:NativeReceivingChild<OriginalReceivingPrimary>,authority:ManuallyDrop<Option<Authority<T>>>,closing:bool}
impl<T:NativeDefault> NativeDefaultSlot<T>{
 pub fn new()->Self{Self{cursor:NativeOwnedStorage::empty(),retired_header:None,output:ManuallyDrop::new(None),retirement:Default::default(),action:Default::default(),authority:ManuallyDrop::new(Some(OriginalEffectAuthority::new(NativeDefaultSlotProducer::new()))),closing:false}}
 pub fn ready(&self)->bool{!self.closing&&self.output.is_some()}
 pub fn demands(&self,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{if self.closing||self.output.is_some(){return Err(invalid())}if let Some(d)=self.action.demand{return Ok(d)}if !self.cursor.reserved(){return Ok(NativeOwnedStorage::<T::Cursor>::reserve_demand())}if !self.cursor.initialized(){return Ok(NativeOwnedStorage::<T::Cursor>::seed_demand())}let cursor=self.cursor.get().ok_or_else(||ValueError::invariant("get target absent"))?;if cursor.output_ready(){nested(cursor.take_demand()?,0)}else{nested(cursor.demands(body,maximum_work_units)?,0)}}
 pub fn advance(&mut self,control:&mut NativeDecodeControl<'_>,body:usize,maximum_work_units:u64,g:RetainedCloneGrant)->crate::RetainedWorkPoll<RetainedCloneStep,OriginalReceivingPrimary>{
  if maximum_work_units==0{return if self.action.demand.is_some(){Pending{work:0}}else{Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}}
  let result:Result<crate::RetainedWorkPoll<RetainedCloneStep,OriginalReceivingPrimary>,OriginalReceivingPrimary>=(||{
   let d=self.demands(body,maximum_work_units)?;if !retained_clone_grant_admits(g,d){return Ok(if self.action.demand.is_some(){Pending{work:0}}else{Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})})}
   if self.action.demand.is_none(){control.checkpoint()?;receiving_capacity(control,d.capacity_bytes)?;}
   let authority=self.authority.as_mut().ok_or_else(invalid)?;
   if !self.cursor.reserved()||self.action.tag==2{self.action.tag=2;let units=self.action.work.unwrap_or_else(||self.cursor.reserve_work_demand());return Ok(self.action.poll(control,d,g,units,maximum_work_units,authority,|_,effect,authority,action|authority.drive(&mut NativeDefaultSlotOperation::Reserve(&mut self.cursor),effect,action)))}
   if !self.cursor.initialized()||self.action.tag==3{self.action.tag=3;let units=self.action.work.unwrap_or_else(||self.cursor.seed_work_demand());return Ok(self.action.poll(control,d,g,units,maximum_work_units,authority,|_,effect,authority,action|authority.drive(&mut NativeDefaultSlotOperation::Seed(&mut self.cursor),effect,action)))}
   let cursor=self.cursor.get_mut().ok_or_else(||ValueError::invariant("get_mut target absent"))?;
   if cursor.output_ready()||self.action.tag==5{self.action.tag=5;let units=match self.action.work{Some(work)=>work,None=>cursor.take_work_demand()?};return Ok(self.action.poll(control,d,g,units,maximum_work_units,authority,|_,effect,authority,action|authority.drive(&mut NativeDefaultSlotOperation::Publish(cursor,&mut self.output),effect,action)))}
   Ok(cursor.advance(control,body,maximum_work_units,child_grant(g,0)).map(|step|RetainedCloneStep::Progress(step.progress())))
  })();match result{Ok(turn)=>turn,Err(error)=>Complete(RetainedWorkOutcome{result:Err(error),work:0})}
 }
 semio_framework_value_native_receiver_protocol::native_owned_public_output!(T,invalid,ready);
 /// 📤️ Moves the resident output into a parent-owned slot inside the parent's prepared local work.
 pub fn take_local(&mut self,output:&mut Option<T>,g:RetainedCloneGrant)->RetainedWorkOutcome<Option<RetainedCloneProgress>>{let mut work=0;let result=(||{let d=self.take_demand()?;if !retained_clone_grant_admits(g,d){return Ok(None)}if output.is_some()||self.output.is_none(){return Err(invalid())}*output=self.output.take();work=1;Ok(Some(receipt(d)))})();RetainedWorkOutcome{result,work}}
 pub fn begin_close(&mut self){self.closing=true;if let Some(cursor)=self.cursor.get_mut(){cursor.begin_close();}}
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(invalid())}if self.cursor.reserved()||self.retired_header.is_some(){return backing_demands::<T>(&self.cursor,&self.retired_header,body)}if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<T>(false,&None,body)}if self.action.failure.is_some(){return self.action.failure_demand(&self.retirement,body)}if self.output.is_some(){return self.retirement.next_owner_demands::<T>(true,&None,body)}if self.authority.is_some(){return self.retirement.next_owner_demands::<Authority<T>>(true,&None,body)}Ok(Default::default())}
 pub fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.closing{return Err(invalid())}if let Some(cursor)=self.cursor.get(){if !cursor.terminal_is_empty(){return cursor.close_work_demand(body)}}if self.cursor.reserved()||self.retired_header.is_some(){return Ok(1)}if !self.retirement.is_empty(){return self.retirement.next_owner_work::<T>(false,&None,body)}if self.action.failure.is_some(){return self.action.failure_work(&self.retirement,body)}if self.output.is_some(){return self.retirement.next_owner_work::<T>(true,&None,body)}if self.authority.is_some(){return self.retirement.next_owner_work::<Authority<T>>(true,&None,body)}Ok(0)}
 pub fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{let g=effect.grant();if self.action.pending(){return Complete(RetainedWorkOutcome{result:Err(invalid().into()),work:0})}if self.terminal_is_empty(){return Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0})}let d=match self.close_demands(g.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})};if !retained_clone_grant_admits(g,d){return Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}
  if self.cursor.reserved()||self.retired_header.is_some(){return close_backing::<T>(&mut self.cursor,&mut self.retired_header,effect)}
  if !self.retirement.is_empty(){return self.retirement.step_granted(effect)}
  if self.action.failure.is_some(){return Complete(self.action.close_failure(&mut self.retirement,effect))}
  if self.output.is_some(){let turn=self.retirement.begin_granted(&mut self.output,effect);return Complete(RetainedWorkOutcome{result:turn.result.map(|s|s.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into),work:turn.work})}
  let turn=self.retirement.begin_granted(&mut self.authority,effect);Complete(RetainedWorkOutcome{result:turn.result.map(|s|s.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into),work:turn.work})
 }
 pub fn terminal_is_empty(&self)->bool{self.cursor.terminal_is_empty()&&self.retired_header.is_none()&&self.output.is_none()&&self.retirement.is_empty()&&self.action.terminal_is_empty()&&self.authority.is_none()}
}
impl<T:NativeDefault> Default for NativeDefaultSlot<T>{fn default()->Self{Self::new()}}
impl<T:NativeDefault> Drop for NativeDefaultSlot<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"native default slot requires funded closure");}}
