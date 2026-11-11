//! 🔗️ The defining shared controlled owner is expanded at its retirement boundary.
#[macro_export]
macro_rules! shared_controlled_owner {() => {

pub use semio_framework_value_allocation_return::original;

use super::{RetireOwned,RetirementCursor,RetirementStep,controlled::ControlledRetirement};
use crate::{ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use std::{sync::Arc,mem::ManuallyDrop};




#[derive(Clone,Copy,PartialEq,Eq)]
struct SharedCloseRoute{boundary:crate::OriginalWorkActionBoundary,body:usize,invocation:crate::OriginalEffectInvocationIdentity,quote:crate::RetirementDemand,work:u64,original:bool,original_loan:Option<(crate::NativeAllocationIdentity,Option<u64>)>}
/// 🔗️ Owns an issued shared alias through election, payload handback and controlled retirement.
pub struct SharedControlledRetirement<T:RetireOwned+Sync>{
 source:ManuallyDrop<Option<original::SharedOriginalStorage<T>>>,elected:ManuallyDrop<Option<original::SharedOriginal<T>>>,
 value:ManuallyDrop<Option<T>>,owned:ManuallyDrop<Option<ControlledRetirement<T>>>,route:Option<SharedCloseRoute>,reply:crate::ReplyStash<crate::OriginalRetirementFailure>,
}
impl<T:RetireOwned+Sync> SharedControlledRetirement<T>{
 pub fn new(source:original::SharedOriginalStorage<T>)->Self{Self{source:ManuallyDrop::new(Some(source)),elected:ManuallyDrop::new(None),value:ManuallyDrop::new(None),owned:ManuallyDrop::new(None),route:None,reply:Default::default()}}
 /// 👁️ Borrows only the actual still-live issued alias.
 pub fn source(&self)->Option<&original::SharedOriginalStorage<T>>{self.source.as_ref().filter(|source|!source.terminal_is_empty())}
 pub fn terminal_is_empty(&self)->bool{self.source.is_none()&&self.elected.is_none()&&self.value.is_none()&&self.owned.is_none()&&self.route.is_none()}
 fn header_demand<B>()->crate::RetirementDemand{crate::RetirementDemand{copy_bytes:size_of::<Option<B>>(),depth:1,..Default::default()}}
 pub fn next_demand(&self,body:usize)->Result<crate::RetirementDemand,ValueError>{
  if let Some(route)=self.route{return Ok(route.quote)}
  if let Some(source)=self.source.as_ref(){return if source.terminal_is_empty(){Ok(Self::header_demand::<original::SharedOriginalStorage<T>>())}else{source.retire_into_demand()}}
  if let Some(original)=self.elected.as_ref(){return if original.terminal_is_empty(){Ok(Self::header_demand::<original::SharedOriginal<T>>())}else{original.take_value_demand()}}
  if self.value.is_some(){return Ok(crate::RetirementDemand{copy_bytes:size_of::<Option<T>>().checked_add(size_of::<Option<ControlledRetirement<T>>>()).ok_or_else(||ValueError::invariant("shared payload placement extent overflow"))?,depth:1,..Default::default()})}
  if let Some(owner)=self.owned.as_ref(){if owner.terminal_is_empty(){if crate::ErasedSnapshotRetirement::native_header_release(owner).is_none(){return Err(ValueError::invariant("shared controlled owner retains native obligations"))}return Ok(Self::header_demand::<ControlledRetirement<T>>())}let mut d=owner.next_demand(body)?;d.depth=d.depth.checked_add(1).ok_or_else(||ValueError::invariant("shared controlled depth overflow"))?;return Ok(d)}
  Ok(Default::default())
 }
 pub fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{if let Some(route)=self.route{return Ok(route.work)}if let Some(source)=self.source.as_ref(){return if source.terminal_is_empty(){Ok(1)}else{source.retire_work_demand()}}if let Some(original)=self.elected.as_ref(){return if original.terminal_is_empty(){Ok(1)}else{original.take_value_work_demand()}}if self.value.is_some(){return Ok(1)}match self.owned.as_ref(){Some(owner)if owner.terminal_is_empty()=>Ok(1),Some(owner)=>owner.next_computational_work(body),None=>Ok(0)}}
 pub fn requires_original_birth(&self)->bool{self.route.is_none()&&self.owned.as_ref().is_some_and(|owner|!owner.terminal_is_empty()&&owner.requires_original_birth())}
 pub fn step_original_birth(&mut self,original:&mut semio_framework_value_retirement_contract::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->crate::OriginalRetirementPoll{let Some(owner)=self.owned.as_mut()else{return crate::RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"shared owner has no selected original birth").into()),work:0})};if !owner.requires_original_birth(){return crate::RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"shared owner has no selected original birth").into()),work:0})}let Some(depth)=grant.maximum_depth.checked_sub(1)else{return crate::RetainedWorkPoll::Pending{work:0}};owner.step_original_birth(original,RetainedCloneGrant{maximum_depth:depth,..grant})}
 pub fn requires_original_control(&self)->bool{if let Some(route)=self.route{return route.original}self.owned.as_ref().is_some_and(|owner|!owner.terminal_is_empty()&&crate::ErasedSnapshotRetirement::requires_original_control(owner))}
 fn drive<'reply>(&mut self,mut original:Option<&mut semio_framework_value_retirement_contract::OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,recover:bool)->Result<crate::OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{
  if self.terminal_is_empty(){return Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}
  let grant=effect.grant();
  if self.route.is_none(){if recover{return Err(ValueError::invariant("shared owner has no original child to recover"))}let demand=self.next_demand(body)?;let work=self.next_computational_work(body)?;if !crate::retained_clone_grant_admits(grant,demand)||effect.remaining_work_units()?<work{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
   if let Some(source)=self.source.as_mut(){if !source.terminal_is_empty(){return source.retire_into(&mut self.elected,effect,reply.progress())}if !effect.performed(1)?{return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"shared retirement exceeded its original Work").into())))}self.source.take();return Ok(reply.complete(Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))))}
   if let Some(elected)=self.elected.as_mut(){if !elected.terminal_is_empty(){return elected.take_value_into(&mut self.value,effect,reply.progress())}if !effect.performed(1)?{return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"shared retirement exceeded its original Work").into())))}self.elected.take();return Ok(reply.complete(Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))))}
   if self.value.is_some(){if !effect.performed(1)?{return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"shared retirement exceeded its original Work").into())))}*self.owned=Some(ControlledRetirement::retain(self.value.take().expect("funded shared payload retained")));return Ok(reply.complete(Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))))}
   if let Some(owner)=self.owned.as_ref(){if owner.terminal_is_empty(){if crate::ErasedSnapshotRetirement::native_header_release(owner).is_none(){return Err(ValueError::invariant("shared controlled owner retains native obligations"))}if !effect.performed(1)?{return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"shared retirement exceeded its original Work").into())))}self.owned.take();return Ok(reply.complete(Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))))}}
  }
  let key=effect.invocation().ok_or_else(||ValueError::invariant("shared close requires its original invocation"))?.identity();let original_loan=original.as_ref().map(|loan|(loan.ledger_identity(),loan.retirement_recipient_identity()));let route=match self.route{Some(route)=>{if !recover||route.invocation!=key||route.original!=original.is_some()||route.original_loan!=original_loan{return Err(ValueError::invariant("shared close changed its original child"))}if !crate::retained_clone_grant_admits(grant,route.quote)||u128::from(effect.remaining_work_units()?)<u128::from(route.work).saturating_sub(effect.performed_since_action(route.boundary)?){return Ok(reply.pending())}route},None=>SharedCloseRoute{boundary:effect.action_boundary(),body,invocation:key,quote:self.next_demand(body)?,work:self.next_computational_work(body)?,original:original.is_some(),original_loan}};let body=route.body;
  let depth=grant.maximum_depth.checked_sub(1).ok_or_else(||ValueError::invariant("shared controlled child depth absent"))?;let remaining=effect.remaining_work_units()?;let mut child=effect.reborrow(RetainedCloneGrant{maximum_depth:depth,..grant},remaining)?;let owner=self.owned.as_mut().ok_or_else(||ValueError::invariant("shared controlled child absent"))?;let mut scope=reply.progress().scope(&mut self.route,route)?;let child_reply=scope.borrow()?;match(original.as_deref_mut(),recover){(Some(original),true)=>owner.recover_step_original_into(original,body,&mut child,child_reply)?,(Some(original),false)=>owner.step_original_into(original,body,&mut child,child_reply)?,(None,true)=>owner.recover_step_into(body,&mut child,child_reply)?,(None,false)=>owner.step_into(body,&mut child,child_reply)?};Ok(scope.forwarded())
 }
 pub fn step_into<'reply>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.drive(None,body,effect,reply,false)}
 pub fn recover_step_into<'reply>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.drive(None,body,effect,reply,true)}
 pub fn step_original_into<'reply>(&mut self,original:&mut semio_framework_value_retirement_contract::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.drive(Some(original),body,effect,reply,false)}
 pub fn recover_step_original_into<'reply>(&mut self,original:&mut semio_framework_value_retirement_contract::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.drive(Some(original),body,effect,reply,true)}
}
impl<T:RetireOwned+Sync> crate::ErasedSnapshotRetirement for SharedControlledRetirement<T>{
 fn native_header_release(&self)->Option<crate::RetirementHeaderRelease>{self.terminal_is_empty().then(||unsafe{crate::RetirementHeaderRelease::native(self,0,0)})}
 fn close_into<'r>(&mut self,body:usize,e:&mut crate::RetainedAllocationEffect<'_>,r:crate::OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.step_into(body,e,r)}
 fn recover_close_into<'r>(&mut self,body:usize,e:&mut crate::RetainedAllocationEffect<'_>,r:crate::OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.recover_step_into(body,e,r)}
 fn close_original_into<'r>(&mut self,o:&mut semio_framework_value_retirement_contract::OriginalRetirementCloseLoan<'_>,body:usize,e:&mut crate::RetainedAllocationEffect<'_>,r:crate::OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.step_original_into(o,body,e,r)}
 fn recover_close_original_into<'r>(&mut self,o:&mut semio_framework_value_retirement_contract::OriginalRetirementCloseLoan<'_>,body:usize,e:&mut crate::RetainedAllocationEffect<'_>,r:crate::OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.recover_step_original_into(o,body,e,r)}
 fn requires_original_birth(&self)->bool{Self::requires_original_birth(self)}
 fn close_original_birth_step(&mut self,o:&mut semio_framework_value_retirement_contract::OriginalRetirementCloseLoan<'_>,g:RetainedCloneGrant)->crate::OriginalRetirementPoll{self.step_original_birth(o,g)}
 fn requires_original_control(&self)->bool{Self::requires_original_control(self)}
 fn next_computational_work(&self,b:usize)->Result<u64,ValueError>{Self::next_computational_work(self,b)}
 fn next_demand(&self,b:usize)->Result<crate::RetirementDemand,ValueError>{Self::next_demand(self,b)}
 fn resident_reply(&mut self)->Option<&mut crate::ReplyStash<crate::OriginalRetirementFailure>>{Some(&mut self.reply)}
 fn close_step(&mut self,body:usize,_:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{crate::RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"shared retirement requires its borrowed producer").into()),work:0})}
 fn terminal_is_empty(&self)->bool{Self::terminal_is_empty(self)}
 fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.next_demand(0)?.copy_bytes)}
 fn next_capacity_byte_demand(&self,b:usize)->Result<usize,ValueError>{Ok(self.next_demand(b)?.capacity_bytes)}
 fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.next_demand(0)?.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.next_demand(0)?.depth)}
}
impl<T:RetireOwned+Sync> Drop for SharedControlledRetirement<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"shared retirement retains an issued alias or original payload");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.source);ManuallyDrop::drop(&mut self.elected);ManuallyDrop::drop(&mut self.value);ManuallyDrop::drop(&mut self.owned)}}}}
/// 🧳️ Holds the child reply and invocation while mapping the actual shared owner into a typed cursor.
pub struct SharedControlledCursor<T:RetireOwned+Sync>{owner:SharedControlledRetirement<T>,reply:ManuallyDrop<Option<Result<RetainedCloneStep,crate::OriginalRetirementFailure>>>,route:Option<SharedCloseRoute>}
impl<T:RetireOwned+Sync> SharedControlledCursor<T>{
 fn new(owner:SharedControlledRetirement<T>)->Self{Self{owner,reply:ManuallyDrop::new(None),route:None}}
 fn drive<'reply>(&mut self,mut original:Option<&mut semio_framework_value_retirement_contract::OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::OriginalEffectReply<'reply,RetirementStep,crate::OriginalRetirementFailure>,recover:bool)->Result<crate::OriginalEffectReplyComplete<'reply,RetirementStep,crate::OriginalRetirementFailure>,ValueError>{
  if self.owner.terminal_is_empty()&&self.reply.is_none()&&self.route.is_none(){return Ok(reply.complete(Ok(RetirementStep::Complete(0))))}
  let grant=effect.grant();let key=effect.invocation().ok_or_else(||ValueError::invariant("shared cursor requires its original invocation"))?.identity();let original_loan=original.as_ref().map(|loan|(loan.ledger_identity(),loan.retirement_recipient_identity()));let route=match self.route{Some(route)=>{if !recover||route.invocation!=key||route.original!=original.is_some()||route.original_loan!=original_loan{return Err(ValueError::invariant("shared cursor changed its original child"))}if !crate::retained_clone_grant_admits(grant,route.quote)||u128::from(effect.remaining_work_units()?)<u128::from(route.work).saturating_sub(effect.performed_since_action(route.boundary)?){return Ok(reply.pending())}route},None=>{if recover{return Err(ValueError::invariant("shared cursor has no original action to recover"))}SharedCloseRoute{boundary:effect.action_boundary(),body,invocation:key,quote:self.owner.next_demand(body)?,work:self.owner.next_computational_work(body)?,original:original.is_some(),original_loan}}};let body=route.body;let before=effect.performed_boundary();let mut scope=reply.scope(&mut self.route,route)?;
  if self.reply.is_none(){let child=crate::OriginalEffectReply::borrow(&mut self.reply)?;match(original.as_deref_mut(),recover){(Some(o),true)=>self.owner.recover_step_original_into(o,body,effect,child)?,(Some(o),false)=>self.owner.step_original_into(o,body,effect,child)?,(None,true)=>self.owner.recover_step_into(body,effect,child)?,(None,false)=>self.owner.step_into(body,effect,child)?};}
  let Some(result)=self.reply.take()else{return Ok(scope.forwarded())};let work=effect.performed_since(before)?;let result=result.map(|step|match step{RetainedCloneStep::Complete(progress)if progress==RetainedCloneProgress::default()&&self.owner.terminal_is_empty()=>RetirementStep::Complete(work),other=>RetirementStep::Progress(other.progress(),work)});scope.borrow()?.complete(result);Ok(scope.forwarded())
 }
}
impl<T:RetireOwned+Sync> RetirementCursor for SharedControlledCursor<T>{
 fn close_into<'r>(&mut self,body:usize,e:&mut crate::RetainedAllocationEffect<'_>,r:crate::OriginalEffectReply<'r,RetirementStep,crate::OriginalRetirementFailure>)->Result<crate::OriginalEffectReplyComplete<'r,RetirementStep,crate::OriginalRetirementFailure>,ValueError>{self.drive(None,body,e,r,false)}
 fn recover_close_into<'r>(&mut self,body:usize,e:&mut crate::RetainedAllocationEffect<'_>,r:crate::OriginalEffectReply<'r,RetirementStep,crate::OriginalRetirementFailure>)->Result<crate::OriginalEffectReplyComplete<'r,RetirementStep,crate::OriginalRetirementFailure>,ValueError>{self.drive(None,body,e,r,true)}
 fn close_original_into<'r>(&mut self,o:&mut semio_framework_value_retirement_contract::OriginalRetirementCloseLoan<'_>,body:usize,e:&mut crate::RetainedAllocationEffect<'_>,r:crate::OriginalEffectReply<'r,RetirementStep,crate::OriginalRetirementFailure>)->Result<crate::OriginalEffectReplyComplete<'r,RetirementStep,crate::OriginalRetirementFailure>,ValueError>{self.drive(Some(o),body,e,r,false)}
 fn recover_close_original_into<'r>(&mut self,o:&mut semio_framework_value_retirement_contract::OriginalRetirementCloseLoan<'_>,body:usize,e:&mut crate::RetainedAllocationEffect<'_>,r:crate::OriginalEffectReply<'r,RetirementStep,crate::OriginalRetirementFailure>)->Result<crate::OriginalEffectReplyComplete<'r,RetirementStep,crate::OriginalRetirementFailure>,ValueError>{self.drive(Some(o),body,e,r,true)}
 fn requires_original_birth(&self)->bool{self.route.is_none()&&self.owner.requires_original_birth()}
 fn close_original_birth_step(&mut self,o:&mut semio_framework_value_retirement_contract::OriginalRetirementCloseLoan<'_>,g:RetainedCloneGrant)->RetirementStep{super::erased_cursor_turn(self.owner.step_original_birth(o,g))}
 fn requires_original_control(&self)->bool{self.route.map_or_else(||self.owner.requires_original_control(),|route|route.original)}
 fn next_computational_work(&self,b:usize)->Result<u64,ValueError>{self.route.map_or_else(||self.owner.next_computational_work(b),|route|Ok(route.work))}
 fn next_demand(&self,b:usize)->Result<crate::RetirementDemand,ValueError>{if let Some(route)=self.route{return Ok(route.quote)}self.owner.next_demand(b)}
 fn terminal_is_empty(&self)->bool{self.owner.terminal_is_empty()&&self.reply.is_none()&&self.route.is_none()}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.next_demand(0)?.copy_bytes)}
 fn allows_admitted_narrow_work(&self)->bool{true}
 fn next_birth_bytes(&self,b:usize)->Option<usize>{self.next_demand(b).ok().map(|d|d.capacity_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.next_demand(0).ok().map(|d|d.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.next_demand(0)?.depth)}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl<T:RetireOwned+Sync> Drop for SharedControlledCursor<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"shared cursor retains its original reply");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.reply)}}}}
