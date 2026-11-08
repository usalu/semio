//! 🔁️ Native map cloning delegates page ownership to the actual nested vector cursor.
use super::*;
use std::mem::{ManuallyDrop,MaybeUninit};
use semio_framework_value_native_clone_backing::{NativeRetainedCloneBacking,NativeRetainedCloneState,NativeSeedTemplate};
use crate::retained_clone::{OriginalEffectReply as Reply,OriginalEffectReplyComplete as Complete,OriginalEffectCallbackMode as Mode,OriginalEffectInvocationIdentity,OriginalWorkBindingIdentity,OriginalRetirementCloseLoan,RetainedCloneSelection,RetainedCursorRelease,RetainedCloneOwner,forward_original_value_reply};
type Pages<K,V>=Vec<Vec<(K,V)>>;
type PageCursor<K,V>=<Pages<K,V> as RetainedClone>::Cursor;
type Reference=(u64,usize,usize,usize);
#[derive(Clone,Copy,PartialEq,Eq)]
struct MapBinding{source:Reference,len:usize,work:OriginalWorkBindingIdentity}
#[derive(Clone,Copy,PartialEq,Eq)]
struct MapRoute{invocation:OriginalEffectInvocationIdentity,operation:u8,body:usize,maximum:u64,grant:RetainedCloneGrant,work:OriginalWorkBindingIdentity,original:Option<(crate::NativeAllocationIdentity,Option<u64>,OriginalWorkBindingIdentity)>}
/// 🗺️ Retains the original nested producer, output and reply until their funded handoffs finish.
pub struct RetainedOrderedMapCloneState<K:RetainedClone,V:RetainedClone>{
 child:ManuallyDrop<PageCursor<K,V>>,
 pages:ManuallyDrop<Option<Pages<K,V>>>,
 output:ManuallyDrop<Option<RetainedOrderedMap<K,V>>>,
 close:ManuallyDrop<RetainedCloneClose>,
 take_reply:ManuallyDrop<Option<Result<RetainedCloneStep,ValueError>>>,
 binding:Option<MapBinding>,
 route:Option<MapRoute>,
 closing:bool,
 spent:bool,
}