impl<T:RetireOwned+Sync> RetireOwned for original::SharedOriginalStorage<T>{
 type RetirementCursor=SharedControlledCursor<T>;fn retirement_seed(self)->Self::RetirementCursor{SharedControlledCursor::new(SharedControlledRetirement::new(self))}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<Self::RetirementCursor>())}
 fn controlled_retirement_supported()->bool{crate::retirement::support::controlled_retirement_supported::<Self>()}fn controlled_retirement_supported_in(path:&crate::retirement::RetirementSupport<'_>)->bool{path.supports::<T>()}
}
impl<T:RetireOwned+Sync> RetireOwned for SharedControlledRetirement<T>{
 type RetirementCursor=SharedControlledCursor<T>;fn retirement_seed(self)->Self::RetirementCursor{SharedControlledCursor::new(self)}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<Self::RetirementCursor>())}
 fn controlled_retirement_supported()->bool{crate::retirement::support::controlled_retirement_supported::<Self>()}fn controlled_retirement_supported_in(path:&crate::retirement::RetirementSupport<'_>)->bool{path.supports::<T>()}
}
/// 📏️ Quotes admission of the exact issued alias and its resident controlled owner.
pub fn shared_retirement_birth_demand<T:RetireOwned+Sync>()->crate::RetirementDemand{crate::RetirementDemand{copy_bytes:size_of::<Option<original::SharedOriginalStorage<T>>>()+size_of::<SharedControlledRetirement<T>>()+size_of::<Option<Box<dyn crate::ErasedSnapshotRetirement>>>(),capacity_bytes:size_of::<SharedControlledRetirement<T>>(),depth:1,..Default::default()}}
pub const fn shared_retirement_birth_bytes<T:RetireOwned+Sync>()->usize{size_of::<SharedControlledRetirement<T>>()}
/// 🪑️ Allocates the actual controlled owner and publishes it directly into caller custody.
pub fn admit_shared_retirement_into<'reply,T:RetireOwned+Sync>(source:&mut Option<original::SharedOriginalStorage<T>>,output:&mut Option<Box<dyn crate::ErasedSnapshotRetirement>>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{
 if output.is_some(){return Ok(reply.complete(Err(ValueError::invariant("shared retirement destination occupied").into())))}if source.is_none(){return Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}let demand=shared_retirement_birth_demand::<T>();if !crate::retained_clone_grant_admits(effect.grant(),demand)||effect.remaining_work_units()?<1{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}if effect.actual().is_some(){return Ok(reply.complete(Err(ValueError::invariant("shared retirement allocator marker occupied").into())))}if !effect.performed(1)?{return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"shared retirement exceeded its original Work").into())))}
 *effect.actual()=Some(0);let layout=std::alloc::Layout::new::<SharedControlledRetirement<T>>();let Some(pointer)=std::ptr::NonNull::new(unsafe{std::alloc::alloc(layout)}.cast::<SharedControlledRetirement<T>>())else{return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"shared retirement allocation failed").into())))};*effect.actual()=Some(layout.size());let owner=unsafe{pointer.as_ptr().write(SharedControlledRetirement::new(source.take().expect("funded issued alias retained")));Box::from_raw(pointer.as_ptr())};*output=Some(owner);Ok(reply.complete(Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:layout.size(),..Default::default()}))))
}
/// 📦️ Exact original shared backing demand used by the admitted shared retirement owners.
pub fn shared_retirement_allocation_bytes<T>()->usize {std::alloc::Layout::new::<[usize;2]>().extend(std::alloc::Layout::new::<T>()).expect("shared Arc allocation layout").0.pad_to_align().size()}
pub fn arc_bytes<T>()->usize {shared_retirement_allocation_bytes::<T>()}



pub use semio_framework_value_allocation_return::{admit_shared_allocation,admit_shared_allocation_from,shared_allocation_bytes,shared_uninitialized_demand,shared_allocation_value_demand,shared_allocation_slot_demand,try_uninitialized_arc,SharedAllocation};



}}