pub type RetainedOrderedMapCloneCursor<K:RetainedClone,V:RetainedClone>=NativeRetainedCloneBacking<RetainedOrderedMap<K,V>,RetainedOrderedMapCloneState<K,V>>;
unsafe impl<K:RetainedClone,V:RetainedClone> NativeRetainedCloneState<RetainedOrderedMap<K,V>> for RetainedOrderedMapCloneState<K,V>{
 const NATIVE_SEED:NativeSeedTemplate<Self>=unsafe{NativeSeedTemplate::fields(9,Self::seed_size,Self::seed_write)};
 fn has_pending_native_action(&self)->bool{self.route.is_some()||self.take_reply.is_some()}
}
impl<K:RetainedClone,V:RetainedClone> RetainedOrderedMapCloneState<K,V>{
 unsafe fn seed_size(target:*mut MaybeUninit<Self>,index:usize)->Option<usize>{
  fn extent<T>(_:*mut T)->usize{size_of::<T>()}let target=target.cast::<Self>();Some(match index{
   0=>extent(unsafe{std::ptr::addr_of_mut!((*target).child)}),
   1=>extent(unsafe{std::ptr::addr_of_mut!((*target).pages)}),
   2=>extent(unsafe{std::ptr::addr_of_mut!((*target).output)}),
   3=>extent(unsafe{std::ptr::addr_of_mut!((*target).close)}),
   4=>extent(unsafe{std::ptr::addr_of_mut!((*target).take_reply)}),
   5=>extent(unsafe{std::ptr::addr_of_mut!((*target).binding)}),
   6=>extent(unsafe{std::ptr::addr_of_mut!((*target).route)}),
   7|8=>size_of::<bool>(),
   _=>return None,
  })
 }
 unsafe fn seed_write(target:*mut MaybeUninit<Self>,index:usize)->bool{
  let target=target.cast::<Self>();unsafe{match index{
   0=>std::ptr::addr_of_mut!((*target).child).write(ManuallyDrop::new(<crate::retained_clone::VecCursor<Vec<(K,V)>>>::default())),
   1=>std::ptr::addr_of_mut!((*target).pages).write(ManuallyDrop::new(None)),
   2=>std::ptr::addr_of_mut!((*target).output).write(ManuallyDrop::new(None)),
   3=>std::ptr::addr_of_mut!((*target).close).write(ManuallyDrop::new(RetainedCloneClose::vacant())),
   4=>std::ptr::addr_of_mut!((*target).take_reply).write(ManuallyDrop::new(None)),
   5=>std::ptr::addr_of_mut!((*target).binding).write(None),
   6=>std::ptr::addr_of_mut!((*target).route).write(None),
   7=>std::ptr::addr_of_mut!((*target).closing).write(false),
   8=>std::ptr::addr_of_mut!((*target).spent).write(false),
   _=>return false,
  }}true
 }
}
impl<K:RetainedClone,V:RetainedClone> RetainedOrderedMapCloneState<K,V>{
 fn invalid()->ValueError{ValueError::invariant("map clone changed its original selected action")}
 fn binding_demand()->crate::RetirementDemand{crate::RetirementDemand{copy_bytes:size_of::<Option<MapBinding>>(),depth:1,..Default::default()}}
 fn publication_demand()->crate::RetirementDemand{crate::RetirementDemand{copy_bytes:size_of::<Option<Pages<K,V>>>()+size_of::<Option<RetainedOrderedMap<K,V>>>(),depth:1,..Default::default()}}
 fn source(&self,source:RetainedCloneRef<'_,RetainedOrderedMap<K,V>>)->Result<(),ValueError>{if self.binding.is_some_and(|binding|(binding.source,binding.len)!=(source.reference_identity(),source.get().len)){Err(Self::invalid())}else{Ok(())}}
 fn effect(&self,effect:&crate::RetainedAllocationEffect<'_>)->Result<(),ValueError>{if self.binding.is_some_and(|binding|!binding.work.matches_effect(effect)){Err(Self::invalid())}else{Ok(())}}
 fn depth(mut demand:crate::RetirementDemand)->Result<crate::RetirementDemand,ValueError>{demand.depth=demand.depth.checked_add(1).ok_or_else(Self::invalid)?;Ok(demand)}
 fn funded(effect:&crate::RetainedAllocationEffect<'_>,demand:crate::RetirementDemand,work:u64)->Result<bool,ValueError>{Ok(retained_clone_grant_admits(effect.grant(),demand)&&effect.remaining_work_units()?>=work)}
 fn key(effect:&crate::RetainedAllocationEffect<'_>,operation:u8,body:usize,maximum:u64,original:Option<&mut OriginalRetirementCloseLoan<'_>>,recover:bool)->Result<MapRoute,ValueError>{let invocation=effect.invocation().ok_or_else(Self::invalid)?;if invocation.mode()!=if recover{Mode::Recover}else{Mode::Receive}{return Err(Self::invalid())}let work=OriginalWorkBindingIdentity::from_effect(effect);let original=match original{Some(original)=>{let binding=original.original_work().binding_identity()?;if !binding.matches_effect(effect){return Err(Self::invalid())}Some((original.ledger_identity(),original.retirement_recipient_identity(),binding))},None=>None};Ok(MapRoute{invocation:invocation.identity(),operation,body,maximum,grant:effect.grant(),work,original})}
 fn selected(&self,source:RetainedCloneRef<'_,RetainedOrderedMap<K,V>>,body:usize,maximum:u64)->Result<RetainedCloneSelection,ValueError>{
  self.source(source)?;if self.closing||self.spent||self.route.is_some()||self.take_reply.is_some(){return Err(Self::invalid())}if maximum==0||self.output.is_some(){return Ok(RetainedCloneSelection::new(Default::default(),0))}
  let(demand,work)=if self.binding.is_none(){(Self::binding_demand(),1)}else if self.pages.is_some(){(Self::publication_demand(),1)}else if self.child.output_ready(){(Self::depth(self.child.take_demand()?)?,self.child.take_work_demand()?)}else{let selected=self.child.advance_selection(source.project(1,|map|&map.pages),body,maximum)?;(Self::depth(selected.demand())?,selected.work())};Ok(RetainedCloneSelection::new(demand,if work<=maximum{work}else{0}))
 }
 fn close_demands(&self,body:usize)->Result<crate::RetirementDemand,ValueError>{
  #[cfg(test)] selected_close_tests::record(self as*const Self as usize,body);
  if self.route.is_some_and(|route|route.operation<2||route.body!=body){return Err(Self::invalid())}if !self.closing{return Ok(Default::default())}
  if !self.child.close_started(){return Self::depth(self.child.begin_close_demand()?)}if !self.child.terminal_is_empty(){return Self::depth(self.child.close_demands(body)?)}
  if !self.close.is_empty(){return self.close.next_owner_demands::<Pages<K,V>>(false,&None,body)}if self.pages.is_some(){return self.close.next_owner_demands::<Pages<K,V>>(true,&None,body)}if self.output.is_some(){return self.close.next_owner_demands::<RetainedOrderedMap<K,V>>(true,&None,body)}
  Ok(if self.binding.is_some(){Self::binding_demand()}else{Default::default()})
 }
 fn close_dispatch<'r>(&mut self,body:usize,mut original:Option<&mut OriginalRetirementCloseLoan<'_>>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:Reply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,recover:bool)->Result<Complete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{
  self.effect(effect)?;if !self.closing{return Err(Self::invalid())}
  if recover{
   let selected=self.route.ok_or_else(Self::invalid)?;let key=Self::key(effect,selected.operation,body,0,original.as_deref_mut(),true)?;if selected!=key||selected.operation<2{return Err(Self::invalid())}
   let mut scope=reply.progress().scope(&mut self.route,key)?;
   let done=match key.operation{2=>{let grant=nested_retained_clone_grant(effect.grant())?;let remaining=effect.remaining_work_units()?;self.child.recover_begin_close_into(&mut effect.reborrow(grant,remaining)?,scope.borrow()?)},3=>{let grant=nested_retained_clone_grant(effect.grant())?;let remaining=effect.remaining_work_units()?;let mut nested=effect.reborrow(grant,remaining)?;match original{Some(original)=>self.child.recover_close_original_into(original,body,&mut nested,scope.borrow()?),None=>self.child.recover_close_into(body,&mut nested,scope.borrow()?)}},4=>match original{Some(original)=>self.close.recover_step_original_into(original,body,effect,scope.borrow()?),None=>self.close.recover_step_into(body,effect,scope.borrow()?)},_=>Err(Self::invalid())}?;drop(done);return Ok(scope.forwarded())
  }
  if self.route.is_some()||self.take_reply.is_some(){return Err(Self::invalid())}if self.terminal_is_empty(){return Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}
  let demand=self.close_demands(body)?;let work=self.close_work_demand(body)?;if !Self::funded(effect,demand,work)?{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
  let operation=if !self.child.close_started(){Some(2)}else if !self.child.terminal_is_empty(){Some(3)}else if !self.close.is_empty(){Some(4)}else{None};
  if let Some(operation)=operation{let key=Self::key(effect,operation,body,0,original.as_deref_mut(),false)?;let mut scope=reply.progress().scope(&mut self.route,key)?;let done=if operation==4{match original{Some(original)=>self.close.step_original_into(original,body,effect,scope.borrow()?),None=>self.close.step_into(body,effect,scope.borrow()?)}}else{let grant=nested_retained_clone_grant(effect.grant())?;let remaining=effect.remaining_work_units()?;let mut nested=effect.reborrow(grant,remaining)?;if operation==2{self.child.begin_close_into(&mut nested,scope.borrow()?)}else{match original{Some(original)=>self.child.close_original_into(original,body,&mut nested,scope.borrow()?),None=>self.child.close_into(body,&mut nested,scope.borrow()?)}}}?;drop(done);return Ok(scope.forwarded())}
  if self.pages.is_some(){let turn=self.close.begin_granted(&mut self.pages,effect);return Ok(reply.complete(turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into)))}
  if self.output.is_some(){let turn=self.close.begin_granted(&mut self.output,effect);return Ok(reply.complete(turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into)))}
  if !effect.performed(1)?{return Err(Self::invalid())}self.binding=None;Ok(reply.complete(Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))))
 }
}
impl<K:RetainedClone,V:RetainedClone> RetainedCloneCursor<RetainedOrderedMap<K,V>> for RetainedOrderedMapCloneState<K,V>{
 fn advance_selection(&self,source:RetainedCloneRef<'_,RetainedOrderedMap<K,V>>,body:usize,maximum:u64)->Result<RetainedCloneSelection,ValueError>{self.selected(source,body,maximum)}
 fn advance_demands(&self,source:RetainedCloneRef<'_,RetainedOrderedMap<K,V>>,body:usize,maximum:u64)->Result<crate::RetirementDemand,ValueError>{Ok(self.selected(source,body,maximum)?.demand())}
 fn advance_work_demand(&self,source:RetainedCloneRef<'_,RetainedOrderedMap<K,V>>,body:usize,maximum:u64)->Result<u64,ValueError>{Ok(self.selected(source,body,maximum)?.work())}
 fn advance_into<'r>(&mut self,source:RetainedCloneRef<'_,RetainedOrderedMap<K,V>>,body:usize,maximum:u64,effect:&mut crate::RetainedAllocationEffect<'_>,reply:Reply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<Complete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{
  self.effect(effect)?;let selected=self.selected(source,body,maximum)?;let demand=selected.demand();if self.output.is_some(){return Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}if selected.work()==0||!Self::funded(effect,demand,selected.work())?{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
  if self.binding.is_none(){if effect.work_source_turn().is_none(){return Err(Self::invalid())}if !effect.performed(1)?{return Err(Self::invalid())}self.binding=Some(MapBinding{source:source.reference_identity(),len:source.get().len,work:OriginalWorkBindingIdentity::from_effect(effect)});return Ok(reply.complete(Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))))}
  if self.pages.is_some(){if !effect.performed(1)?{return Err(Self::invalid())}*self.output=Some(RetainedOrderedMap{pages:self.pages.take().expect("selected native pages"),len:self.binding.expect("selected map binding").len});return Ok(reply.complete(Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))))}
  let operation=u8::from(self.child.output_ready());let key=Self::key(effect,operation,body,maximum,None,false)?;let mut scope=reply.progress().scope(&mut self.route,key)?;let grant=nested_retained_clone_grant(effect.grant())?;let remaining=effect.remaining_work_units()?;let mut nested=effect.reborrow(grant,remaining)?;
  if operation==1{let done=self.child.take(&mut self.pages,&mut nested,Reply::borrow(&mut self.take_reply)?)?;drop(done);let done=forward_original_value_reply(&mut self.take_reply,scope.borrow()?)?;drop(done)}else{let done=self.child.advance_into(source.project(1,|map|&map.pages),body,maximum,&mut nested,scope.borrow()?)?;drop(done)}Ok(scope.forwarded())
 }
 fn recover_advance_into<'r>(&mut self,source:RetainedCloneRef<'_,RetainedOrderedMap<K,V>>,body:usize,maximum:u64,effect:&mut crate::RetainedAllocationEffect<'_>,reply:Reply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<Complete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{
  self.source(source)?;self.effect(effect)?;let selected=self.route.ok_or_else(Self::invalid)?;let key=Self::key(effect,selected.operation,body,maximum,None,true)?;if selected!=key||selected.operation>1{return Err(Self::invalid())}
  let mut scope=reply.progress().scope(&mut self.route,key)?;let grant=nested_retained_clone_grant(effect.grant())?;let remaining=effect.remaining_work_units()?;let mut nested=effect.reborrow(grant,remaining)?;if key.operation==1{if self.take_reply.is_none(){let done=self.child.recover_take(&mut self.pages,&mut nested,Reply::borrow(&mut self.take_reply)?)?;drop(done)}let done=forward_original_value_reply(&mut self.take_reply,scope.borrow()?)?;drop(done)}else{let done=self.child.recover_advance_into(source.project(1,|map|&map.pages),body,maximum,&mut nested,scope.borrow()?)?;drop(done)}Ok(scope.forwarded())
 }
 fn output_ready(&self)->bool{self.output.is_some()}
 fn take_demand(&self)->Result<crate::RetirementDemand,ValueError>{crate::retained_clone::retained_clone_cursor_take_demand::<RetainedOrderedMap<K,V>,bool>()}
 fn take_work_demand(&self)->Result<u64,ValueError>{Ok(u64::from(self.output_ready()))}
 fn take<'r>(&mut self,output:&mut Option<RetainedOrderedMap<K,V>>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:Reply<'r,RetainedCloneStep,ValueError>)->Result<Complete<'r,RetainedCloneStep,ValueError>,ValueError>{self.effect(effect)?;crate::retained_clone::take_retained_clone_cursor_output!(self.output_ready(),self.output,output,self.spent,true,effect,reply,RetainedOrderedMap<K,V>,bool)}
 fn begin_close_demand(&self)->Result<crate::RetirementDemand,ValueError>{if self.route.is_some()||self.take_reply.is_some(){return Err(Self::invalid())}Ok(crate::retained_clone::retained_close_start_demand(self.closing))}
 fn begin_close_work_demand(&self)->Result<u64,ValueError>{Ok(crate::retained_clone::retained_close_start_work_demand(self.closing))}
 fn begin_close_paid(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<RetainedCloneProgress,ValueError>{if self.route.is_some()||self.take_reply.is_some(){return crate::RetainedWorkOutcome{result:Err(Self::invalid()),work:0}}if let Err(error)=self.effect(effect){return crate::RetainedWorkOutcome{result:Err(error),work:0}}crate::retained_clone::start_retained_close(&mut self.closing,effect)}
 fn begin_close_into<'r>(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,reply:Reply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<Complete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{let turn=self.begin_close_paid(effect);Ok(reply.complete(turn.result.map(RetainedCloneStep::Progress).map_err(Into::into)))}
 fn recover_begin_close_into<'r>(&mut self,_:&mut crate::RetainedAllocationEffect<'_>,reply:Reply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<Complete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{Ok(reply.pending())}
 fn close_started(&self)->bool{self.closing}
 fn close_demands(&self,body:usize)->Result<crate::RetirementDemand,ValueError>{Self::close_demands(self,body)}
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if self.route.is_some_and(|route|route.operation<2||route.body!=body){return Err(Self::invalid())}if !self.closing||self.terminal_is_empty(){return Ok(0)}if !self.child.close_started(){return self.child.begin_close_work_demand()}if !self.child.terminal_is_empty(){return self.child.close_work_demand(body)}if !self.close.is_empty(){return self.close.next_owner_work::<Pages<K,V>>(false,&None,body)}if self.pages.is_some(){return self.close.next_owner_work::<Pages<K,V>>(true,&None,body)}if self.output.is_some(){return self.close.next_owner_work::<RetainedOrderedMap<K,V>>(true,&None,body)}Ok(1)}
 fn close_into<'r>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:Reply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<Complete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.close_dispatch(body,None,effect,reply,false)}
 fn recover_close_into<'r>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:Reply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<Complete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.close_dispatch(body,None,effect,reply,true)}
 fn close_original_into<'r>(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:Reply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<Complete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.close_dispatch(body,Some(original),effect,reply,false)}
 fn recover_close_original_into<'r>(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:Reply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<Complete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.close_dispatch(body,Some(original),effect,reply,true)}
 fn requires_original_birth(&self)->bool{if !self.child.terminal_is_empty(){self.child.requires_original_birth()}else{self.close.requires_original_birth()}}
 fn requires_original_control(&self)->bool{if !self.child.terminal_is_empty(){self.child.requires_original_control()}else{self.close.requires_original_control()}}
 fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->crate::OriginalRetirementPoll{if !self.child.terminal_is_empty(){self.child.close_original_birth_step(original,grant)}else{self.close.step_original_birth(original,grant)}}
 fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.depth)}
 fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.copy_bytes)}
 fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.close_demands(body)?.capacity_bytes)}
 fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.release_bytes)}
 fn terminal_is_empty(&self)->bool{self.closing&&self.child.terminal_is_empty()&&self.pages.is_none()&&self.output.is_none()&&self.close.is_empty()&&self.take_reply.is_none()&&self.binding.is_none()&&self.route.is_none()}
 fn native_terminal_release(&self)->Option<RetainedCursorRelease<Self>>{self.terminal_is_empty().then(||unsafe{RetainedCursorRelease::native(|cursor:&Self|cursor.terminal_is_empty())})}
}
impl<K:RetainedClone,V:RetainedClone> RetainedClone for RetainedOrderedMap<K,V>{type Cursor=RetainedOrderedMapCloneCursor<K,V>;fn retained_clone_cursor()->Self::Cursor{Default::default()}}
impl<K:RetainedClone,V:RetainedClone> Drop for RetainedOrderedMapCloneState<K,V>{fn drop(&mut self){if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.child);ManuallyDrop::drop(&mut self.pages);ManuallyDrop::drop(&mut self.output);ManuallyDrop::drop(&mut self.close);ManuallyDrop::drop(&mut self.take_reply)}}else if !std::thread::panicking(){panic!("native map clone requires paid original retirement")}}}
#[cfg(test)]
#[path="🧪️tests/🎯️dispatch/🦀️.rs"]
mod selected_close_tests;
